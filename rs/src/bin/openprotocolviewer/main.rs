use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Write};
use std::iter::FromIterator;
use std::num::NonZeroU32;
use std::sync::mpsc::channel;
use std::thread;

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

use ichen_openprotocol::Message as OP_Message;
use ichen_openprotocol::{Filter, JobCard};

struct Constants<'a> {
    users: HashMap<&'a str, (u8, String)>,
    jobs: Vec<JobCard<'a>>,
}

// Act on Open Protocol message and generate response
fn process_message<'a>(json: &'a str, constants: &'a Constants<'a>) -> Option<OP_Message<'a>> {
    let message;

    // Parse message
    match OP_Message::parse_from_json_str(json) {
        Ok(m) => {
            println!(">>> {:?}", m);
            message = m;
        }
        Err(err) => {
            eprintln!("Error parsing message: {}", err);
            return None;
        }
    }

    match message {
        // Send an ALIVE when received an ALIVE from the server
        OP_Message::Alive { .. } => Some(OP_Message::new_alive()),
        // Response of the JOIN
        OP_Message::JoinResponse { result, .. } => {
            if result < 100 {
                // Result less than 100 indicates failure
                eprintln!("Failed to JOIN: error code = {}", result);
                None
            } else {
                // When the JOIN is successful, send RequestControllersList
                Some(OP_Message::RequestControllersList {
                    controller_id: None,
                    options: Default::default(),
                })
            }
        }
        // MIS integration - User login
        OP_Message::LoginOperator {
            controller_id,
            password,
            ..
        } => {
            match constants.users.get(password) {
                Some((level, name)) => {
                    println!("User found: password={}, access level={}.", password, level);
                    // Return access level
                    Some(OP_Message::OperatorInfo {
                        controller_id: controller_id,
                        operator_id: NonZeroU32::new((*level + 1) as u32),
                        name: Cow::from(name),
                        password: password,
                        level: *level,
                        options: Default::default(),
                    })
                }
                None => {
                    println!("No user found with password: {}.", password);
                    // Return no access
                    Some(OP_Message::OperatorInfo {
                        controller_id: controller_id,
                        operator_id: None,
                        name: Cow::from("Not Allowed"),
                        password: password,
                        level: 0,
                        options: Default::default(),
                    })
                }
            }
        }
        // MIS integration - Load jobs list
        OP_Message::RequestJobCardsList { controller_id, .. } => Some(OP_Message::JobCardsList {
            controller_id: controller_id,
            data: constants
                .jobs
                .iter()
                .map(|jc| (jc.job_card_id.as_ref(), jc.clone()))
                .collect(),
            options: Default::default(),
        }),
        // Other messages - Nothing to process
        _ => None,
    }
}

