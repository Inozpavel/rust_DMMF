#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum SendResult {
    Sent,
    NotSent,
}
