//! Rust access library to read/write iChen<sup>®</sup> 4 Open Protocol<sup>™</sup> messages.
//!
//! Details on the protocol can be found [here](https://github.com/chenhsong/OpenProtocol/blob/master/cs/doc/messages_reference.md).

use std::borrow::Cow;

// External crates
use lazy_static;

// Modules
mod controller;
mod enums;
mod filters;
mod messages;
mod utils;

/// Result error type.
///
#[derive(Debug)]
pub enum OpenProtocolError<'a> {
    /// The value of a field is the empty string "" or all white-spaces,
    /// which is not allowed as value of that field.
    EmptyField(Cow<'a, str>),
    /// The value (second parameter) of a field (first parameter) is not valid for that field.
    ///
    /// The strings are `Box`'ed to make the enum small.
    InvalidField {
        field: Cow<'a, str>,
        value: Cow<'a, str>,
        description: Cow<'a, str>,
    },
    /// An enfored constraint is broken.
    ///
    /// The string is `Box`'ed to make the enum small.
    ConstraintViolated(Cow<'a, str>),
    /// Error when serializing/deserializing JSON.
    JsonError(serde_json::Error),
}

impl std::error::Error for OpenProtocolError<'_> {
    fn description(&self) -> &str {
        match self {
            OpenProtocolError::JsonError(err) => err.description(),
            OpenProtocolError::InvalidField { description, .. } => {
                if description.is_empty() {
                    "Invalid field value."
                } else {
                    description
                }
            }
            OpenProtocolError::ConstraintViolated(err) => err,
            OpenProtocolError::EmptyField(_) => "Field cannot be empty or all white-space.",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            OpenProtocolError::JsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for OpenProtocolError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            OpenProtocolError::JsonError(err) => err.fmt(f),
            OpenProtocolError::InvalidField {
                field,
                value,
                description,
            } => {
                if description.is_empty() {
                    f.write_fmt(format_args!("Value [{}] is invalid for the field {}", value, field))
                } else {
                    f.write_fmt(format_args!(
                        "Value [{}] is invalid for the field {}: {}.",
                        value, field, description
                    ))
                }
            }
            OpenProtocolError::ConstraintViolated(err) => err.fmt(f),
            OpenProtocolError::EmptyField(field) => {
                f.write_fmt(format_args!("Field {} cannot be empty or all white-space.", field))
            }
        }
    }
}

/// Result type.
///
pub type Result<'a, T> = std::result::Result<T, OpenProtocolError<'a>>;

// Re-exports
pub use controller::{Controller, GeoLocation, Operator};
pub use enums::{JobMode, Language, OpMode};
pub use filters::Filter;
pub use messages::*;
