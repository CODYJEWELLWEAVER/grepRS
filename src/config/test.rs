use super::*;

#[test]
fn single_pattern_and_file() {
    let args = vec!(String::from("dew"), String::from("../../res/test/haiku.txt"));
    let config = Config::new(args);
    assert_eq!(config.patterns.len(), 1);
    assert_eq!(config.files.len(), 1);
    assert_eq!(config.options.len(), 0);
}