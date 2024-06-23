use async_trait::async_trait;

use crate::unvalidated::unvalidated_address::UnvalidatedAddress;
use crate::value_objects::address::CheckedAddress;

#[async_trait]
pub trait CheckAddressExistsService {
    async fn check(&self, address: &UnvalidatedAddress) -> CheckedAddress;
}
