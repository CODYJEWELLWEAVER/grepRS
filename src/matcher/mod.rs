use crate::config;

use config::Config;
use regex::Matches;
use regex::Regex;
use std::error::Error;

/// Constructs regular expression from options.
pub fn build_regex(config: &Config) -> Result<Regex, Box<dyn Error>> {
    let combined_patterns = config.options.patterns.join("|");
    let regex_string = format!(r"{}", combined_patterns);

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