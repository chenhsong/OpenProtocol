use super::utils::*;
use super::{BoundedValidationResult, Error, JobMode, OpMode, ValidationResult, ID};
use chrono::{DateTime, FixedOffset};
use lazy_static::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

/// A data structure containing information on a single user on the system.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator<'a> {
    /// Unique user ID, which cannot be zero.
    pub operator_id: ID,
    //
    /// Name of the user.
    pub operator_name: Option<&'a str>,
}

impl<'a> Operator<'a> {
    /// Create an `Operator` with just an ID and no name.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use std::convert::TryFrom;
    /// # use ichen_openprotocol::*;
    /// let op = Operator::new(ID::try_from(12345).unwrap());
    /// assert_eq!(12345, u32::from(op.operator_id));
    /// assert_eq!(None, op.operator_name);
    /// ~~~
    pub fn new(id: ID) -> Self {
        Self { operator_id: id, operator_name: None }
    }

    /// Create an `Operator` with ID and name.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use std::convert::TryFrom;
    /// # use ichen_openprotocol::*;
    /// let op = Operator::new_with_name(ID::try_from(12345).unwrap(), "John");
    /// assert_eq!(12345, u32::from(op.operator_id));
    /// assert_eq!(Some("John"), op.operator_name);
    /// ~~~
    pub fn new_with_name(id: ID, name: &'a str) -> Self {
        Self { operator_name: Some(name), ..Self::new(id) }
    }

    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if the `operator_name` field is
    /// set to an empty string or is all whitespace.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use std::convert::TryFrom;
    /// # use ichen_openprotocol::*;
    ///
    /// let op = Operator::new_with_name(ID::try_from(12345).unwrap(), "");
    /// assert_eq!(r#"Err(EmptyField("operator_name"))"#, format!("{:?}", op.validate()));
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use std::convert::TryFrom;
    /// # use ichen_openprotocol::*;
    ///
    /// let op = Operator::new_with_name(ID::try_from(12345).unwrap(), "John");
    /// assert_eq!(Ok(()), op.validate());
    /// ~~~
    pub fn validate(&self) -> ValidationResult {
        check_optional_str_empty(&self.operator_name, "operator_name")
    }
}

/// A data structure containing a single physical geo-location.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    ///     r#"Err(InvalidField { field: "geo_longitude", value: "-inf", description: "Infinity is not a supported value" })"#,
    ///     format!("{:?}", geo1.validate())
    /// );
    ///
    /// let geo2 = GeoLocation::new(123.456, 987.654);
    /// assert_eq!(
    ///     r#"Err(ConstraintViolated("latitude (123.456) must be between -90 and 90"))"#,
    ///     format!("{:?}", geo2.validate())
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
                format!("latitude ({}) must be between -90 and 90", self.geo_latitude).into(),
            ));
        }

        check_f64(self.geo_longitude, "geo_longitude")?;

        if !(-180.0..=180.0).contains(&self.geo_longitude) {
            return Err(Error::ConstraintViolated(
                format!("longitude ({}) must be between -180 and 180", self.geo_longitude).into(),
            ));
        }

        Ok(())
    }
}

/// A data structure containing the current known status of a controller.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller<'a> {
    /// Unique ID of the controller, which cannot be zero.
    pub controller_id: ID,
    //
    /// User-specified human-friendly name for the machine.
    pub display_name: &'a str,
    //
    /// Controller type.
    ///
    /// # Examples
    ///
    /// * `Ai01`
    /// * `Ai12`
    /// * `CDC2000WIN`
    /// * `MPC7`
    pub controller_type: &'a str,
    //
    /// Version of the controller's firmware.
    pub version: &'a str,
    //
    /// Machine model.
    pub model: &'a str,
    //
    /// Address of the controller.
    ///
    /// For a network-connected controller, this is usually the IP address and port, in the format `x.x.x.x:port`.
    ///
    /// For a serial-connected controller, this is usually the serial port device name, such as `COM1`, `ttyS0`.
    #[serde(rename = "IP")]
    pub address: &'a str,
    //
    /// Physical geo-location of the controller (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub geo_location: Option<GeoLocation>,
    //
    /// Current operating mode of the controller.
    pub op_mode: OpMode,
    //
    /// Current job mode of the controller.
    pub job_mode: JobMode,
    //
    /// Last set of cycle data (if any) received from the controller.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub last_cycle_data: HashMap<&'a str, f64>,
    //
    /// Last-known states (if any) of controller variables.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub variables: HashMap<&'a str, f64>,
    //
    /// Time of last connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_time: Option<DateTime<FixedOffset>>,
    //
    /// Current logged-in user (if any) on the controller
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator<'a>>,
    //
    /// Active job ID (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub job_card_id: Option<Cow<'a, str>>,
    //
    /// ID of the set of mold data currently loaded (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub mold_id: Option<Cow<'a, str>>,
}

