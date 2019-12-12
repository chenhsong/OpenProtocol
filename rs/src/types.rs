use derive_more::*;
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::num::NonZeroU32;

/// Supported UI languages for the controller's HMI.
///
/// See [this document] for details.
///
/// [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#languages
///
#[derive(
    Debug, Display, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, Copy, Clone,
)]
pub enum Language {
    /// Unknown language.
    #[display(fmt = "Unknown")]
    Unknown,
    /// English (en)
    #[display(fmt = "English")]
    EN,
    /// Traditional Chinese (zh-tw)
    #[display(fmt = "䌓體中文")]
    B5,
    /// Simplified Chinese (zh-cn)
    #[display(fmt = "简体中文")]
    GB,
    /// French (fr)
    #[display(fmt = "Français")]
    FR,
    /// German (de)
    #[display(fmt = "Deutsch")]
    DE,
    /// Italian (it)
    #[display(fmt = "Italiano")]
    IT,
    /// Spanish (es)
    #[display(fmt = "Español")]
    ES,
    /// Portuguese (pt)
    #[display(fmt = "Português")]
    PT,
    /// Japanese (ja)
    #[display(fmt = "日本語")]
    JA,
}

impl Language {
    /// Returns true if Language::Unknown.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(Language::Unknown.is_unknown());
    /// assert!(!Language::FR.is_unknown());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_unknown(&self) -> bool {
        *self == Language::Unknown
    }
}

impl Default for Language {
    /// Default value for `Language`.
    fn default() -> Self {
        Language::Unknown
    }
}

/// Operating modes of the controller.
///
/// See [this document] for details.
///
/// [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#opmodes
///
#[derive(
    Debug, Display, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, Copy, Clone,
)]
pub enum OpMode {
    /// Unknown operation mode.
    Unknown,
    /// Manual mode.
    Manual,
    /// Semi-Automatic mode.
    #[display(fmt = "Semi-Automataic")]
    SemiAutomatic,
    /// Automatic mode.
    Automatic,
    /// Other unspecified operation mode.
    Others,
    /// The controller is off-line.
    ///
    /// When the controller is off-line, both its operating mode and [job mode] should be `Offline`.
    ///
    /// [job mode]: enum.JobMode.html
    ///
    #[display(fmt = "Off-Line")]
    Offline,
}

impl OpMode {
    /// Returns true if `Unknown`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(OpMode::Unknown.is_unknown());
    /// assert!(!OpMode::Manual.is_unknown());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_unknown(&self) -> bool {
        *self == OpMode::Unknown
    }

    /// Returns true if `Offline`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(OpMode::Offline.is_offline());
    /// assert!(!OpMode::Manual.is_offline());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_offline(&self) -> bool {
        *self == OpMode::Offline
    }

    /// Returns true for all variants other than `Unknown` and `Offline`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(!OpMode::Offline.is_online());
    /// assert!(!OpMode::Unknown.is_online());
    /// assert!(OpMode::Automatic.is_online());
    /// assert!(OpMode::Manual.is_online());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_online(&self) -> bool {
        match self {
            OpMode::Unknown | OpMode::Offline => false,
            _ => true,
        }
    }

    /// A machine is producing if it is in either `Automatic` or `Semi-Automatic` mode.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(!OpMode::Offline.is_producing());
    /// assert!(!OpMode::Unknown.is_producing());
    /// assert!(OpMode::Automatic.is_producing());
    /// assert!(!OpMode::Manual.is_producing());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_producing(&self) -> bool {
        match self {
            OpMode::SemiAutomatic | OpMode::Automatic => true,
            _ => false,
        }
    }
}

impl Default for OpMode {
    /// Default value for `OpMode`.
    fn default() -> Self {
        OpMode::Unknown
    }
}

/// Job modes of the controller.
///
/// On some controller models, job modes 1-15 (`ID01` - `ID15`) can be user-defined.
///
/// See [this document] for details.
///
/// [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#jobmodes
///
#[derive(
    Debug, Display, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, Copy, Clone,
)]
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
    /// When the controller is off-line, both its [operating mode] and job mode should be `Offline`.
    ///
    /// [operating mode]: enum.OpMode.html
    ///
    #[display(fmt = "Off-Line")]
    Offline,
}

