use super::utils::*;
use super::{Error, ValidationResult};
use decorum::R32;
use derive_more::*;
use serde::{Deserialize, Serialize};

/// A data structure containing a single physical geo-location.
///
#[derive(Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[display(fmt = "({},{})", geo_latitude, geo_longitude)]
pub struct GeoLocation {
    /// Latitude
    #[serde(serialize_with = "serialize_r32")]
    #[serde(deserialize_with = "deserialize_r32")]
    pub(crate) geo_latitude: R32,
    //
    /// Longitude
    #[serde(serialize_with = "serialize_r32")]
    #[serde(deserialize_with = "deserialize_r32")]
    pub(crate) geo_longitude: R32,
}

impl std::fmt::Debug for GeoLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl GeoLocation {
    /// Get the latitude.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let geo = GeoLocation::new(12.345, -98.765)?;
    /// assert_eq!(12.345, geo.latitude());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn latitude(self) -> f32 {
        self.geo_latitude.into()
    }

    /// Get the longitude
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let geo = GeoLocation::new(12.345, -98.765)?;
    /// assert_eq!(-98.765, geo.longitude());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn longitude(self) -> f32 {
        self.geo_longitude.into()
    }

    /// Create a new `GeoLocation`.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if either `latitude` or `longitude` is not a valid floating-point
    /// number, or when they together do not represent a valid Geo-Location position.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert_eq!(
    ///     Err("Infinity is not a supported value for longitude".into()),
    ///     GeoLocation::new(23.456, std::f32::NEG_INFINITY)
    /// );
    ///
    /// assert_eq!(
    ///     Err("invalid latitude: 123.456 (must be between -90 and 90)".into()),
    ///     GeoLocation::new(123.456, 987.654)
    /// );
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let geo = GeoLocation::new(12.345, -98.765)?;
    /// assert_eq!(12.345, geo.latitude());
    /// assert_eq!(-98.765, geo.longitude());
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn new(latitude: f32, longitude: f32) -> std::result::Result<Self, String> {
        use std::error::Error;

        check_f64(latitude.into(), "latitude")
            .map_err(|e| format!("{} for latitude", e.description()))?;

        check_f64(longitude.into(), "longitude")
            .map_err(|e| format!("{} for longitude", e.description()))?;

        let g = Self { geo_latitude: latitude.into(), geo_longitude: longitude.into() };
        g.validate().map_err(|e| e.description().to_string())?;
        Ok(g)
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
    /// [`OpenProtocolError::InvalidField`]: enum.OpenProtocolError.html#variant.InvalidField
    /// [`OpenProtocolError::ConstraintViolated`]: enum.OpenProtocolError.html#variant.ConstraintViolated
    ///
    pub(crate) fn validate(self) -> ValidationResult {
        if !(-90.0..=90.0).contains(&self.geo_latitude.into_inner()) {
            return Err(Error::ConstraintViolated(
                format!("invalid latitude: {} (must be between -90 and 90)", self.geo_latitude)
                    .into(),
            ));
        }

        if !(-180.0..=180.0).contains(&self.geo_longitude.into_inner()) {
            return Err(Error::ConstraintViolated(
                format!("invalid longitude: {} (must be between -180 and 180)", self.geo_longitude)
                    .into(),
            ));
        }

        Ok(())
    }
}
