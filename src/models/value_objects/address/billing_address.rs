#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct BillingAddress(String);

impl BillingAddress {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            return Err("OrderId can't be empty");
        } else {
            Ok(BillingAddress(s))
        }
    }

    pub fn get_ref(&self) -> &str {
        &self.0
    }
}
