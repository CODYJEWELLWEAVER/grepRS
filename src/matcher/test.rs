#![allow(unused_imports)]
use super::*;

#[test]
fn ignore_case_flag() {
    let mut options = Options::default();
    options.ignore_case = true;
    let regex_flags = build_flags(&options);
    assert_eq!(regex_flags, String::from("(?i)"));
}

#[test]
fn line_matching_patterns() {
    let mut options = Options::default();
    options.line_match = true;
    let patterns = vec!(String::from("[^z]"), String::from("Hello, World!"));
    let line_patterns = apply_line_matching(&patterns);
    assert_eq!(line_patterns, vec!(String::from("^([^z])$"), String::from("^(Hello, World!)$")));
}

#[test]
fn word_matching_patterns() {
    let mut options = Options::default();
    options.word_match = true;
    let patterns = vec!(String::from("[a-zA-Z]"), String::from("\\d"));
    let word_patterns = apply_word_matching(&patterns);
    assert_eq!(word_patterns, vec!(String::from("\\b[a-zA-Z]\\b"), String::from("\\b\\d\\b")));
}

#[test]
fn build_pattern_string_from_options() {
    let mut options = Options::default();
    options.patterns = vec!(String::from("[xyz]"), String::from("orchestra"));
    let pattern = build_pattern_string(&options);
    assert_eq!(pattern, String::from("[xyz]|orchestra"));

    options.line_match = true;
    let pattern = build_pattern_string(&options);
    assert_eq!(pattern, String::from("^([xyz])$|^(orchestra)$"));
    options.line_match = false;

    options.word_match = true;
    let pattern = build_pattern_string(&options);
    assert_eq!(pattern, String::from("\\b[xyz]\\b|\\borchestra\\b"));
    options.word_match = false;

    // word matching should have no effect
    // when line matching is also applied
    options.line_match = true;
    options.word_match = true;
    let pattern = build_pattern_string(&options);
    assert_eq!(pattern, String::from("^([xyz])$|^(orchestra)$"));
}