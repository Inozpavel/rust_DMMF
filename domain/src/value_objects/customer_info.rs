use crate::unvalidated::unvalidated_customer::UnvalidatedCustomerInfo;
use crate::value_objects::email_address::EmailAddress;
use crate::value_objects::personal_name::PersonalName;
use crate::value_objects::string50::String50;
use std::rc::Rc;

#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CustomerInfo {
    pub personal_name: Rc<PersonalName>,
    pub email_address: Rc<EmailAddress>,
}

impl CustomerInfo {
    pub fn create(unvalidated_customer: UnvalidatedCustomerInfo) -> Result<Self, &'static str> {
        let email_address = EmailAddress::create(unvalidated_customer.email)?;
        let first_name = String50::create(unvalidated_customer.first_name).unwrap();
        let last_name = String50::create(unvalidated_customer.last_name).unwrap();

        let personal_name = PersonalName::new(first_name, last_name);
        Ok(CustomerInfo {
            email_address: Rc::new(email_address),
            personal_name: Rc::new(personal_name),
        })
    }
}

pub mod first_name {}

pub mod last_name {}
