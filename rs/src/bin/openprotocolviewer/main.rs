//! Open Protocol™ Viewer
//! =====================
//!
//! A simple client program that connects to an iChen® System using Open Protocol™.
//!
//! Open Protocol™ messages to and from the server are displayed to the standard output.
//!
//! This program also acts as a user authentication and job cards provider to test out
//! the operator login and job card features.
//!
//! How to Use
//! ----------
//!
//! Build the project. This will automatically build all example programs as well.
//! The program executable will be under the `target/debug` or `target/release` directory.
//!
//! Run the executable (e.g. `openprotocolviewer.exe` on Windows) and enter the following
//! information:
//!
//! **`WebSocket URL`** : URL of the Open Protocol™ interface,
//! usually `ws://MyiChenServerUrl:5788` or `ws://x.x.x.x:5788`
//! (5788 is the default Open Protocol™ interface port).
//! Use `wss://` for secured connection.
//!
//! **`Password`** : A login password to connect to the system.
//! System default is `chenhsong` for the `admin` user with unlimited admin rights
//! (other than MIS/MES rights).
//! To try out the MIS/MES features (e.g. operator login, job cards), first set up a new
//! user account with the appropriate rights, then login with that password.
//! Otherwise, the user authentication and job cards provider will not work.
//!
//! _Warning: If you do not enter a password of a user account that has the appropriate
//! access rights, you'll fail to see all Open Protocol™ messages._

use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::io::{stdin, Write};

// This program uses the `websocket` crate for connection.
use websocket::client::ClientBuilder;
use websocket::{CloseData, OwnedMessage, WebSocketError, WebSocketResult};
type Client = websocket::client::sync::Client<
    std::boxed::Box<dyn websocket::stream::sync::NetworkStream + std::marker::Send>,
>;

// Pull in the `ichen_openprotocol` namespace.
// Beware that `ichen_openprotocol::Message` will conflict with `websocket::Message`
// so you'll need to alias on of them if you pull both into scope.
use ichen_openprotocol::Message;
use ichen_openprotocol::{Filters, JobCard};

struct Constants<'a> {
    users: HashMap<&'a str, (u8, String)>,
    jobs: Vec<JobCard<'a>>,
}

// Format common messages nicely for display
fn display_message(prefix: &str, msg: &Message) {
    print!("{}", prefix);

    match msg {
        Message::Alive { options, .. } => println!("Alive({})", options.sequence),
        Message::RequestControllersList { controller_id: None, options, .. } => {
            println!("RequestControllersList({})", options.sequence)
        }
        Message::RequestControllersList { controller_id: Some(id), options, .. } => {
            println!("RequestControllersList({}, {})", id, options.sequence)
        }
        Message::RequestJobCardsList { controller_id, options, .. } => {
            println!("RequestJobCardsList({}, {})", controller_id, options.sequence)
        }
        Message::RequestMoldData { controller_id, options, .. } => {
            println!("RequestMoldData({}, {})", controller_id, options.sequence)
        }
        Message::ReadMoldData { controller_id, field: None, options, .. } => {
            println!("RequestMoldData({}, ALL, {})", controller_id, options.sequence)
        }
        Message::ReadMoldData { controller_id, field: Some(fld), options, .. } => {
            println!("RequestMoldData({}, [{}], {})", controller_id, fld, options.sequence)
        }
        Message::ControllerAction { controller_id, action_id, options, .. } => {
            println!("ControllerAction({}, [{}], {})", controller_id, action_id, options.sequence)
        }
        m if prefix.is_empty() => println!("{:#?}", m),
        m => println!("\n{:#?}", m),
    }
}

