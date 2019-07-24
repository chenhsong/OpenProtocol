use ichen_openprotocol::*;
use Message::*;

#[test]
fn test_serialize() {
    let mut m = Message::new_join("hello", &[Filter::All, Filter::Cycle, Filter::Operators]);
    if let Join { options, .. } = &mut m {
        options.sequence = 999;
    }

    let json = m.to_json_str().unwrap();
    assert_eq!(
        r#"{"$type":"Join","version":"4.0","password":"hello","language":"EN","filter":"All, Operators","sequence":999}"#,
        json
    );
}

#[test]
fn test_deserialize() {
    let m = Message::parse_from_json_str(
        r#"{"$type":"Join","version":"1.0.0","password":"hello","filters":"Cycle, Mold","sequence":42,"priority":10}"#,
    )
    .unwrap();

    if let Join { version, password, filter, options, .. } = m {
        assert_eq!("1.0.0", version);
        assert_eq!("hello", password);
        assert_eq!(42, options.sequence);
        assert_eq!(10, options.priority);
        assert_eq!(2, filter.len());
        assert!(filter.contains(&Filter::Cycle));
        assert!(filter.contains(&Filter::Mold));
        assert!(!filter.contains(&Filter::Alarms));
        assert!(!filter.contains(&Filter::All));
        assert!(!filter.is_all());
    } else {
        panic!("Wrong type of message deserialized!");
    }
}
