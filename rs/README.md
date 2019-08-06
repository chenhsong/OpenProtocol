Open Protocol™ Rust Interface Library
====================================

Rust Edition: 2018

Rust crate to interface with the iChen® System 4 using Open Protocol™.

Details on the protocol can be found in
[this document](https://github.com/chenhsong/OpenProtocol/blob/master/cs/doc/messages_reference.md).

Crate
-----

The [`ichen-openprotocol`](https://crates.io/crates/ichen-openprotocol)
crate is available on `crates.io`.

Examples
--------

Sample programs can be found in the `bin` directory under `src`.

Design Notes
------------

Beware that all data types defined in this crate use borrowed string slices
(i.e. `&str`) extensively. This is because the most common usage pattern is to create
a data variable, set fields, immediately serialize it into JSON, then dispose of the
data variable.  The deserialization story is similar.

Error values also borrow heavily from the input fields as these errors are expected
to be handled as soon as possible.

The result is minimal allocations and copying, but at the cost of stricter lifetime
management, especially when deserializing -- the message struct cannot out-live
the original JSON text string as fields are borrowed extensively from the original
JSON string.

Another implication due to extensive usage of borrowed string slices is that strings
literals with escape sequences will cause parsing errors because the actual string
cannot be simply borrowed from the original JSON string.  Luckily this is extremely rare
for most fields holding names, ID's etc. For this reason, only certain user-defined
text fields (such as `job_card_id`) that may contain escaped characters (especially
the double-quote) and therefore are modeled using `Cow<&str>` instead.

How to Use
----------

Import the [`ichen-openprotocol`](https://crates.io/crates/ichen-openprotocol)
crate in `Cargo.toml`, and also a WebSocket client crate (such as `websocket`):

~~~toml
[dependencies]
ichen-openprotocol = "0.4.0"
websocket = "0.23.*"
~~~

Import the namespaces:

~~~rust
use ichen_openprotocol::*;
use websocket::client::ClientBuilder;
use websocket::OwnedMessage;
~~~

Connect to the iChen 4 Server via WebSocket (the default port is 5788):

~~~rust
let client = ClientBuilder::new("1.2.3.4:5788").connect_insecure()?;

// Split WebSocket into sender and receiver
let (mut receiver, mut sender) = client.split()?;
~~~

Create a `JOIN` message with the appropriate password and filters, serialize it into JSON
using `Message::to_json_str()`, then send the JSON string to the WebSocket:

~~~rust
// Create the JOIN message
let join = Message::new_join("mypassword", Filters::All + Filters::JobCards + Filters::Operators)?;

// Serialize the JOIN message with to_json_str()
let json = join.to_json_str();

// Send it over the WebSocket
sender.send(OwnedMessage::Text(json))?;
~~~

Listen to and parse messages in a loop:

~~~rust
for msg in receiver.incoming_messages() {
    match msg.unwrap() {
    OwnedMessage::Text(json) => {
        // Got a JSON message!  Parse it.
        let message = Message::parse_from_json_str(&json)?;

        // Process it...
                :
                :
        }
        // Handle other WebSocket message types, e.g. OwnedMessage::Close
                :
                :
    }
}
~~~

Process received message:

Remember that string fields borrow heavily from the original JSON string, so the correct usage
pattern is to parse the JSON string into a `Message` struct (using `Message::parse_from_json()`),
and then consume the struct immediately, releasing all the borrowed string data.

~~~rust
match message {
    // Response of the `JOIN`
    // Result < 100 indicates failure
    Message::JoinResponse { result, .. } if result < 100 => {
    // Failed to join
            :
            :
    }
    // Response of the `JOIN`
    // Result >= 100 indicates success
    Message::JoinResponse { result, level, .. } => {
    // Success!
            :
            :
    }),
    // Process other messages
            :
            :
}
~~~

Periodically send an `ALIVE` message to keep the connection alive:

~~~rust
let alive = Message::new_alive();
let json = alive.to_json_str();
sender.send(OwnedMessage::Text(json))?;
~~~
