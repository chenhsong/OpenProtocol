use super::utils::*;
use super::ValidationResult;
use serde::{Deserialize, Serialize};

/// A general data structure holding a key and value pair.
///
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair<K, V> {
    pub(crate) key: K,
    pub(crate) value: V,
}

impl<K: Copy, V> KeyValuePair<K, V> {
    /// Get the key.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let kv = KeyValuePair::new("TheKey", 42.0);
    /// assert_eq!("TheKey", kv.key());
    /// ~~~
    pub fn key(&self) -> K {
        self.key
    }
}

impl<K, V: Copy> KeyValuePair<K, V> {
    /// Get the value.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let kv = KeyValuePair::new("TheKey", 42.0);
    /// assert_eq!(42.0, kv.value());
    /// ~~~
    pub fn value(&self) -> V {
        self.value
    }
}

impl<K, V> KeyValuePair<K, V> {
    /// Get the key.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let kv = KeyValuePair::new("TheKey", 42.0);
    /// assert_eq!("TheKey", *kv.key_ref());
    /// ~~~
    pub fn key_ref(&self) -> &K {
        &self.key
    }

    /// Get the value.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let kv = KeyValuePair::new("TheKey", 42.0);
    /// assert_eq!(42.0, *kv.value_ref());
    /// ~~~
    pub fn value_ref(&self) -> &V {
        &self.value
    }

    /// Create a `KewValuePair`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let kv = KeyValuePair::new("TheKey", 42.0);
    /// assert_eq!("TheKey", kv.key());
    /// assert_eq!(42.0, kv.value());
    /// ~~~
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K: AsRef<str>> KeyValuePair<K, bool> {
    /// Validate a `KeyValuePair` data structure with a string-like key
    /// and boolean value by making sure that the `key` cannot be empty
    /// or all whitespace.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if `key` is set to an empty string
    /// or is all whitespace.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let kv = KeyValuePair::new("    ", true);
    /// assert_eq!(Err(Error::EmptyField("key")), kv.validate());
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    ///
    pub fn validate(&self) -> ValidationResult {
        check_str_empty(&self.key, "key")
    }
}

impl<K: AsRef<str>> KeyValuePair<K, f64> {
    /// Validate a `KeyValuePair` data structure with a string-like key
    /// and `f64` value by making sure that the `key` cannot be empty
    /// or all whitespace, and that the `value` is a valid floating-point
    /// number.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::EmptyField`]`)` if `key` is set to an empty string
    /// or is all whitespace.
    ///
    /// Returns `Err(`[`OpenProtocolError::InvalidField`]`)` if `value` is not a valid floating-point
    /// number.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let kv1 = KeyValuePair::new("     ", 42.0);
    /// assert_eq!(Err(Error::EmptyField("key")), kv1.validate());
    ///
    /// let kv2 = KeyValuePair::new("K2", std::f64::NAN);
    /// assert_eq!(
    ///     Err(Error::InvalidField {
    ///         field: "value",
    ///         value: "NaN".into(),
    ///         description: "NaN is not a supported value".into()
    ///     }),
    ///     kv2.validate()
    /// );
    /// ~~~
    ///
    /// [`OpenProtocolError::EmptyField`]: enum.OpenProtocolError.html#variant.EmptyField
    /// [`OpenProtocolError::InvalidField`]: enum.OpenProtocolError.html#variant.InvalidField
    ///
    pub fn validate(&self) -> ValidationResult {
        check_str_empty(&self.key, "key")?;
        check_f64(self.value, "value")
    }
}
