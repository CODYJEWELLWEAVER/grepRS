mod test;

use crate::config;
use crate::options;

use config::Config;
use options::Options;
use regex::Matches;
use regex::Regex;
use std::error::Error;

/// Constructs regular expression from options.
pub fn build_regex(config: &Config) -> Result<Regex, Box<dyn Error>> {
    let patterns = config.options.patterns.join("|");
    let flags = build_flags(&config.options);

    let regex_string = format!(r"{}{}", patterns, flags);

    let regex = Regex::new(regex_string.as_str())?;

    Ok(regex)
}

/// Searches content of a source line by line.
pub fn search_lines<'a>(regex: &'a Regex, content: &'a String) -> Vec<Matches<'a, 'a>> {
    let mut matches: Vec<Matches> = Vec::new();

    let lines = content.split("\n");
    for line in lines {
        matches.push(regex.find_iter(&line));
    }

    matches
}

/// Combines flags for regex.
fn build_flags(options: &Options) -> String {
    let mut flags = String::new();

    if options.ignore_case {
        flags.push_str("(?i)");
    }

    flags
}