use crate::config;
use crate::source::Source;

use config::Config;
use regex::Matches;
use std::iter::zip;

pub fn display(config: &Config, source: &Source, source_matches: Vec<Matches>) {
    let options = &config.options;

    let source_lines = source.content.split("\n");
    let line_matches = zip(source_lines, source_matches);

    for (line, matches) in line_matches {
        let mut matches = matches.peekable();
        if matches.peek().is_some() {
            let mut output = String::new();

            if options.file_prefix {
                let prefix = String::from(&source.path) + ": ";
                output.push_str(prefix.as_str());
            }

            output.push_str(line);

            if !output.ends_with("\n") {
                output.push_str("\n");
            }

            print!("{}", output);
        }
    }
}