use serde::{Deserialize, Serialize};

/// A general data structure holding a key and value pair.
///
#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
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
