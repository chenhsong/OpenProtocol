use self::utils::*;
use super::*;
use chrono::{DateTime, FixedOffset};
use lazy_static::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroU32;

/// A data structure containing information on a single user on the system.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    /// Unique user ID, which cannot be zero.
    pub operator_id: NonZeroU32,
    /// Name of the user.
    pub operator_name: Option<String>,
}

/// A data structure containing a single physical geo-location.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocation {
    /// Latitude
    pub geo_latitude: f64,
    /// Longitude
    pub geo_longitude: f64,
}

impl GeoLocation {
    fn check(&self) -> Result<()> {
        check_f64(&self.geo_latitude, "geo_latitude")?;
        check_f64(&self.geo_longitude, "geo_longitude")?;
        Ok(())
    }
}
/// A data structure containing the current known status of a controller.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    /// Unique ID of the controller, which cannot be zero.
    pub controller_id: NonZeroU32,
    //
    /// User-specified human-friendly name for the machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    //
    /// Controller type.
    ///
    /// # Examples
    ///
    /// * `Ai01`
    /// * `Ai12`
    /// * `CDC2000WIN`
    /// * `MPC7`
    pub controller_type: String,
    //
    /// Version of the controller's firmware.
    pub version: String,
    //
    /// Machine model.
    pub model: String,
    //
    /// Address of the controller.
    ///
    /// For a network-connected controller, this is usually the IP address and port, in the format `x.x.x.x:port`.
    ///
    /// For a serial-connected controller, this is usually the serial port device name, such as `COM1`, `ttyS0`.
    #[serde(rename = "IP")]
    pub address: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_cycle_data: Option<HashMap<String, f64>>,
    /// Last-known states (if any) of controller variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, f64>>,
    /// Time of last connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_time: Option<DateTime<FixedOffset>>,
    //
    /// Current logged-in user (if any) on the controller
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    //
    /// Active job ID (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_card_id: Option<String>,
    /// ID of the set of mold data currently loaded (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mold_id: Option<String>,
    /// Private field.
    #[serde(skip_serializing)]
    #[serde(default)]
    private: (),
}

impl Controller {
    pub(crate) fn check(&self) -> Result<()> {
        // String fields should not be empty
        check_string_empty(&self.controller_type, "controller_type")?;
        check_string_empty(&self.version, "version")?;
        check_string_empty(&self.model, "version")?;
        check_optional_string_empty(&self.job_card_id, "job_card_id")?;
        check_optional_string_empty(&self.mold_id, "mold_id")?;

        // Check Geo-location
        if let Some(geo) = &self.geo_location {
            geo.check()?;
        }

        // Check IP address
        check_string_empty(&self.address, "address")?;

        lazy_static! {
            static ref IP_REGEX: Regex = Regex::new(r#"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$"#).unwrap();
            static ref TTY_REGEX: Regex = Regex::new(r#"^tty\w+$"#).unwrap();
            static ref COM_REGEX: Regex = Regex::new(r#"^COM(\d+)$"#).unwrap();
        }

        if !IP_REGEX.is_match(&self.address) {
            if !TTY_REGEX.is_match(&self.address) {
                if COM_REGEX.is_match(&self.address) {
                    //self.ip = COM_REGEX.replace_all(&self.ip, "COM$port").to_string();
                } else {
                    return Err(OpenProtocolError::InvalidField(
                        Box::new("ip".to_string()),
                        Box::new(self.address.clone()),
                    ));
                }
            }
        }

        Ok(())
    }
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            controller_id: NonZeroU32::new(1).unwrap(),
            display_name: None,
            controller_type: "Unknown".to_string(),
            version: "Unknown".to_string(),
            model: "Unknown".to_string(),
            address: "0.0.0.0:0".to_string(),
            geo_location: None,
            op_mode: OpMode::Unknown,
            job_mode: JobMode::Unknown,
            job_card_id: None,
            last_cycle_data: None,
            variables: None,
            last_connection_time: None,
            operator: None,
            mold_id: None,
            private: (),
        }
    }
}

// Tests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_controller_serialize() {
        let c = Controller {
            op_mode: OpMode::Automatic,
            job_mode: JobMode::ID02,
            operator: Some(Operator {
                operator_id: NonZeroU32::new(123).unwrap(),
                operator_name: Some("John".to_string()),
            }),
            ..Default::default()
        };
        c.check().unwrap();
        let serialized = serde_json::to_string(&c).unwrap();
        assert_eq!(
            r#"{"controllerId":1,"controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"0.0.0.0:0","opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#,
            serialized);
    }

    #[test]
    fn test_controller_deserialize() {
        let c: Controller = serde_json::from_str(r#"{"controllerId":1,"controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"0.0.0.0:0","opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#).unwrap();
        c.check().unwrap();

        assert_eq!(
            r#"Controller { controller_id: 1, display_name: None, controller_type: "Unknown", version: "Unknown", model: "Unknown", ip: "0.0.0.0:0", geo_latitude: None, geo_longitude: None, op_mode: Automatic, job_mode: ID02, last_cycle_data: None, variables: None, last_connection_time: None, operator: Some(Operator { operator_id: 123, operator_name: Some("John") }), job_card_id: None, mold_id: None, private: () }"#,
            format!("{:?}", &c));
    }

    #[test]
    fn test_controller_check() {
        let c: Controller = Default::default();
        c.check().unwrap();
    }

    #[test]
    fn test_controller_check_operator() {
        let c = Controller {
            operator: Some(Operator {
                operator_id: NonZeroU32::new(123).unwrap(),
                operator_name: Some("John".to_string()),
            }),
            ..Default::default()
        };
        c.check().unwrap();
    }

    #[test]
    fn test_controller_check_ip() {
        let mut c: Controller = Default::default();

        // 1.02.003.004:05
        c.address = "1.02.003.004:05".to_string();
        c.check().unwrap();
        assert_eq!("1.02.003.004:05", c.address);

        // COM123
        c.address = "COM123".to_string();
        c.check().unwrap();
        assert_eq!("COM123", c.address);

        // ttyABC
        c.address = "ttyABC".to_string();
        c.check().unwrap();
        assert_eq!("ttyABC", c.address);
    }
}
