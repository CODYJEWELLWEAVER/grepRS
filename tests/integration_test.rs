#[test]
fn run_is_ok() {
    let args = vec!(String::from("./target"), String::from("dew"), String::from("res/test/haiku.txt"));
    let result = greprs::run(args);
    assert!(result.is_ok());
}

#[test]
fn single_pattern_single_source() {
    use greprs::config::Config;
    use greprs::source::Source;
    use greprs::options::Options;

    let args = vec!(String::from("./target"), String::from("dew"), String::from("res/test/haiku.txt"));
    let mut config = Config::new(args).unwrap();

    let expected_source = Source::new(String::from("res/test/haiku.txt"));
    let mut expected_options = Options::default();
    expected_options.patterns = vec!(String::from("dew"));

    assert_eq!(config, Config { sources: vec!(expected_source), options: expected_options });

    config.sources[0].read_data().unwrap();
    assert_eq!(config.sources[0].data, String::from("This world of dew,\nis a world of dew,\nand yet, and yet."));
}