impl JobMode {
    /// Returns true if `Unknown`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(JobMode::Unknown.is_unknown());
    /// assert!(!JobMode::ID08.is_unknown());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_unknown(&self) -> bool {
        *self == JobMode::Unknown
    }

    /// Returns true if `Offline`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(JobMode::Offline.is_offline());
    /// assert!(!JobMode::ID08.is_offline());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_offline(&self) -> bool {
        *self == JobMode::Offline
    }

    /// Returns true for all variants other than `Unknown` and `Offline`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert!(!JobMode::Offline.is_online());
    /// assert!(!JobMode::Unknown.is_online());
    /// assert!(JobMode::ID01.is_online());
    /// assert!(JobMode::ID15.is_online());
    /// ~~~
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_online(&self) -> bool {
        match self {
            JobMode::Unknown | JobMode::Offline => false,
            _ => true,
        }
    }
}

impl Default for JobMode {
    /// Default value for `JobMode`.
    fn default() -> Self {
        JobMode::Unknown
    }
}

/// A 32-bit numeric ID that cannot be zero or negative.
///
/// This type is usually used for specifying a unique identification number.
///
#[derive(
    Display,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    From,
    Into,
    FromStr,
    Serialize,
    Deserialize,
)]
pub struct ID(NonZeroU32);

impl ID {
    /// Create a new ID from a `u32` value.
    ///
    /// # Errors
    ///
    /// Returns `Err(&'static str)` if `value` is zero.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let id = ID::new(42).unwrap();
    /// assert_eq!(42, u32::from(id));
    /// assert_eq!(Err("ID value cannot be zero."), ID::new(0));
    /// ~~~
    pub fn new(value: u32) -> std::result::Result<Self, &'static str> {
        Self::try_from(value)
    }
    //
    /// Create a new ID from a `u32` value.
    ///
    /// # Panics
    ///
    /// Panics if `value` is zero.
    ///
    /// ## Error Examples
    ///
    /// ~~~should_panic
    /// # use ichen_openprotocol::*;
    /// let id = ID::from_u32(0);    // This will panic.
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let id = ID::from_u32(42);
    /// assert_eq!(42, u32::from(id));
    /// ~~~
    pub fn from_u32(value: u32) -> Self {
        Self::try_from(value).unwrap()
    }
}

impl Debug for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl TryFrom<u32> for ID {
    type Error = &'static str;

    /// Create a new ID from an integer value;
    ///
    /// # Errors
    ///
    /// Return `Err(&'static str)` if `num` is zero.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use std::convert::TryFrom;
    /// # use ichen_openprotocol::*;
    /// let id = ID::try_from(42).unwrap();
    /// assert_eq!(42, u32::from(id));
    /// assert_eq!(Err("ID value cannot be zero."), ID::try_from(0));
    /// ~~~
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        NonZeroU32::new(value).map(Self).ok_or("ID value cannot be zero.")
    }
}

impl From<ID> for u32 {
    fn from(id: ID) -> Self {
        id.0.get()
    }
}

impl PartialEq<u32> for ID {
    fn eq(&self, other: &u32) -> bool {
        self.0.get() == *other
    }
}

impl PartialEq<ID> for u32 {
    fn eq(&self, other: &ID) -> bool {
        other == self
    }
}

impl PartialOrd<u32> for ID {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.get().partial_cmp(other)
    }
}

impl PartialOrd<ID> for u32 {
    fn partial_cmp(&self, other: &ID) -> Option<Ordering> {
        other.partial_cmp(self)
    }
}

/// A 32-bit ID that represents a controller action.
///
#[derive(
    Display,
    Constructor,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    From,
    Into,
    FromStr,
    Serialize,
    Deserialize,
)]
pub struct ActionID(i32);

impl Debug for ActionID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl PartialEq<i32> for ActionID {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ActionID> for i32 {
    fn eq(&self, other: &ActionID) -> bool {
        other.0 == *self
    }
}

impl PartialOrd<i32> for ActionID {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<ActionID> for i32 {
    fn partial_cmp(&self, other: &ActionID) -> Option<Ordering> {
        other.partial_cmp(self)
    }
}
