use rust_decimal::Decimal;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct KilogramQuantity(pub(super) Decimal);

impl KilogramQuantity {
    pub fn get(&self) -> Decimal {
        self.0
    }
}
