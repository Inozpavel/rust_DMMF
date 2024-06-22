use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::models::value_objects::order_quantity::kilogram_quantity::KilogramQuantity;
use crate::models::value_objects::order_quantity::unit_quantity::UnitQuantity;
use crate::models::value_objects::product_code::ProductCode;

pub mod kilogram_quantity;
pub mod unit_quantity;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}

impl OrderQuantity {
    pub fn create(code: &ProductCode, value: Decimal) -> Result<Self, &'static str> {
        if value < Decimal::from(0) {
            Err("OrderQuantity can't be < 0")
        } else {
            let result = match code {
                ProductCode::Widget(_) => match value.to_i32() {
                    None => return Err("Unit quantity must be an integer"),
                    Some(v) => OrderQuantity::Unit(UnitQuantity::create(v)?),
                },
                ProductCode::Gizmo(_) => OrderQuantity::Kilogram(KilogramQuantity::create(value)?),
            };
            Ok(result)
        }
    }
}
