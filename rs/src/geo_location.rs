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
    pub geo_latitude: f32,
    //
    /// Longitude
    pub geo_longitude: f32,
}

impl GeoLocation {
    /// Create a new `GeoLocation`.
    ///
    /// # Errors
    ///
    /// Returns `Err(&'static str)` if either the provided latitude or longitude
    /// is invalid as a geo-location coordinate.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert_eq!(
    ///     Err("latitude must be between -90 and +90"),
    ///     GeoLocation::new(123.456, -987.654)
    /// );
    /// assert_eq!(
    ///     Err("longitude must be between -180 and +180"),
    ///     GeoLocation::new(12.345, -987.654)
    /// );
    /// assert_eq!(
    ///     Err("latitude must be between -90 and +90"),
    ///     GeoLocation::new(std::f32::INFINITY, 0.0)
    /// );
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), &'static str> {
    /// let geo = GeoLocation::new(12.345, -98.765)?;
    /// assert_eq!(12.345, geo.geo_latitude);
    /// assert_eq!(-98.765, geo.geo_longitude);
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn new(latitude: f32, longitude: f32) -> std::result::Result<Self, &'static str> {
        if latitude.is_nan()
            || latitude.is_infinite()
            || !latitude.is_normal()
            || !(-90.0..=90.0).contains(&latitude)
        {
            return Err("latitude must be between -90 and +90");
        }
        if longitude.is_nan()
            || longitude.is_infinite()
            || !longitude.is_normal()
            || !(-180.0..=180.0).contains(&longitude)
        {
            return Err("longitude must be between -180 and +180");
        }

        Ok(GeoLocation { geo_latitude: latitude, geo_longitude: longitude })
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
    /// let geo1 = GeoLocation { geo_latitude: 23.456, geo_longitude: std::f32::NEG_INFINITY };
    /// assert_eq!(
    ///     Err(Error::InvalidField {
    ///         field: "geo_longitude",
    ///         value: "-inf".into(),
    ///         description: "Infinity is not a supported value".into()
    ///     }),
    ///     geo1.validate()
    /// );
    ///
    /// let geo2 = GeoLocation { geo_latitude: 123.456, geo_longitude: 987.654 };
    /// assert_eq!(
    ///     Err(Error::ConstraintViolated("latitude out-of-bounds: 123.456 (must be between -90 and 90)".into())),
    ///     geo2.validate()
    /// );
    /// ~~~
    ///
    /// [`OpenProtocolError::InvalidField`]: enum.OpenProtocolError.html#variant.InvalidField
    /// [`OpenProtocolError::ConstraintViolated`]: enum.OpenProtocolError.html#variant.ConstraintViolated
    ///
    pub fn validate(self) -> ValidationResult {
        check_f64(self.geo_latitude.into(), "geo_latitude")?;

        if !(-90.0..=90.0).contains(&self.geo_latitude) {
            return Err(Error::ConstraintViolated(
                format!(
                    "latitude out-of-bounds: {} (must be between -90 and 90)",
                    self.geo_latitude
                )
                .into(),
            ));
        }

        check_f64(self.geo_longitude.into(), "geo_longitude")?;

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
