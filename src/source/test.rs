#![allow(unused_imports)]
use super::*;

#[test]
fn load_source_content() {
    let mut source = Source::new(String::from("res/test/haiku.txt"));
    source.read_content().unwrap();
    let expected_content = String::from(
        "This world of dew,\nis a world of dew,\nand yet, and yet."
    );
    assert_eq!(source.content, expected_content);
}