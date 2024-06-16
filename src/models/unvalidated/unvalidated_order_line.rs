use rust_decimal::Decimal;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct UnvalidatedOrderLine {
    pub id: String,
    pub order_id: String,
    pub product_code: String,
    pub order_quantity: Decimal,
}
