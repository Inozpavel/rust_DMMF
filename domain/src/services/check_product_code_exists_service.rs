use crate::value_objects::product_code::ProductCode;
use async_trait::async_trait;

#[async_trait]
pub trait CheckProductCodeExistsService {
    async fn check(&self, product_code: &ProductCode) -> bool;
}
