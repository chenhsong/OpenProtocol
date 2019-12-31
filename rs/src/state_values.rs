use super::{JobMode, OpMode, TextName, ID};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// A data structure containing a snapshot of the current known states of the controller.
///
#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateValues<'a> {
    /// Current operating mold of the controller.
    #[serde(skip_serializing_if = "OpMode::is_unknown")]
    #[serde(default)]
    op_mode: OpMode,
    //
    /// Current job mode of the controller.
    #[serde(skip_serializing_if = "JobMode::is_unknown")]
    #[serde(default)]
    job_mode: JobMode,
    //
    /// Unique ID of the current logged-in user (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_id: Option<ID>,
    //
    /// Current active job ID (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    job_card_id: Option<Box<TextName<'a>>>,
    //
    /// Unique ID of the set of mold data currently loaded (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    mold_id: Option<Box<TextName<'a>>>,
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
    /// let state = StateValues::try_new_with_all(
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
    /// let state = StateValues::try_new_with_all(
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
    /// let state = StateValues::try_new_with_all(
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
    /// let state = StateValues::try_new_with_all(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     Some("JC001"),
    ///     None
    /// )?;
    ///
    /// assert_eq!(Some("JC001"), state.job_card_id());
    /// # Ok(())
    /// # }
    /// ~~~
    #[allow(clippy::borrowed_box)]
    pub fn job_card_id(&self) -> Option<&str> {
        self.job_card_id.as_ref().map(|jc| jc.get())
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
    /// let state = StateValues::try_new_with_all(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     None,
    ///     Some("M001")
    /// )?;
    ///
    /// assert_eq!(Some("M001"), state.mold_id());
    /// # Ok(())
    /// # }
    /// ~~~
    #[allow(clippy::borrowed_box)]
    pub fn mold_id(&self) -> Option<&str> {
        self.mold_id.as_ref().map(|m| m.get())
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
    /// # Errors
    ///
    /// Returns `Err(String)` if `job_card_id` or `mold_id` is set to an empty string
    /// or is all whitespace.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let sv = StateValues::try_new_with_all(
    ///     OpMode::Automatic,
    ///     JobMode::ID02,
    ///     Some(ID::from_u32(123)),
    ///     Some(""),    // <-- Notice empty string for job_Card_id which is not allowed
    ///     None
    /// );
    /// assert_eq!(Err("invalid value: a non-empty, non-whitespace string required for job card ID".into()), sv);
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::borrow::Cow;
    /// # fn main() -> std::result::Result<(), String> {
    /// let state = StateValues::try_new_with_all(
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
    /// assert_eq!(Some("M001"), state.mold_id());
    /// # Ok(())
    /// # }
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    ///
    pub fn try_new_with_all(
        op: OpMode,
        job: JobMode,
        operator_id: Option<ID>,
        job_card_id: Option<&'a str>,
        mold_id: Option<&'a str>,
    ) -> std::result::Result<Self, String> {
        let job_card_id = if let Some(jc) = job_card_id {
            Some(jc.try_into().map(Box::new).map_err(|e| format!("{} for job card ID", e))?)
        } else {
            None
        };

        let mold_id = if let Some(m) = mold_id {
            Some(m.try_into().map(Box::new).map_err(|e| format!("{} for mold ID", e))?)
        } else {
            None
        };

        Ok(Self { operator_id, job_card_id, mold_id, ..Self::new(op, job) })
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
