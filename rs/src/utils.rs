use super::{OpenProtocolError, Result};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;

pub fn check_str_empty<S: AsRef<str>>(text: S, field: &'static str) -> Result<'static, ()> {
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

pub fn check_f64(value: f64, field: &str) -> Result<()> {
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
    } else if !value.is_normal() && value != 0.0 {
        Err(OpenProtocolError::InvalidField {
            field: field.into(),
            value: "Sub-normal".into(),
            description: "Sub-normal numbers are not supported.".into(),
        })
    } else {
        Ok(())
    }
}

#[allow(clippy::option_option)]
pub fn deserialize_null_to_none<'de, D>(d: D) -> std::result::Result<Option<Option<&'de str>>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<Option<&str>>| Some(x.unwrap_or(None)))
}

#[allow(clippy::option_option)]
pub fn deserialize_null_to_cow_none<'de, D>(d: D) -> std::result::Result<Option<Option<Cow<'de, str>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<Option<Cow<'de, str>>>| Some(x.unwrap_or(None)))
}
