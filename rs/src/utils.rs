use super::{OpenProtocolError, Result};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;

pub fn check_string_empty<S: AsRef<str>>(text: S, field: &'static str) -> Result<'static, ()> {
    if text.as_ref().trim().is_empty() {
        Err(OpenProtocolError::EmptyField(field.into()))
    } else {
        Ok(())
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &i32) -> bool {
    *num == 0
}

pub fn check_optional_str_empty<S: AsRef<str>>(opt: &Option<S>, field: &'static str) -> Result<'static, ()> {
    if let Some(text) = opt {
        if text.as_ref().trim().is_empty() {
            Err(OpenProtocolError::EmptyField(field.into()))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

pub fn check_optional_str_whitespace<S: AsRef<str>>(opt: &Option<S>, field: &'static str) -> Result<'static, ()> {
    if let Some(text) = opt {
        if !text.as_ref().is_empty() && text.as_ref().trim().is_empty() {
            return Err(OpenProtocolError::EmptyField(field.into()));
        }
    }
    Ok(())
}

pub fn check_f64<'a>(value: &f64, field: &'a str) -> Result<'a, ()> {
    if value.is_nan() {
        Err(OpenProtocolError::InvalidField {
            field: field.into(),
            value: "NaN".into(),
            description: "NaN is not supported.".into(),
        })
    } else if value.is_infinite() {
        Err(OpenProtocolError::InvalidField {
            field: field.into(),
            value: "Infinity".into(),
            description: "Infinity is not supported.".into(),
        })
    } else if !value.is_normal() && *value != 0.0 {
        Err(OpenProtocolError::InvalidField {
            field: field.into(),
            value: "Sub-normal".into(),
            description: "Sub-normal numbers are not supported.".into(),
        })
    } else {
        Ok(())
    }
}

pub fn deserialize_null_to_empty_str<'de, D>(d: D) -> std::result::Result<Option<&'de str>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<&str>| Some(x.unwrap_or("")))
}

pub fn deserialize_null_to_empty_cowstr<'de, D>(d: D) -> std::result::Result<Option<Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<&str>| Some(x.unwrap_or("").into()))
}
