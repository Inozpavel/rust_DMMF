#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct WidgetCode(pub String);

impl WidgetCode {
    pub fn create(value: String) -> Result<Self, &'static str> {
        if value.chars().all(|x| x.is_whitespace()) {
            Err("WidgetCode can't be empty")
        } else {
            Ok(Self(value))
        }
    }
}
