#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct UnvalidatedAddress {
    pub address_line1: String,
    pub address_line2: String,
    pub address_line3: String,
    pub address_line4: String,
    pub city: String,
    pub zip_code: String,
}
