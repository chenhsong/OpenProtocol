use super::utils::*;
use super::{ValidationResult, ID};
use serde::{Deserialize, Serialize};

/// A data structure containing information on a single user on the system.
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator<'a> {
    /// Unique user ID, which cannot be zero.
    pub operator_id: ID,
    //
    /// Name of the user.
    pub operator_name: Option<&'a str>,
}

impl<'a> Operator<'a> {
    /// Create an `Operator` with just an ID and no name.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let op = Operator::new(ID::from_u32(12345));
    /// assert_eq!(12345, u32::from(op.operator_id));
    /// assert_eq!(None, op.operator_name);
    /// ~~~
    pub fn new(id: ID) -> Self {
        Self { operator_id: id, operator_name: None }
    }

    /// Create an `Operator` with ID and name.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let op = Operator::new_with_name(ID::from_u32(12345), "John");
    /// assert_eq!(12345, u32::from(op.operator_id));
    /// assert_eq!(Some("John"), op.operator_name);
    /// ~~~
    pub fn new_with_name(id: ID, name: &'a str) -> Self {
        Self { operator_name: Some(name), ..Self::new(id) }
    }

    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if the `operator_name` field is
    /// set to an empty string or is all whitespace.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let op = Operator::new_with_name(ID::from_u32(12345), "");
    /// assert_eq!(
    ///     Err(Error::EmptyField("operator_name")),
    ///     op.validate()
    /// );
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), Error<'static>> {
    /// let op = Operator::new_with_name(ID::from_u32(12345), "John");
    /// op.validate()?;
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn validate(&self) -> ValidationResult {
        check_optional_str_empty(&self.operator_name, "operator_name")
    }
}
