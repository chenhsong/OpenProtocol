use super::{Error, ValidationResult};
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

impl Address<'_> {
    /// Create a new `Address::IPv4` from an IP address string and port number.
    ///
    /// The IP address cannot be unspecified (e.g. `0.0.0.0`).
    /// The IP port cannot be zero.
    ///
    /// # Errors
    ///
    /// Returns `Err(&'static str)` if:
    /// * The IP address string is invalid,
    /// * The IP address is unspecified (e.g. `0.0.0.0`),
    /// * The IP port is zero.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert_eq!(Err("invalid IP address"), Address::new_ipv4("hello", 123));
    /// assert_eq!(Err("IP port cannot be zero"), Address::new_ipv4("1.02.003.004", 0));
    /// assert_eq!(Err("invalid null IP address"), Address::new_ipv4("0.00.000.0", 123));
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::str::FromStr;
    /// # use std::net::Ipv4Addr;
    /// # use std::num::NonZeroU16;
    /// # fn main() -> std::result::Result<(), &'static str> {
    /// assert_eq!(
    ///     Address::IPv4(Ipv4Addr::from_str("1.2.3.4").unwrap(), NonZeroU16::new(5).unwrap()),
    ///     Address::new_ipv4("1.02.003.004", 5)?
    /// );
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn new_ipv4(addr: &str, port: u16) -> Result<Self, &'static str> {
        let addr = Ipv4Addr::from_str(addr).map_err(|_| "invalid IP address")?;

        if !addr.is_unspecified() {
            Ok(Self::IPv4(addr, NonZeroU16::new(port).ok_or("IP port cannot be zero")?))
        } else {
            Err("invalid null IP address")
        }
    }
    //
    /// Create a new `Address::ComPort` from a Windows serial port number.
    ///
    /// The COM port number cannot be zero.
    ///
    /// # Errors
    ///
    /// Returns `Err(&'static str)` if the COM port number is zero.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert_eq!(Err("COM port cannot be zero"), Address::new_com_port(0));
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::num::NonZeroU8;
    /// # fn main() -> std::result::Result<(), &'static str> {
    /// assert_eq!(
    ///     Address::ComPort(NonZeroU8::new(5).unwrap()),
    ///     Address::new_com_port(5)?
    /// );
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn new_com_port(port: u8) -> Result<Self, &'static str> {
        Ok(Self::ComPort(NonZeroU8::new(port).ok_or("COM port cannot be zero")?))
    }
    //
    /// Create a new `Address::TtyDevice` from a UNIX-style tty device name.
    ///
    /// The device name should start with `tty`.
    ///
    /// # Errors
    ///
    /// Returns `Err(&'static str)` if the device name is not valid for a tty.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// assert_eq!(Err("invalid tty device"), Address::new_tty_device("hello"));
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), &'static str> {
    /// # use std::borrow::Cow;
    /// assert_eq!(
    ///     Address::TtyDevice(Cow::Borrowed("ttyHello")),
    ///     Address::new_tty_device("ttyHello")?
    /// );
    /// # Ok(())
    /// # }
    /// ~~~
    pub fn new_tty_device(device: &str) -> Result<Self, &'static str> {
        if TTY_REGEX.is_match(device) {
            Ok(Address::TtyDevice(device.to_owned().into()))
        } else {
            Err("invalid tty device")
        }
    }
    /// Validate the data structure.
    ///
    /// # Errors
    ///
    /// Returns `Err(`[`OpenProtocolError::ConstraintViolated`]`)` if the address
    /// is invalid.
    ///
    /// ## Error Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::net::Ipv4Addr;
    /// # use std::num::NonZeroU16;
    /// # use std::str::FromStr;
    /// let address = Address::IPv4(Ipv4Addr::from_str("0.0.0.0").unwrap(), NonZeroU16::new(123).unwrap());
    /// assert_eq!(
    ///     Err(Error::ConstraintViolated("invalid null IP address".into())),
    ///     address.validate()
    /// );
    ///
    /// let address = Address::TtyDevice("hello".into());
    /// assert_eq!(
    ///     Err(Error::ConstraintViolated("invalid tty device: hello".into())),
    ///     address.validate()
    /// );
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # fn main() -> std::result::Result<(), Error<'static>> {
    /// let address = Address::new_ipv4("12.23.34.45", 123).unwrap();
    /// address.validate()?;
    ///
    /// let address = Address::new_com_port(123).unwrap();
    /// address.validate()?;
    ///
    /// let address = Address::new_tty_device("ttyHello").unwrap();
    /// address.validate()?;
    /// # Ok(())
    /// # }
    /// ~~~
    ///
    /// [`OpenProtocolError::ConstraintViolated`]: enum.OpenProtocolError.html#variant.ConstraintViolated
    ///
    pub fn validate(&self) -> ValidationResult {
        match self {
            Self::Unknown => {}
            Self::IPv4(addr, _) => {
                if addr.is_unspecified() {
                    return Err(Error::ConstraintViolated("invalid null IP address".into()));
                }
            }
            Self::ComPort(_) => {}
            Self::TtyDevice(ref device) => {
                if !TTY_REGEX.is_match(device) {
                    return Err(Error::ConstraintViolated(
                        format!("invalid tty device: {}", device).into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for Address<'a> {
    type Error = &'static str;

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
    ///     Err("IP port cannot be zero"),
    ///     Address::try_from("1.02.003.004:0")
    /// );
    ///
    /// // The following should error because port must be zero if IP address is zero
    /// assert_eq!(
    ///     Err("null IP must have zero port number"),
    ///     Address::try_from("0.0.0.0:123")
    /// );
    /// ~~~
    ///
    /// # Examples
    ///
    /// ~~~
    /// # use ichen_openprotocol::*;
    /// # use std::convert::TryFrom;
    /// # use std::borrow::Cow;
    /// # use std::str::FromStr;
    /// # use std::num::{NonZeroU16, NonZeroU8};
    /// # use std::net::Ipv4Addr;
    /// # fn main() -> std::result::Result<(), &'static str> {
    /// assert_eq!(
    ///     Address::IPv4(Ipv4Addr::from_str("1.2.3.4").unwrap(), NonZeroU16::new(5).unwrap()),
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
    /// assert_eq!(
    ///     Address::TtyDevice(Cow::Borrowed("ttyABC")),
    ///     Address::try_from("ttyABC")?
    /// );
    /// # Ok(())
    /// # }
    /// ~~~
    fn try_from(item: &'a str) -> std::result::Result<Self, Self::Error> {
        const PREFIX_COM: &str = "COM";

        Ok(match item {
            // Match COM port syntax
            text if text.starts_with(PREFIX_COM) => {
                let port =
                    u8::from_str(&text[PREFIX_COM.len()..]).map_err(|_| "invalid COM port")?;
                Address::new_com_port(port)?
            }
            //
            // Match tty syntax
            text if TTY_REGEX.is_match(text) => Address::new_tty_device(text)?,
            //
            // Match IP:port syntax
            text if IP_REGEX.is_match(text) => {
                // Check IP address validity
                let (address, port) = text.split_at(text.find(':').unwrap());

                let address = Ipv4Addr::from_str(address).map_err(|_| "invalid IP address")?;

                // Check port
                match u16::from_str(&port[1..]) {
                    // Allow port 0 on unspecified addresses only
                    Ok(0) => {
                        if !address.is_unspecified() {
                            return Err("IP port cannot be zero");
                        } else {
                            Address::Unknown
                        }
                    }
                    // Port must be 0 on unspecified addresses
                    Ok(p) => {
                        if address.is_unspecified() {
                            return Err("null IP must have zero port number");
                        } else {
                            Address::IPv4(address, NonZeroU16::new(p).unwrap())
                        }
                    }
                    // Other errors
                    Err(_) => return Err("invalid IP port"),
                }
            }
            // Failed to match any address type
            _ => return Err("invalid address"),
        })
    }
}
