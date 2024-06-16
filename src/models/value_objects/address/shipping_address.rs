#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ShippingAddress(String);

impl ShippingAddress {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            return Err("OrderId can't be empty");
        } else {
            Ok(ShippingAddress(s))
        }
    }

    pub fn get_ref(&self) -> &str {
        &self.0
    }
}
