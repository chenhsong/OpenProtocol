use serde::{Deserialize, Serialize};

/// Supported UI languages for the controller's HMI.
///
/// For details see [here](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#languages).
///
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, Copy, Clone)]
pub enum Language {
    /// Unknown language.
    Unknown,
    /// English (en)
    EN,
    /// Traditional Chinese (zh-tw)
    B5,
    /// Simplified Chinese (zh-cn)
    GB,
    /// French (fr)
    FR,
    /// German (de)
    DE,
    /// Italian (it)
    IT,
    /// Spanish (es)
    ES,
    /// Portuguese (pt)
    PT,
    /// Japanese (ja)
    JA,
}

impl Language {
    pub fn is_unknown(&self) -> bool {
        *self == Language::Unknown
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Unknown
    }
}

/// Operating modes of the controller.
///
/// For details, see [here](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#opmodes).
///
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, Copy, Clone)]
pub enum OpMode {
    /// Unknown operation mode.
    Unknown,
    /// Manual mode.
    Manual,
    /// Semi-Automatic mode.
    SemiAutomatic,
    /// Automatic mode.
    Automatic,
    /// Other unspecified operation mode.
    Others,
    /// The controller is off-line.
    ///
    /// # Note
    ///
    /// When the controller is off-line, both its operating mode and job mode should be `Offline`.
    Offline,
}

impl OpMode {
    pub fn is_unknown(&self) -> bool {
        *self == OpMode::Unknown
    }

    pub fn is_offline(&self) -> bool {
        *self == OpMode::Offline
    }

    pub fn is_online(&self) -> bool {
        match self {
            OpMode::Unknown | OpMode::Offline => false,
            _ => true,
        }
    }
}

impl Default for OpMode {
    fn default() -> Self {
        OpMode::Unknown
    }
}

/// Job modes of the controller.
///
/// On some controller models, job modes 1-15 (`ID01` - `ID15`) can be user-defined.
///
/// For details, see [here](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#jobmodes).
///
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, Copy, Clone)]
pub enum JobMode {
    /// Unknown job mode.
    Unknown,
    ID01,
    ID02,
    ID03,
    ID04,
    ID05,
    ID06,
    ID07,
    ID08,
    ID09,
    ID10,
    ID11,
    ID12,
    ID13,
    ID14,
    ID15,
    /// The controller is off-line.
    ///
    /// # Note
    ///
    /// When the controller is off-line, both its operating mode and job mode should be `Offline`.
    Offline,
}

impl JobMode {
    pub fn is_unknown(&self) -> bool {
        *self == JobMode::Unknown
    }

    pub fn is_offline(&self) -> bool {
        *self == JobMode::Offline
    }

    pub fn is_online(&self) -> bool {
        match self {
            JobMode::Unknown | JobMode::Offline => false,
            _ => true,
        }
    }
}

impl Default for JobMode {
    fn default() -> Self {
        JobMode::Unknown
    }
}
