use super::utils::*;
use super::{JobMode, OpMode, ValidationResult, ID};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

/// A data structure containing a snapshot of the current known states of the controller.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateValues<'a> {
    /// Current operating mold of the controller.
    #[serde(skip_serializing_if = "OpMode::is_unknown")]
    #[serde(default)]
    pub(crate) op_mode: OpMode,
    //
    /// Current job mode of the controller.
    #[serde(skip_serializing_if = "JobMode::is_unknown")]
    #[serde(default)]
    pub(crate) job_mode: JobMode,
    //
    /// Unique ID of the current logged-in user (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) operator_id: Option<ID>,
    //
    /// Current active job ID (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub(crate) job_card_id: Option<Box<Cow<'a, str>>>,
    //
    /// Unique ID of the set of mold data currently loaded (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub(crate) mold_id: Option<Box<Cow<'a, str>>>,
}

impl<'a> StateValues<'a> {
    /// Get the op-mode.
    ///
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::borrow::Cow;
    /// # fn main() -> std::result::Result<(), String> {
    /// let state = StateValues::new_with_all::<&str, _>(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     None,
    ///     Some("M001")
    /// )?;
    ///
    /// assert_eq!(OpMode::Automatic, state.op_mode());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn op_mode(&self) -> OpMode {
        self.op_mode
    }

    /// Get the job mode.
    ///
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::borrow::Cow;
    /// # fn main() -> std::result::Result<(), String> {
    /// let state = StateValues::new_with_all::<&str, _>(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     None,
    ///     Some("M001")
    /// )?;
    ///
    /// assert_eq!(JobMode::ID02, state.job_mode());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn job_mode(&self) -> JobMode {
        self.job_mode
    }

    /// Get the operator ID, if any.
    ///
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::borrow::Cow;
    /// # fn main() -> std::result::Result<(), String> {
    /// let state = StateValues::new_with_all::<&str, _>(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     None,
    ///     Some("M001")
    /// )?;
    ///
    /// assert_eq!(123, state.operator_id().unwrap());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn operator_id(&self) -> Option<ID> {
        self.operator_id
    }

    /// Get the job card ID, if any.
    ///
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::borrow::Cow;
    /// # fn main() -> std::result::Result<(), String> {
    /// let state = StateValues::new_with_all::<&str, _>(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     None,
    ///     Some("M001")
    /// )?;
    ///
    /// assert_eq!(None, state.job_card_id());
    /// # Ok(())
    /// # }
    /// ~~~
    #[allow(clippy::borrowed_box)]
    pub fn job_card_id(&self) -> Option<&Box<Cow<'a, str>>> {
        self.job_card_id.as_ref()
    }

    /// Get the mold ID, if any.
    ///
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::borrow::Cow;
    /// # fn main() -> std::result::Result<(), String> {
    /// let state = StateValues::new_with_all::<&str, _>(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     None,
    ///     Some("M001")
    /// )?;
    ///
    /// assert_eq!(Some(&Box::new(Cow::Borrowed("M001"))), state.mold_id());
    /// # Ok(())
    /// # }
    /// ~~~
    #[allow(clippy::borrowed_box)]
    pub fn mold_id(&self) -> Option<&Box<Cow<'a, str>>> {
        self.mold_id.as_ref()
    }

    /// Create a new `StateValues` wth no operator ID, job card ID and mold ID.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let state = StateValues::new(OpMode::Automatic, JobMode::ID02);
    /// assert_eq!(OpMode::Automatic, state.op_mode());
    /// assert_eq!(JobMode::ID02, state.job_mode());
    /// assert_eq!(None, state.operator_id());
    /// assert_eq!(None, state.job_card_id());
    /// assert_eq!(None, state.mold_id());
    /// ~~~
    pub fn new(op: OpMode, job: JobMode) -> Self {
        Self { op_mode: op, job_mode: job, operator_id: None, job_card_id: None, mold_id: None }
    }

    /// Create a new `StateValues` with all fields set.
    ///
    /// # Note
    ///
    /// If either `job_card_id` or `mold_id` is `None`, then you need to use the
    /// turbo-fish syntax to specify the data type for the `None` parameter,
    /// because the compiler won't be able to figure out the underlying type
    /// if it doesn't exist!
    ///
    /// For example, if you pass a `Some(String)` to `job_card_id` and `None` to
    /// `mold_id`, you need to call with:
    ///
    /// > `new_with_all::<String, String>` or `new_with_all::<_, String>`  
    /// > `new_with_all::<String, &str>` or `new_with_all::<_, &str>`
    ///
    /// Any type will work fine, as long as it can be converted into `Cow<'_, str>`.
    ///
    /// However, avoid type combinations that are not used anywhere else in your
    /// code, otherwise a new function will be unnecessarily instantiated just for
    /// this constructor all.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if `job_card_id` or `mold_id` is set to an empty string
    /// or is all whitespace.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let result = StateValues::new_with_all::<_, &str>(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     Some(""),    // <-- Notice empty string for job_Card_id which is not allowed
    ///     None
    /// );
    /// assert_eq!(Err("job_card_id cannot be empty or all whitespace".into()), result);
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::borrow::Cow;
    /// # fn main() -> std::result::Result<(), String> {
    /// let state = StateValues::new_with_all::<&str, _>(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     None,
    ///     Some("M001")
    /// )?;
    ///
    /// assert_eq!(OpMode::Automatic, state.op_mode());
    /// assert_eq!(JobMode::ID02, state.job_mode());
    /// assert_eq!(123, state.operator_id().unwrap());
    /// assert_eq!(None, state.job_card_id());
    /// assert_eq!(Some(&Box::new(Cow::Borrowed("M001"))), state.mold_id());
    /// # Ok(())
    /// # }
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    ///
    pub fn new_with_all<S, T>(
        op: OpMode,
        job: JobMode,
        operator_id: Option<ID>,
        job_card_id: Option<S>,
        mold_id: Option<T>,
    ) -> std::result::Result<Self, String>
    where
        S: Into<Cow<'a, str>> + Deref,
        T: Into<Cow<'a, str>> + Deref,
        S::Target: AsRef<str>,
        T::Target: AsRef<str>,
    {
        check_optional_str_empty(&job_card_id, "job_card_id")
            .map_err(|_| "job_card_id cannot be empty or all whitespace")?;

        check_optional_str_empty(&mold_id, "mold_id")
            .map_err(|_| "mold_id cannot be empty or all whitespace")?;

        Ok(Self {
            operator_id,
            job_card_id: job_card_id.map(|j| j.into()).map(Box::new),
            mold_id: mold_id.map(|m| m.into()).map(Box::new),
            ..Self::new(op, job)
        })
    }

    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if `job_card_id` or `mold_id`
    /// is set to an empty string or is all whitespace.
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    ///
    pub(crate) fn validate(&self) -> ValidationResult {
        check_optional_str_empty(&self.job_card_id, "job_card_id")?;
        check_optional_str_empty(&self.mold_id, "mold_id")
    }
}

impl Default for StateValues<'_> {
    /// Default value of `StateValues`.
    ///
    fn default() -> Self {
        Self {
            op_mode: OpMode::Unknown,
            job_mode: JobMode::Unknown,
            operator_id: None,
            job_card_id: None,
            mold_id: None,
        }
    }
}