// Parse an Open Protocol message, act on it, and generate a response (if appropriate)
// to send back to the server.
//
fn process_incoming_message<'a>(json: &'a str, builtin: &'a Constants<'a>) -> Option<Message<'a>> {
    // Parse message
    let message = match Message::parse_from_json_str(json) {
        // Valid Open Protocol message.
        Ok(m) => {
            display_message(">>> ", &m);
            m
        }
        // Invalid message for Open Protocol!
        Err(err) => {
            eprintln!("Error parsing message: {}", err);
            return None;
        }
    };

    match message {
        // Send an `ALIVE` when received an `ALIVE` from the server
        Message::Alive { .. } => Some(Message::new_alive()),
        //
        // Response of the `JOIN`
        // Result < 100 indicates failure
        Message::JoinResponse { result, .. } if result < 100 => {
            eprintln!("Failed to JOIN: error code = {}", result);
            None
        }
        // Result >= 100 indicates success
        // When the `JOIN` is successful, send `RequestControllersList`
        Message::JoinResponse { .. } => Some(Message::RequestControllersList {
            controller_id: None,
            options: Default::default(),
        }),
        //
        // MIS/MES integration - User login
        // Find password in built-in list
        Message::LoginOperator { controller_id, password, .. } => match builtin.users.get(password)
        {
            Some((level, name)) => {
                println!("User found: password=[{}], access level={}.", password, level);

                // Return access level
                Some(Message::OperatorInfo {
                    controller_id,
                    operator_id: Some((u32::from(*level) + 1).try_into().unwrap()), // Cheap: Use the access level as the operator's ID
                    name,
                    password,
                    level: *level,
                    options: Default::default(),
                })
            }
            None => {
                println!("No user found with password: [{}].", password);

                // Return no access
                Some(Message::OperatorInfo {
                    controller_id,
                    operator_id: None,
                    name: "Not Allowed",
                    password,
                    level: 0,
                    options: Default::default(),
                })
            }
        },
        //
        // MIS/MES integration - request list of jobs
        Message::RequestJobCardsList { controller_id, .. } => Some(Message::JobCardsList {
            controller_id,
            data: builtin.jobs.iter().map(|jc| (jc.job_card_id.as_ref(), jc.clone())).collect(), // Load jobs list
            options: Default::default(),
        }),
        //
        // Other messages - Nothing to process
        _ => None,
    }
}

fn send(client: &mut Client, message: &OwnedMessage) -> WebSocketResult<()> {
    match client.send_message(message) {
        Ok(_) => match message {
            OwnedMessage::Close(Some(data)) => {
                println!("Closing WebSocket connection: ({}) {}", data.status_code, data.reason)
            }
            OwnedMessage::Close(None) => println!("Closing WebSocket connection..."),
            OwnedMessage::Text(json) => println!("Sent [{}]: {}", json.len(), json),
            OwnedMessage::Binary(data) => println!("Sent data: {} byte(s)", data.len()),
            _ => (),
        },
        // Error when sending message to the WebSocket
        Err(err) => {
            // Log the error, send Close command
            eprintln!("Error sending message: {}", err);
            client.send_message(&websocket::Message::close())?;
            println!("Closing WebSocket connection...");
        }
    }

    Ok(())
}

fn run(mut client: Client, builtin: &Constants<'_>) -> WebSocketResult<()> {
    loop {
        let message = match client.recv_message() {
            Ok(msg) => msg,
            // Error when receiving message from the WebSocket
            Err(err) => {
                // Log the error, send Close command
                eprintln!("Error receiving message: {}", err);
                let data = CloseData::new(1, format!("Error receiving message: {}", err));
                send(&mut client, &OwnedMessage::Close(Some(data)))?;
                // Terminate the receive loop
                return Ok(());
            }
        };

        match message {
            // Close command received
            OwnedMessage::Close(Some(data)) => {
                println!("WebSocket closed: ({}) {}", data.status_code, data.reason);
                // Terminate the receive loop
                return Ok(());
            }
            // Close command received
            OwnedMessage::Close(None) => {
                println!("WebSocket closed.");
                // Terminate the receive loop
                return Ok(());
            }
            // Ping-Pong
            OwnedMessage::Ping(data) => send(&mut client, &OwnedMessage::Pong(data))?,
            // Display received text to screen
            OwnedMessage::Text(json) => {
                println!("Received [{}]: {}", json.len(), json);

                // Process the message, get reply message (if any)
                if let Some(msg) = process_incoming_message(&json, &builtin) {
                    // Serialize reply message to JSON and send it to the send loop
                    match msg.to_json_str() {
                        Ok(resp) => {
                            send(&mut client, &OwnedMessage::Text(resp))?;
                            display_message("<<< ", &msg);
                        }
                        Err(err) => eprintln!("Error serializing message: {}", err),
                    }
                }
            }
            // Display info if binary data received
            OwnedMessage::Binary(data) => println!("Received binary data: {} byte(s)", data.len()),
            // Everything else
            _ => println!("Received: {:#?}", message),
        }
    }
}

