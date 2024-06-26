#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CustomerId(pub i32);

impl CustomerId {
    pub fn create(value: i32) -> Result<Self, &'static str> {
        if value < 0 {
            Err("CustomerId can't be < 0")
        } else {
            Ok(CustomerId(value))
        }
    }
}
