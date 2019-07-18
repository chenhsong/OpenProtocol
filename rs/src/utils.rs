use super::{OpenProtocolError, Result};
use serde::{Deserialize, Deserializer};

pub fn check_string_empty(text: &str, field: &'static str) -> Result<()> {
    if text.trim().is_empty() {
        return Err(OpenProtocolError::EmptyField(field));
    }
    Ok(())
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &i32) -> bool {
    *num == 0
}

pub fn check_optional_string_empty(opt: &Option<String>, field: &'static str) -> Result<()> {
    if let Some(text) = opt {
        if text.trim().is_empty() {
            return Err(OpenProtocolError::EmptyField(field));
        }
    }
    Ok(())
}

pub fn check_optional_str_empty(opt: &Option<&str>, field: &'static str) -> Result<()> {
    if let Some(text) = opt {
        if text.trim().is_empty() {
            return Err(OpenProtocolError::EmptyField(field));
        }
    }
    Ok(())
}

pub fn check_optional_string_whitespace(opt: &Option<String>, field: &'static str) -> Result<()> {
    if let Some(text) = opt {
        if !text.is_empty() && text.trim().is_empty() {
            return Err(OpenProtocolError::EmptyField(field));
        }
    }
    Ok(())
}

pub fn check_f64(value: &f64, field: &str) -> Result<()> {
    if value.is_nan() {
        Err(OpenProtocolError::InvalidField(
            Box::new(field.to_string()),
            Box::new("NaN".to_string()),
        ))
    } else if value.is_infinite() {
        Err(OpenProtocolError::InvalidField(
            Box::new(field.to_string()),
            Box::new("Infinity".to_string()),
        ))
    } else if !value.is_normal() && *value != 0.0 {
        Err(OpenProtocolError::InvalidField(
            Box::new(field.to_string()),
            Box::new("Subnormal".to_string()),
        ))
    } else {
        Ok(())
    }
}

pub fn deserialize_null_to_empty_string<'de, D>(d: D) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.or(Some("".to_string())))
}
