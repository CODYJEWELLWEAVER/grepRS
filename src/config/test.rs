#[allow(unused_imports)]
use super::Config;

// TODO: Refactor to integration tests.

#[test]
fn single_pattern_and_file() {
    let args = vec!(String::from("./target"), String::from("dew"), String::from("res/test/haiku.txt"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.sources.len(), 1);
    assert_eq!(config.options.patterns, vec!(String::from("dew")));
}

#[test]
fn explicit_pattern_and_file() {
    let args = vec!(String::from("./target"), String::from("-e"), String::from("dew"), String::from("res/test/haiku.txt"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.sources.len(), 1);
    assert_eq!(config.options.patterns, vec!(String::from("dew")));
}

#[test]
fn explicit_patterns_newline() {
    let args = vec!(String::from("./target"), String::from("-e"), String::from("dew\nis"), String::from("res/test/haiku.txt"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.options.patterns, vec!(String::from("dew"), String::from("is")));
}

#[test]
fn explicit_patterns_quoted_newline() {
    let args = vec!(String::from("./target"), String::from("-e"), String::from("\"dew\nis\""), String::from("res/test/haiku.txt"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.options.patterns, vec!(String::from("dew"), String::from("is")));
}

#[test]
fn patterns_newline_separators_single_file() {
    let args = vec!(String::from("./target"), String::from("dew\nis"), String::from("res/test/haiku.txt"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.sources.len(), 1);
    assert_eq!(config.options.patterns, vec!(String::from("dew"), String::from("is")));
}

#[test]
fn patterns_quoted_newline_separators_single_file() {
    let args = vec!(String::from("./target"), String::from("\"dew\nis\""), String::from("res/test/haiku.txt"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.sources.len(), 1);
    assert_eq!(config.options.patterns, vec!(String::from("dew"), String::from("is")));
}

#[test]
fn single_pattern_no_file() {
    let args = vec!(String::from("./target"), String::from("dew"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.sources.len(), 1);
    assert_eq!(config.options.patterns, vec!(String::from("dew")));
}

#[test]
fn single_pattern_stdin() {
    let args = vec!(String::from("./target"), String::from("dew"), String::from("-"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.sources.len(), 1);
    assert_eq!(config.options.patterns, vec!(String::from("dew")));
}

#[test]
fn single_pattern_no_space() {
    let args = vec!(String::from("./target"), String::from("-edew"));
    let config = Config::new(args).unwrap();
    assert_eq!(config.sources.len(), 1);
    assert_eq!(config.options.patterns, vec!(String::from("dew")));
}