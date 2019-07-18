use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

/// General authorizations to access the iChen System via Open Protocol.
///
/// For details, see [here](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#filters).
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
    /// `All` = `Status` + `Cycle` + `Mold` + `Actions` + `Alarms` + `Audit`
    All,
    //
    /// MIS/MES integration: Job scheduling messages.
    JobCards,
    /// MIS/MES integration: User authorization messages.
    Operators,
    //
    /// MIS/MES integration: Connect to the OPC UA server.
    OPCUA,
}

// Custom serializer and deserializer

pub fn serialize_to_flatten_array<S>(x: &HashSet<Filter>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Streamline filters
    let has_all = x.contains(&Filter::All);
    let mut fstr = String::with_capacity(64);

    for f in x.iter() {
        match f {
            Filter::Status | Filter::Cycle | Filter::Mold | Filter::Actions | Filter::Alarms | Filter::Audit => {
                // Skip these if Filter::All exists
                if !has_all {
                    if fstr.len() > 0 {
                        fstr.push_str(", ");
                    }

                    fstr.push_str(f.as_ref());
                }
            }
            _ => {
                if fstr.len() > 0 {
                    fstr.push_str(", ");
                }

                fstr.push_str(f.as_ref());
            }
        }
    }

    if fstr.is_empty() {
        fstr.push_str("None");
    }

    s.serialize_str(&fstr)
}

pub fn deserialize_flattened_array<'de, D>(d: D) -> Result<HashSet<Filter>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(d)?;

    let mut dict: HashSet<Filter> = HashSet::new();

    if text == "None" {
        return Ok(dict);
    }

    for key in text.split(",") {
        if let Ok(filter) = Filter::from_str(key.trim()) {
            dict.insert(filter);
        }
    }

    if dict.contains(&Filter::All) {
        // Has All, remove details
        dict.remove(&Filter::Status);
        dict.remove(&Filter::Cycle);
        dict.remove(&Filter::Mold);
        dict.remove(&Filter::Actions);
        dict.remove(&Filter::Alarms);
        dict.remove(&Filter::Audit);
    }

    Ok(dict)
}
