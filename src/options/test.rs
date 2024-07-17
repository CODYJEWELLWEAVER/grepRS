#![allow(unused_imports)]
#![allow(unused_must_use)]
use super::*;

#[test]
fn default() {
    let default_options = Options::default();
    let expected_options = Options {
        patterns: Vec::new(),
        color_output: true, // ignore for testing
        file_prefix: false,
    };
    assert_eq!(default_options, expected_options);
}

#[test]
fn parse_explicit_patterns_shorthand() {
    let arg = String::from("-e=dew");
    let expected_patterns = vec!("dew");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.patterns, expected_patterns);
}

#[test]
fn parse_explicit_patterns() {
    let arg = String::from("--regexp=\"dew\nis\"");
    let expected_patterns = vec!("dew", "is");
    let mut options: Options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.patterns, expected_patterns);
}