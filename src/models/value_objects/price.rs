use rust_decimal::Decimal;
use crate::models::value_objects::order_quantity::OrderQuantity;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Price(Decimal);

impl Price {
    pub fn create(value: Decimal) -> Result<Self, &'static str> {
        if value < Decimal::from(0) {
            Err("Price can't be < 0")
        } else {
            Ok(Price(value))
        }
    }

    pub fn get_value(&self) -> Decimal {
        self.0
    }

    pub fn multiply(&self, order_quantity: &OrderQuantity) -> Self {
        let multiplier = match order_quantity {
            OrderQuantity::Unit(unit) => Decimal::from(unit.get()),
            OrderQuantity::Kilogram(kilogram) => kilogram.get()
        };
        Self(self.0 * multiplier)
    }
}
