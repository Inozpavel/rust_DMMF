#[readonly::make]
pub struct HtmlString(pub String);

impl HtmlString {
    pub fn create(value: String) -> Self {
        Self(value)
    }
}