fn main() {
    println!("iChen 4 Open Protocol Viewer");
    println!();

    // Read URL and password
    print!("WebSocket URL (example: ws://x.x.x.x:port): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let conn = input.trim();
    if conn.is_empty() {
        eprintln!("URL cannot be empty.");
        return;
    }

    print!("Password: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let password = input.trim();

    if password.is_empty() {
        eprintln!("Password cannot be empty.");
        return;
    }

    // Connect to WebSocket server
    println!("Connecting to iChen Server at {}...", conn);

    let mut builder;

    match ClientBuilder::new(conn) {
        Ok(b) => builder = b,
        Err(err) => {
            eprintln!("Invalid URL: {}", err.to_string());
            return;
        }
    }

    let client;

    match builder.connect_insecure() {
        Ok(c) => client = c,
        Err(err) => {
            eprintln!("Connect connet to server: {}", err.to_string());
            return;
        }
    }

    println!("Connection to iChen Server established.");

    let constants = Constants {
        // Mock users database mapping user password --> access level (0-10)
        users: HashMap::from_iter(
            [
                "000000", "111111", "222222", "333333", "444444", "555555", "666666", "777777", "888888", "999999",
                "123456",
            ]
            .iter()
            .enumerate()
            .map(|(index, value)| (*value, (index as u8, format!("MISUser{}", index)))),
        ),
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
    constants
        .users
        .iter()
        .for_each(|(u, (a, n))| println!("> Name={}, Password={}, Level={}", n, u, a));
    println!("=================================================");
    println!("Built-in Job Cards for Testing:");
    constants.jobs.iter().for_each(|j| {
        println!(
            "> Name={}, Mold={}, Quantity={}/{}",
            j.job_card_id, j.mold_id, j.progress, j.total
        )
    });
    println!("=================================================");
    println!("Press ENTER to quit...");

    // Split into channels
    let (mut receiver, mut sender) = client.split().unwrap();

    let (tx, rx) = channel();
    let txx = tx.clone();

    // Receive loop
    let receive_loop = thread::spawn(move || {
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(msg) => msg,
                Err(err) => {
                    // Cennot read from channel
                    println!("Error: {}", err);
                    println!("Closing WebSocket connection...");
                    let _ = txx.send(OwnedMessage::Close(None));
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = txx.send(OwnedMessage::Close(None));
                    println!("Closing WebSocket connection...");
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match txx.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(err) => {
                            println!("Error sending Pong in response to Ping: {}", err);
                            return;
                        }
                    }
                }
                OwnedMessage::Text(json) => {
                    // Output JSON received
                    println!("Received ({}): {}", json.len(), json);

                    // Process the message
                    match process_message(&json, &constants) {
                        // Has a response message...
                        Some(msg) => match msg.to_json_str() {
                            // Serialize it to JSON and send it
                            Ok(resp) => match txx.send(OwnedMessage::Text(resp)) {
                                Ok(_) => println!("<<< {:?}", msg),
                                Err(err) => {
                                    println!("Error sending message: {}", err);
                                    println!("Closing WebSocket connection...");
                                    let _ = txx.send(OwnedMessage::Close(None));
                                    return;
                                }
                            },
                            Err(err) => {
                                println!("Error serializing message: {}", err);
                            }
                        },
                        _ => (),
                    }
                }
                // Output binary data received
                OwnedMessage::Binary(data) => println!("Received binary data: {} byte(s)", data.len()),
                // Everything else
                _ => println!("Received: {:?}", message),
            }
        }
    });

    // Send loop
    let send_loop = thread::spawn(move || {
        // Sleep for 1 sec. before sending anything for the WebSocket connection to stablize
        thread::sleep(std::time::Duration::from_secs(1));

        loop {
            let message = match rx.recv() {
                Ok(msg) => msg,
                Err(err) => {
                    // Cannot read from channel
                    println!("Error: {}", err);
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    println!("Closing WebSocket connection...");
                    return;
                }
                _ => (),
            }
            // Send the message and display it
            match sender.send_message(&message) {
                Ok(()) => match message {
                    OwnedMessage::Text(json) => println!("Sent ({}): {}", json.len(), json),
                    OwnedMessage::Binary(data) => println!("Sent data: {} byte(s)", data.len()),
                    _ => (),
                },
                Err(err) => {
                    println!("Error sending message: {}", err);
                    println!("Closing WebSocket connection...");
                    let _ = sender.send_message(&Message::close());
                    return;
                }
            }
        }
    });

    // Send a JOIN message
    println!("Sending JOIN message...");

    let filters = HashSet::<Filter>::from_iter([Filter::All, Filter::JobCards, Filter::Operators].iter().map(|f| *f));

    let msg = OP_Message::new_join(password, filters);

    match msg.to_json_str() {
        Ok(m) => match tx.send(OwnedMessage::Text(m)) {
            Ok(()) => (),
            Err(err) => eprintln!("Error when sending JOIN message: {}", err),
        },
        Err(err) => eprintln!("Error in JOIN message: {}", err),
    }

    // Wait for ENTER to quit

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // Close the connection
    let _ = tx.send(OwnedMessage::Close(None));

    // Exit

    println!("Waiting for child threads to exit...");

    let _ = send_loop.join();
    let _ = receive_loop.join();

    println!("Program terminated.");
}
