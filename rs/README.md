Open Protocol™ Rust Interface Library
====================================

Rust Edition: 2018

Rust crate to interface with the iChen® System 4 using Open Protocol™.

Details on the protocol can be found in [this document](https://github.com/chenhsong/OpenProtocol/blob/master/cs/doc/messages_reference.md).

Crate
-----

The [`ichen-openprotocol`](https://crates.io/crates/ichen-openprotocol) crate is available on `crates.io`.

Examples
--------

Sample programs can be found in the `bin` directory under `src`.

Notes on Usage
--------------

Beware that all data types defined in this crate use borrowed string slices (i.e. `&str`) extensively.
This is because the most common usage pattern is to create a data variable, set fields, immediately
serialize it into JSON, then dispose of the data variable.  The deserialization story is similar.

Error values also borrow heavily from the input fields as these errors are expected to be handled
as soon as possible.

The result is minimal allocations and copying, but at the cost of stricter lifetime management,
especially when deserializing -- the message struct cannot out-live the original JSON text string as
fields are borrowed extensively from the original JSON string.

Another implication due to extensive usage of borrowed string slices is that strings literals with
escape sequences will cause parsing errors because the actual string cannot be simply borrowed from
the original JSON string.  Luckily this is extremely rare for most fields holding names, ID's etc.
For this reason, only certain user-defined text fields (such as `job_card_id`) that may contain
escaped characters (especially the double-quote) and therefore are modeled using `Cow<&str>` instead.

