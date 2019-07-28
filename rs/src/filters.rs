use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashSet;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

pub trait Filters {
    /// Does the collection of `Filter` values contain administrator rights?
    fn is_all(&self) -> bool;

    /// Does the collection of `Filter` values contain the specified filter?
    fn has(&self, filter: Filter) -> bool;

    /// Normalize the collection of `Filter` values to remove duplications.
    fn normalize(&mut self);
}

/// General authorizations to access the iChen System via Open Protocol.
///
/// For details, see [this document](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#filters).
///
#[derive(
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Copy,
    Clone,
    EnumString,
    Display,
    EnumIter,
    AsRefStr,
    IntoStaticStr,
)]
pub enum Filter {
    /// Controller status update messages.
    Status,
    /// Cycle data messages.
    Cycle,
    /// Mold data messages.
    Mold,
    /// Controller action messages.
    Actions,
    /// Controller alarm messages.
    Alarms,
    /// Controller audit messages.
    Audit,
    /// Administrator rights.
    ///
    /// `All` implies `Status` + `Cycle` + `Mold` + `Actions` + `Alarms` + `Audit`
    All,
    //
    /// MIS/MES integration: Job scheduling messages.
    JobCards,
    /// MIS/MES integration: User authorization messages.
    Operators,
    //
    /// Industrial bus integration: Connect to the OPC UA server.
    OPCUA,
}

impl Filter {
    /// Returns true if `Filter::All`.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_all(&self) -> bool {
        *self == Filter::All
    }

    /// Returns true if machine-related filter flags.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_machine(&self) -> bool {
        match self {
            Filter::Status
            | Filter::Cycle
            | Filter::Mold
            | Filter::Actions
            | Filter::Alarms
            | Filter::Audit => true,
            _ => false,
        }
    }

    /// Returns true if the filter flags is covered by `Filter::All`.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_part_of_all(&self) -> bool {
        match self {
            Filter::Status
            | Filter::Cycle
            | Filter::Mold
            | Filter::Actions
            | Filter::Alarms
            | Filter::Audit => true,
            _ => false,
        }
    }

    /// Returns true if MIS/MES-related filter flags.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_mis(&self) -> bool {
        match self {
            Filter::JobCards | Filter::Operators => true,
            _ => false,
        }
    }

    /// Returns true if interface of an industrial bus (e.g. OPC UA).
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_bus(&self) -> bool {
        match self {
            Filter::OPCUA => true,
            _ => false,
        }
    }
}

impl Filters for [Filter] {
    fn is_all(&self) -> bool {
        // If Filter::All is present
        self.has(Filter::All)
    }

    fn has(&self, filter: Filter) -> bool {
        self.contains(&filter)
    }

    fn normalize(&mut self) {
        panic!("[Filter].normalize() is not supported.");
    }
}

impl Filters for Vec<Filter> {
    fn is_all(&self) -> bool {
        (self as &[Filter]).is_all()
    }

    fn has(&self, filter: Filter) -> bool {
        (self as &[Filter]).has(filter)
    }

    fn normalize(&mut self) {
        if self.contains(&Filter::All) {
            // Has All, remove any filter that is covered by all
            self.retain(|f| !f.is_part_of_all());
        }

        // Remove duplications
        self.dedup();
    }
}

impl<S: std::hash::BuildHasher> Filters for HashSet<Filter, S> {
    fn is_all(&self) -> bool {
        self.contains(&Filter::All)
    }

    fn has(&self, filter: Filter) -> bool {
        self.contains(&filter)
    }

    fn normalize(&mut self) {
        if self.is_all() {
            // Has All, remove any filter that is covered by all
            self.retain(|f| !f.is_part_of_all());
        }
    }
}

// Custom serializer and deserializer

static EMPTY_FILTERS: &[Filter] = &[];

pub fn serialize_to_flatten_array<S>(x: &[Filter], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Streamline filters
    let has_all = x.contains(&Filter::All);
    let fstr = x
        .iter()
        .filter(|f| !has_all || !f.is_machine())
        .map(|f| f.as_ref())
        .collect::<Vec<&str>>()
        .join(", ");

    if fstr.is_empty() {
        s.serialize_str("None")
    } else {
        s.serialize_str(&fstr)
    }
}

pub fn deserialize_flattened_array<'de, D>(d: D) -> Result<Cow<'de, [Filter]>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(d)?;
    let text = text.trim();

    if text == "None" {
        return Ok(EMPTY_FILTERS.into());
    }

    let mut list: Vec<Filter> =
        text.split(',').filter_map(|key| Filter::from_str(key.trim()).ok()).collect();

    list.normalize();

    if list.is_empty() {
        Ok(EMPTY_FILTERS.into())
    } else {
        Ok(list.into())
    }
}
