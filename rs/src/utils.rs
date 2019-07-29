use super::{Controller, OpenProtocolError, Result};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroU32;

pub fn check_str_empty<S: AsRef<str>>(text: S, field: &'static str) -> Result<'static, ()> {
    if text.as_ref().trim().is_empty() {
        return Err(OpenProtocolError::EmptyField(field.into()));
    }
    Ok(())
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &i32) -> bool {
    *num == 0
}

pub fn check_optional_str_empty<S: AsRef<str>>(
    opt: &Option<S>,
    field: &'static str,
) -> Result<'static, ()> {
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
) -> Result<'static, ()> {
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
pub fn deserialize_null_to_none<'de, D>(
    d: D,
) -> std::result::Result<Option<Option<&'de str>>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<Option<&str>>| Some(x.unwrap_or(None)))
}

#[allow(clippy::option_option)]
pub fn deserialize_null_to_cow_none<'de, D>(
    d: D,
) -> std::result::Result<Option<Option<Cow<'de, str>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<Option<Cow<'de, str>>>| Some(x.unwrap_or(None)))
}

pub fn deserialize_string_to_u32<'de, D>(d: D) -> std::result::Result<NonZeroU32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d).map_err(serde::de::Error::custom)?;
    s.parse::<NonZeroU32>().map_err(serde::de::Error::custom)
}

pub fn deserialize_hashmap_with_u32_key<'de, D>(
    d: D,
) -> std::result::Result<HashMap<NonZeroU32, Controller<'de>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Hash, Eq, PartialEq)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_string_to_u32")] NonZeroU32);

    let dict: HashMap<Wrapper, Controller<'de>> = Deserialize::deserialize(d)?;
    Ok(dict.into_iter().map(|(Wrapper(k), v)| (k, v)).collect())
}
