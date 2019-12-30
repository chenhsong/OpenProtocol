use super::{
    Address, BoundedValidationResult, TextName, GeoLocation, JobMode, OpMode, Operator, TextID, ID,
    R32,
};
use chrono::{DateTime, FixedOffset};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A data structure containing the current known status of a controller.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller<'a> {
    /// Unique ID of the controller, which cannot be zero.
    pub controller_id: ID,
    //
    /// User-specified human-friendly name for the machine.
    pub display_name: TextName<'a>,
    //
    /// Controller type.
    ///
    /// # Examples
    ///
    /// * `Ai01`
    /// * `Ai12`
    /// * `CDC2000WIN`
    /// * `MPC7`
    pub controller_type: TextID<'a>,
    //
    /// Version of the controller's firmware.
    pub version: TextID<'a>,
    //
    /// Machine model.
    pub model: TextID<'a>,
    //
    /// Address of the controller.
    ///
    /// For a network-connected controller, this is usually the IP address and port, in the format `x.x.x.x:port`.
    ///
    /// For a serial-connected controller, this is usually the serial port device name, such as `COM1`, `ttyS0`.
    #[serde(rename = "IP")]
    pub address: Address<'a>,
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
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[serde(default)]
    pub last_cycle_data: IndexMap<TextID<'a>, R32>,
    //
    /// Last-known states (if any) of controller variables.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[serde(default)]
    pub variables: IndexMap<TextID<'a>, R32>,
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
    pub job_card_id: Option<Box<Cow<'a, str>>>,
    //
    /// ID of the set of mold data currently loaded (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub mold_id: Option<Box<Cow<'a, str>>>,
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
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), Error<'static>> {
    /// // Default values should pass validation
    /// let c: Controller = Default::default();
    /// c.validate()?;
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn validate(&self) -> BoundedValidationResult<'a> {
        // Check Address
        self.address.validate()
    }
}

impl Default for Controller<'_> {
    /// Default value for `Controller`.
    ///
    /// `controller_id` is set to 1 because zero is not allowed.  
    /// All other fields are set to `Unknown` or empty.
    ///
    fn default() -> Self {
        Controller {
            controller_id: ID::from_u32(1),
            display_name: TextName::new_from_str("Unknown").unwrap(),
            controller_type: TextID::new("Unknown").unwrap(),
            version: TextID::new("Unknown").unwrap(),
            model: TextID::new("Unknown").unwrap(),
            address: Address::Unknown,
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
            operator: Some(Operator::try_new_with_name(ID::from_u32(123), "John")?),
            geo_location: Some(GeoLocation::new(88.0, 123.0)?),
            ..Default::default()
        };
        c.validate()?;
        let serialized = serde_json::to_string(&c).map_err(|x| x.to_string())?;
        assert_eq!(
            r#"{"controllerId":1,"displayName":"Unknown","controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"0.0.0.0:0","geoLatitude":88.0,"geoLongitude":123.0,"opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#,
            serialized
        );

        Ok(())
    }

    #[test]
    fn test_controller_from_json() -> Result<(), String> {
        let c: Controller = serde_json::from_str(r#"{"controllerId":1,"geoLatitude":88,"geoLongitude":-123,"displayName":"Hello","controllerType":"Unknown","version":"Unknown","model":"Unknown","IP":"127.0.0.1:123","opMode":"Automatic","jobMode":"ID02","operatorId":123,"operatorName":"John"}"#).map_err(|x| x.to_string())?;
        c.validate()?;

        assert_eq!(
            r#"Controller { controller_id: 1, display_name: "Hello", controller_type: "Unknown", version: "Unknown", model: "Unknown", address: IPv4(127.0.0.1, 123), geo_location: Some((88,-123)), op_mode: Automatic, job_mode: ID02, last_cycle_data: {}, variables: {}, last_connection_time: None, operator: Some(Operator { operator_id: 123, operator_name: Some("John") }), job_card_id: None, mold_id: None }"#,
            format!("{:?}", &c)
        );

        Ok(())
    }
}
