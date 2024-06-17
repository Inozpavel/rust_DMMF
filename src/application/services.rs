use rust_decimal::Decimal;

use crate::models::value_objects::order_acknowledgment::OrderAcknowledgment;
use crate::models::value_objects::product_code::ProductCode;
use crate::models::value_objects::send_result::SendResult;

pub trait CheckProductCodeExistsService {
    async fn check(&self, product_code: &ProductCode) -> bool;
}

pub trait CheckAddressExistsService {
    async fn check(&self, product_code: &ProductCode) -> bool;
}

pub trait ProductPriceService {
    async fn get(&self, code: &ProductCode) -> Result<Decimal, &'static str>;
}

pub trait AcknowledgementService {
    async fn send(&self, acknowledgment: &OrderAcknowledgment) -> Result<SendResult, &'static str>;
}
