use aws_sdk_dynamodb::types::AttributeValue;
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum DeezError {
    #[error("AttributeValue error")]
    AWSAttributeValue(#[from] AttributeValueError),
    #[error("missing key in map: {0}")]
    MapKey(String),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError)
}

// todo: cringe, don't know what im doing
impl std::convert::From<&AttributeValue> for DeezError {
    fn from(_: &AttributeValue) -> Self {
        DeezError::AWSAttributeValue(AttributeValueError)
    }
}

// Error wrapper for the aws-sdk AttributeValue result type, which doesn't
// implement `Error`.
#[derive(Debug)]
pub struct AttributeValueError;

impl std::error::Error for AttributeValueError {
    fn description(&self) -> &str {
        "AttributeValue error"
    }
}

impl std::fmt::Display for AttributeValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "20s")
    }
}

impl std::convert::From<&AttributeValue> for AttributeValueError {
    fn from(_: &AttributeValue) -> Self {
        AttributeValueError
    }
}
