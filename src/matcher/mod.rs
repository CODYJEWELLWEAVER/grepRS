mod test;

use crate::options;

use options::Options;
use regex::Matches;
use regex::Regex;
use std::error::Error;

/// Constructs regular expression from options.
pub fn build_regex(options: &Options) -> Result<Regex, Box<dyn Error>> {
    let patterns = build_pattern(options);
    let flags = build_flags(options);

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

/// Combines patterns into a single regex
fn build_pattern(options: &Options) -> String {
    let mut patterns = if options.line_match {
        let patterns = options.patterns.clone()
            .into_iter()
            .map(|pattern| {
                String::from("^(") + &pattern + ")$"
            });

        patterns.collect::<Vec<String>>()
    }
    else {
        options.patterns.clone()
    };

    patterns = if options.word_match {
        patterns.into_iter()
            .map(|pattern| {
                String::from(r"\b") + &pattern + r"\b"
            })
            .collect::<Vec<String>>()
    }
    else {
        patterns
    };

    patterns.join("|")
}

/// Combines flags for regex.
fn build_flags(options: &Options) -> String {
    let mut flags = String::new();

    if options.ignore_case {
        flags.push_str("(?i)");
    }

    flags
}