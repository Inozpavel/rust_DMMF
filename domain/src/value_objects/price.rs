use crate::value_objects::order_quantity::OrderQuantity;
use rust_decimal::Decimal;

#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Price(pub Decimal);

impl Price {
    pub fn create(value: Decimal) -> Result<Self, &'static str> {
        if value < Decimal::from(0) {
            Err("Price can't be < 0")
        } else {
            Ok(Price(value))
        }
    }

    pub fn multiply(&self, order_quantity: &OrderQuantity) -> Self {
        let multiplier = match order_quantity {
            OrderQuantity::Unit(unit) => Decimal::from(unit.0),
            OrderQuantity::Kilogram(kilogram) => kilogram.0,
        };
        Self(self.0 * multiplier)
    }
}
