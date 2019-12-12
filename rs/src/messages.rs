use super::filters::Filters;
use super::utils::*;
use super::{
    ActionID, BoundedValidationResult, Controller, Error, JobCard, JobMode, KeyValuePair, Language,
    OpMode, Result, StateValues, ValidationResult, ID,
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use Message::*;

// Auto-incrementing global counter for message sequence numbers.
static SEQ: AtomicU64 = AtomicU64::new(1);

/// Common options of an Open Protocol message.
///
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageOptions<'a> {
    /// Unique ID (if any) of the message for tracking and storage retrieval purposes.
    ///
    /// The iChen Server may tag certain messages with a unique tracking key that can be used to
    /// retrieve the message from persistent storage later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    //
    /// Ever-increasing message sequence number.
    ///
    /// This number is usually auto-incremented with each message created, starting from 1.
    pub sequence: u64,
    //
    /// Priority of the message, smaller number is higher priority.  Default = 0.
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub priority: i32,
}

impl<'a> MessageOptions<'a> {
    /// Create a `MessageOptions` with default values (for example, the `sequence` field
    /// auto-increments).
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let opt1 = MessageOptions::new();
    /// assert_eq!(1, opt1.sequence);
    /// assert_eq!(0, opt1.priority);
    ///
    /// let opt2 = MessageOptions::new();
    /// assert_eq!(2, opt2.sequence);       // `sequence` auto-increments.
    /// assert_eq!(0, opt2.priority);
    /// ~~~
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a `MessageOptions` with a particular `priority` but otherwise
    /// default values (for example, the `sequence` field auto-increments).
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let opt1 = MessageOptions::new_with_priority(100);
    /// assert_eq!(1, opt1.sequence);
    /// assert_eq!(100, opt1.priority);
    ///
    /// let opt2 = MessageOptions::new_with_priority(-42);
    /// assert_eq!(2, opt2.sequence);       // `sequence` auto-increments.
    /// assert_eq!(-42, opt2.priority);
    /// ~~~
    pub fn new_with_priority(priority: i32) -> Self {
        Self { priority, ..Self::new() }
    }

    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if the `id` field is set to an empty string
    /// or is all whitespace.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let opt1 = MessageOptions { id: Some(""), ..Default::default() };
    /// assert_eq!(Err(Error::EmptyField("id")), opt1.validate());
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    ///
    pub fn validate(&self) -> ValidationResult {
        check_optional_str_empty(&self.id, "id")?;
        Ok(())
    }
}

impl Default for MessageOptions<'_> {
    /// Default value for `MessageOptions`.
    ///
    /// The `sequence` field is auto-incrementing.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let opt1: MessageOptions = Default::default();
    /// assert_eq!(1, opt1.sequence);
    /// assert_eq!(0, opt1.priority);
    ///
    /// let opt2: MessageOptions = Default::default();
    /// assert_eq!(2, opt2.sequence);       // `sequence` auto-increments.
    /// assert_eq!(0, opt2.priority);
    /// ~~~
    fn default() -> Self {
        Self { id: None, sequence: SEQ.fetch_add(1, Ordering::SeqCst), priority: 0 }
    }
}

