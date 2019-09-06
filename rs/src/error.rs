use std::borrow::Cow;
use std::fmt::{Display, Formatter};

/// Result error type.
///
#[derive(Debug)]
pub enum OpenProtocolError<'a> {
    /// The value of a field is the empty string `""` or containing all white-spaces,
    /// which is not allowed as value of that field.
    EmptyField(Cow<'a, str>),
    //
    /// The value (second parameter) of a field (first parameter) is not valid for that field.
    InvalidField { field: Cow<'a, str>, value: Cow<'a, str>, description: Cow<'a, str> },
    //
    /// The value of a field is not consistent with the matching value in the [`state`].
    ///
    /// [`state`]: struct.StateValues.html
    ///
    InconsistentState(Cow<'a, str>),
    //
    /// The value of a field is not consistent with the matching value in the
    /// [`Controller`] structure.
    ///
    /// [`Controller`]: struct.Controller.html
    InconsistentField(Cow<'a, str>),
    //
    /// An enforced constraint is broken.
    ConstraintViolated(Cow<'a, str>),
    //
    /// Error when serializing/deserializing JSON.
    JsonError(serde_json::Error),
}

impl std::error::Error for OpenProtocolError<'_> {
    fn description(&self) -> &str {
        match self {
            // JSON error
            Self::JsonError(err) => err.description(),
            //
            // Invalid field value
            Self::InvalidField { description, .. } if description.is_empty() => {
                "invalid field value"
            }
            Self::InvalidField { description, .. } => description,
            //
            // Constraint violation
            Self::ConstraintViolated(err) => err,
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

impl Display for OpenProtocolError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            // JSON error
            Self::JsonError(err) => err.fmt(f),
            //
            // Invalid field value
            Self::InvalidField { field, value, description } if description.is_empty() => {
                write!(f, "value [{}] is invalid for the field {}", value, field)
            }
            Self::InvalidField { field, value, description } => {
                write!(f, "value [{}] is invalid for the field {}: {}", value, field, description)
            }
            //
            // Constraint violation
            Self::ConstraintViolated(err) => err.fmt(f),
            //
            // Inconsistent field value
            Self::InconsistentField(field) => write!(
                f,
                "value of field {} is not the same as the matching field in the Controller",
                field
            ),
            //
            // Inconsistent state value
            Self::InconsistentState(field) => write!(
                f,
                "value of field {} is not the same as the matching field in the state",
                field
            ),
            //
            // Field empty
            Self::EmptyField(field) => {
                write!(f, "field {} cannot be empty or all whitespace", field)
            }
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
        match self {
            // JSON error - since serde::error::Error does not implement PartialEq,
            //              the only thing we can do is compare the debug representation.
            Self::JsonError(err1) => match other {
                Self::JsonError(err2) => format!("{:?}", err1) == format!("{:?}", err2),
                _ => false,
            },
            //
            // All other variants that implement PartialEq
            _ => *self == *other,
        }
    }
}

impl Eq for OpenProtocolError<'_> {}

impl std::convert::From<OpenProtocolError<'_>> for String {
    fn from(error: OpenProtocolError<'_>) -> Self {
        error.to_string()
    }
}