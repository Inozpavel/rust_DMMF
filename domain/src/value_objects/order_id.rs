#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderId(pub String);

impl OrderId {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            Err("OrderId can't be empty")
        } else {
            Ok(OrderId(s))
        }
    }
}
