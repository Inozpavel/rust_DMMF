use crate::unvalidated::unvalidated_address::UnvalidatedAddress;
use crate::value_objects::address::CheckedAddress;
use crate::value_objects::order_acknowledgment::OrderAcknowledgment;
use crate::value_objects::product_code::ProductCode;
use crate::value_objects::send_result::SendResult;
use async_trait::async_trait;
use rust_decimal::Decimal;

#[async_trait]
pub trait CheckProductCodeExistsService {
    async fn check(&self, product_code: &ProductCode) -> bool;
}

#[async_trait]
pub trait CheckAddressExistsService {
    async fn check(&self, address: &UnvalidatedAddress) -> CheckedAddress;
}

#[async_trait]
pub trait ProductPriceService {
    async fn get(&self, code: &ProductCode) -> Result<Decimal, &'static str>;
}

#[async_trait]
pub trait AcknowledgementService {
    async fn send(&self, acknowledgment: &OrderAcknowledgment) -> Result<SendResult, &'static str>;
}
