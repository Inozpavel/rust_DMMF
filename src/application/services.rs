use rust_decimal::Decimal;
use crate::models::value_objects::product_code::ProductCode;

pub trait CheckProductCodeExistsService {
    async fn check(&self, product_code: &ProductCode) -> bool;
}

pub trait CheckAddressExistsService {
    async fn check(&self, product_code: &ProductCode) -> bool;
}

pub trait GetProductPriceService {
    async fn get(&self, code: &ProductCode) -> Result<Decimal, &'static str>;
}
