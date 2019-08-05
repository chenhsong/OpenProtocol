use std::borrow::Cow;

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
    /// The value of a field is not consistent with the matching value in the state.
    InconsistentState(Cow<'a, str>),
    //
    /// The value of a field is not consistent with the matching value in the
    /// [`Controller`](struct.Controller.html) structure.
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
            OpenProtocolError::JsonError(err) => err.description(),
            //
            // Invalid field value
            OpenProtocolError::InvalidField { description, .. } if description.is_empty() => {
                "Invalid field value."
            }
            OpenProtocolError::InvalidField { description, .. } => description,
            //
            // Constraint violation
            OpenProtocolError::ConstraintViolated(err) => err,
            //
            // Inconsistent field
            OpenProtocolError::InconsistentField(_) => {
                "Value of field is not the same as matching field in the Controller."
            }
            //
            // Inconsistent state
            OpenProtocolError::InconsistentState(_) => {
                "Value of field is not the same as matching field in the state."
            }
            //
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
            //
            // Invalid field value
            OpenProtocolError::InvalidField { field, value, description }
                if description.is_empty() =>
            {
                write!(f, "Value [{}] is invalid for the field {}", value, field)
            }
            OpenProtocolError::InvalidField { field, value, description } => {
                write!(f, "Value [{}] is invalid for the field {}: {}.", value, field, description)
            }
            //
            // Constraint violation
            OpenProtocolError::ConstraintViolated(err) => err.fmt(f),
            //
            // Inconsistent field value
            OpenProtocolError::InconsistentField(field) => {
                write!(f, "Value of field {} is not the same as the matching field in the Controller.", field)
            }
            //
            // Inconsistent state value
            OpenProtocolError::InconsistentState(field) => {
                write!(f, "Value of field {} is not the same as the matching field in state.", field)
            }
            //
            // Field empty
            OpenProtocolError::EmptyField(field) => {
                write!(f, "Field {} cannot be empty or all white-space.", field)
            }
        }
    }
}
