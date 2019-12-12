use derive_more::*;
use std::borrow::Cow;

/// Result error type.
///
#[derive(Debug, Display)]
pub enum OpenProtocolError<'a> {
    /// The value of a field is the empty string `""` or containing all white-spaces,
    /// which is not allowed as value of that field.
    #[display(fmt = "field {} cannot be empty or all whitespace", _0)]
    EmptyField(&'a str),
    //
    /// The value of a field is not valid.
    #[display(fmt = "value [{}] is invalid for the field {} - {}", value, field, description)]
    InvalidField { field: &'a str, value: Cow<'a, str>, description: Cow<'a, str> },
    //
    /// The value of a field is not consistent with the matching value in the [`state`].
    ///
    /// [`state`]: struct.StateValues.html
    ///
    #[display(fmt = "value of field {} is not the same as the matching field in the state", _0)]
    InconsistentState(&'a str),
    //
    /// The value of a field is not consistent with the matching value in the
    /// [`Controller`] structure.
    ///
    /// [`Controller`]: struct.Controller.html
    #[display(
        fmt = "value of field {} is not the same as the matching field in the Controller",
        _0
    )]
    InconsistentField(&'a str),
    //
    /// An enforced constraint is broken.
    #[display(fmt = "{}", _0)]
    ConstraintViolated(Cow<'a, str>),
    //
    /// Error when serializing/deserializing JSON.
    #[display(fmt = "[{:?}] {}", "_0.classify()", _0)]
    JsonError(serde_json::Error),
    //
    /// An unexpected system error.
    #[display(fmt = "{}", _0)]
    SystemError(Cow<'a, str>),
}

impl std::error::Error for OpenProtocolError<'_> {
    fn description(&self) -> &str {
        match self {
            // JSON error
            Self::JsonError(err) => err.description(),
            //
            // Invalid field value
            Self::InvalidField { description, .. } => {
                if description.is_empty() {
                    "invalid field value"
                } else {
                    description
                }
            }
            //
            // Constraint violation
            Self::ConstraintViolated(err) => err,
            //
            // System error
            Self::SystemError(err) => err,
            //
            // Inconsistent field
            Self::InconsistentField(_) => {
                "value of field is not the same as matching field in the Controller"
            }
            //
            // Inconsistent state
            Self::InconsistentState(_) => {
                "value of field is not the same as matching field in the state"
            }
            //
            // Field empty
            Self::EmptyField(_) => "field cannot be empty or all whitespace",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::JsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl PartialEq for OpenProtocolError<'_> {
    /// Implement `PartialEq` for `OpenProtocolError`.
    ///
    /// Most variants already implement `PartialEq` and are simply delegated.
    ///
    /// The only variant that doesn't automatically implement `PartialEq` is [`JsonError`]
    /// which encapsulates a `serde::error::Error` object that does not implement
    /// `PartialEq`.  In this case, we test for equality simply by comparing the `Debug`
    /// output of `self` and `other`.
    ///
    /// [`JsonError`]: #variant.JsonError
    ///
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // JSON error - since serde::error::Error does not implement PartialEq,
            //              the only thing we can do is compare the debug representation.
            (Self::JsonError(err1), Self::JsonError(err2)) => {
                format!("{:?}", err1) == format!("{:?}", err2)
            }
            //
            // All other variants need to manually implement PartialEq
            (Self::EmptyField(err1), Self::EmptyField(err2)) => err1 == err2,
            (
                Self::InvalidField { field: field1, value: value1, description: err1 },
                Self::InvalidField { field: field2, value: value2, description: err2 },
            ) => field1 == field2 && value1 == value2 && err1 == err2,
            (Self::InconsistentState(err1), Self::InconsistentState(err2)) => err1 == err2,
            (Self::InconsistentField(err1), Self::InconsistentField(err2)) => err1 == err2,
            (Self::ConstraintViolated(err1), Self::ConstraintViolated(err2)) => err1 == err2,
            _ => false,
        }
    }
}

impl Eq for OpenProtocolError<'_> {}

impl std::convert::From<OpenProtocolError<'_>> for String {
    fn from(error: OpenProtocolError<'_>) -> Self {
        error.to_string()
    }
}
