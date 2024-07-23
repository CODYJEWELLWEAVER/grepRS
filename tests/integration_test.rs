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

#[test]
fn ignore_case() {
    use greprs::config::Config;
    use greprs::matcher;
    use regex::{Regex, Matches};

    let args = vec!(String::from("./target"), String::from("t"), String::from("res/test/haiku.txt"), String::from("-i"));
    let mut config = Config::new(args).unwrap();

    config.sources[0].read_data().unwrap();

    let regex: Regex = matcher::build_regex(&config.options).unwrap();

    let matches: Vec<Matches> = matcher::search_lines(
        &regex,
        &config.sources[0].data
    );

    let mut num_matches = 0;
    for mut match_obj in matches {
        while let Some(_) = match_obj.next() {
            num_matches += 1;
        }
    }

    assert_eq!(num_matches, 3);
}