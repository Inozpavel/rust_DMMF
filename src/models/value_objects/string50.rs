#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct String50(String);

impl String50 {
    pub fn create(s: String) -> Option<Self> {
        if s.chars().count() < 50 {
            Some(String50(s))
        } else {
            None
        }
    }

    pub fn get_ref(&self) -> &String {
        &self.0
    }
}
