#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderLineId(String);

impl OrderLineId {
    pub fn create(s: String) -> Result<Self, &'static str> {
        if s.chars().all(|x| x.is_whitespace()) {
            Err("OrderLineId can't be empty")
        } else {
            Ok(OrderLineId(s))
        }
    }
}
