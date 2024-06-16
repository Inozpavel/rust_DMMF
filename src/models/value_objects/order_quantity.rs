use crate::models::value_objects::order_quantity::kilogram_quantity::KilogramQuantity;
use crate::models::value_objects::order_quantity::unit_quantity::UnitQuantity;

pub mod kilogram_quantity;
pub mod unit_quantity;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}
