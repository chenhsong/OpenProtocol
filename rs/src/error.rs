use std::borrow::Cow;

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
    /// An enforced constraint is broken.
    ///
    /// The string is `Box`'ed to make the enum small.
    ConstraintViolated(Cow<'a, str>),
    /// Error when serializing/deserializing JSON.
    JsonError(serde_json::Error),
}

impl std::error::Error for OpenProtocolError<'_> {
    fn description(&self) -> &str {
        match self {
            // JSON error
            OpenProtocolError::JsonError(err) => err.description(),
            // Invalid field value
            OpenProtocolError::InvalidField { description, .. } if description.is_empty() => {
                "Invalid field value."
            }
            OpenProtocolError::InvalidField { description, .. } => description,
            // Constraint violation
            OpenProtocolError::ConstraintViolated(err) => err,
            // Field empty
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
            // JSON error
            OpenProtocolError::JsonError(err) => err.fmt(f),
            // Invalid field value
            OpenProtocolError::InvalidField { field, value, description }
                if description.is_empty() =>
            {
                write!(f, "Value [{}] is invalid for the field {}", value, field)
            }
            OpenProtocolError::InvalidField { field, value, description } => {
                write!(f, "Value [{}] is invalid for the field {}: {}.", value, field, description)
            }
            // Constraint violation
            OpenProtocolError::ConstraintViolated(err) => err.fmt(f),
            // Field empty
            OpenProtocolError::EmptyField(field) => {
                write!(f, "Field {} cannot be empty or all white-space.", field)
            }
        }
    }
}
