use self::filters::*;
use self::utils::*;
use super::*;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU64, Ordering};
use Message::*;

// Auto-increment global counter for message sequences
static SEQ: AtomicU64 = AtomicU64::new(1);

// Current protocol version
const PROTOCOL_VERSION: &str = "4.0";

// Default language to use
const DEFAULT_LANGUAGE: Language = Language::EN;

// Maximum operator level
const MAX_OPERATOR_LEVEL: u8 = 10;

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
    /// Ever-increasing message sequence number.
    ///
    /// This number is usually auto-incremented with each message created, starting from 1.
    pub sequence: u64,
    /// Priority of the message, smaller number is higher priority.  Default = 0.
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub priority: i32,
}

impl<'a> MessageOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    fn check(&self) -> Result<'static, ()> {
        check_optional_str_empty(&self.id, "id")
    }
}

impl Default for MessageOptions<'_> {
    fn default() -> Self {
        Self {
            id: None,
            sequence: SEQ.fetch_add(1, Ordering::SeqCst),
            priority: 0,
        }
    }
}

/// A data structure containing information on a production job (i.e. a *job card*).
///
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobCard<'a> {
    /// Unique job ID, which must not be empty or all white-spaces.
    #[serde(borrow)]
    pub job_card_id: Cow<'a, str>,
    /// ID of the set of mold data to load for this job.
    #[serde(borrow)]
    pub mold_id: Cow<'a, str>,
    /// Current production progress, which must not be larger than `total`.
    pub progress: u32,
    /// Total production count ordered.
    pub total: u32,
}

impl JobCard<'_> {
    fn check(&self) -> Result<'static, ()> {
        check_string_empty(&self.job_card_id, "job_card_id")?;
        check_string_empty(&self.mold_id, "mold_id")?;
        if self.progress > self.total {
            return Err(OpenProtocolError::ConstraintViolated(
                format!(
                    "JobCard progress ({}) must not be larger than total ({}).",
                    self.progress, self.total
                )
                .into(),
            ));
        }
        Ok(())
    }
}

/// A general data structure holding a key and value pair.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair<K, V> {
    pub key: K,
    pub value: V,
}

/// A data structure containing a snapshot of the current known states of the controller.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateValues<'a> {
    /// Current operating mold of the controller.
    #[serde(skip_serializing_if = "OpMode::is_unknown")]
    #[serde(default)]
    pub op_mode: OpMode,
    /// Current job mode of the controller.
    #[serde(skip_serializing_if = "JobMode::is_unknown")]
    #[serde(default)]
    pub job_mode: JobMode,
    /// Unique ID of the current logged-in user (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<NonZeroU32>,
    /// Current active job ID (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub job_card_id: Option<Cow<'a, str>>,
    /// Unique ID of the set of mold data currently loaded (if any) on the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub mold_id: Option<Cow<'a, str>>,
}

impl StateValues<'_> {
    fn check(&self) -> Result<'static, ()> {
        check_optional_str_empty(&self.job_card_id, "job_card_id")?;
        check_optional_str_empty(&self.mold_id, "mold_id")?;
        Ok(())
    }
}

