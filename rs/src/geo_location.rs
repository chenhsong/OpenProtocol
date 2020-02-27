use super::utils::*;
use super::R32;
use derive_more::*;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

/// A data structure containing a single physical geo-location.
///
#[derive(Display, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize)]
#[display(fmt = "({},{})", geo_latitude, geo_longitude)]
#[serde(try_from = "GeoWrapper", into = "GeoWrapper")]
pub struct GeoLocation {
    /// Latitude
    geo_latitude: R32,
    //
    /// Longitude
    geo_longitude: R32,
}

impl std::fmt::Debug for GeoLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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
    /// number, or when they together do not represent a valid geo-location position.
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
        check_f32(latitude).map_err(|e| format!("{} for latitude", e))?;
        check_f32(longitude).map_err(|e| format!("{} for longitude", e))?;

        Self::check_constraints(latitude, longitude)?;

        Ok(Self {
            geo_latitude: latitude.try_into().unwrap(),
            geo_longitude: longitude.try_into().unwrap(),
        })
    }

    // Check if the latitude/longitude pair is with constraints.
    fn check_constraints(latitude: f32, longitude: f32) -> Result<(), String> {
        if !(-90.0..=90.0).contains(&latitude) {
            Err(format!("invalid latitude: {} (must be between -90 and 90)", latitude))
        } else if !(-180.0..=180.0).contains(&longitude) {
            Err(format!("invalid longitude: {} (must be between -180 and 180)", longitude))
        } else {
            Ok(())
        }
    }
}

// Wrapper for serialization/deserialization
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeoWrapper {
    pub geo_latitude: f32,
    pub geo_longitude: f32,
}

impl TryFrom<GeoWrapper> for GeoLocation {
    type Error = String;

    fn try_from(value: GeoWrapper) -> Result<Self, Self::Error> {
        Self::new(value.geo_latitude, value.geo_longitude)
    }
}

impl From<GeoLocation> for GeoWrapper {
    fn from(value: GeoLocation) -> Self {
        Self { geo_latitude: value.latitude(), geo_longitude: value.longitude() }
    }
}
