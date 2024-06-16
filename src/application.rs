use std::convert::Infallible;
use futures::stream::{FuturesOrdered, FuturesUnordered};
use futures::StreamExt;
use crate::application::services::{CheckAddressExistsService, CheckProductCodeExistsService, GetProductPriceService};

use crate::models::entities::order::{PricedOrder, ValidatedOrder};
use crate::models::entities::order_line::{PricedOrderLine, ValidatedOrderLine};
use crate::models::unvalidated::unvalidated_order::UnvalidatedOrder;
use crate::models::ValidationError;
use crate::models::value_objects::customer_info::CustomerInfo;
use crate::models::value_objects::order_id::OrderId;
use crate::models::value_objects::price::Price;

pub mod services;

async fn place_order(
    order: UnvalidatedOrder,
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    get_product_price_service: impl GetProductPriceService,
) -> Result<(), &'static str> {
    let validated_order =
        validate_order(check_product_code_service, check_address_service, order).await?;

    let priced_order = price_order(get_product_price_service, validated_order).await?;
    Ok(())
}

async fn validate_order(
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    order: UnvalidatedOrder,
) -> Result<ValidatedOrder, &'static str> {
    let order_id = OrderId::create(order.order_id).unwrap();
    let customer_info = CustomerInfo::create(order.customer_info).unwrap();
    // let shipping_address = ShippingAddress::create(order).unwrap();
    // let billing_address = BillingAddress::create(order.shipping_address).unwrap();

    let lines = order.lines.into_iter().map(|line| ValidatedOrderLine::create(line)).collect::<Result<Vec<_>, _>>()?;

    for line in lines {
        if !check_product_code_service.check(line.get_product_code_ref()).await {
            // return Err(format!("Product code {} wasn't found", line.get_product_code_ref().));
            return Err("Product code wasn't found");
        }
    }
    let result = ValidatedOrder::create(order_id, customer_info, lines);

    Ok(result)
}

async fn price_order(price_order_service: impl GetProductPriceService, order: ValidatedOrder) -> Result<PricedOrder, &'static str> {
    let futures = order.into_lines().into_iter().map(|line| {
        async {
            let price_value = price_order_service.get(line.get_product_code_ref()).await?;
            let price = Price::create(price_value)?;
            let result = PricedOrderLine::create(line, price).into();

            Result::<PricedOrderLine, &'static str>::Ok(result)
        }
    }).collect::<FuturesOrdered<_>>();

    futures.collect::<Vec<_>>().await;

    let result = PricedOrder::create()

    Ok(result)
}

fn acknowledge_order(order: PricedOrder) {}



