#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ShippingAddress(pub String);

impl ShippingAddress {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            Err("ShippingAddress can't be empty")
        } else {
            Ok(ShippingAddress(s))
        }
    }
}
