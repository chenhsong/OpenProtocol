use super::utils::*;
use super::{Error, ValidationResult};
use derive_more::*;
use serde::{Deserialize, Serialize};

/// A data structure containing a single physical geo-location.
///
#[derive(Debug, Display, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[display(fmt = "({}, {})", geo_latitude, geo_longitude)]
pub struct GeoLocation {
    /// Latitude
    pub geo_latitude: f64,
    //
    /// Longitude
    pub geo_longitude: f64,
}

impl GeoLocation {
    /// Create a new `GeoLocation`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// // Notice this is an invalid geo-location position, but still works
    /// // To validate the data structure, call the `validate` method.
    /// let geo = GeoLocation::new(123.456, -987.654);
    /// assert_eq!(123.456, geo.geo_latitude);
    /// assert_eq!(-987.654, geo.geo_longitude);
    /// ~~~
    pub fn new(latitude: f64, longitude: f64) -> Self {
        GeoLocation { geo_latitude: latitude, geo_longitude: longitude }
    }

    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::InvalidField`]`)` if either `geo_latitude` or `geo_longitude`
    /// is not a valid floating-point number.
    ///
    /// Returns `Err(`[`OpenProtocolError::ConstraintViolated`]`)` if `geo_latitude` and `geo_longitude`
    /// together does not represent a valid Geo-Location position.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let geo1 = GeoLocation::new(23.456, std::f64::NEG_INFINITY);
    /// assert_eq!(
    ///     Err(Error::InvalidField {
    ///         field: "geo_longitude",
    ///         value: "-inf".into(),
    ///         description: "Infinity is not a supported value".into()
    ///     }),
    ///     geo1.validate()
    /// );
    ///
    /// let geo2 = GeoLocation::new(123.456, 987.654);
    /// assert_eq!(
    ///     Err(Error::ConstraintViolated("latitude out-of-bounds: 123.456 (must be between -90 and 90)".into())),
    ///     geo2.validate()
    /// );
    /// ~~~
    ///
    /// [`OpenProtocolError::InvalidField`]: enum.OpenProtocolError.html#variant.InvalidField
    /// [`OpenProtocolError::ConstraintViolated`]: enum.OpenProtocolError.html#variant.ConstraintViolated
    ///
    pub fn validate(&self) -> ValidationResult {
        check_f64(self.geo_latitude, "geo_latitude")?;

        if !(-90.0..=90.0).contains(&self.geo_latitude) {
            return Err(Error::ConstraintViolated(
                format!(
                    "latitude out-of-bounds: {} (must be between -90 and 90)",
                    self.geo_latitude
                )
                .into(),
            ));
        }

        check_f64(self.geo_longitude, "geo_longitude")?;

        if !(-180.0..=180.0).contains(&self.geo_longitude) {
            return Err(Error::ConstraintViolated(
                format!(
                    "longitude out-of-bounds: {} (must be between -180 and 180)",
                    self.geo_longitude
                )
                .into(),
            ));
        }

        Ok(())
    }
}
