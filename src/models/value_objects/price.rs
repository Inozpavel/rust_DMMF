use rust_decimal::Decimal;

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
}
