use domain::entities::order::{PricedOrder, ValidatedOrder};
use domain::entities::order_line::{PricedOrderLine, ValidatedOrderLine};
use domain::events::{BillableOrderPlaced, OrderAcknowledgementSent, OrderPlaced, PlaceOrderEvent};
use domain::services::{
    AcknowledgementService, CheckAddressExistsService, CheckProductCodeExistsService,
    ProductPriceService,
};
use domain::unvalidated::unvalidated_order::UnvalidatedOrder;
use domain::value_objects::address::Address;
use domain::value_objects::amount_to_bill::AmountToBill;
use domain::value_objects::customer_info::CustomerInfo;
use domain::value_objects::html_string::HtmlString;
use domain::value_objects::order_acknowledgment::OrderAcknowledgment;
use domain::value_objects::order_id::OrderId;
use domain::value_objects::price::Price;
use domain::value_objects::send_result::SendResult;
use futures::stream::FuturesOrdered;
use futures::TryStreamExt;
use std::borrow::Cow;

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

    // let events = create_events(, acknowledge_event, )
    Ok(())
}

async fn validate_order(
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    order: UnvalidatedOrder,
) -> Result<ValidatedOrder, Cow<'static, str>> {
    let order_id = OrderId::create(order.order_id).unwrap();
    let customer_info = CustomerInfo::create(order.customer_info).unwrap();
    let shipping_address = Address::create(&check_address_service, order.shipping_address).await?;
    let billing_address = Address::create(&check_address_service, order.billing_address).await?;

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
    let validated_order = ValidatedOrder::create(
        order_id,
        customer_info,
        shipping_address,
        billing_address,
        lines,
    );

    Ok(validated_order)
}

async fn price_order(
    price_service: impl ProductPriceService,
    order: ValidatedOrder,
) -> Result<PricedOrder, &'static str> {
    let futures = order
        .get_order_lines_ref()
        .iter()
        .map(|line| async {
            let price_value = price_service.get(line.get_product_code_ref()).await?;
            let price = Price::create(price_value)?;
            let result = PricedOrderLine::create(line, price);

            Result::<PricedOrderLine, &'static str>::Ok(result)
        })
        .collect::<FuturesOrdered<_>>();

    let lines = futures.try_collect::<Vec<_>>().await?;

    let result = PricedOrder::create(&order, lines);

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

    if billing_amount > AmountToBill::ZERO {
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
