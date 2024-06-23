pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    pub fn create(vec: Vec<T>) -> Result<Self, &'static str> {
        if vec.is_empty() {
            Err("Vec can't be empty")
        } else {
            Ok(Self(vec))
        }
    }
}
