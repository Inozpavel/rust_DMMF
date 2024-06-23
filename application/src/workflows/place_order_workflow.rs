use futures::stream::FuturesOrdered;
use futures::TryStreamExt;

use domain::entities::order::{PricedOrder, ValidatedOrder};
use domain::entities::order_line::{PricedOrderLine, ValidatedOrderLine};
use domain::errors::{PlaceOrderError, PricingError, ValidationError};
use domain::events::{BillableOrderPlaced, OrderAcknowledgementSent, OrderPlaced, PlaceOrderEvent};
use domain::services::acknowledgement_service::AcknowledgementService;
use domain::services::check_address_exists_service::CheckAddressExistsService;
use domain::services::check_product_code_exists_service::CheckProductCodeExistsService;
use domain::services::product_price_service::ProductPriceService;
use domain::unvalidated::unvalidated_order::UnvalidatedOrder;
use domain::value_objects::address::Address;
use domain::value_objects::amount_to_bill::AmountToBill;
use domain::value_objects::customer_info::CustomerInfo;
use domain::value_objects::html_string::HtmlString;
use domain::value_objects::order_acknowledgment::OrderAcknowledgment;
use domain::value_objects::order_id::OrderId;
use domain::value_objects::price::Price;
use domain::value_objects::send_result::SendResult;

async fn place_order(
    order: UnvalidatedOrder,
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    acknowledgment_service: impl AcknowledgementService,
    price_service: impl ProductPriceService,
) -> Result<Vec<PlaceOrderEvent>, PlaceOrderError> {
    let validated_order =
        validate_order(check_product_code_service, check_address_service, order).await?;
    let priced_order = price_order(price_service, validated_order).await?;
    let acknowledge_event = acknowledge_order(acknowledgment_service, &priced_order)
        .await
        .unwrap();

    let events = create_events(&priced_order, acknowledge_event);
    Ok(events)
}

async fn validate_order(
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    order: UnvalidatedOrder,
) -> Result<ValidatedOrder, ValidationError> {
    let order_id = OrderId::create(order.order_id).unwrap();
    let customer_info = CustomerInfo::create(order.customer_info).unwrap();
    let shipping_address = Address::create(&check_address_service, order.shipping_address)
        .await
        .map_err(ValidationError::from)?;
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
            return Err(ValidationError::from(format!(
                "Product code {} wasn't found",
                line.get_product_code_ref()
            )));
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
) -> Result<PricedOrder, PricingError> {
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
    order: &PricedOrder,
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
    HtmlString::create(format!("<h1>Acknowledgment letter!</h1>"))
}

fn create_billing_event(priced_order: &PricedOrder) -> Option<BillableOrderPlaced> {
    let billing_amount = priced_order.amount_to_bill;

    if billing_amount > AmountToBill::ZERO {
        return Some(BillableOrderPlaced::create(priced_order));
    }

    None
}

fn create_events(
    priced_order: &PricedOrder,
    acknowledgement_sent: Option<OrderAcknowledgementSent>,
) -> Vec<PlaceOrderEvent> {
    let order_placed = OrderPlaced::create(priced_order.order_id.clone());
    let place_order_event = PlaceOrderEvent::OrderPlaced(order_placed);

    let acknowledgement_sent_event = acknowledgement_sent.map(PlaceOrderEvent::AcknowledgementSent);

    let billable_order_placed_event =
        create_billing_event(&priced_order).map(PlaceOrderEvent::BillableOrderPlaced);

    let result = [
        Some(place_order_event),
        acknowledgement_sent_event,
        billable_order_placed_event,
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    result
}
