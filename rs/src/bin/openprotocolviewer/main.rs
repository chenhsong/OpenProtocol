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
//! `wss:` access to secured WebSocket ports with HTTPS is _not_ supported in this sample.
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
use std::error::Error;
use std::io::{stdin, Write};
use std::num::NonZeroU32;
use std::sync::mpsc::channel;
use std::thread;

// This program uses the `websocket` crate for connection.
use websocket::client::ClientBuilder;
use websocket::{CloseData, OwnedMessage, WebSocketError};

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
        m => {
            if prefix.is_empty() {
                println!("{:#?}", m);
            } else {
                println!("\n{:#?}", m)
            }
        }
    }
}

// Act on Open Protocol message and generate response
//
// This function takes a JSON string as input, parse it into an Open Protocol message,
// acts on it, and returns a reply message (if any) to send to the server.
//
fn process_message<'a>(json: &'a str, builtin: &'a Constants<'a>) -> Option<Message<'a>> {
    let message;

    // Parse message
    match Message::parse_from_json_str(json) {
        Ok(m) => {
            display_message(">>> ", &m);
            message = m;
        }
        Err(err) => {
            eprintln!("Error parsing message: {}", err);
            return None;
        }
    }

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
        Message::LoginOperator { controller_id, password, .. } => {
            // Find password in built-in list
            if let Some((level, name)) = builtin.users.get(password) {
                println!("User found: password=[{}], access level={}.", password, level);

                // Return access level
                Some(Message::OperatorInfo {
                    controller_id,
                    operator_id: NonZeroU32::new(u32::from(*level + 1)),
                    name,
                    password,
                    level: *level,
                    options: Default::default(),
                })
            } else {
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
        }
        //
        // MIS/MES integration - request list of jobs
        Message::RequestJobCardsList { controller_id, .. } => Some(Message::JobCardsList {
            controller_id,
            // Load jobs list
            data: builtin.jobs.iter().map(|jc| (jc.job_card_id.as_ref(), jc.clone())).collect(),
            options: Default::default(),
        }),
        //
        // Other messages - Nothing to process
        _ => None,
    }
}

