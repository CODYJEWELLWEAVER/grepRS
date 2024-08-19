#![allow(unused_imports)]
#![allow(unused_must_use)]
use super::*;

#[test]
fn default() {
    let default_options = Options::default();
    let expected_options = Options {
        patterns: Vec::new(),
        color_output: true, // rust.yml sets COLORTERM=truecolor, if testing locally make sure to set.
        colors: Colors::get_colors(),
        file_prefix: false,
        ignore_case: false,
        invert_match: false,
        line_match: false,
        word_match: false,
        silent: false,
        no_messages: false,
        count_lines: false,
    };
    assert_eq!(default_options, expected_options);
}

#[test]
fn parse_explicit_patterns() {
    let arg = String::from("--regexp=\"dew\nis\"");
    let expected_patterns = vec!("dew", "is");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.patterns, expected_patterns);

    let arg = String::from("-e=\"dew\nis\"");
    let expected_patterns = vec!("dew", "is");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.patterns, expected_patterns);

    let arg = String::from("-e\"dew\nis\"");
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

    let arg = String::from("-fres/test/patterns.txt");
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

#[test]
fn parse_line_match() {
    let arg = String::from("-x");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.line_match, true);

    let arg = String::from("--line-regexp");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.line_match, true);
}

#[test]
fn parse_word_match() {
    let arg = String::from("-w");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.word_match, true);

    let arg = String::from("--word-regexp");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.word_match, true);
}

#[test]
fn parse_silent() {
    let arg = String::from("-q");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.silent, true);
    assert_eq!(options.no_messages, true);

    let arg = String::from("--quiet");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.silent, true);
    assert_eq!(options.no_messages, true);

    let arg = String::from("--silent");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.silent, true);
    assert_eq!(options.no_messages, true);
}

#[test]
fn parse_no_messages() {
    let arg = String::from("-s");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.no_messages, true);

    let arg = String::from("--no-messages");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.no_messages, true);
}

#[test]
fn parse_count_lines() {
    let arg = String::from("-c");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.count_lines, true);

    let arg = String::from("--count");
    let mut options = Options::default();
    options.parse_option(arg);
    assert_eq!(options.count_lines, true);
}

#[test]
fn parse_color_output() {
    let arg = String::from("--color=always");
    let mut options = Options::default();
    options.color_output = false;
    options.parse_option(arg);
    assert_eq!(options.color_output, true);

    let arg = String::from("--color=never");
    let mut options = Options::default();
    options.color_output = true;
    options.parse_option(arg);
    assert_eq!(options.color_output, false);

    let arg = String::from("--colour=always");
    let mut options = Options::default();
    options.color_output = false;
    options.parse_option(arg);
    assert_eq!(options.color_output, true);

    let arg = String::from("--colour=never");
    let mut options = Options::default();
    options.color_output = true;
    options.parse_option(arg);
    assert_eq!(options.color_output, false);
}