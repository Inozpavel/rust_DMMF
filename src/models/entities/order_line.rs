use crate::models::value_objects::order_id::OrderId;
use crate::models::value_objects::order_line_id::OrderLineId;
use crate::models::value_objects::order_quantity::OrderQuantity;
use crate::models::value_objects::price::Price;
use crate::models::value_objects::product_code::ProductCode;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderLine {
    id: OrderLineId,
    order_id: OrderId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

pub struct ValidatedOrderLine {}

pub struct PricedOrderLine {}
