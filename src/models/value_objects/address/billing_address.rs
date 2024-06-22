#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct BillingAddress(pub String);

impl BillingAddress {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            Err("BillingAddress can't be empty")
        } else {
            Ok(BillingAddress(s))
        }
    }
}
