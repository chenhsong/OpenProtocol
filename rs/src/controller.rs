use self::utils::*;
use super::*;
use chrono::{DateTime, FixedOffset};
use lazy_static::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::net::IpAddr;
use std::num::NonZeroU32;
use std::str::FromStr;

/// A data structure containing information on a single user on the system.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator<'a> {
    /// Unique user ID, which cannot be zero.
    pub operator_id: NonZeroU32,
    //
    /// Name of the user.
    pub operator_name: Option<&'a str>,
}

impl<'a> Operator<'a> {
    /// Create an `Operator` with just an ID and no name.
    ///
    /// # Panics
    ///
    /// Panics if `id` is zero.
    ///
    pub fn new(id: u32) -> Self {
        Self { operator_id: NonZeroU32::new(id).unwrap(), operator_name: None }
    }

    /// Create an `Operator` with name.
    ///
    /// # Panics
    ///
    /// Panics if `id` is zero.
    ///
    pub fn new_with_name(id: u32, name: &'a str) -> Self {
        Self { operator_name: Some(name), ..Self::new(id) }
    }

    /// Validate the data structure.
    ///
    pub fn validate(&self) -> Result<'static, ()> {
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
    pub fn new(latitude: f64, longitude: f64) -> Self {
        GeoLocation { geo_latitude: latitude, geo_longitude: longitude }
    }

    /// Validate the data structure.
    ///
    pub fn validate(&self) -> Result<'static, ()> {
        check_f64(self.geo_latitude, "geo_latitude")?;
        check_f64(self.geo_longitude, "geo_longitude")
    }
}

/// A data structure containing the current known status of a controller.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller<'a> {
    /// Unique ID of the controller, which cannot be zero.
    pub controller_id: NonZeroU32,
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
    pub fn validate(&self) -> Result<'a, ()> {
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
                return Err(OpenProtocolError::InvalidField {
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
                    return Err(OpenProtocolError::InvalidField {
                        field: "ip[address]".into(),
                        value: address.into(),
                        description: format!("{} ({})", address, err.description()).into(),
                    })
                }
            }

            // Allow port 0 on unspecified addresses only
            let port = &port[1..];

            match u16::from_str(port) {
                Ok(n) => {
                    if n == 0 && !unspecified {
                        return Err(OpenProtocolError::InvalidField {
                            field: "ip[port]".into(),
                            value: port.into(),
                            description: "IP port cannot be zero.".into(),
                        });
                    } else if n > 0 && unspecified {
                        return Err(OpenProtocolError::InvalidField {
                            field: "ip[port]".into(),
                            value: port.into(),
                            description: "Null IP must have zero port number.".into(),
                        });
                    }
                }
                Err(err) => {
                    return Err(OpenProtocolError::InvalidField {
                        field: "ip[port]".into(),
                        value: port.into(),
                        description: err.description().to_string().into(),
                    })
                }
            }
        }

        Ok(())
    }
}

impl Default for Controller<'_> {
    fn default() -> Self {
        Controller {
            controller_id: NonZeroU32::new(1).unwrap(),
            display_name: "Unknown",
            controller_type: "Unknown",
            version: "Unknown",
            model: "Unknown",
            address: "0.0.0.0:0",
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

    #[test]
    fn test_controller_to_json() {
        let c = Controller {
            op_mode: OpMode::Automatic,
            job_mode: JobMode::ID02,
            operator: Some(Operator::new_with_name(123, "John")),
            ..Default::default()
        };
        c.validate().unwrap();
        let serialized = serde_json::to_string(&c).unwrap();
        assert_eq!(
            r#"{"controllerId":1,"displayName":"Unknown","controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"0.0.0.0:0","opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#,
            serialized);
    }

    #[test]
    fn test_controller_from_json() {
        let c: Controller = serde_json::from_str(r#"{"controllerId":1,"displayName":"Hello","controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"127.0.0.1:123","opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#).unwrap();
        c.validate().unwrap();

        assert_eq!(
            r#"Controller { controller_id: 1, display_name: "Hello", controller_type: "Unknown", version: "Unknown", model: "Unknown", address: "127.0.0.1:123", geo_location: None, op_mode: Automatic, job_mode: ID02, last_cycle_data: None, variables: None, last_connection_time: None, operator: Some(Operator { operator_id: 123, operator_name: Some("John") }), job_card_id: None, mold_id: None }"#,
            format!("{:?}", &c));
    }

    #[test]
    fn test_controller_validate() {
        let c: Controller = Default::default();
        c.validate().unwrap();
    }

    #[test]
    fn test_operator_validate() {
        Operator { operator_id: NonZeroU32::new(123).unwrap(), operator_name: Some("John") }
            .validate()
            .unwrap();
    }

    #[test]
    fn test_controller_validate_address() {
        let mut c: Controller = Default::default();

        // 1.02.003.004:05
        c.address = "1.02.003.004:05";
        c.validate().unwrap();

        // 1.02.003.004:0 - should fail
        c.address = "1.02.003.004:0";
        assert!(c.validate().is_err());

        // 0.0.0.0:0
        c.address = "0.0.0.0:0";
        c.validate().unwrap();

        // 0.0.0.0:123 - should fail
        c.address = "0.0.0.0:123";
        assert!(c.validate().is_err());

        // COM123
        c.address = "COM123";
        c.validate().unwrap();

        // ttyABC
        c.address = "ttyABC";
        c.validate().unwrap();
    }
}
