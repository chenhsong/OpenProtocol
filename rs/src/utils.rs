use super::{BoundedValidationResult, Error, ValidationResult, ID};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Display;
use std::hash::Hash;
use std::num::NonZeroU32;
use std::str::FromStr;

/// A trait to specify different _invalid_ values for a type for serialization purposes
///
pub trait InvalidValues {
    type Marker;

    /// Returns the standard invalid value for an implementing type
    ///
    fn invalid() -> Self::Marker;
}

impl InvalidValues for ID {
    type Marker = u32;

    /// [`ID`] cannot be zero.
    ///
    /// [`ID`]: struct.ID.html
    ///
    fn invalid() -> Self::Marker {
        0
    }
}

impl InvalidValues for NonZeroU32 {
    type Marker = u32;

    /// `NonZeroU32` cannot be zero.
    ///
    fn invalid() -> Self::Marker {
        0
    }
}

impl InvalidValues for f32 {
    type Marker = f32;

    /// Use NaN for floating-point numbers.
    ///
    fn invalid() -> Self::Marker {
        std::f32::NAN
    }
}

impl InvalidValues for f64 {
    type Marker = f64;

    /// Use NaN for floating-point numbers.
    ///
    fn invalid() -> Self::Marker {
        std::f64::NAN
    }
}

/// Used to suppress serialization numeric fields that are zero (e.g. priority).
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &i32) -> bool {
    *num == 0
}

/// Check if a string is empty or contains all whitespace.
///
/// # Errors
///
/// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if `text` is empty or contains
/// all whitespace.
///
/// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
///
pub fn check_str_empty<S: AsRef<str>>(text: S, field: &'static str) -> ValidationResult {
    if text.as_ref().trim().is_empty() {
        return Err(Error::EmptyField(field.into()));
    }
    Ok(())
}

/// Check if an optional string is empty or contains all whitespace.
///
/// # Errors
///
/// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if `opt` is `Some` text which either
/// is empty or contains all whitespace.
///
/// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
///
pub fn check_optional_str_empty<S: AsRef<str>>(
    opt: &Option<S>,
    field: &'static str,
) -> ValidationResult {
    match opt {
        Some(text) if text.as_ref().trim().is_empty() => Err(Error::EmptyField(field.into())),
        _ => Ok(()),
    }
}

/// Check if an optional string contains all whitespace (but is not empty).
///
/// # Errors
///
/// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if `opt` is `Some` text which is
/// not empty but contains all whitespace.
///
/// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
///
pub fn check_optional_str_whitespace<S: AsRef<str>>(
    opt: &Option<S>,
    field: &'static str,
) -> ValidationResult {
    match opt {
        Some(text) if !text.as_ref().is_empty() && text.as_ref().trim().is_empty() => {
            Err(Error::EmptyField(field.into()))
        }
        _ => Ok(()),
    }
}

/// Check for non-numeric values of an `f64` field.
///
/// # Errors
///
/// Returns `Err(`[`OpenProtocolError::InvalidField`]`)` if `value` is not a normal number
/// (e.g. `NaN`, `Infinity`).
///
/// [`OpenProtocolError::InvalidField`]: enum.OpenProtocolError.html#variant.InvalidField
///
pub fn check_f64(value: f64, field: &str) -> BoundedValidationResult {
    if value.is_nan() {
        Err(Error::InvalidField {
            field: field.into(),
            value: "NaN".into(),
            description: "NaN is not a supported value".into(),
        })
    } else if value.is_infinite() {
        Err(Error::InvalidField {
            field: field.into(),
            value: value.to_string().into(),
            description: "Infinity is not a supported value".into(),
        })
    } else if !value.is_normal() && value != 0.0 {
        Err(Error::InvalidField {
            field: field.into(),
            value: value.to_string().into(),
            description: "sub-normal number is not a supported value".into(),
        })
    } else {
        Ok(())
    }
}

/// Deserialize a JSON `null` value as `Some(None)` instead of `None`.
#[allow(clippy::option_option)]
pub fn deserialize_null_to_some_none<'de, D, T>(d: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Deserialize::deserialize(d).map(Some)
}

/// Serialize a `Some(None)` value as the default value instead of `null`.
#[allow(clippy::option_option)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_some_none_to_invalid<S, T>(
    value: &Option<Option<T>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: InvalidValues + Serialize,
    T::Marker: PartialEq + Serialize,
{
    match value {
        Some(None) => Serialize::serialize(&T::invalid(), s),
        _ => Serialize::serialize(value, s),
    }
}

/// Deserialize an invalid value as `Some(None)` for an Option<Option<ID>> field.
#[allow(clippy::option_option)]
pub fn deserialize_invalid_to_some_none<'de, D, T>(d: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: InvalidValues + Deserialize<'de>,
    T::Marker: PartialEq + Deserialize<'de> + TryInto<T>,
    <T::Marker as TryInto<T>>::Error: Display,
{
    let id: T::Marker = Deserialize::deserialize(d)?;

    if id == T::invalid() {
        Ok(Some(None))
    } else {
        id.try_into().map(|val| Some(Some(val))).map_err(serde::de::Error::custom)
    }
}

/// Deserialize a `HashMap` with keys that are not `String` (but is of a type
/// that implements `FromStr`).
///
/// Serialization is usually not a problem because `serde_json` automatically calls
/// `to_string()` (for key types that implement `Display`) when serializing.
///
pub fn deserialize_hashmap<'de, D, K, T>(d: D) -> Result<HashMap<K, T>, D::Error>
where
    D: Deserializer<'de>,
    K: FromStr + Eq + Hash,
    K::Err: Display,
    T: Deserialize<'de>,
{
    fn deserialize_string_key<'de, D, S>(d: D) -> Result<S, D::Error>
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
