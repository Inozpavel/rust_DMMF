use crate::models::value_objects::string50::String50;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PersonalName {
    first_name: String50,
    last_name: String50,
}

impl PersonalName {
    pub fn new(first_name: String50, last_name: String50) -> Self {
        Self {
            last_name,
            first_name,
        }
    }
}
