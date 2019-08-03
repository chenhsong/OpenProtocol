use super::{OpenProtocolError, Result, ValidationResult};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &i32) -> bool {
    *num == 0
}

pub fn check_str_empty<S: AsRef<str>>(text: S, field: &'static str) -> ValidationResult {
    if text.as_ref().trim().is_empty() {
        return Err(OpenProtocolError::EmptyField(field.into()));
    }
    Ok(())
}

pub fn check_optional_str_empty<S: AsRef<str>>(
    opt: &Option<S>,
    field: &'static str,
) -> ValidationResult {
    match opt {
        Some(text) if text.as_ref().trim().is_empty() => {
            Err(OpenProtocolError::EmptyField(field.into()))
        }
        _ => Ok(()),
    }
}

pub fn check_optional_str_whitespace<S: AsRef<str>>(
    opt: &Option<S>,
    field: &'static str,
) -> ValidationResult {
    match opt {
        Some(text) if !text.as_ref().is_empty() && text.as_ref().trim().is_empty() => {
            Err(OpenProtocolError::EmptyField(field.into()))
        }
        _ => Ok(()),
    }
}

pub fn check_f64(value: f64, field: &str) -> Result<()> {
    if value.is_nan() {
        Err(OpenProtocolError::InvalidField {
            field: field.into(),
            value: "NaN".into(),
            description: "NaN is not a supported value.".into(),
        })
    } else if value.is_infinite() {
        Err(OpenProtocolError::InvalidField {
            field: field.into(),
            value: "Infinity".into(),
            description: "Infinity is not a supported value.".into(),
        })
    } else if !value.is_normal() && value != 0.0 {
        Err(OpenProtocolError::InvalidField {
            field: field.into(),
            value: "Sub-normal".into(),
            description: "Sub-normal number is not a supported value.".into(),
        })
    } else {
        Ok(())
    }
}

#[allow(clippy::option_option)]
pub fn deserialize_null_to_some_none<'de, D, T>(
    d: D,
) -> std::result::Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Deserialize::deserialize(d).map(Some)
}

pub fn deserialize_hashmap<'de, D, K, T>(d: D) -> std::result::Result<HashMap<K, T>, D::Error>
where
    D: Deserializer<'de>,
    K: FromStr + Eq + Hash,
    K::Err: Display,
    T: Deserialize<'de>,
{
    fn deserialize_string_key<'de, D, S>(d: D) -> std::result::Result<S, D::Error>
    where
        D: Deserializer<'de>,
        S: FromStr,
        S::Err: Display,
    {
        let s = Deserialize::deserialize(d).map_err(serde::de::Error::custom)?;
        S::from_str(s).map_err(serde::de::Error::custom)
    }

    #[derive(Deserialize, Hash, Eq, PartialEq)]
    struct Wrapper<S>(#[serde(deserialize_with = "deserialize_string_key")] S)
    where
        S: FromStr,
        S::Err: Display;

    let dict: HashMap<Wrapper<K>, T> = Deserialize::deserialize(d)?;
    Ok(dict.into_iter().map(|(Wrapper(k), v)| (k, v)).collect())
}
