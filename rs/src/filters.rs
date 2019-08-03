#![allow(non_upper_case_globals)]

use bitflags::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

bitflags! {
    /// General authorizations to access the iChen System via Open Protocol.
    ///
    /// For details, see [this document](https://github.com/chenhsong/OpenProtocol/blob/master/doc/enums.md#filters).
    ///
    pub struct Filters: u32 {
        /// No rights.
        const None = 0;

        /// Controller status update messages.
        const Status = 0b_0000_0001;
        /// Cycle data messages.
        const Cycle = 0b_0000_0010;
        /// Mold data messages.
        const Mold = 0b_0000_0100;
        /// Controller action messages.
        const Actions = 0b_0000_1000;
        /// Controller alarm messages.
        const Alarms = 0b_0001_0000;
        /// Controller audit messages.
        const Audit = 0b_0010_0000;
        /// Administrator rights.
        ///
        /// `All` implies `Status` + `Cycle` + `Mold` + `Actions` + `Alarms` + `Audit`
        const All = 0b_1111_1111;
        //
        /// MIS/MES integration: Job scheduling messages.
        const JobCards = 0b_0001_0000_0000_0000;
        /// MIS/MES integration: User authorization messages.
        const Operators = 0b_0010_0000_0000_0000;
        //
        /// Industrial bus integration: Connect to the OPC UA server.
        const OPCUA = 0b_0001_0000_0000_0000_0000_0000_0000_0000;
    }
}

static ALL: &str = "Status, Cycle, Mold, Actions, Alarms, Audit, All";

impl Filters {
    /// Is a particular set of filters set?
    pub fn has(self, other: Self) -> bool {
        self.contains(other)
    }
}

impl FromStr for Filters {
    type Err = String;

    // Notice that `from_str` never fails.
    // Unmatched tokens will simply be discarded.  If nothing matches, `Filters::None` will be returned.
    //
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();
        if text == "None" || text.is_empty() {
            return Ok(Filters::None);
        }

        let filters = text
            .split(',')
            .map(|t| match t.trim() {
                "Status" => Filters::Status,
                "Cycle" => Filters::Cycle,
                "Mold" => Filters::Mold,
                "Actions" => Filters::Actions,
                "Alarms" => Filters::Alarms,
                "Audit" => Filters::Audit,
                "All" => Filters::All,
                "JobCards" => Filters::JobCards,
                "Operators" => Filters::Operators,
                "OPCUA" => Filters::OPCUA,
                _ => Filters::None,
            })
            .fold(Filters::None, |f, x| f | x);

        Ok(filters)
    }
}

impl std::ops::Add for Filters {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        self | rhs
    }
}

impl std::ops::AddAssign for Filters {
    fn add_assign(&mut self, other: Self) {
        *self |= other;
    }
}

impl std::fmt::Display for Filters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = format!("{:?}", self).replace(" | ", ", ");
        let mut slice = text.trim();

        // Remove redundant flags when All is present
        if slice.starts_with(ALL) {
            slice = text[ALL.len() - 3..].trim();
        }

        if text.is_empty() {
            write!(f, "None")
        } else {
            write!(f, "{}", slice)
        }
    }
}

impl Serialize for Filters {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Filters {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Deserialize::deserialize(d).map_err(serde::de::Error::custom)?;
        Filters::from_str(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_filters() {
        let f = Filters::All + Filters::Cycle + Filters::OPCUA;
        assert_eq!("All, OPCUA", format!("{}", f));
    }

    #[test]
    fn test_deserialize_filters() {
        let f = Filters::from_str("All, OPCUA").unwrap();
        assert!(f.has(Filters::All));
        assert!(f.has(Filters::OPCUA));
        assert!(!f.has(Filters::Operators));
        assert!(!f.has(Filters::JobCards));
        assert!(f.has(Filters::Cycle));
        assert!(f.has(Filters::Status));
        assert!(f.has(Filters::Mold));
        assert!(f.has(Filters::Audit));
        assert!(f.has(Filters::Alarms));
    }
}
