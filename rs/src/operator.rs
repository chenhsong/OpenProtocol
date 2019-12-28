use super::utils::*;
use super::{ValidationResult, ID};
use serde::{Deserialize, Serialize};

/// A data structure containing information on a single user on the system.
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator<'a> {
    /// Unique user ID, which cannot be zero.
    pub(crate) operator_id: ID,
    //
    /// Name of the user.
    pub(crate) operator_name: Option<&'a str>,
}

impl<'a> Operator<'a> {
    /// Get the operator ID.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let opr = Operator::new(ID::from_u32(12345));
    /// assert_eq!(12345, opr.id());
    /// ~~~
    pub fn id(&self) -> ID {
        self.operator_id
    }

    // Get the operator's name, if any.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let opr = Operator::new_with_name(ID::from_u32(12345), "John")?;
    /// assert_eq!(Some("John"), opr.name());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn name(&self) -> Option<&'a str> {
        self.operator_name
    }

    /// Create an `Operator` with just an ID and no name.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let opr = Operator::new(ID::from_u32(12345));
    /// assert_eq!(12345, opr.id());
    /// assert_eq!(None, opr.name());
    /// ~~~
    pub fn new(id: ID) -> Self {
        Self { operator_id: id, operator_name: None }
    }

    /// Create an `Operator` with ID and name.
    ///
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
    /// let result = Operator::new_with_name(ID::from_u32(12345), "");
    /// assert_eq!(Err("operator name cannot be empty or all whitespace".into()), result);
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let opr = Operator::new_with_name(ID::from_u32(12345), "John")?;
    /// assert_eq!(12345, opr.id());
    /// assert_eq!(Some("John"), opr.name());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn new_with_name(id: ID, name: &'a str) -> std::result::Result<Self, String> {
        check_str_empty(name, "name")
            .map_err(|_| "operator name cannot be empty or all whitespace")?;

        Ok(Self { operator_name: Some(name), ..Self::new(id) })
    }

    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if the `operator_name` field is
    /// set to an empty string or is all whitespace.
    pub(crate) fn validate(&self) -> ValidationResult {
        check_optional_str_empty(&self.operator_name, "operator_name")
    }
}
