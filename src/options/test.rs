#![allow(unused_imports)]
#![allow(unused_must_use)]
use super::*;

#[test]
fn default() {
    let default_options = Options::default();
    let expected_options = Options {
        patterns: Vec::new(),
        color_output: true, // ignored for testing purposes
        file_prefix: false,
        ignore_case: false,
        invert_match: false,
    };
    assert_eq!(default_options.patterns, expected_options.patterns);
    assert_eq!(default_options.file_prefix, expected_options.file_prefix);
    assert_eq!(default_options.ignore_case, expected_options.ignore_case);
    assert_eq!(default_options.invert_match, expected_options.invert_match);
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
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.patterns, expected_patterns);
}

#[test]
fn parse_no_filename() {
    let arg = String::from("-h");
    let mut options = Options::default();
    options.file_prefix = true;
    options.parse_option(arg);
    assert_eq!(options.file_prefix, false);

    let arg = String::from("--no-filename");
    let mut options = Options::default();
    options.file_prefix = true;
    options.parse_option(arg);
    assert_eq!(options.file_prefix, false);
}

#[test]
fn parse_with_filename() {
    let arg = String::from("-H");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.file_prefix, true);

    let arg = String::from("--with-filename");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.file_prefix, true);
}

#[test]
fn read_pattern_file() {
    let arg = String::from("-f=res/test/patterns.txt");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.patterns, vec!(String::from("dew"), String::from("s")));

    let arg = String::from("--file=res/test/patterns.txt");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.patterns, vec!(String::from("dew"), String::from("s")));
}

#[test]
fn parse_ignore_case() {
    let arg = String::from("-i");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.ignore_case, true);

    let arg = String::from("-y");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.ignore_case, true);

    let arg = String::from("--ignore-case");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.ignore_case, true);
}

#[test]
fn parse_no_ignore_case() {
    let arg = String::from("--no-ignore-case");
    let mut options = Options::default();
    options.ignore_case = true;
    options.parse_option(arg);
    assert_eq!(options.ignore_case, false);
}

#[test]
fn parse_invert_match() {
    let arg = String::from("-v");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.invert_match, true);

    let arg = String::from("--invert-match");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.invert_match, true);
}