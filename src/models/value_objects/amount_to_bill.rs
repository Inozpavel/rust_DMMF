use rust_decimal::Decimal;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct AmountToBill(Decimal);
