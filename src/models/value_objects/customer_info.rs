use crate::models::unvalidated::unvalidated_customer::UnvalidatedCustomerInfo;
use crate::models::value_objects::email_address::EmailAddress;
use crate::models::value_objects::personal_name::PersonalName;
use crate::models::value_objects::string50::String50;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CustomerInfo {
    personal_name: PersonalName,
    email_address: EmailAddress,
}

impl CustomerInfo {
    pub fn create(unvalidated_customer: UnvalidatedCustomerInfo) -> Result<Self, &'static str> {
        let email_address = EmailAddress::create(unvalidated_customer.email)?;
        let first_name = String50::create(unvalidated_customer.first_name).unwrap();
        let last_name = String50::create(unvalidated_customer.last_name).unwrap();

        let personal_name = PersonalName::new(first_name, last_name);
        Ok(CustomerInfo {
            email_address,
            personal_name,
        })
    }

    pub fn into_inner(self) -> (PersonalName, EmailAddress) {
        (self.personal_name, self.email_address)
    }
}

pub mod first_name {}

pub mod last_name {}
