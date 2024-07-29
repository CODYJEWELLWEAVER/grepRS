#[test]
fn run_is_ok() {
    let args = vec!(
        String::from("./target"),
        String::from("dew"),
        String::from("res/test/haiku.txt"),
        String::from("-q")
    );
    let result = greprs::run(args);
    assert!(result.is_ok());
}

#[test]
fn config_source_setup() {
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

    let num_matches = matches.len();

    assert_eq!(num_matches, 3);
}

#[test]
fn invert_match() {
    use greprs::config::Config;
    use greprs::matcher;
    use regex::Regex;

    let args = vec!(
        String::from("./target"),
        String::from("[cgCG]"),
        String::from("res/test/poem.txt"),
        String::from("-v"),
    );

    let mut config: Config = Config::new(args).unwrap();

    let regex: Regex = matcher::build_regex(&config.options).unwrap();

    config.sources[0].read_data().unwrap();

    let matches = matcher::search_lines(&regex, &config.sources[0].data);

    let line_matches = config.sources[0].data.split("\n").zip(matches);

    let mut matched_line_nums: Vec<i32> = vec!();

    let mut line_num = 0;
    for (line, matches) in line_matches {
        let mut matches = matches.peekable();

        if matches.peek().is_none() && line != "" {
            matched_line_nums.push(line_num);
        }

        line_num += 1;
    }

    let expected_matched_lines = vec!(6, 27, 31);

    assert_eq!(matched_line_nums, expected_matched_lines);
}

#[test]
fn counting_search() {
    use greprs::config::Config;
    use greprs::matcher;
    use regex::Regex;

    let args = vec!(
        String::from("./target"),
        String::from("[da]"),
        String::from("res/test/haiku.txt"),
        String::from("--count")
    );

    let mut config: Config = Config::new(args).unwrap();

    config.sources[0].read_data().unwrap();

    let regex: Regex = matcher::build_regex(&config.options).unwrap();

    let source_counts = matcher::count_matching_lines(
        &regex,
        &config.sources[0].data,
        config.options.invert_match
    );

    assert_eq!(source_counts, 3);

    let args = vec!(
        String::from("./target"),
        String::from("[da]"),
        String::from("res/test/haiku.txt"),
        String::from("--count"),
        String::from("--invert-match")
    );

    let mut config: Config = Config::new(args).unwrap();

    config.sources[0].read_data().unwrap();

    let regex: Regex = matcher::build_regex(&config.options).unwrap();

    let source_counts = matcher::count_matching_lines(
        &regex,
        &config.sources[0].data,
        config.options.invert_match
    );

    assert_eq!(source_counts, 0);
}