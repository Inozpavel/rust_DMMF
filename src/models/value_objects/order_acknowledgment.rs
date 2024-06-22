use crate::models::value_objects::email_address::EmailAddress;
use crate::models::value_objects::html_string::HtmlString;
use std::rc::Rc;

#[readonly::make]
pub struct OrderAcknowledgment {
    pub email_address: Rc<EmailAddress>,
    pub letter: Rc<HtmlString>,
}

impl OrderAcknowledgment {
    pub fn create(email_address: Rc<EmailAddress>, letter: HtmlString) -> Self {
        Self {
            letter: Rc::new(letter),
            email_address,
        }
    }
}
