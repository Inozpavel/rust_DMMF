#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct UnitQuantity(i32);

impl UnitQuantity {
    pub fn create(quantity: i32) -> Result<Self, &'static str> {
        if quantity < 1 {
            Err("UnitQuantity can't be negative")
        } else if quantity > 1000 {
            Err("UnitQuantity can't be more than 1000")
        } else {
            Ok(UnitQuantity(quantity))
        }
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}
