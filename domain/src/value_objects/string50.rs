#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct String50(pub String);

impl String50 {
    pub fn create(s: String) -> Option<Self> {
        if s.chars().count() < 50 {
            Some(String50(s))
        } else {
            None
        }
    }
}
