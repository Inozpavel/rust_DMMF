pub struct HtmlString(String);

impl HtmlString {
    pub fn create(value: String) -> Self {
        Self(value)
    }

    pub fn get_ref(&self) -> &str {
        &self.0
    }
}