/// All Open Protocol message types.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum Message<'a> {
    #[serde(rename_all = "camelCase")]
    Alive {
        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    ControllerAction {
        controller_id: NonZeroU32,
        action_id: i32,
        timestamp: DateTime<FixedOffset>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    RequestControllersList {
        #[serde(skip_serializing_if = "Option::is_none")]
        controller_id: Option<NonZeroU32>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    ControllersList {
        data: HashMap<NonZeroU32, Controller<'a>>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    ControllerStatus {
        controller_id: NonZeroU32,

        #[serde(skip_serializing_if = "Option::is_none")]
        display_name: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_connected: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        op_mode: Option<OpMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        job_mode: Option<JobMode>,

        #[serde(skip_serializing_if = "Option::is_none")]
        alarm: Option<KeyValuePair<&'a str, bool>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        audit: Option<KeyValuePair<&'a str, f64>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        variable: Option<KeyValuePair<&'a str, f64>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        operator_id: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(deserialize_with = "deserialize_null_to_empty_str")]
        #[serde(default)]
        operator_name: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(deserialize_with = "deserialize_null_to_empty_cowstr")]
        #[serde(default)]
        #[serde(borrow)]
        job_card_id: Option<Cow<'a, str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(deserialize_with = "deserialize_null_to_empty_cowstr")]
        #[serde(default)]
        #[serde(borrow)]
        mold_id: Option<Cow<'a, str>>,

        state: StateValues<'a>,

        #[serde(skip_serializing_if = "Option::is_none")]
        controller: Option<Box<Controller<'a>>>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    CycleData {
        controller_id: NonZeroU32,
        data: HashMap<&'a str, f64>,

        timestamp: DateTime<FixedOffset>,

        #[serde(flatten)]
        state: StateValues<'a>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    RequestJobCardsList {
        controller_id: NonZeroU32,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    JobCardsList {
        controller_id: NonZeroU32,
        data: HashMap<&'a str, JobCard<'a>>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    Join {
        #[serde(skip_serializing_if = "Option::is_none")]
        org_id: Option<&'a str>,
        version: &'a str,
        password: &'a str,
        language: Language,
        #[serde(
            serialize_with = "serialize_to_flatten_array",
            deserialize_with = "deserialize_flattened_array"
        )]
        filter: HashSet<Filter>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    JoinResponse {
        result: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        level: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(borrow)]
        message: Option<Cow<'a, str>>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    RequestMoldData {
        controller_id: NonZeroU32,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    MoldData {
        controller_id: NonZeroU32,
        data: HashMap<&'a str, f64>,

        timestamp: DateTime<FixedOffset>,

        #[serde(flatten)]
        state: StateValues<'a>,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    ReadMoldData {
        controller_id: NonZeroU32,
        field: &'a str,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    MoldDataValue {
        controller_id: NonZeroU32,
        field: &'a str,
        value: f64,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    LoginOperator {
        controller_id: NonZeroU32,
        password: &'a str,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
    #[serde(rename_all = "camelCase")]
    OperatorInfo {
        controller_id: NonZeroU32,
        #[serde(skip_serializing_if = "Option::is_none")]
        operator_id: Option<NonZeroU32>,
        name: &'a str,
        password: &'a str,
        level: u8,

        #[serde(flatten)]
        options: MessageOptions<'a>,
    },
}

impl<'a> Message<'a> {
    /// Parse a JSON string into a `Message`.
    ///
    /// # Errors
    ///
    /// Returns [`Err(OpenProtocolError)`] if there is an error, which may be one of the following:
    ///
    /// * `JsonError`: Error during parsing, which may be due to malformed JSON text, missing fields,
    ///   wrong data types for fields etc.
    /// * `EmptyField`: A mandatory `String` field is empty (i.e. zero-length) or all white-spaces.
    /// * `InvalidField`: The value of a field is inappropriate for that field, although there is no syntax error.
    ///   For example, encountering `NaN` on a numeric field usually yields this error.
    /// * `ConstraintViolated`: An integrity constraint is violated by the data structure.
    ///   For example, if current progress (`progress`) field of a `JobCard` structure is larger than
    ///   its total production count (`total`) field.
    ///
    pub fn parse_from_json_str(json: &'a str) -> Result<'a, Self> {
        match serde_json::from_str::<Message>(json) {
            Ok(m) => {
                m.check()?;
                Ok(m)
            }
            Err(err) => Err(OpenProtocolError::JsonError(err)),
        }
    }

    /// Validate all the fields in the `Message`, then serialize it into a JSON string.
    ///
    /// # Errors
    ///
    /// Returns [`Err(OpenProtocolError)`] if there is an error, which may be one of the following:
    ///
    /// * `JsonError`: Error during parsing, which may be due to malformed JSON text, missing fields,
    ///   wrong data types for fields etc.
    /// * `EmptyField`: A mandatory `String` field is empty (i.e. zero-length) or all white-spaces.
    /// * `InvalidField`: The value of a field is inappropriate for that field, although there is no syntax error.
    ///   For example, encountering `NaN` on a numeric field usually yields this error.
    /// * `ConstraintViolated`: An integrity constraint is violated by the data structure.
    ///   For example, if current progress (`progress`) field of a `JobCard` structure is larger than
    ///   its total production count (`total`) field.
    ///
    pub fn to_json_str(&self) -> Result<'_, String> {
        self.check()?;

        match serde_json::to_string(self) {
            Ok(text) => Ok(text),
            Err(err) => Err(OpenProtocolError::JsonError(err)),
        }
    }

    /// Create an `ALIVE` message.
    ///
    pub fn new_alive() -> Self {
        Alive {
            options: Default::default(),
        }
    }

    /// Create a `JOIN` message.
    ///
    pub fn new_join(password: &'a str, filter: HashSet<Filter>) -> Self {
        Self::new_join_with_org(password, filter, None)
    }

    /// Create a `JOIN` message with a non-default organization.
    ///
    pub fn new_join_with_org(password: &'a str, filter: HashSet<Filter>, org: Option<&'a str>) -> Self {
        Join {
            org_id: org,
            version: PROTOCOL_VERSION,
            password: password,
            language: DEFAULT_LANGUAGE,
            filter: filter,
            options: Default::default(),
        }
    }

    fn check(&self) -> Result<'a, ()> {
        match self {
            Alive { options, .. }
            | ControllerAction { options, .. }
            | RequestControllersList { options, .. }
            | RequestJobCardsList { options, .. }
            | JoinResponse { options, .. }
            | RequestMoldData { options, .. } => options.check(),
            ControllersList { options, data, .. } => {
                for c in data.iter() {
                    c.1.check()?;
                }
                options.check()
            }
            ControllerStatus {
                options,
                display_name,
                alarm,
                audit,
                variable,
                operator_name,
                job_card_id,
                mold_id,
                state,
                controller,
                ..
            } => {
                check_optional_str_empty(display_name, "display_name")?;
                check_optional_str_whitespace(operator_name, "operator_name")?;
                check_optional_str_whitespace(job_card_id, "job_card_id")?;
                check_optional_str_whitespace(mold_id, "mold_id")?;
                state.check()?;

                if let Some(kv) = alarm {
                    check_string_empty(kv.key, "alarm.key")?;
                }
                if let Some(kv) = audit {
                    check_string_empty(kv.key, "audit.key")?;
                    check_f64(&kv.value, "audit.value")?;
                }
                if let Some(kv) = variable {
                    check_string_empty(kv.key, "variable.key")?;
                    check_f64(&kv.value, "variable.value")?;
                }
                if let Some(c) = controller {
                    c.check()?;
                }

                options.check()
            }
            CycleData {
                options, data, state, ..
            } => {
                for d in data.iter() {
                    check_f64(d.1, d.0)?;
                }
                check_optional_str_empty(&state.job_card_id, "job_card_id")?;
                check_optional_str_empty(&state.mold_id, "mold_id")?;
                options.check()
            }
            JobCardsList { options, data, .. } => {
                for jc in data.iter() {
                    jc.1.check()?;
                }
                options.check()
            }
            Join {
                options,
                org_id,
                version,
                password,
                language,
                ..
            } => {
                check_optional_str_empty(org_id, "org_id")?;
                check_string_empty(version, "version")?;
                check_string_empty(password, "password")?;
                if *language == Language::Unknown {
                    return Err(OpenProtocolError::InvalidField {
                        field: "language".into(),
                        value: "Unknown".into(),
                        description: "Language cannot be Unknown.".into(),
                    });
                }
                options.check()
            }
            MoldData {
                options, data, state, ..
            } => {
                for d in data.iter() {
                    check_f64(d.1, d.0)?;
                }
                check_optional_str_empty(&state.job_card_id, "job_card_id")?;
                check_optional_str_empty(&state.mold_id, "mold_id")?;
                options.check()
            }
            ReadMoldData { options, field, .. } => {
                check_string_empty(field, "field")?;
                options.check()
            }
            MoldDataValue {
                options, field, value, ..
            } => {
                check_string_empty(field, "field")?;
                check_f64(&value, "value")?;
                options.check()
            }
            LoginOperator { options, password, .. } => {
                check_string_empty(&password, "password")?;
                options.check()
            }
            OperatorInfo {
                options,
                name,
                password,
                level,
                ..
            } => {
                check_string_empty(name, "name")?;
                check_string_empty(password, "password")?;
                if *level > MAX_OPERATOR_LEVEL {
                    return Err(OpenProtocolError::ConstraintViolated(
                        format!(
                            "Level {} is too high - must be between 0 and {}.",
                            level, MAX_OPERATOR_LEVEL
                        )
                        .into(),
                    ));
                }
                options.check()
            }
        }
    }
}

// Tests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alive() {
        let m = Alive {
            options: MessageOptions {
                id: Some("Hello"),
                sequence: 999,
                priority: 20,
            },
        };

        let serialized = serde_json::to_string(&m).unwrap();

        assert_eq!(
            r#"{"$type":"Alive","id":"Hello","sequence":999,"priority":20}"#,
            serialized
        );
    }

    #[test]
    fn test_mold_data() {
        let mut map = HashMap::<&str, f64>::new();

        map.insert("Hello", 123.0);
        map.insert("World", -987.6543);
        map.insert("foo", 0.0);

        let m = MoldData {
            controller_id: NonZeroU32::new(123).unwrap(),
            data: map,

            timestamp: DateTime::parse_from_rfc3339("2019-02-26T02:03:04+08:00").unwrap(),

            state: StateValues {
                job_card_id: Some("Hello World!".into()),
                mold_id: None,
                operator_id: Some(NonZeroU32::new(42).unwrap()),
                op_mode: OpMode::SemiAutomatic,
                job_mode: JobMode::Offline,
            },

            options: MessageOptions {
                id: None,
                sequence: 999,
                priority: -20,
            },
        };

        let serialized = serde_json::to_string(&m).unwrap();

        assert_eq!(
            r#"{"$type":"MoldData","controllerId":123,"data":{"foo":0.0,"Hello":123.0,"World":-987.6543},"timestamp":"2019-02-26T02:03:04+08:00","jobCardId":"Hello World!","operatorId":42,"opMode":"SemiAutomatic","jobMode":"Offline","sequence":999,"priority":-20}"#,
            serialized
        );

        let m2: Message = serde_json::from_str(&serialized).unwrap();
        m2.check().unwrap();

        assert_eq!(m, m2);
    }

    #[test]
    fn test_controllers_list() {
        let json = r#"{"$type":"ControllersList","data":{"12345":{"controllerId":12345,"displayName":"Hello","controllerType":"Ai12","version":"1.0.0","model":"JM128-Ai","IP":"192.168.5.1","opMode":"Manual","jobMode":"ID11","lastCycleData":{"Z_QDGODCNT":8567,"Z_QDCYCTIM":979,"Z_QDINJTIM":5450,"Z_QDPLSTIM":7156,"Z_QDINJENDPOS":8449,"Z_QDPLSENDPOS":2212,"Z_QDFLAG":8988,"Z_QDPRDCNT":65500,"Z_QDCOLTIM":4435,"Z_QDMLDOPNTIM":652,"Z_QDMLDCLSTIM":2908,"Z_QDVPPOS":4732,"Z_QDMLDOPNENDPOS":6677,"Z_QDMAXINJSPD":7133,"Z_QDMAXPLSRPM":641,"Z_QDNOZTEMP":6693,"Z_QDTEMPZ01":9964,"Z_QDTEMPZ02":7579,"Z_QDTEMPZ03":4035,"Z_QDTEMPZ04":5510,"Z_QDTEMPZ05":8460,"Z_QDTEMPZ06":9882,"Z_QDBCKPRS":2753,"Z_QDHLDTIM":9936},"lastConnectionTime":"2016-03-06T23:11:27.1442177+08:00"},"22334":{"controllerId":22334,"displayName":"World","controllerType":1,"version":"1.0.0","model":"JM128-Ai","IP":"192.168.5.2","opMode":"SemiAutomatic","jobMode":"ID12","lastCycleData":{"Z_QDGODCNT":6031,"Z_QDCYCTIM":7526,"Z_QDINJTIM":4896,"Z_QDPLSTIM":5196,"Z_QDINJENDPOS":1250,"Z_QDPLSENDPOS":8753,"Z_QDFLAG":3314,"Z_QDPRDCNT":65500,"Z_QDCOLTIM":3435,"Z_QDMLDOPNTIM":7854,"Z_QDMLDCLSTIM":4582,"Z_QDVPPOS":7504,"Z_QDMLDOPNENDPOS":7341,"Z_QDMAXINJSPD":7322,"Z_QDMAXPLSRPM":6024,"Z_QDNOZTEMP":3406,"Z_QDTEMPZ01":3067,"Z_QDTEMPZ02":9421,"Z_QDTEMPZ03":2080,"Z_QDTEMPZ04":8845,"Z_QDTEMPZ05":4478,"Z_QDTEMPZ06":3126,"Z_QDBCKPRS":2807,"Z_QDHLDTIM":3928},"lastConnectionTime":"2016-03-06T23:11:27.149218+08:00"}},"sequence":68568}"#;

        let m: Message = serde_json::from_str(&json).unwrap();
        m.check().unwrap();

        match m {
            ControllersList { data, .. } => {
                assert_eq!(2, data.len());
                let c = data.get(&NonZeroU32::new(12345).unwrap()).unwrap();
                assert_eq!("Hello", c.display_name.unwrap());
            }
            _ => panic!("Expected ControllersList, got {:?}", m),
        };
    }

    #[test]
    fn test_cycle_data() {
        let json = r#"{"$type":"CycleData","timestamp":"2016-02-26T01:12:23+08:00","opMode":"Automatic","jobMode":"ID02","controllerId":123,"data":{"Z_QDGODCNT":123,"Z_QDCYCTIM":12.33,"Z_QDINJTIM":3,"Z_QDPLSTIM":4.4,"Z_QDINJENDPOS":30.1,"Z_QDPLSENDPOS":20.3,"Z_QDFLAG":1,"Z_QDPRDCNT":500,"Z_QDCOLTIM":12.12,"Z_QDMLDOPNTIM":2.1,"Z_QDMLDCLSTIM":1.3,"Z_QDVPPOS":12.11,"Z_QDMLDOPNENDPOS":130.1,"Z_QDMAXINJSPD":213.12,"Z_QDMAXPLSRPM":551,"Z_QDNOZTEMP":256,"Z_QDTEMPZ01":251,"Z_QDTEMPZ02":252,"Z_QDTEMPZ03":253,"Z_QDTEMPZ04":254,"Z_QDTEMPZ05":255,"Z_QDTEMPZ06":256,"Z_QDBCKPRS":54,"Z_QDHLDTIM":2.3,"Z_QDCPT01":231,"Z_QDCPT02":232,"Z_QDCPT03":233,"Z_QDCPT04":234,"Z_QDCPT05":235,"Z_QDCPT06":236,"Z_QDCPT07":237,"Z_QDCPT08":238,"Z_QDCPT09":239,"Z_QDCPT10":240,"Z_QDCPT11":241,"Z_QDCPT12":242,"Z_QDCPT13":243,"Z_QDCPT14":244,"Z_QDCPT15":245,"Z_QDCPT16":246,"Z_QDCPT17":247,"Z_QDCPT18":248,"Z_QDCPT19":249,"Z_QDCPT20":250,"Z_QDCPT21":251,"Z_QDCPT22":252,"Z_QDCPT23":253,"Z_QDCPT24":254,"Z_QDCPT25":255,"Z_QDCPT26":256,"Z_QDCPT27":257,"Z_QDCPT28":258,"Z_QDCPT29":259,"Z_QDCPT30":260,"Z_QDCPT31":261,"Z_QDCPT32":262,"Z_QDCPT33":263,"Z_QDCPT34":264,"Z_QDCPT35":265,"Z_QDCPT36":266,"Z_QDCPT37":267,"Z_QDCPT38":268,"Z_QDCPT39":269,"Z_QDCPT40":270},"sequence":1}"#;

        let m: Message = serde_json::from_str(&json).unwrap();
        m.check().unwrap();

        match m {
            CycleData {
                options,
                controller_id,
                data,
                ..
            } => {
                assert_eq!(0, options.priority);
                assert_eq!(123, controller_id.get());
                assert_eq!(64, data.len());
                assert_eq!(243.0, *data.get("Z_QDCPT13").unwrap());
            }
            _ => panic!("Expected CycleData, got {:?}", m),
        };
    }

    #[test]
    fn test_controller_status() {
        let json = r#"{"$type":"ControllerStatus","controllerId":123,"displayName":"Testing","opMode":"Automatic","jobMode":"ID05","jobCardId":"XYZ","moldId":"Mold-123","state":{"opMode":"Automatic","jobMode":"ID05","jobCardId":"XYZ","moldId":"Mold-123"},"controller":{"controllerId":123,"displayName":"Testing","controllerType":"Ai02","version":"2.2","model":"JM138Ai","IP":"192.168.1.1:12345","geoLatitude":123.0,"geoLongitude":-21.0,"opMode":"Automatic","jobMode":"ID05","jobCardId":"XYZ","lastCycleData":{"INJ":5,"CLAMP":400},"moldId":"Mold-123"},"sequence":1,"priority":50}"#;

        let m: Message = serde_json::from_str(&json).unwrap();
        m.check().unwrap();

        match m {
            ControllerStatus {
                options,
                controller_id,
                display_name,
                controller,
                ..
            } => {
                assert_eq!(50, options.priority);
                assert_eq!(1, options.sequence);
                assert_eq!(123, controller_id.get());
                assert_eq!("Testing", display_name.unwrap());
                let c = controller.unwrap();
                assert_eq!("JM138Ai", c.model);
                let d = c.last_cycle_data.unwrap();
                assert!(c.operator.is_none());
                assert_eq!(2, d.len());
                assert_eq!(5.0, *d.get("INJ").unwrap());
            }
            _ => panic!("Expected CycleData, got {:?}", m),
        };
    }
}
