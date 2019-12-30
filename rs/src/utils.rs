use super::{Error, ValidationResult, ID};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryInto;
use std::fmt::Display;
use std::hash::Hash;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::str::FromStr;

/// A trait to specify different _invalid_ values for a type for serialization purposes
///
pub trait HasInvalidValue {
    type Marker;

    /// Returns the standard invalid value for an implementing type
    fn invalid() -> Self::Marker;
}

impl HasInvalidValue for ID {
    type Marker = u32;

    /// `ID` cannot be zero.
    fn invalid() -> Self::Marker {
        0
    }
}

impl HasInvalidValue for NonZeroU32 {
    type Marker = u32;

    /// `NonZeroU32` cannot be zero.
    fn invalid() -> Self::Marker {
        0
    }
}

impl HasInvalidValue for f32 {
    type Marker = f32;

    /// Use NaN as an invalid value for floating-point numbers.
    fn invalid() -> Self::Marker {
        std::f32::NAN
    }
}

impl HasInvalidValue for f64 {
    type Marker = f64;

    /// Use NaN as an invalid value for floating-point numbers.
    fn invalid() -> Self::Marker {
        std::f64::NAN
    }
}

/// Used to suppress serialization numeric fields that are zero (e.g. priority).
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &i32) -> bool {
    *num == 0
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
pub fn check_optional_str_empty<S: Deref>(opt: &Option<S>, field: &'static str) -> ValidationResult
where
    S::Target: AsRef<str>,
{
    match opt {
        Some(text) if text.as_ref().trim().is_empty() => Err(Error::EmptyField(field)),
        _ => Ok(()),
    }
}

/// Check for non-numeric values of an `f32` field.
///
/// # Errors
///
/// Returns `Err(&'static str)` if `value` is not a normal number
/// (e.g. `NaN`, `Infinity`).
///
pub fn check_f32(value: f32) -> std::result::Result<(), &'static str> {
    if value.is_nan() {
        Err("NaN is not a supported value")
    } else if value.is_infinite() {
        Err("Infinity is not a supported value")
    } else if !value.is_normal() && value != 0.0 {
        Err("sub-normal number is not a supported value")
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

/// Serialize a `Some(None)` value as the invalid value instead of `null`.
#[allow(clippy::option_option)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_some_none_to_invalid<S, T>(
    value: &Option<Option<T>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: HasInvalidValue + Serialize,
    T::Marker: PartialEq + Serialize,
{
    match value {
        Some(None) => Serialize::serialize(&T::invalid(), s),
        val => Serialize::serialize(val, s),
    }
}

/// Deserialize an invalid value as `Some(None)` for an Option<Option<ID>> field.
#[allow(clippy::option_option)]
pub fn deserialize_invalid_to_some_none<'de, D, T>(d: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: HasInvalidValue + Deserialize<'de>,
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

/// Deserialize an `IndexMap` with keys that are not `String` (but is of a type
/// that implements `FromStr`).
///
/// Serialization is usually not a problem because `serde_json` automatically calls
/// `to_string()` (for key types that implement `Display`) when serializing.
///
pub fn deserialize_indexmap<'de, D, K, T>(d: D) -> Result<IndexMap<K, T>, D::Error>
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
        S::from_str(s).map_err(|err| serde::de::Error::custom(format!("{}: {}", err, s)))
    }

    #[derive(Deserialize, Hash, Eq, PartialEq)]
    struct Wrapper<S>(#[serde(deserialize_with = "deserialize_string_key")] S)
    where
        S: FromStr,
        S::Err: Display;

    let dict: IndexMap<Wrapper<K>, T> = Deserialize::deserialize(d)?;
    Ok(dict.into_iter().map(|(Wrapper(k), v)| (k, v)).collect())
}
