use aws_sdk_dynamodb::types::AttributeValue;
use std::num::ParseFloatError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeezError {
    #[error("AttributeValue error")]
    AWSAttributeValue(#[from] AttributeValueError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
    // #[error("invalid composite: {0}, must use String")]
    // InvalidComposite(String),
    #[error("unknown key: {0}")]
    UnknownSchemaIndex(String),
    #[error("empty entity vector")]
    EmptyEntityVec,
    #[error("unknown struct index: {0}")]
    UnknownStructIndex(usize),
    #[error("unknown struct field: {0}")]
    UnknownStructField(String),
    #[error("unknown attribute: {0}")]
    UnknownAttribute(String),
    #[error("failed downcast for struct field: {0}")]
    FailedDowncast(String),
    #[error("unknown attribute value key: {0}")]
    UnknownAttributeValueKey(String),
    #[error("invalid type for key composite: boolean")]
    InvalidComposite,
}

// todo: cringed
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
        write!(f, "AttributeValue error")
    }
}

impl std::convert::From<&AttributeValue> for AttributeValueError {
    fn from(_: &AttributeValue) -> Self {
        AttributeValueError
    }
}
