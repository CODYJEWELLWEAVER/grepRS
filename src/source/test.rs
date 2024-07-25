#![allow(unused_imports)]
use super::*;

#[test]
fn load_source_content() {
    let mut source = Source::new(String::from("res/test/haiku.txt"));
    source.read_data().unwrap();
    let expected_content = String::from(
        "This world of dew,\nis a world of dew,\nand yet, and yet."
    );
    assert_eq!(source.data, expected_content);
}

#[test]
fn invalid_file() {
    let mut source = Source::new(String::from("file"));
    let read_result = source.read_data();
    assert!(read_result.is_err());
}