fn main() {
    println!("iChen 4 Open Protocol Viewer");
    println!();

    // Read URL and password
    print!("WebSocket URL (example: ws://x.x.x.x:port or wss://x.x.x.x:port): ");
    std::io::stdout().flush().expect("Failed to flush stdout.");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line from stdin.");
    let conn = input.trim();

    if conn.is_empty() {
        eprintln!("URL cannot be empty.");
        return;
    } else if !conn.starts_with("ws://") && !conn.starts_with("wss://") {
        eprintln!(
            "Invalid WebSocket URL format.  Should be: ws://x.x.x.x:port or wss://x.x.x.x:port"
        );
        return;
    }

    print!("Password: ");
    std::io::stdout().flush().expect("Failed to flush stdout.");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line from stdin.");
    let password = input.trim();

    if password.is_empty() {
        eprintln!("Password cannot be empty.");
        return;
    }

    // Build connection to WebSocket server
    println!("Connecting to iChen Server at {}...", conn);

    let mut ws_builder = match ClientBuilder::new(conn) {
        Ok(b) => b,
        Err(err) => {
            eprintln!("Invalid URL: {}", err);
            return;
        }
    };

    // Attempt to connect
    let mut client = match ws_builder.connect(None) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Connect connect to server: {}", &err);
            eprintln!(
                "{}",
                match err {
                    // Errors with text string messages
                    WebSocketError::ProtocolError(e)
                    | WebSocketError::RequestError(e)
                    | WebSocketError::ResponseError(e)
                    | WebSocketError::DataFrameError(e) => e.to_string(),
                    //
                    // Errors with embedded error types
                    WebSocketError::IoError(e) => e.description().to_string(),
                    WebSocketError::HttpError(e) => e.description().to_string(),
                    WebSocketError::UrlError(e) => e.description().to_string(),
                    WebSocketError::TlsError(e) => e.description().to_string(),
                    WebSocketError::Utf8Error(e) => e.description().to_string(),
                    WebSocketError::WebSocketUrlError(e) => e.description().to_string(),
                    //
                    // Errors with status code
                    WebSocketError::StatusCodeError(code) => format!("status code = {}", code),
                    //
                    // Errors with no more information
                    WebSocketError::NoDataAvailable
                    | WebSocketError::TlsHandshakeFailure
                    | WebSocketError::TlsHandshakeInterruption => "".to_string(),
                }
            );
            return;
        }
    };

    println!("Connection to iChen Server established.");

    // Built-in database of users and jobs
    let builtin = Constants {
        // Mock users database mapping user password --> access level (0-10)
        users: [
            "000000", "111111", "222222", "333333", "444444", "555555", "666666", "777777",
            "888888", "999999", "123456",
        ]
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, (index as u8, format!("MISUser{}", index))))
        .collect(),
        //
        // Mock job scheduling system
        jobs: vec![
            JobCard::new("JOB_CARD_1", "ABC-123", 0, 8000),
            JobCard::new("JOB_CARD_2", "M002", 2000, 10000),
            JobCard::new("JOB_CARD_3", "MOULD_003", 888, 3333),
            JobCard::new("JOB_CARD_4", "MOULD_004", 123, 45678),
        ],
    };

    // Display built-in's
    println!("=================================================");
    println!("Built-in Users for Testing:");
    builtin.users.iter().for_each(|(user, (level, name))| {
        println!("> Name={}, Password={}, Level={}", name, user, level)
    });
    println!("=================================================");
    println!("Built-in Job Cards for Testing:");
    builtin.jobs.iter().for_each(|j| {
        println!(
            "> Name={}, Mold={}, Quantity={}/{}",
            j.job_card_id, j.mold_id, j.progress, j.total
        )
    });
    println!("=================================================");

    println!("Sending JOIN message...");

    // Send a `JOIN` message with these filters: `All`, `JobCards` and `Operators`
    //
    // `All` is administrator rights.  You typically do not need such rights to connect to the server.
    // However, since `All` already includes _all_ the machine-related filters, it is sometimes used as
    // an alternate format to specify them all (for lazy people).
    //
    // Filter flags are specified with either `|` (OR operator) or `+` (PLUS operator).
    // Either way is fine as they are equivalent.
    // Using the OR operator is a common style for C-family languages like C, C++, C# and Java.
    // Using the PLUS operator makes the code intention more clear.
    //
    // For example, this filter expression can also be written as:
    //
    //     Filters::Status | Filters::Cycle | Filters::Mold | Filters::Actions | Filters::Alarms |
    //     Filters::Audit | Filters::JobCards | Filters::Operators
    //
    let msg = Message::new_join(password, Filters::All + Filters::JobCards + Filters::Operators);

    match msg.to_json_str() {
        Ok(m) => {
            if let Err(err) = send(&mut client, &OwnedMessage::Text(m)) {
                eprintln!("Error when sending JOIN message: {}", err);
            }
        }
        Err(err) => eprintln!("Error in JOIN message: {}", err),
    }

    // After sending the `JOIN` message, start processing messages...
    println!("Process loop started...");

    match run(client, &builtin) {
        Ok(_) => println!("Process loop stopped."),
        Err(err) => eprintln!("Error in process loop: {}", err),
    }

    // Exit
    println!("Program terminated.");
}
