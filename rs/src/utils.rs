use super::{OpenProtocolError, Result};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;

pub fn check_string_empty(text: &str, field: &'static str) -> Result<'static, ()> {
    if text.trim().is_empty() {
        Err(OpenProtocolError::EmptyField(field.into()))
    } else {
        Ok(())
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &i32) -> bool {
    *num == 0
}

pub fn check_optional_str_empty<'a>(opt: &Option<&str>, field: &'static str) -> Result<'static, ()> {
    if let Some(text) = opt {
        if text.trim().is_empty() {
            Err(OpenProtocolError::EmptyField(field.into()))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

pub fn check_optional_cowstr_empty(opt: &Option<Cow<'_, str>>, field: &'static str) -> Result<'static, ()> {
    if let Some(text) = opt {
        if text.trim().is_empty() {
            Err(OpenProtocolError::EmptyField(field.into()))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

pub fn check_optional_cowstr_whitespace(opt: &Option<Cow<'_, str>>, field: &'static str) -> Result<'static, ()> {
    if let Some(text) = opt {
        if !text.is_empty() && text.trim().is_empty() {
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

pub fn deserialize_null_to_empty_string<'de, D>(d: D) -> std::result::Result<Option<Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<&str>| Some(x.unwrap_or("").into()))
}
