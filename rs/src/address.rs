use derive_more::*;
use lazy_static::*;
use regex::Regex;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::net::Ipv4Addr;
use std::num::{NonZeroU16, NonZeroU8};
use std::str::FromStr;

lazy_static! {
    static ref IP_REGEX: Regex =
        Regex::new(r#"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$"#).unwrap();
    static ref TTY_REGEX: Regex = Regex::new(r#"^tty\w+$"#).unwrap();
}

/// A data structure holding a controller's physical address.
///
#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Address<'a> {
    /// Address unknown.
    #[display(fmt = "0.0.0.0:0")]
    Unknown,
    //
    /// An IP v.4 address plus port.
    #[display(fmt = "{}:{}", _0, _1)]
    IPv4(Ipv4Addr, NonZeroU16),
    //
    /// A Windows COM port.
    #[display(fmt = "COM{}", _0)]
    ComPort(NonZeroU8),
    //
    /// A UNIX-style tty serial port device.
    #[display(fmt = "{}", _0)]
    TtyDevice(Cow<'a, str>),
}

impl<'a> TryFrom<&'a str> for Address<'a> {
    type Error = String;

    /// Parse a text string into an `Address`.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if the input string is not recognized as a valid address.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::convert::TryFrom;
    /// // The following should error because port cannot be zero if IP address is not zero
    /// assert_eq!(
    ///     Err("IP port cannot be zero".into()),
    ///     Address::try_from("1.02.003.004:0")
    /// );
    ///
    /// // The following should error because port must be zero if IP address is zero
    /// assert_eq!(
    ///     Err("null IP must have zero port number".into()),
    ///     Address::try_from("0.0.0.0:123")
    /// );
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::convert::TryFrom;
    /// # use std::str::FromStr;
    /// # use std::num::{NonZeroU16, NonZeroU8};
    /// # use std::net::Ipv4Addr;
    /// # fn main() -> std::result::Result<(), String> {
    /// assert_eq!(
    ///     Address::IP(Ipv4Addr::from_str("1.2.3.4").unwrap(), NonZeroU16::new(5).unwrap()),
    ///     Address::try_from("1.02.003.004:05")?
    /// );
    ///
    /// // 0.0.0.0:0 is OK because both IP address and port are zero
    /// assert_eq!(Address::Unknown, Address::try_from("0.0.0.0:0")?);
    ///
    /// assert_eq!(
    ///     Address::ComPort(NonZeroU8::new(123).unwrap()),
    ///     Address::try_from("COM123")?
    /// );
    ///
    /// let addr = Address::try_from("ttyABC")?;
    /// assert_eq!("ttyABC", addr.to_string());
    /// # Ok(())
    /// # }
    /// ~~~
    fn try_from(item: &'a str) -> std::result::Result<Self, Self::Error> {
        const PREFIX_COM: &str = "COM";

        Ok(match item {
            // Match COM port syntax
            text if text.starts_with(PREFIX_COM) => {
                let port = u8::from_str(&text[PREFIX_COM.len()..])
                    .map_err(|_| format!("invalid COM port: {}", text))?;
                let port = NonZeroU8::new(port).ok_or("")?;
                Address::ComPort(port)
            }
            //
            // Match tty syntax
            text if TTY_REGEX.is_match(text) => Address::TtyDevice(text.into()),
            //
            // Match IP:port syntax
            text if IP_REGEX.is_match(text) => {
                // Check IP address validity
                let (address, port) = text.split_at(text.find(':').unwrap());

                let address = Ipv4Addr::from_str(address)
                    .map_err(|_| format!("invalid IP address: {}", address))?;

                // Check port
                match u16::from_str(&port[1..]) {
                    // Allow port 0 on unspecified addresses only
                    Ok(0) => {
                        if !address.is_unspecified() {
                            return Err("IP port cannot be zero".into());
                        } else {
                            Address::Unknown
                        }
                    }
                    // Port must be 0 on unspecified addresses
                    Ok(p) => {
                        if address.is_unspecified() {
                            return Err("null IP must have zero port number".into());
                        } else {
                            Address::IPv4(address, NonZeroU16::new(p).unwrap())
                        }
                    }
                    // Other errors
                    Err(_) => return Err(format!("invalid IP port: {}", port)),
                }
            }
            // Failed to match any address type
            text => return Err(format!("invalid address: {}", text)),
        })
    }
}
