use std::convert::Infallible;

use crate::models::entities::order::{PricedOrder, ValidatedOrder};
use crate::models::unvalidated::unvalidated_order::UnvalidatedOrder;
use crate::models::ValidationError;
use crate::models::value_objects::address::billing_address::BillingAddress;
use crate::models::value_objects::address::shipping_address::ShippingAddress;
use crate::models::value_objects::customer_info::CustomerInfo;
use crate::models::value_objects::order_id::OrderId;
use crate::models::value_objects::product_code::ProductCode;

async fn place_order(
    order: UnvalidatedOrder,
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
) -> Result<(), Infallible> {
    let validated_order =
        validate_order(check_product_code_service, check_address_service, order).await?;

    Ok(())
}

async fn validate_order(
    check_product_code_service: impl CheckProductCodeExistsService,
    check_address_service: impl CheckAddressExistsService,
    order: UnvalidatedOrder,
) -> Result<ValidatedOrder, Vec<ValidationError>> {
    let order_id = OrderId::create(order.order_id).unwrap();
    let customer_info = CustomerInfo::create(order.customer_info).unwrap();
    let shipping_address = ShippingAddress::create(order).unwrap();
    let billing_address = BillingAddress::create(order.shipping_address).unwrap();

    ValidatedOrder::create(order_id, customer_info, shipping_address)
}

fn price_order(order: ValidatedOrder) -> Result<PricedOrder, Infallible> {}

fn acknowledge_order(order: PricedOrder) {}

trait CheckProductCodeExistsService {
    fn check(product_code: ProductCode) -> bool;
}

trait CheckAddressExistsService {
    async fn check(product_code: ProductCode) -> bool;
}
