use crate::entities::order_line::PricedOrderLine;
use rust_decimal::Decimal;

#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct AmountToBill(pub Decimal);

impl AmountToBill {
    pub const ZERO: AmountToBill = AmountToBill(Decimal::ZERO);

    pub fn sum_prices(lines: &Vec<PricedOrderLine>) -> Self {
        let sum = lines.iter().map(|x| x.get_price().0).sum();

        Self(sum)
    }
}
