//! Rust access library to read/write iChen<sup>®</sup> 4 Open Protocol<sup>™</sup> messages.
//!
//! Details on the protocol can be found [here](https://github.com/chenhsong/OpenProtocol/blob/master/cs/doc/messages_reference.md).
//!
//! Notes on Usage
//! ==============
//!
//! Beware that all data types defined in this crate use borrowed string slices (i.e. `&str`) extensively.
//! This is because the most common usage pattern is to create a data variable, set fields, immediately
//! serialize it into JSON, then dispose of the data variable.  The deserialization story is similar.
//!
//! Error values also borrow heavily from the input fields as these errors are expected to be handled
//! as soon as possible.
//!
//! The result is minimal allocations and copying, but at the cost of stricter lifetime management,
//! especially when deserializing -- the message struct cannot out-live the original JSON text string as
//! fields are borrowed extensively from the original JSON string.
//!
//! Another implication due to extensive usage of borrowed string slices is that string literals with
//! escape sequences will cause parsing errors because the actual string cannot be simply borrowed from
//! the original JSON string.  Luckily this is extremely rare for most fields holding names, ID's etc.
//! For this reason, only certain user-defined text fields (such as `job_card_id`) may contain
//! escaped characters (especially the double-quote); those are therefore modeled using `Cow<&str>` instead.

use lazy_static;
use std::borrow::Cow;

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
    InvalidField { field: Cow<'a, str>, value: Cow<'a, str>, description: Cow<'a, str> },
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
            OpenProtocolError::InvalidField { field, value, description } => {
                if description.is_empty() {
                    f.write_fmt(format_args!("Value [{}] is invalid for the field {}", value, field))
                } else {
                    f.write_fmt(format_args!("Value [{}] is invalid for the field {}: {}.", value, field, description))
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
pub use filters::{Filter, Filters};
pub use messages::*;