/// All Open Protocol message types.
///
/// See [this document] for details.
///
/// [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/cs/doc/messages_reference.md
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum Message<'a> {
    /// The `ALIVE` message, sent periodically as the keep-alive mechanism.
    #[serde(rename_all = "camelCase")]
    Alive {
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `CNTRLER_ACTION` message, sent by the server whenever the current *action* of a controller changes.
    #[serde(rename_all = "camelCase")]
    ControllerAction {
        /// Unique ID of the controller.
        controller_id: ID,
        /// Unique action code.
        ///
        /// See [this document] for details.
        ///
        /// [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/doc/actions.md
        action_id: ActionID,
        //
        /// Time-stamp of the event.
        timestamp: DateTime<FixedOffset>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `REQ_CNTRLER_LIST` message, sent to the server to request a list of controllers (i.e. machines)
    /// within the user's organization.
    ///
    /// # Response
    ///
    /// The Server should reply with a [`ControllersList`] message.
    ///
    /// [`ControllersList`]: enum.Message.html#variant.ControllersList
    ///
    #[serde(rename_all = "camelCase")]
    RequestControllersList {
        /// Unique ID of the controller to request.
        ///
        /// If omitted, all controllers of the user's organization will be returned.
        #[serde(skip_serializing_if = "Option::is_none")]
        controller_id: Option<ID>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `RESP_CNTRLER_LIST` message, sent by the server in response to a
    /// [`RequestControllersList`] message.
    ///
    /// [`RequestControllersList`]: enum.Message.html#variant.RequestControllersList
    ///
    #[serde(rename_all = "camelCase")]
    ControllersList {
        /// List of controllers requested by a previous `RequestControllersList` message.
        ///
        /// Each controller data structure contains the last-known values of the controller's state.
        //
        // Custom deserialization of string into integer key.
        // No need for custom serialization because ID to string is fine.
        #[serde(deserialize_with = "deserialize_hashmap")]
        data: HashMap<ID, Controller<'a>>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `UPD_CNTRLER` message, sent by the server whenever the status of a connected controller changes.
    ///
    /// Only the changed fields will be set, with other fields/properties being set to
    /// `None` as they are not relevant.
    #[serde(rename_all = "camelCase")]
    ControllerStatus {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// Human-friendly name for display (or `None` if not relevant).
        #[allow(clippy::option_option)]
        #[serde(skip_serializing_if = "Option::is_none")]
        display_name: Option<Box<&'a str>>,
        //
        /// If true, the controller has disconnected from the iChen® Server.
        #[serde(skip_serializing_if = "Option::is_none")]
        is_disconnected: Option<bool>,
        //
        /// Current operation mode of the controller (or `None` if not relevant).
        #[serde(skip_serializing_if = "Option::is_none")]
        op_mode: Option<OpMode>,
        //
        /// Current job mode of the controller (or `None` if not relevant).
        #[serde(skip_serializing_if = "Option::is_none")]
        job_mode: Option<JobMode>,
        //
        /// State of an alarm (if any) on the controller (or `None` if not relevant).
        ///
        /// See [this document] for valid alarm codes.
        ///
        /// [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/doc/alarms.md
        #[serde(skip_serializing_if = "Option::is_none")]
        alarm: Option<Box<KeyValuePair<&'a str, bool>>>,
        //
        /// Change of a setting (if any) on the controller for audit trail purpose
        /// (or `None` if not relevant).
        #[serde(skip_serializing_if = "Option::is_none")]
        audit: Option<Box<KeyValuePair<&'a str, f64>>>,
        //
        /// Change of a variable (if any) on the controller (or `None` if not relevant).
        #[serde(skip_serializing_if = "Option::is_none")]
        variable: Option<Box<KeyValuePair<&'a str, f64>>>,
        //
        /// Unique ID of the current logged-on user, `Some(None)` if a user has logged out
        /// (or `None` if not relevant).
        #[allow(clippy::option_option)]
        #[serde(serialize_with = "serialize_some_none_to_invalid")]
        #[serde(deserialize_with = "deserialize_invalid_to_some_none")]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        operator_id: Option<Option<ID>>,
        //
        /// Name of the current logged-on user, `Some(None)` if the current user has no name
        /// (or `None` if not relevant).
        #[allow(clippy::option_option)]
        #[serde(deserialize_with = "deserialize_null_to_some_none")]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        operator_name: Option<Option<Box<&'a str>>>,
        //
        /// Unique ID of the current job card loaded, `Some(None)` if no job card is currently loaded
        /// (or `None` if not relevant).
        #[allow(clippy::option_option)]
        #[serde(deserialize_with = "deserialize_null_to_some_none")]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(borrow)]
        job_card_id: Option<Option<Box<Cow<'a, str>>>>,
        //
        /// Unique ID of the current mold data set loaded, `Some(None)` if no mold data set is currently loaded
        /// (or `None` if not relevant).
        #[allow(clippy::option_option)]
        #[serde(deserialize_with = "deserialize_null_to_some_none")]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(borrow)]
        mold_id: Option<Option<Box<Cow<'a, str>>>>,
        //
        /// Snapshot of the current known states of the controller.
        state: StateValues<'a>,
        //
        /// A [`Controller`] data structure containing the last-known state of the controller.
        ///
        /// This field is only sent once by the server as soon as a new controller has connected
        /// to the network.
        /// All subsequent messages have this field set to `None`.
        ///
        /// If this field is not `None`, then all other info fields should be `None` or have values
        /// equal to the matching fields in `controller`.
        ///
        /// [`Controller`]: struct.Controller.html
        #[serde(skip_serializing_if = "Option::is_none")]
        controller: Option<Box<Controller<'a>>>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `CYCLE_DATA` message, sent by the server at the end of each machine cycle.
    #[serde(rename_all = "camelCase")]
    CycleData {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// A data dictionary containing a set of cycle data.
        ///
        /// See [this document] for examples.
        ///
        /// [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/doc/cycledata.md
        data: HashMap<&'a str, f64>,
        //
        /// Time-stamp of the event.
        timestamp: DateTime<FixedOffset>,
        //
        /// Snapshot of the current known states of the controller.
        #[serde(flatten)]
        state: StateValues<'a>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `REQ_JOBCARDS_LIST` message, sent by the server when a connected controller
    /// requests a list of job cards.
    ///
    /// # Action Required
    ///
    /// The user should send a [`JobCardsList`] message to the Server as a reply.
    ///
    /// [`JobCardsList`]: enum.Message.html#variant.JobCardsList
    #[serde(rename_all = "camelCase")]
    RequestJobCardsList {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `RESP_JOBSLIST` message, sent to the server in response to a [`RequestJobCardsList`] message.
    ///
    /// [`RequestJobCardsList`]: enum.Message.html#variant.RequestJobCardsList
    #[serde(rename_all = "camelCase")]
    JobCardsList {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// A data dictionary containing a set of `JobCard` data structures.
        data: HashMap<&'a str, JobCard<'a>>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `JOIN` message, sent to log onto the server.
    ///
    /// # Response
    ///
    /// The Server should reply with a [`JoinResponse`] message.
    ///
    /// [`JoinResponse`]: enum.Message.html#variant.JoinResponse
    #[serde(rename_all = "camelCase")]
    Join {
        /// Organization ID (if any).
        #[serde(skip_serializing_if = "Option::is_none")]
        org_id: Option<&'a str>,
        //
        /// The maximum protocol version supported, in the format `x.x.x.x`.
        ///
        /// The current protocol version implemented is in the constant `PROTOCOL_VERSION`.
        version: &'a str,
        //
        /// Password to log onto the server.
        password: &'a str,
        //
        /// Language encoding.
        language: Language,
        //
        /// A collection of [`Filter`] values containing what type(s) of messages to receive.
        ///
        /// [`Filter`]: struct.Filters.html
        filter: Filters,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `RESP_JOIN` message, sent by the Server in response to a [`Join`] message.
    ///
    /// [`Join`]: enum.Message.html#variant.Join
    #[serde(rename_all = "camelCase")]
    JoinResponse {
        /// Result code, >= 100 indicates success.
        result: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        //
        /// The allowed access level for this client.
        level: Option<u32>,
        //
        /// A message (mostly likely an error message in case of failure), if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(borrow)]
        message: Option<Box<Cow<'a, str>>>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `REQ_MOLD` message, sent to the server to request the set of mold settings data of a controller.
    ///
    /// # Response
    ///
    /// The Server should reply with a [`MoldData`] message.
    ///
    /// [`MoldData`]: enum.Message.html#variant.MoldData
    #[serde(rename_all = "camelCase")]
    RequestMoldData {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `RESP_MOLD` message, sent by the server in response to a [`RequestMoldData`] message
    /// or a [`ReadMoldData`] message with `field` set to `None` (meaning read all).
    ///
    /// [`RequestMoldData`]: enum.Message.html#variant.RequestMoldData
    /// [`ReadMoldData`]: enum.Message.html#variant.ReadMoldData
    #[serde(rename_all = "camelCase")]
    MoldData {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// A data dictionary containing a set of mold settings.
        data: HashMap<&'a str, f64>,
        //
        /// Time-stamp of the event.
        timestamp: DateTime<FixedOffset>,
        //
        /// Snapshot of the current known states of the controller.
        #[serde(flatten)]
        state: StateValues<'a>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `READ_MOLD_DATA` message, sent to the server to read the current value of a
    /// particular mold setting.
    ///
    /// The server keeps a cache of the states of all mold settings for each controller.
    /// The value returned is based on the server cache.
    /// No command is sent to controller to poll the latest value.
    ///
    /// # Response
    ///
    /// The Server should reply with a [`MoldData`] message if `field` is `None`,
    /// or a [`MoldDataValue`] message.
    ///
    /// [`MoldData`]: enum.Message.html#variant.MoldData
    /// [`MoldDataValue`]: enum.Message.html#variant.MoldDataValue
    #[serde(rename_all = "camelCase")]
    ReadMoldData {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// Name of the mold setting to read, `None` for all.
        field: Option<&'a str>,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `RESP_MOLD_DATA_VALUE` message, sent by the server in response to a
    /// [`ReadMoldData`] message.
    ///
    /// [`ReadMoldData`]: enum.Message.html#variant.ReadMoldData
    #[serde(rename_all = "camelCase")]
    MoldDataValue {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// Name of the mold setting to read.
        field: &'a str,
        //
        /// Current cached value of the mold setting.
        value: f64,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `REQ_PWD_LEVEL` message, sent by server when a connected controller attempts to
    /// authenticate and authorize a user password.
    ///
    /// # Action Required
    ///
    /// The user should send an [`OperatorInfo`] message to the Server as a reply.
    ///
    /// [`OperatorInfo`]: enum.Message.html#variant.OperatorInfo
    #[serde(rename_all = "camelCase")]
    LoginOperator {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// User password.
        password: &'a str,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    //
    /// The `RESP_PWD_LEVEL` message, sent to the server in response to a
    /// [`LoginOperator`] message.
    ///
    /// [`LoginOperator`]: enum.Message.html#variant.LoginOperator
    #[serde(rename_all = "camelCase")]
    OperatorInfo {
        /// Unique ID of the controller.
        controller_id: ID,
        //
        /// Unique ID of the authenticated user.
        #[serde(skip_serializing_if = "Option::is_none")]
        operator_id: Option<ID>,
        //
        /// Name of the user.
        name: &'a str,
        //
        /// User password.
        password: &'a str,
        //
        /// Allowed access level for the user.
        ///
        /// Valid values are from 0 to [`MAX_OPERATOR_LEVEL`] (usually 10).
        ///
        /// [`MAX_OPERATOR_LEVEL`]: enum.Message.html#associatedconstant.MAX_OPERATOR_LEVEL
        level: u8,
        //
        /// Message configuration options.
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
}

impl<'a> Message<'a> {
    /// Current protocol version: 4.0.
    pub const PROTOCOL_VERSION: &'static str = "4.0";

    /// Default language to use: `EN` (English).
    pub const DEFAULT_LANGUAGE: Language = Language::EN;

    /// Maximum operator level: 10.
    pub const MAX_OPERATOR_LEVEL: u8 = 10;

    /// Parse a JSON string into a `Message`.
    ///
    /// # Errors
    ///
    /// Return `Err(`[`OpenProtocolError`]`)` if there is an error during parsing.
    ///
    /// [`OpenProtocolError`]: enum.OpenProtocolError.html
    ///
    pub fn parse_from_json_str(json: &'a str) -> Result<'a, Self> {
        match serde_json::from_str::<Message>(json) {
            // Do validation check if successfully parsed
            Ok(m) => m.validate().map(|_| m),
            // Otherwise return error
            Err(err) => Err(Error::JsonError(err)),
        }
    }

    /// Validate all the fields in the `Message`, then serialize it into a JSON string.
    ///
    /// # Errors
    ///
    /// Return `Err(`[`OpenProtocolError`]`)` if there is an error.
    ///
    /// [`OpenProtocolError`]: enum.OpenProtocolError.html
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), String> {
    /// let msg = Message::new_join_with_org("MyPassword", Filters::Status + Filters::Cycle, "MyCompany");
    /// assert_eq!(
    ///     r#"{"$type":"Join","orgId":"MyCompany","version":"4.0","password":"MyPassword","language":"EN","filter":"Status, Cycle","sequence":1}"#,
    ///     msg.to_json_str()?
    /// );
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn to_json_str(&self) -> Result<'_, String> {
        self.validate()?;
        serde_json::to_string(self).map_err(Error::JsonError)
    }

    /// Create an `ALIVE` message.
    pub fn new_alive() -> Self {
        Alive { options: Default::default() }
    }

    /// Create a `JOIN` message with default language and protocol version.
    ///
    /// The default language is [`DEFAULT_LANGUAGE`] (usually `EN`).
    ///
    /// The default protocol version is given in [`PROTOCOL_VERSION`].
    ///
    /// [`DEFAULT_LANGUAGE`]: enum.Message.html#associatedconstant.DEFAULT_LANGUAGE
    /// [`PROTOCOL_VERSION`]: enum.Message.html#associatedconstant.PROTOCOL_VERSION
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let msg = Message::new_join("MyPassword", Filters::Status + Filters::Cycle);
    /// assert_eq!(
    ///     Message::Join {
    ///         org_id: None,
    ///         version: Message::PROTOCOL_VERSION,
    ///         password: "MyPassword",
    ///         language: Message::DEFAULT_LANGUAGE,
    ///         filter: Filters::Status + Filters::Cycle,
    ///         options: MessageOptions { id: None, sequence: 1, priority: 0 }
    ///     },
    ///     msg
    /// );
    /// ~~~
    pub fn new_join(password: &'a str, filter: Filters) -> Self {
        Join {
            org_id: None,
            version: Self::PROTOCOL_VERSION,
            password,
            language: Self::DEFAULT_LANGUAGE,
            filter,
            options: Default::default(),
        }
    }

    /// Create a `JOIN` message with non-default organization.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let msg = Message::new_join_with_org("MyPassword", Filters::Status + Filters::Cycle, "MyCompany");
    /// assert_eq!(
    ///     Message::Join {
    ///         org_id: Some("MyCompany"),
    ///         version: Message::PROTOCOL_VERSION,
    ///         password: "MyPassword",
    ///         language: Message::DEFAULT_LANGUAGE,
    ///         filter: Filters::Status + Filters::Cycle,
    ///         options: MessageOptions { id: None, sequence: 1, priority: 0 }
    ///     },
    ///     msg
    /// );
    /// ~~~
    pub fn new_join_with_org(password: &'a str, filter: Filters, org: &'a str) -> Self {
        let mut msg = Self::new_join(password, filter);
        if let Join { ref mut org_id, .. } = msg {
            *org_id = Some(org)
        }
        msg
    }

    /// Validate the `Message` data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError`]`)` if some fields in the `Message` are not valid.
    ///
    /// [`OpenProtocolError`]: enum.OpenProtocolError.html
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let msg = Message::ControllerStatus {
    ///     controller_id: ID::from_u32(12345),
    ///     display_name: None,
    ///     is_disconnected: None,
    ///     op_mode: None,
    ///     job_mode: None,
    ///     job_card_id: Some(None),
    ///     mold_id: Some(Some("Test-123".into())),     // Value is "Test-123"
    ///     operator_id: None,
    ///     operator_name: None,
    ///     variable: None,
    ///     audit: None,
    ///     alarm: None,
    ///     controller: None,
    ///     state: StateValues::new_with_all(
    ///         OpMode::Automatic,
    ///         JobMode::ID02,
    ///         None,
    ///         None,
    ///         Some("Test-FooBar"),    // Notice that this state value should be "Test-123"
    ///     ),
    ///     options: Default::default(),
    /// };
    ///
    /// // Validation should error because `state.mold_id` is not the same as the `mold_id` field.
    /// assert_eq!(
    ///     Err(Error::InconsistentState("mold_id")),
    ///     msg.validate()
    /// );
    /// ~~~
    pub fn validate(&self) -> BoundedValidationResult<'a> {
        match self {
            Alive { options, .. }
            | ControllerAction { options, .. }
            | RequestControllersList { options, .. }
            | RequestJobCardsList { options, .. }
            | JoinResponse { options, .. }
            | RequestMoldData { options, .. } => options.validate(),
            ControllersList { options, data, .. } => {
                data.iter().try_for_each(|c| c.1.validate())?;
                options.validate()
            }
            ControllerStatus {
                options,
                display_name,
                is_disconnected,
                op_mode,
                job_mode,
                alarm,
                audit,
                variable,
                operator_id,
                operator_name,
                job_card_id,
                mold_id,
                state,
                controller,
                ..
            } => {
                if let Some(c) = controller {
                    // If controller is present, some fields must be None
                    if !is_disconnected.is_none()
                        || !alarm.is_none()
                        || !audit.is_none()
                        || !variable.is_none()
                    {
                        return Err(Error::ConstraintViolated(
                            "All other fields must be set to None if controller is present.".into(),
                        ));
                    }
                    c.validate()?;

                    // Check controller fields with specified fields
                    if display_name.is_some()
                        && display_name.as_ref().unwrap().as_ref() != &c.display_name
                    {
                        return Err(Error::InconsistentField("display_name"));
                    }
                    if op_mode.is_some() && *op_mode != Some(c.op_mode) {
                        return Err(Error::InconsistentField("op_mode"));
                    }
                    if job_mode.is_some() && *job_mode != Some(c.job_mode) {
                        return Err(Error::InconsistentField("job_mode"));
                    }
                    if operator_id.is_some()
                        && *operator_id != Some(c.operator.as_ref().map(|user| user.operator_id))
                    {
                        return Err(Error::InconsistentField("operator_id"));
                    }
                    if operator_name.is_some()
                        && operator_name.as_ref().unwrap().as_ref().map(|x| *x.as_ref())
                            != c.operator.as_ref().map(|u| u.operator_name).and_then(|n| n)
                    {
                        return Err(Error::InconsistentField("operator_name"));
                    }
                    if let Some(jc) = job_card_id {
                        if *jc != c.job_card_id {
                            return Err(Error::InconsistentField("job_card_id"));
                        }
                    }
                    if let Some(m) = mold_id {
                        if *m != c.mold_id {
                            return Err(Error::InconsistentField("mold_id"));
                        }
                    }
                }

                check_optional_str_empty(display_name, "display_name")?;

                if let Some(op) = op_mode {
                    if *op != state.op_mode {
                        return Err(Error::InconsistentState("op_mode"));
                    }
                }

                if let Some(job) = job_mode {
                    if *job != state.job_mode {
                        return Err(Error::InconsistentState("job_mode"));
                    }
                }

                if let Some(opr) = operator_id {
                    if *opr != state.operator_id {
                        return Err(Error::InconsistentState("operator_id"));
                    }
                }

                if let Some(opr) = operator_name {
                    check_optional_str_whitespace(opr, "operator_name")?;
                }

                if let Some(jc) = job_card_id {
                    check_optional_str_whitespace(jc, "job_card_id")?;

                    if *jc != state.job_card_id {
                        return Err(Error::InconsistentState("job_card_id"));
                    }
                }

                if let Some(m) = mold_id {
                    check_optional_str_whitespace(m, "mold_id")?;

                    if *m != state.mold_id {
                        return Err(Error::InconsistentState("mold_id"));
                    }
                }

                state.validate()?;

                if let Some(kv) = alarm {
                    kv.validate()?;
                }
                if let Some(kv) = audit {
                    kv.validate()?;
                }
                if let Some(kv) = variable {
                    kv.validate()?;
                }

                options.validate()
            }
            CycleData { options, data, state, .. } => {
                data.iter().try_for_each(|d| check_f64(*d.1, d.0))?;
                check_optional_str_empty(&state.job_card_id, "job_card_id")?;
                check_optional_str_empty(&state.mold_id, "mold_id")?;
                options.validate()
            }
            JobCardsList { options, data, .. } => {
                if data.is_empty() {
                    return Err(Error::EmptyField("data"));
                }
                data.iter().try_for_each(|jc| jc.1.validate())?;
                options.validate()
            }
            Join { options, org_id, version, password, language, .. } => {
                check_optional_str_empty(org_id, "org_id")?;
                check_str_empty(version, "version")?;
                check_str_empty(password, "password")?;
                // Check for invalid language
                if *language == Language::Unknown {
                    return Err(Error::InvalidField {
                        field: "language",
                        value: "Unknown".into(),
                        description: "language cannot be Unknown".into(),
                    });
                }
                options.validate()
            }
            MoldData { options, data, state, .. } => {
                if data.is_empty() {
                    return Err(Error::EmptyField("data"));
                }
                data.iter().try_for_each(|d| check_f64(*d.1, d.0))?;
                check_optional_str_empty(&state.job_card_id, "job_card_id")?;
                check_optional_str_empty(&state.mold_id, "mold_id")?;
                options.validate()
            }
            ReadMoldData { options, field, .. } => {
                check_optional_str_empty(field, "field")?;
                options.validate()
            }
            MoldDataValue { options, field, value, .. } => {
                check_str_empty(field, "field")?;
                check_f64(*value, "value")?;
                options.validate()
            }
            LoginOperator { options, password, .. } => {
                check_str_empty(&password, "password")?;
                options.validate()
            }
            OperatorInfo { options, name, password, level, .. } => {
                check_str_empty(name, "name")?;
                check_optional_str_whitespace(&Some(*password), "password")?;
                if *level > Self::MAX_OPERATOR_LEVEL {
                    return Err(Error::ConstraintViolated(
                        format!(
                            "Level {} is too high - must be between 0 and {}.",
                            level,
                            Self::MAX_OPERATOR_LEVEL
                        )
                        .into(),
                    ));
                }
                options.validate()
            }
        }
    }
}

// Tests

#[cfg(test)]
mod test {
    use super::*;
    use std::result::Result;

    impl<'a> MessageOptions<'a> {
        /// A private constructor function that creates a `MessageOptions` structure
        /// with `sequence` always set to 1 (for testing purposes).
        fn default_new() -> Self {
            Self { sequence: 1, ..Self::new() }
        }
    }

    #[test]
    fn test_message_alive_to_json() -> Result<(), String> {
        let msg =
            Alive { options: MessageOptions { id: Some("Hello"), sequence: 999, priority: 20 } };

        let serialized = serde_json::to_string(&msg).map_err(|x| x.to_string())?;

        assert_eq!(r#"{"$type":"Alive","id":"Hello","sequence":999,"priority":20}"#, serialized);

        Ok(())
    }

    #[test]
    fn test_message_mold_data_to_json() -> Result<(), String> {
        let mut map = HashMap::<&str, f64>::new();

        map.insert("Hello", 123.0);
        map.insert("World", -987.6543);
        map.insert("foo", 0.0);

        let msg = MoldData {
            controller_id: ID::from_u32(123),
            data: map,

            timestamp: DateTime::parse_from_rfc3339("2019-02-26T02:03:04+08:00")
                .map_err(|x| x.to_string())?,

            state: StateValues::new_with_all(
                OpMode::SemiAutomatic,
                JobMode::Offline,
                Some(42),
                Some("Hello World!"),
                None,
            ),

            options: MessageOptions { id: None, sequence: 999, priority: -20 },
        };

        let serialized = serde_json::to_string(&msg).map_err(|x| x.to_string())?;

        assert_eq!(
            r#"{"$type":"MoldData","controllerId":123,"data":{"foo":0.0,"Hello":123.0,"World":-987.6543},"timestamp":"2019-02-26T02:03:04+08:00","jobCardId":"Hello World!","operatorId":42,"opMode":"SemiAutomatic","jobMode":"Offline","sequence":999,"priority":-20}"#,
            serialized
        );

        let m2 = Message::parse_from_json_str(&serialized).map_err(|x| x.to_string())?;

        assert_eq!(msg, m2);

        Ok(())
    }

    #[test]
    fn test_message_controllers_list_from_json() -> Result<(), String> {
        let json = r#"{"$type":"ControllersList","data":{"12345":{"controllerId":12345,"displayName":"Hello","controllerType":"Ai12","version":"1.0.0","model":"JM128-Ai","IP":"192.168.5.1:123","opMode":"Manual","jobMode":"ID11","lastCycleData":{"Z_QDGODCNT":8567,"Z_QDCYCTIM":979,"Z_QDINJTIM":5450,"Z_QDPLSTIM":7156,"Z_QDINJENDPOS":8449,"Z_QDPLSENDPOS":2212,"Z_QDFLAG":8988,"Z_QDPRDCNT":65500,"Z_QDCOLTIM":4435,"Z_QDMLDOPNTIM":652,"Z_QDMLDCLSTIM":2908,"Z_QDVPPOS":4732,"Z_QDMLDOPNENDPOS":6677,"Z_QDMAXINJSPD":7133,"Z_QDMAXPLSRPM":641,"Z_QDNOZTEMP":6693,"Z_QDTEMPZ01":9964,"Z_QDTEMPZ02":7579,"Z_QDTEMPZ03":4035,"Z_QDTEMPZ04":5510,"Z_QDTEMPZ05":8460,"Z_QDTEMPZ06":9882,"Z_QDBCKPRS":2753,"Z_QDHLDTIM":9936},"lastConnectionTime":"2016-03-06T23:11:27.1442177+08:00"},"22334":{"controllerId":22334,"displayName":"World","controllerType":"Ai01","version":"1.0.0","model":"JM128-Ai","IP":"192.168.5.2:234","opMode":"SemiAutomatic","jobMode":"ID12","lastCycleData":{"Z_QDGODCNT":6031,"Z_QDCYCTIM":7526,"Z_QDINJTIM":4896,"Z_QDPLSTIM":5196,"Z_QDINJENDPOS":1250,"Z_QDPLSENDPOS":8753,"Z_QDFLAG":3314,"Z_QDPRDCNT":65500,"Z_QDCOLTIM":3435,"Z_QDMLDOPNTIM":7854,"Z_QDMLDCLSTIM":4582,"Z_QDVPPOS":7504,"Z_QDMLDOPNENDPOS":7341,"Z_QDMAXINJSPD":7322,"Z_QDMAXPLSRPM":6024,"Z_QDNOZTEMP":3406,"Z_QDTEMPZ01":3067,"Z_QDTEMPZ02":9421,"Z_QDTEMPZ03":2080,"Z_QDTEMPZ04":8845,"Z_QDTEMPZ05":4478,"Z_QDTEMPZ06":3126,"Z_QDBCKPRS":2807,"Z_QDHLDTIM":3928},"lastConnectionTime":"2016-03-06T23:11:27.149218+08:00"}},"sequence":68568}"#;

        let msg = Message::parse_from_json_str(&json).map_err(|x| x.to_string())?;

        if let ControllersList { data, .. } = &msg {
            assert_eq!(2, data.len());
            let c = data.get(&ID::from_u32(12345)).unwrap();
            assert_eq!("Hello", c.display_name);
            Ok(())
        } else {
            Err(format!("Expected ControllersList, got {:#?}", msg))
        }
    }

    #[test]
    fn test_message_cycle_data_from_json() -> Result<(), String> {
        let json = r#"{"$type":"CycleData","timestamp":"2016-02-26T01:12:23+08:00","opMode":"Automatic","jobMode":"ID02","controllerId":123,"data":{"Z_QDGODCNT":123,"Z_QDCYCTIM":12.33,"Z_QDINJTIM":3,"Z_QDPLSTIM":4.4,"Z_QDINJENDPOS":30.1,"Z_QDPLSENDPOS":20.3,"Z_QDFLAG":1,"Z_QDPRDCNT":500,"Z_QDCOLTIM":12.12,"Z_QDMLDOPNTIM":2.1,"Z_QDMLDCLSTIM":1.3,"Z_QDVPPOS":12.11,"Z_QDMLDOPNENDPOS":130.1,"Z_QDMAXINJSPD":213.12,"Z_QDMAXPLSRPM":551,"Z_QDNOZTEMP":256,"Z_QDTEMPZ01":251,"Z_QDTEMPZ02":252,"Z_QDTEMPZ03":253,"Z_QDTEMPZ04":254,"Z_QDTEMPZ05":255,"Z_QDTEMPZ06":256,"Z_QDBCKPRS":54,"Z_QDHLDTIM":2.3,"Z_QDCPT01":231,"Z_QDCPT02":232,"Z_QDCPT03":233,"Z_QDCPT04":234,"Z_QDCPT05":235,"Z_QDCPT06":236,"Z_QDCPT07":237,"Z_QDCPT08":238,"Z_QDCPT09":239,"Z_QDCPT10":240,"Z_QDCPT11":241,"Z_QDCPT12":242,"Z_QDCPT13":243,"Z_QDCPT14":244,"Z_QDCPT15":245,"Z_QDCPT16":246,"Z_QDCPT17":247,"Z_QDCPT18":248,"Z_QDCPT19":249,"Z_QDCPT20":250,"Z_QDCPT21":251,"Z_QDCPT22":252,"Z_QDCPT23":253,"Z_QDCPT24":254,"Z_QDCPT25":255,"Z_QDCPT26":256,"Z_QDCPT27":257,"Z_QDCPT28":258,"Z_QDCPT29":259,"Z_QDCPT30":260,"Z_QDCPT31":261,"Z_QDCPT32":262,"Z_QDCPT33":263,"Z_QDCPT34":264,"Z_QDCPT35":265,"Z_QDCPT36":266,"Z_QDCPT37":267,"Z_QDCPT38":268,"Z_QDCPT39":269,"Z_QDCPT40":270},"sequence":1}"#;

        let msg = Message::parse_from_json_str(&json).map_err(|x| x.to_string())?;

        if let CycleData { options, controller_id, data, .. } = msg {
            assert_eq!(0, options.priority);
            assert_eq!(123, controller_id);
            assert_eq!(64, data.len());
            assert!((*data.get("Z_QDCPT13").unwrap() - 243.0).abs() < std::f64::EPSILON);
            Ok(())
        } else {
            Err(format!("Expected CycleData, got {:#?}", msg))
        }
    }

    #[test]
    fn test_message_controller_status_without_controller_from_json() -> Result<(), String> {
        let json = r#"{"$type":"ControllerStatus","controllerId":123,"displayName":"Testing","opMode":"Automatic","alarm":{"key":"hello","value":true},"jobMode":"ID05","jobCardId":"XYZ","moldId":"Mold-123","state":{"opMode":"Automatic","jobMode":"ID05","jobCardId":"XYZ","moldId":"Mold-123"},"sequence":1,"priority":50}"#;

        let msg = Message::parse_from_json_str(&json).map_err(|x| x.to_string())?;

        if let ControllerStatus {
            options, controller_id, display_name, controller, alarm, ..
        } = msg
        {
            assert_eq!(50, options.priority);
            assert_eq!(1, options.sequence);
            assert_eq!(123, controller_id);
            assert_eq!(Some(Box::new("Testing")), display_name);
            assert_eq!(None, controller);
            assert_eq!(Some(Box::new(KeyValuePair::new("hello", true))), alarm);
            Ok(())
        } else {
            Err(format!("Expected ControllerStatus, got {:#?}", msg))
        }
    }

    #[test]
    fn test_message_controller_status_with_controller_from_json() -> Result<(), String> {
        let json = r#"{"$type":"ControllerStatus","controllerId":123,"state":{"opMode":"Automatic","jobMode":"ID05"},"controller":{"controllerId":123,"displayName":"Testing","controllerType":"Ai02","version":"2.2","model":"JM138Ai","IP":"192.168.1.1:12345","geoLatitude":23.0,"geoLongitude":-121.0,"opMode":"Automatic","jobMode":"ID05","jobCardId":"XYZ","lastCycleData":{"INJ":5,"CLAMP":400},"moldId":"Mold-123"},"sequence":1}"#;

        let msg = Message::parse_from_json_str(&json).map_err(|x| x.to_string())?;

        if let ControllerStatus {
            options, controller_id, display_name, state, controller, ..
        } = msg
        {
            assert_eq!(0, options.priority);
            assert_eq!(1, options.sequence);
            assert_eq!(123, controller_id);
            assert_eq!(None, display_name);
            assert_eq!(OpMode::Automatic, state.op_mode);
            assert_eq!(JobMode::ID05, state.job_mode);
            assert_eq!(None, state.job_card_id);
            let c = controller.unwrap();
            assert_eq!("JM138Ai", c.model);
            let d = c.last_cycle_data;
            assert!(c.operator.is_none());
            assert_eq!(2, d.len());
            assert!((*d.get("INJ").unwrap() - 5.0).abs() < std::f64::EPSILON);
            Ok(())
        } else {
            Err(format!("Expected ControllerStatus, got {:#?}", msg))
        }
    }

    #[test]
    fn test_message_controller_status_to_json() -> Result<(), String> {
        let status = ControllerStatus {
            controller_id: ID::from_u32(12345),
            display_name: None,
            is_disconnected: None,
            op_mode: None,
            job_mode: None,
            job_card_id: None,
            mold_id: Some(None),
            operator_id: Some(Some(ID::from_u32(123))),
            operator_name: Some(None),
            variable: None,
            audit: None,
            alarm: Some(Box::new(KeyValuePair::new("hello", true))),
            controller: None,
            state: StateValues::new_with_all(
                OpMode::Automatic,
                JobMode::ID02,
                Some(123),
                None,
                None,
            ),
            options: MessageOptions::default_new(),
        };

        let msg = status.to_json_str()?;
        assert_eq!(r#"{"$type":"ControllerStatus","controllerId":12345,"alarm":{"key":"hello","value":true},"operatorId":123,"operatorName":null,"moldId":null,"state":{"opMode":"Automatic","jobMode":"ID02","operatorId":123},"sequence":1}"#, msg);
        Ok(())
    }

    #[test]
    fn test_message_controller_status_to_json2() -> Result<(), String> {
        let status = ControllerStatus {
            controller_id: ID::from_u32(12345),
            display_name: None,
            is_disconnected: Some(true),
            op_mode: None,
            job_mode: None,
            job_card_id: Some(None),
            mold_id: Some(Some(Box::new("Test".into()))),
            operator_id: Some(None),
            operator_name: Some(None),
            variable: None,
            audit: None,
            alarm: None,
            controller: None,
            state: StateValues::new_with_all(
                OpMode::Automatic,
                JobMode::ID02,
                None,
                None,
                Some("Test"),
            ),
            options: MessageOptions::default_new(),
        };

        let msg = status.to_json_str()?;
        assert_eq!(r#"{"$type":"ControllerStatus","controllerId":12345,"isDisconnected":true,"operatorId":0,"operatorName":null,"jobCardId":null,"moldId":"Test","state":{"opMode":"Automatic","jobMode":"ID02","moldId":"Test"},"sequence":1}"#, msg);
        Ok(())
    }
}
