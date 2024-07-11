use crate::config;
use crate::options;

use config::Config;
use options::Options;
use regex::Regex;

/// Runs search for patterns in sources.
pub fn matcher(config: Config) {
    let sources = config.sources;
    let options = config.options;

    let regex = build_regex(&options);

    if sources.is_empty() {
        // read from stdin

    }
    else {
        for source in sources {
            // open file

            // search for matches

            // markup with color

            // annotate with files names for multi-file searches.
        }
    }
}

/// Constructs regular expression from options.
fn build_regex(options: &Options) -> Regex {
    let combined_patterns = options.patterns.join("|");
    let regex_string = format!(r"{}", combined_patterns);

    Regex::new(regex_string.as_str()).expect("Cannot build regex!") // fails if regex can't be built
}

fn search_source(source: String) -> String {

    return String::from("");
}