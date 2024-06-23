use rust_decimal::Decimal;

#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct KilogramQuantity(pub Decimal);

impl KilogramQuantity {
    pub fn create(value: Decimal) -> Result<Self, &'static str> {
        if value <= Decimal::from(0) {
            Err("KilogramQuantity must be > 0")
        } else {
            Ok(Self(value))
        }
    }
}