fn main() {
    println!("iChen 4 Open Protocol Viewer");
    println!();

    // Read URL and password
    print!("WebSocket URL (example: ws://x.x.x.x:port): ");
    std::io::stdout().flush().expect("Failed to flush stdout.");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line from stdin.");
    let conn = input.trim();

    if conn.is_empty() {
        eprintln!("URL cannot be empty.");
        return;
    } else if conn.starts_with("wss://") {
        eprintln!("This program is intended as a simple example for illustration purposes only.");
        eprintln!("Due to added complexity, the wss: protocol is not supported by this program.");
        return;
    } else if !conn.starts_with("ws://") {
        eprintln!("Invalid WebSocket URL format.  Should be: ws://x.x.x.x:port");
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

    let mut ws_builder;

    match ClientBuilder::new(conn) {
        Ok(b) => ws_builder = b,
        Err(err) => {
            eprintln!("Invalid URL: {}", err);
            return;
        }
    }

    let client;

    // Attempt to connect
    match ws_builder.connect_insecure() {
        Ok(c) => client = c,
        Err(err) => {
            eprintln!("Connect connect to server: {}", &err);
            eprintln!(
                "{}",
                match &err {
                    // Errors with text string messages
                    WebSocketError::ProtocolError(e)
                    | WebSocketError::RequestError(e)
                    | WebSocketError::ResponseError(e)
                    | WebSocketError::DataFrameError(e) => e,
                    //
                    // Errors with embedded error types
                    WebSocketError::IoError(e) => e.description(),
                    WebSocketError::HttpError(e) => e.description(),
                    WebSocketError::UrlError(e) => e.description(),
                    WebSocketError::TlsError(e) => e.description(),
                    WebSocketError::Utf8Error(e) => e.description(),
                    WebSocketError::WebSocketUrlError(e) => e.description(),
                    //
                    // Errors with no more information
                    WebSocketError::StatusCodeError(_)
                    | WebSocketError::NoDataAvailable
                    | WebSocketError::TlsHandshakeFailure
                    | WebSocketError::TlsHandshakeInterruption => return,
                }
            );
            return;
        }
    }

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
        .map(|(index, value)| (*value, (index as u8, format!("MISUser{}", index))))
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
    builtin
        .users
        .iter()
        .for_each(|(u, (a, n))| println!("> Name={}, Password={}, Level={}", n, u, a));
    println!("=================================================");
    println!("Built-in Job Cards for Testing:");
    builtin.jobs.iter().for_each(|j| {
        println!(
            "> Name={}, Mold={}, Quantity={}/{}",
            j.job_card_id, j.mold_id, j.progress, j.total
        )
    });
    println!("=================================================");
    println!("Press ENTER to quit...");

    // Split WebSocket into sender and receiver
    let (mut receiver, mut sender) = client.split().expect("Failed to split WebSocket channel.");

    // Create a channel for communications between the send and receive loops
    let (tx, rx) = channel();
    let txx = tx.clone();

    // Receive loop
    let receive_loop = thread::spawn(move || {
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(msg) => msg,
                Err(err) => {
                    // Error when receiving message from the WebSocket
                    // Log the error, send Close command and terminate the receive loop
                    eprintln!("Error receiving message: {}", err);
                    txx.send(OwnedMessage::Close(Some(CloseData::new(
                        1,
                        format!("Error receiving message: {}", err),
                    ))))
                    .unwrap();
                    return;
                }
            };

            match message {
                OwnedMessage::Close(data) => {
                    // Got a Close command, so send a Close command back and terminate the receive loop
                    if let Some(d) = data {
                        println!("WebSocket closed: ({}) {}", d.status_code, d.reason);
                    } else {
                        println!("WebSocket closed.");
                    }
                    return;
                }
                OwnedMessage::Ping(data) => txx.send(OwnedMessage::Pong(data)).unwrap(),
                OwnedMessage::Text(json) => {
                    // Display received text to screen
                    println!("Received ({}): {}", json.len(), json);

                    // Process the message, get reply message (if any)
                    if let Some(msg) = process_message(&json, &builtin) {
                        match msg.to_json_str() {
                            // Serialize reply message to JSON and send it to the send loop
                            Ok(resp) => {
                                txx.send(OwnedMessage::Text(resp)).unwrap();
                                display_message("<<< ", &msg);
                            }
                            Err(err) => eprintln!("Error serializing message: {}", err),
                        }
                    }
                }
                OwnedMessage::Binary(data) => {
                    // Display info if binary data received
                    println!("Received binary data: {} byte(s)", data.len())
                }
                // Everything else
                _ => println!("Received: {:#?}", message),
            }
        }
    });

    // Send loop
    let send_loop = thread::spawn(move || {
        // Sleep for 1 sec. before sending anything for the WebSocket connection to stabilize
        thread::sleep(std::time::Duration::from_secs(1));

        for message in rx {
            // Send the message and display it to screen
            match sender.send_message(&message) {
                Ok(()) => match message {
                    OwnedMessage::Close(data) => {
                        // If it's a Close command, just send it and then terminate the send loop
                        if let Some(d) = data {
                            println!(
                                "Closing WebSocket connection: ({}) {}",
                                d.status_code, d.reason
                            );
                        } else {
                            println!("Closing WebSocket connection...");
                        }
                        return;
                    }
                    OwnedMessage::Text(json) => println!("Sent ({}): {}", json.len(), json),
                    OwnedMessage::Binary(data) => println!("Sent data: {} byte(s)", data.len()),
                    _ => (),
                },
                Err(err) => {
                    // Error when sending message to the WebSocket
                    // Log the error, send Close command and terminate the send loop
                    eprintln!("Error sending message: {}", err);
                    sender.send_message(&websocket::Message::close()).unwrap();
                    println!("Closing WebSocket connection...");
                    return;
                }
            }
        }
    });

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
        Ok(m) => match tx.send(OwnedMessage::Text(m)) {
            Ok(()) => (),
            Err(err) => eprintln!("Error when sending JOIN message: {}", err),
        },
        Err(err) => eprintln!("Error in JOIN message: {}", err),
    }

    // After sending the `JOIN` message, everything else should occur automatically in the background.
    // So just wait for an ENTER key to quit...
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line from stdin.");

    // Close the connection
    tx.send(OwnedMessage::Close(Some(<CloseData>::new(0, "Program termination.".to_string()))))
        .expect("Cannot send to channel!");

    // Exit
    println!("Waiting for child threads to exit...");

    let _ = send_loop.join();
    let _ = receive_loop.join();

    println!("Program terminated.");
}
