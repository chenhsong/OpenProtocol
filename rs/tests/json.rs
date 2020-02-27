use ichen_openprotocol::{Filters, Message};
use Message::*;

#[test]
fn integration_test_serialize_to_json() -> Result<(), String> {
    let msg = Message::new_join(
        "hello",
        Filters::Status + Filters::All + Filters::Cycle + Filters::Operators,
    );

    let json = msg.to_json_str()?;

    let check = format!(
        r#"{{"$type":"Join","version":"4.0","password":"hello","language":"EN","filter":"All, Operators","sequence":{}}}"#,
        msg.sequence()
    );

    assert_eq!(check, json);

    Ok(())
}

#[test]
fn integration_test_deserialize_from_json() -> Result<(), String> {
    let msg = Message::parse_from_json_str(
        r#"{"$type":"Join","version":"1.0.0","password":"hello","language":"EN","filter":"Mold, Cycle","sequence":42,"priority":10}"#,
    )?;

    if let Join { version, password, filter, options, .. } = msg {
        assert_eq!("1.0.0", &version);
        assert_eq!("hello", password);
        assert_eq!(42, options.sequence());
        assert_eq!(10, options.priority());
        assert!(filter.has(Filters::Cycle));
        assert!(filter.has(Filters::Mold));
        assert!(!filter.has(Filters::Alarms));
        assert!(!filter.has(Filters::All));
        Ok(())
    } else {
        Err(format!("Wrong type of message deserialized! Expected Join but got {:?}", msg))
    }
}
