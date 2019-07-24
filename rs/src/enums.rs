use serde::{Deserialize, Serialize};

/// Supported UI languages for the controller's HMI.
///
/// For details see [this document](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#languages).
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
    /// Returns true if Language::Unknown.
    #[allow(clippy::trivially_copy_pass_by_ref)]
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
/// For details, see [this document](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#opmodes).
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
    /// Returns true if OpMode::Unknown.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_unknown(&self) -> bool {
        *self == OpMode::Unknown
    }

    /// Returns true if OpMode::Offline.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_offline(&self) -> bool {
        *self == OpMode::Offline
    }

    /// All variants other than OpMode::Unknown and OpMode::Offline means on-line.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_online(&self) -> bool {
        match self {
            OpMode::Unknown | OpMode::Offline => false,
            _ => true,
        }
    }

    /// A machine is producing if it is in either Automatic or Semi-Automatic mode.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_producing(&self) -> bool {
        match self {
            OpMode::SemiAutomatic | OpMode::Automatic => true,
            _ => false,
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
/// For details, see [this document](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#jobmodes).
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
    /// Returns true if JobMode::Unknown.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_unknown(&self) -> bool {
        *self == JobMode::Unknown
    }

    /// Returns true if JobMode::Offline.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_offline(&self) -> bool {
        *self == JobMode::Offline
    }

    /// All variants other than JobMode::Unknown and JobMode::Offline means on-line.
    #[allow(clippy::trivially_copy_pass_by_ref)]
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
