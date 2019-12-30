//! Rust access library to read/write iChen® 4 Open Protocol™ messages.
//!
//! Details on the protocol can be found in [this document].
//!
//! Design Notes
//! ============
//!
//! Beware that all data types defined in this crate use borrowed string slices (i.e. `&str`) extensively.
//! This is because the most common usage pattern is to create a data variable, set fields, immediately
//! serialize it into JSON, then dispose of the data variable.  The deserialization story is similar.
//!
//! Error values also borrow heavily from the input fields as these errors are expected to be handled
//! as soon as possible.
//!
//! The result is minimal allocations and copying, but at the cost of stricter lifetime management,
//! especially when deserializing -- the message struct cannot out-live the original JSON text string as
//! fields are borrowed extensively from the original JSON string.
//!
//! Another implication due to extensive usage of borrowed string slices is that string literals with
//! escape sequences will cause parsing errors because the actual string cannot be simply borrowed from
//! the original JSON string.  Luckily this is extremely rare for most fields holding names, ID's etc.
//! For this reason, only certain user-defined text fields (such as `job_card_id`) may contain
//! escaped characters (especially the double-quote); those are therefore modeled using `Cow<&str>` instead.
//!
//! [this document]: https://github.com/chenhsong/OpenProtocol/blob/master/cs/doc/messages_reference.md
//!

#![doc(html_logo_url = "https://chenhsong.github.io/iChen/images/ichen_40_logo_small.png")]
#![doc(html_root_url = "https://docs.rs/ichen-openprotocol")]

// Modules
mod address;
mod controller;
mod error;
mod filters;
mod geo_location;
mod job_card;
mod key_value_pair;
mod messages;
mod operator;
mod state_values;
mod text;
mod types;
mod utils;

/// Result type.
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

type ValidationResult = Result<'static, ()>;
type BoundedValidationResult<'a> = Result<'a, ()>;

/// Result error type.
pub type Error<'a> = OpenProtocolError<'a>;

/// 32-bit real floating-point number.
pub type R32 = noisy_float::types::R32;

// Re-exports
pub use address::Address;
pub use controller::Controller;
pub use error::OpenProtocolError;
pub use filters::Filters;
pub use geo_location::GeoLocation;
pub use job_card::JobCard;
pub use key_value_pair::KeyValuePair;
pub use messages::*;
pub use operator::Operator;
pub use state_values::StateValues;
pub use text::{TextID, TextName};
pub use types::{ActionID, JobMode, Language, OpMode, ID};
