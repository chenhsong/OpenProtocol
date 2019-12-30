use super::{TextName, ID};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// A data structure containing information on a single user on the system.
///
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator<'a> {
    /// Unique user ID, which cannot be zero.
    pub(crate) operator_id: ID,
    //
    /// Name of the user.
    #[serde(borrow)]
    pub(crate) operator_name: Option<TextName<'a>>,
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
    /// let opr = Operator::try_new_with_name(ID::from_u32(12345), "John")?;
    /// assert_eq!(Some("John"), opr.name());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn name(&self) -> Option<&str> {
        self.operator_name.as_ref().map(|name| name.as_ref())
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
    /// let result = Operator::try_new_with_name(ID::from_u32(12345), "");
    /// assert_eq!(Err("invalid value: a non-empty, non-whitespace string required for operator name".into()), result);
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let opr = Operator::try_new_with_name(ID::from_u32(12345), "John")?;
    /// assert_eq!(12345, opr.id());
    /// assert_eq!(Some("John"), opr.name());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn try_new_with_name(id: ID, name: &'a str) -> std::result::Result<Self, String> {
        Ok(Self {
            operator_name: Some(name.try_into().map_err(|e| format!("{} for operator name", e))?),
            ..Self::new(id)
        })
    }
}
