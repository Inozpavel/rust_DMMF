#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct GizmoCode(pub String);

impl GizmoCode {
    pub fn create(value: String) -> Result<Self, &'static str> {
        if value.chars().all(|x| x.is_whitespace()) {
            Err("GizmoCode can't be empty")
        } else {
            Ok(Self(value))
        }
    }
}
