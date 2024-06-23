use crate::value_objects::order_acknowledgment::OrderAcknowledgment;
use crate::value_objects::send_result::SendResult;
use async_trait::async_trait;

#[async_trait]
pub trait AcknowledgementService {
    async fn send(&self, acknowledgment: &OrderAcknowledgment) -> Result<SendResult, &'static str>;
}
