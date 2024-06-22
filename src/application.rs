use std::borrow::Cow;

use futures::stream::FuturesOrdered;
use futures::TryStreamExt;
use rust_decimal::Decimal;

use crate::application::services::{
    AcknowledgementService, CheckAddressExistsService, CheckProductCodeExistsService,
    ProductPriceService,
};
use crate::models::entities::order::{PricedOrder, ValidatedOrder};
use crate::models::entities::order_line::{PricedOrderLine, ValidatedOrderLine};
use crate::models::events::{
    BillableOrderPlaced, OrderAcknowledgementSent, OrderPlaced, PlaceOrderEvent,
};
use crate::models::unvalidated::unvalidated_order::UnvalidatedOrder;
use crate::models::value_objects::customer_info::CustomerInfo;
use crate::models::value_objects::html_string::HtmlString;
use crate::models::value_objects::order_acknowledgment::OrderAcknowledgment;
use crate::models::value_objects::order_id::OrderId;
use crate::models::value_objects::price::Price;
use crate::models::value_objects::send_result::SendResult;

pub mod services;

async fn place_order(
    order: UnvalidatedOrder,
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    acknowledgment_service: impl AcknowledgementService,
    price_service: impl ProductPriceService,
) -> Result<(), Cow<'static, str>> {
    let validated_order =
        validate_order(check_product_code_service, check_address_service, order).await?;
    let priced_order = price_order(price_service, validated_order).await?;

    let acknowledge_event = acknowledge_order(acknowledgment_service, priced_order).await?;

    Ok(())
}

async fn validate_order(
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    order: UnvalidatedOrder,
) -> Result<ValidatedOrder, Cow<'static, str>> {
    let order_id = OrderId::create(order.order_id).unwrap();
    let customer_info = CustomerInfo::create(order.customer_info).unwrap();
    // let shipping_address = ShippingAddress::create(order).unwrap();
    // let billing_address = BillingAddress::create(order.shipping_address).unwrap();

    let lines = order
        .lines
        .into_iter()
        .map(ValidatedOrderLine::create)
        .collect::<Result<Vec<_>, _>>()?;

    for line in &lines {
        if !check_product_code_service
            .check(line.get_product_code_ref())
            .await
        {
            return Err(
                format!("Product code {} wasn't found", line.get_product_code_ref()).into(),
            );
        }
    }
    let validated_order = ValidatedOrder::create(order_id, customer_info, lines);

    Ok(validated_order)
}

async fn price_order(
    price_service: impl ProductPriceService,
    order: ValidatedOrder,
) -> Result<PricedOrder, &'static str> {
    let futures = order
        .get_order_lines_ref()
        .into_iter()
        .map(|line| async {
            let price_value = price_service.get(line.get_product_code_ref()).await?;
            let price = Price::create(price_value)?;
            let result = PricedOrderLine::create(line, price);

            Result::<PricedOrderLine, &'static str>::Ok(result)
        })
        .collect::<FuturesOrdered<_>>();

    let lines = futures.try_collect::<Vec<_>>().await?;

    let result = PricedOrder::create(order.order_id.clone(), order.customer_info.clone(), lines);

    Ok(result)
}

async fn acknowledge_order(
    acknowledgment_service: impl AcknowledgementService,
    order: PricedOrder,
) -> Result<Option<OrderAcknowledgementSent>, &'static str> {
    let letter = create_acknowledgment_letter(&order).await;

    let acknowledgment =
        OrderAcknowledgment::create(order.customer_info.email_address.clone(), letter);

    let result = match acknowledgment_service.send(&acknowledgment).await? {
        SendResult::Sent => Some(OrderAcknowledgementSent::create(
            order.order_id.clone(),
            acknowledgment,
        )),
        SendResult::NotSent => None,
    };

    Ok(result)
}

async fn create_acknowledgment_letter(order: &PricedOrder) -> HtmlString {
    HtmlString::create(format!("Hello!"))
}

fn create_billing_event(priced_order: &PricedOrder) -> Option<BillableOrderPlaced> {
    let billing_amount = priced_order.amount_to_bill;

    if billing_amount.0 > Decimal::from(0) {
        return Some(BillableOrderPlaced::create(
            priced_order.order_id.clone(),
            priced_order.amount_to_bill,
        ));
    }

    None
}

async fn create_events(
    placed_order: OrderPlaced,
    acknowledgement_sent: Option<OrderAcknowledgementSent>,
    billable_order_placed: Option<BillableOrderPlaced>,
) -> Vec<PlaceOrderEvent> {
    let place_order_event = PlaceOrderEvent::OrderPlaced(placed_order);
    let acknowledgement_sent_event =
        acknowledgement_sent.map(|event| PlaceOrderEvent::AcknowledgementSent(event));
    let billable_order_placed_event =
        billable_order_placed.map(|event| PlaceOrderEvent::BillableOrderPlaced(event));

    let result = [
        Some(place_order_event),
        acknowledgement_sent_event,
        billable_order_placed_event,
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect::<Vec<_>>();

    result
}
