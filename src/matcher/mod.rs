use crate::config;
use crate::options;

use config::Config;
use options::Options;
use regex::Regex;
use std::io::{stdin, Read};
use std::error::Error;
use std::fs::{File};


/// Runs search for patterns in sources.
pub fn search_sources(config: Config) -> Result<(), Box<dyn Error>> {
    let sources = config.sources;
    let options = config.options;

    let regex = build_regex(&options);

    if sources.len() == 0 {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

    }
    else {
        for source_path in sources {
            // open file
            let mut content = String::new();
            let mut source_file = File::open(source_path)?;
            source_file.read_to_string(&mut content)?;

            // search for matches

            // markup with color

            // annotate with files names for multi-file searches.
        }
    }

    Ok(())
}

/// Constructs regular expression from options.
fn build_regex(options: &Options) -> Regex {
    let combined_patterns = options.patterns.join("|");
    let regex_string = format!(r"{}", combined_patterns);

    Regex::new(regex_string.as_str()).expect("Cannot build regex!")
}

fn search_source(source: String) -> String {
    String::from("")
}