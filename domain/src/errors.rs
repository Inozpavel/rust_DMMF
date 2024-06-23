use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("ValidationError: {0}")]
pub struct ValidationError(Cow<'static, str>);

impl From<&'static str> for ValidationError {
    fn from(value: &'static str) -> Self {
        Self(value.into())
    }
}

impl From<String> for ValidationError {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Error)]
#[error("Pricing error: {0}")]
pub struct PricingError(pub String);

impl From<&'static str> for PricingError {
    fn from(value: &'static str) -> Self {
        Self(value.into())
    }
}

#[derive(Error, Debug)]
pub enum AddressValidationError {
    #[error("Invalid address '{0}' format")]
    InvalidFormat(String),
    #[error("Address '{0}' wasn't found")]
    AddressNotFound(String),
}

#[derive(Error, Debug)]
pub enum PlaceOrderError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Pricing(#[from] PricingError),
    #[error("Product out of stock")]
    ProductOutOfStock(String),
    #[error(transparent)]
    RemoteService(anyhow::Error),
}
