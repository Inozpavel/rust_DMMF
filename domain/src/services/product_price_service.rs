use crate::value_objects::product_code::ProductCode;
use async_trait::async_trait;
use rust_decimal::Decimal;

#[async_trait]
pub trait ProductPriceService {
    async fn get(&self, code: &ProductCode) -> Result<Decimal, &'static str>;
}
