#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderId(String);

impl OrderId {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            return Err("OrderId can't be empty");
        } else {
            Ok(OrderId(s))
        }
    }

    pub fn get_ref(&self) -> &str {
        &self.0
    }
}
