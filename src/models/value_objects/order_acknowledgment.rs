use crate::models::value_objects::email_address::EmailAddress;
use crate::models::value_objects::html_string::HtmlString;

pub struct OrderAcknowledgment {
    email_address: EmailAddress,
    letter: HtmlString,
}

impl OrderAcknowledgment {
    pub fn create(email_address: EmailAddress, letter: HtmlString) -> Self {
        Self {
            letter,
            email_address,
        }
    }

    pub fn into_inner(self) -> (EmailAddress, HtmlString) {
        (self.email_address, self.letter)
    }
}