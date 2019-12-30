use derive_more::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// A text string ID that cannot be empty or all-whitespace, and must be all-ASCII.
///
/// This type is usually used for specifying a unique ID.
///
/// It `Deref`s to `&str`.
///
pub type TextID<'a> = ConstrainedText<&'a str, NonEmptyAllASCII>;

/// A `Cow<str>` for a name that cannot be empty or all-whitespace.
///
/// It `Deref`s to `&str`.
///
pub type TextName<'a> = ConstrainedText<Cow<'a, str>, NonEmpty>;

/// A trait that constrains the format of a text string.
pub trait TextConstraint {
    /// Create a new instance of the text constraint.
    fn new() -> Self;

    /// Check if a text string is valid under the text constraint.
    fn check(text: &str) -> bool;

    /// Description of valid text strings.
    fn required() -> &'static str;
}

/// A text constraint that rejects empty strings and strings containing only whitespaces.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NonEmpty;

impl TextConstraint for NonEmpty {
    fn new() -> Self {
        Self
    }
    fn check(text: &str) -> bool {
        !text.trim().is_empty()
    }
    fn required() -> &'static str {
        "a non-empty, non-whitespace string"
    }
}

/// A text constraint that rejects empty strings and strings containing only whitespaces.
/// Only ASCII characters can be in the text string.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NonEmptyAllASCII;

impl TextConstraint for NonEmptyAllASCII {
    fn new() -> Self {
        Self
    }
    fn check(text: &str) -> bool {
        !text.trim().is_empty() && text.chars().all(|c| char::is_ascii(&c))
    }
    fn required() -> &'static str {
        "a non-empty, non-whitespace, all-ASCII string"
    }
}

/// A data structure that wraps a text string (or anything that dereferences into a text string)
/// while guaranteeing that the specified text constraint is upheld.
#[derive(Display, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[display(fmt = "_0")]
pub struct ConstrainedText<T: AsRef<str>, C: TextConstraint>(T, C);

impl<T: AsRef<str>, C: TextConstraint> Debug for ConstrainedText<T, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0.as_ref(), f)
    }
}

impl<T: AsRef<str>, C: TextConstraint> ConstrainedText<T, C> {
    /// Create a new `ConstrainedText` from a text string and a constraint.
    ///
    /// # Errors
    ///
    /// Returns `None` if `text` violates the text constraint.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::str::FromStr;
    /// assert_eq!(None, TextID::new("     "));
    /// assert_eq!(None, TextID::new(""));
    /// assert_eq!(None, TextID::new("你好吗？"));
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let id = TextName::new_from_str("你好吗？").unwrap();
    /// assert_eq!("你好吗？", id);
    /// ~~~
    pub fn new(text: T) -> Option<Self> {
        if !C::check(text.as_ref()) {
            None
        } else {
            Some(ConstrainedText(text, C::new()))
        }
    }

    /// Convert a `ConstrainedText` into a string.
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// let id = TextID::new("hello").unwrap();
    /// assert_eq!("hello", id);
    /// ~~~
    pub fn get(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: AsRef<str>, C: TextConstraint> Deref for ConstrainedText<T, C> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T: AsRef<str>, C: TextConstraint> PartialEq<&str> for ConstrainedText<T, C> {
    fn eq(&self, other: &&str) -> bool {
        self.get() == *other
    }
}

impl<T: AsRef<str>, C: TextConstraint> PartialEq<ConstrainedText<T, C>> for &str {
    fn eq(&self, other: &ConstrainedText<T, C>) -> bool {
        *self == other.get()
    }
}

impl<T: AsRef<str>, C: TextConstraint> PartialOrd<&str> for ConstrainedText<T, C> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.get().partial_cmp(*other)
    }
}

impl<T: AsRef<str>, C: TextConstraint> PartialOrd<ConstrainedText<T, C>> for &str {
    fn partial_cmp(&self, other: &ConstrainedText<T, C>) -> Option<Ordering> {
        (*self).partial_cmp(other.get())
    }
}

impl<T: AsRef<str>, C: TextConstraint> Serialize for ConstrainedText<T, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(self.0.as_ref(), serializer)
    }
}

impl<'a, 'de: 'a, T, C> Deserialize<'de> for ConstrainedText<T, C>
where
    T: AsRef<str> + From<&'a str>,
    C: TextConstraint,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string_val: &str = Deserialize::deserialize(deserializer)?;
        let value = string_val.into();

        ConstrainedText::new(value).ok_or_else(|| {
            serde::de::Error::custom(format!("expected {}, got [{}]", C::required(), string_val))
        })
    }
}

impl<'a> TextName<'a> {
    pub fn new_from_str<T: Into<Cow<'a, str>>>(text: T) -> Option<Self> {
        Self::new(text.into())
    }
}
