use super::TextName;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A data structure containing information on a production job (i.e. a *job card*).
///
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobCard<'a> {
    /// Unique job ID, which must not be empty or all white-spaces.
    #[serde(borrow)]
    pub(crate) job_card_id: TextName<'a>,
    //
    /// ID of the set of mold data to load for this job.
    #[serde(borrow)]
    pub(crate) mold_id: TextName<'a>,
    //
    /// Current production progress, which must not be larger than `total`.
    pub(crate) progress: u32,
    //
    /// Total production count ordered.
    pub(crate) total: u32,
}

impl<'a> JobCard<'a> {
    /// Get the job ID.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let jc = JobCard::try_new("J001", "Mold#001", 100, 1000)?;
    /// assert_eq!("J001", jc.job_card_id());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn job_card_id(&self) -> &str {
        self.job_card_id.as_ref()
    }

    /// Get the mold ID.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let jc = JobCard::try_new("J001", "Mold#001", 100, 1000)?;
    /// assert_eq!("Mold#001", jc.mold_id());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn mold_id(&self) -> &str {
        self.mold_id.as_ref()
    }

    /// Get the production progress.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let jc = JobCard::try_new("J001", "Mold#001", 100, 1000)?;
    /// assert_eq!(100, jc.progress());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn progress(&self) -> u32 {
        self.progress
    }

    /// Get the maximum production order.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let jc = JobCard::try_new("J001", "Mold#001", 100, 1000)?;
    /// assert_eq!(1000, jc.total());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn total(&self) -> u32 {
        self.total
    }

    /// Create a new `JobCard` with the specified field values.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` is there is an error in the parameters.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert_eq!(
    ///     Err("job card ID cannot be empty or all-whitespace".into()),
    ///     JobCard::try_new("", "Mold#001", 0, 10000)
    /// );
    ///
    /// assert_eq!(
    ///     Err("mold ID cannot be empty or all-whitespace".into()),
    ///     JobCard::try_new("J001", "   ", 0, 10000)
    /// );
    ///
    /// assert_eq!(
    ///     Err("progress cannot be larger than total".into()),
    ///     JobCard::try_new("J001", "Mold#001", 1000, 100)
    /// );
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let jobs = vec![
    ///     JobCard::try_new("J001", "Mold#001", 0, 10000)?,
    ///     JobCard::try_new("J002".to_string(), "Mold#002".to_string(), 1000, 5000)?,
    ///     JobCard::try_new("J003", "Mold#003".to_string(), 42, 1000)?,
    ///     JobCard::try_new("J004".to_string(), "Mold#004", 0, 0)?,
    /// ];
    ///
    /// assert_eq!(4, jobs.len());
    /// assert_eq!("J002", jobs[1].job_card_id());
    /// assert_eq!(1000, jobs[2].total());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn try_new<S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        id: S,
        mold: T,
        progress: u32,
        total: u32,
    ) -> std::result::Result<Self, String> {
        if progress > total {
            return Err("progress cannot be larger than total".into());
        }

        Ok(Self {
            job_card_id: match TextName::new_from_str(id) {
                Some(jc) => jc,
                None => return Err("job card ID cannot be empty or all-whitespace".into()),
            },
            mold_id: match TextName::new_from_str(mold) {
                Some(m) => m,
                None => return Err("mold ID cannot be empty or all-whitespace".into()),
            },
            progress,
            total,
        })
    }
}
