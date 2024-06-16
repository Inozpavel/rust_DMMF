use crate::models::entities::order_line::PricedOrderLine;
use rust_decimal::Decimal;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct AmountToBill(Decimal);

impl AmountToBill {
    pub fn sum_prices(lines: &Vec<PricedOrderLine>) -> Self {
        let sum = lines.iter().map(|x| x.get_price().get_value()).sum();

        Self(sum)
    }

    pub fn get_value(&self) -> Decimal {
        self.0
    }
}
