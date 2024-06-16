use crate::models::unvalidated::unvalidated_customer::UnvalidatedCustomerInfo;
use crate::models::value_objects::customer_id::CustomerId;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CustomerInfo {
    id: CustomerId,
}

impl CustomerInfo {
    pub fn create(unvalidated_customer: UnvalidatedCustomerInfo) -> Result<Self, &'static str> {
        CustomerInfo { id }
    }

    pub fn value(&self) -> &CustomerId {
        &self.id
    }
}

pub mod first_name {}

pub mod last_name {}
