#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct EmailAddress(pub String);

impl EmailAddress {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            Err("OrderId can't be empty")
        } else if !s.chars().any(|x| x == '@') {
            Err("Email is in invalid format")
        } else {
            Ok(EmailAddress(s))
        }
    }
}
