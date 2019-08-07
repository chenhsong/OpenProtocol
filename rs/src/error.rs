use std::borrow::Cow;
use OpenProtocolError::*;

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
            JsonError(err) => err.description(),
            //
            // Invalid field value
            InvalidField { description, .. } if description.is_empty() => "invalid field value",
            InvalidField { description, .. } => description,
            //
            // Constraint violation
            ConstraintViolated(err) => err,
            //
            // Inconsistent field
            InconsistentField(_) => {
                "value of field is not the same as matching field in the Controller"
            }
            //
            // Inconsistent state
            InconsistentState(_) => "value of field is not the same as matching field in the state",
            //
            // Field empty
            EmptyField(_) => "field cannot be empty or all whitespace",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            JsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for OpenProtocolError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            // JSON error
            JsonError(err) => err.fmt(f),
            //
            // Invalid field value
            InvalidField { field, value, description } if description.is_empty() => {
                write!(f, "value [{}] is invalid for the field {}", value, field)
            }
            InvalidField { field, value, description } => {
                write!(f, "value [{}] is invalid for the field {}: {}", value, field, description)
            }
            //
            // Constraint violation
            ConstraintViolated(err) => err.fmt(f),
            //
            // Inconsistent field value
            InconsistentField(field) => write!(
                f,
                "value of field {} is not the same as the matching field in the Controller",
                field
            ),
            //
            // Inconsistent state value
            InconsistentState(field) => write!(
                f,
                "value of field {} is not the same as the matching field in the state",
                field
            ),
            //
            // Field empty
            EmptyField(field) => write!(f, "field {} cannot be empty or all whitespace", field),
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
            JsonError(err1) => match other {
                JsonError(err2) => format!("{:?}", err1) == format!("{:?}", err2),
                _ => false,
            },
            //
            // All other variants that implement PartialEq
            _ => *self == *other,
        }
    }
}

impl Eq for OpenProtocolError<'_> {}
