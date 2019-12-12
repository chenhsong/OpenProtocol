use super::utils::*;
use super::{Error, ValidationResult};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A data structure containing information on a production job (i.e. a *job card*).
///
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobCard<'a> {
    /// Unique job ID, which must not be empty or all white-spaces.
    #[serde(borrow)]
    pub job_card_id: Cow<'a, str>,
    //
    /// ID of the set of mold data to load for this job.
    #[serde(borrow)]
    pub mold_id: Cow<'a, str>,
    //
    /// Current production progress, which must not be larger than `total`.
    pub progress: u32,
    //
    /// Total production count ordered.
    pub total: u32,
}

impl<'a> JobCard<'a> {
    /// Create a new `JobCard` with the specified field values.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let jobs = vec![
    ///     JobCard::new("J001".into(), "Mold#001".into(), 0, 10000),
    ///     JobCard::new("J002".into(), "Mold#002".into(), 1000, 5000),
    ///     JobCard::new("J003".into(), "Mold#003".into(), 42, 1000),
    ///     JobCard::new("J004".into(), "Mold#004".into(), 0, 0),
    /// ];
    ///
    /// assert_eq!(4, jobs.len());
    /// assert_eq!("J002", jobs[1].job_card_id);
    /// assert_eq!(1000, jobs[2].total);
    /// ~~~
    pub fn new(id: &'a str, mold: &'a str, progress: u32, total: u32) -> Self {
        Self { job_card_id: id.into(), mold_id: mold.into(), progress, total }
    }

    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if `job_card_id` or `mold_id`
    /// is set to an empty string or is all whitespace.
    ///
    /// Returns `Err(`[`OpenProtocolError::ConstraintViolated`]`)` if `progress` is larger
    /// than `total`.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let job1 = JobCard::new("".into(), "M1".into(), 0, 10000);
    /// let job2 = JobCard::new("J2".into(), "   ".into(), 0, 10000);
    /// let job3 = JobCard::new("J3".into(), "M3".into(), 50000, 10000);
    ///
    /// assert_eq!(Err(Error::EmptyField("job_card_id")), job1.validate());
    /// assert_eq!(Err(Error::EmptyField("mold_id")), job2.validate());
    /// assert_eq!(
    ///     Err(Error::ConstraintViolated("job-card progress (50000) must not be larger than the total production count (10000)".into())),
    ///     job3.validate()
    /// );
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    /// [`OpenProtocolError::ConstraintViolated`]: enum.OpenProtocolError.html#variant.ConstraintViolated
    ///
    pub fn validate(&self) -> ValidationResult {
        check_str_empty(&self.job_card_id, "job_card_id")?;
        check_str_empty(&self.mold_id, "mold_id")?;

        if self.progress > self.total {
            return Err(Error::ConstraintViolated(
                format!(
                    "job-card progress ({}) must not be larger than the total production count ({})",
                    self.progress, self.total
                )
                .into(),
            ));
        }

        Ok(())
    }
}
