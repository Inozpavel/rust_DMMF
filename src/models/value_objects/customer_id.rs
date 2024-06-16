#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CustomerId(pub i32);

impl CustomerId {
    pub fn create(value: i32) -> Result<Self, &'static str> {
        if value < 0 {
            return Err("CustomerId can't be < 0");
        } else {
            Ok(CustomerId(value))
        }
    }

    pub fn get_value(&self) -> i32 {
        self.0
    }
}