impl<'a> Controller<'a> {
    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError`]`)` if there are any errors or inconsistencies.
    ///
    /// [`OpenProtocolError`]: enum.OpenProtocolError.html
    ///
    /// # Examples
    ///
    /// ## Default values should pass validation
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let c: Controller = Default::default();
    /// assert_eq!(Ok(()), c.validate());
    /// ~~~
    ///
    /// ## Address validation
    /// ~~~
    /// # use ichen_openprotocol::*;
    ///
    /// // 1.02.003.004:05
    /// let mut c = Controller {
    ///     address: "1.02.003.004:05",
    ///     .. Default::default()
    /// };
    /// assert_eq!(Ok(()), c.validate());
    ///
    /// // 1.02.003.004:0 - should fail because port cannot be zero if IP address is not zero
    /// c.address = "1.02.003.004:0";
    /// assert_eq!(
    ///     r#"Err(InvalidField { field: "ip[port]", value: "0", description: "IP port cannot be zero" })"#,
    ///     format!("{:?}", c.validate()));
    ///
    /// // 0.0.0.0:0 - OK because both IP address and port are zero
    /// c.address = "0.0.0.0:0";
    /// assert_eq!(Ok(()), c.validate());
    ///
    /// // 0.0.0.0:123 - should fail because port must be zero if IP address is zero
    /// c.address = "0.0.0.0:123";
    /// assert_eq!(
    ///     r#"Err(InvalidField { field: "ip[port]", value: "123", description: "null IP must have zero port number" })"#,
    ///     format!("{:?}", c.validate()));
    ///
    /// // COM123
    /// c.address = "COM123";
    /// assert_eq!(Ok(()), c.validate());
    ///
    /// // ttyABC
    /// c.address = "ttyABC";
    /// assert_eq!(Ok(()), c.validate());
    /// ~~~
    pub fn validate(&self) -> BoundedValidationResult<'a> {
        // String fields should not be empty
        check_str_empty(self.controller_type, "controller_type")?;
        check_str_empty(self.display_name, "display_name")?;
        check_str_empty(self.version, "version")?;
        check_str_empty(self.model, "version")?;
        check_optional_str_empty(&self.job_card_id, "job_card_id")?;
        check_optional_str_empty(&self.mold_id, "mold_id")?;

        // Check Geo-location
        if let Some(geo) = &self.geo_location {
            geo.validate()?;
        }

        // Check Operator
        if let Some(op) = &self.operator {
            op.validate()?;
        }

        // Check IP address
        check_str_empty(self.address, "address")?;

        lazy_static! {
            static ref IP_REGEX: Regex =
                Regex::new(r#"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$"#).unwrap();
            static ref TTY_REGEX: Regex = Regex::new(r#"^tty\w+$"#).unwrap();
            static ref COM_REGEX: Regex = Regex::new(r#"^COM(\d+)$"#).unwrap();
        }

        if !IP_REGEX.is_match(self.address) {
            if !TTY_REGEX.is_match(self.address) && !COM_REGEX.is_match(self.address) {
                return Err(Error::InvalidField {
                    field: "ip".into(),
                    value: self.address.into(),
                    description: "".into(),
                });
            }
        } else {
            // Check IP address validity
            let (address, port) = self.address.split_at(self.address.find(':').unwrap());

            let unspecified: bool;

            match IpAddr::from_str(address) {
                Ok(addr) => unspecified = addr.is_unspecified(),
                Err(err) => {
                    return Err(Error::InvalidField {
                        field: "ip[address]".into(),
                        value: address.into(),
                        description: {
                            use std::error::Error;
                            format!("{} ({})", address, err.description()).into()
                        },
                    })
                }
            }

            // Allow port 0 on unspecified addresses only
            let port = &port[1..];

            match u16::from_str(port) {
                Ok(n) => {
                    if n == 0 && !unspecified {
                        return Err(Error::InvalidField {
                            field: "ip[port]".into(),
                            value: port.into(),
                            description: "IP port cannot be zero".into(),
                        });
                    } else if n > 0 && unspecified {
                        return Err(Error::InvalidField {
                            field: "ip[port]".into(),
                            value: port.into(),
                            description: "null IP must have zero port number".into(),
                        });
                    }
                }
                Err(err) => {
                    return Err(Error::InvalidField {
                        field: "ip[port]".into(),
                        value: port.into(),
                        description: {
                            use std::error::Error;
                            err.description().to_string().into()
                        },
                    })
                }
            }
        }

        Ok(())
    }
}

impl Default for Controller<'_> {
    /// Default value for `Controller`.
    ///
    fn default() -> Self {
        Controller {
            controller_id: ID::from_u32(1),
            display_name: "Unknown",
            controller_type: "Unknown",
            version: "Unknown",
            model: "Unknown",
            address: "0.0.0.0:0", // Address can be either a valid IP address/port or all-zero
            geo_location: None,
            op_mode: OpMode::Unknown,
            job_mode: JobMode::Unknown,
            job_card_id: None,
            last_cycle_data: Default::default(),
            variables: Default::default(),
            last_connection_time: None,
            operator: None,
            mold_id: None,
        }
    }
}

// Tests

#[cfg(test)]
mod test {
    use super::*;
    use std::result::Result;

    #[test]
    fn test_controller_to_json() -> Result<(), String> {
        let c = Controller {
            op_mode: OpMode::Automatic,
            job_mode: JobMode::ID02,
            operator: Some(Operator::new_with_name(ID::from_u32(123), "John")),
            ..Default::default()
        };
        c.validate()?;
        let serialized = serde_json::to_string(&c).map_err(|x| x.to_string())?;
        assert_eq!(
            r#"{"controllerId":1,"displayName":"Unknown","controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"0.0.0.0:0","opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#,
            serialized);

        Ok(())
    }

    #[test]
    fn test_controller_from_json() -> Result<(), String> {
        let c: Controller = serde_json::from_str(r#"{"controllerId":1,"displayName":"Hello","controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"127.0.0.1:123","opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#).map_err(|x| x.to_string())?;
        c.validate()?;

        assert_eq!(
            r#"Controller { controller_id: 1, display_name: "Hello", controller_type: "Unknown", version: "Unknown", model: "Unknown", address: "127.0.0.1:123", geo_location: None, op_mode: Automatic, job_mode: ID02, last_cycle_data: {}, variables: {}, last_connection_time: None, operator: Some(Operator { operator_id: 123, operator_name: Some("John") }), job_card_id: None, mold_id: None }"#,
            format!("{:?}", &c));

        Ok(())
    }
}
