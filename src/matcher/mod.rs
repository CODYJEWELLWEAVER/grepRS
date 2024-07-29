mod test;

use crate::options;

use options::Options;
use regex::Matches;
use regex::Regex;
use std::error::Error;

/// Constructs regular expression from options.
pub fn build_regex(options: &Options) -> Result<Regex, Box<dyn Error>> {
    let pattern = build_pattern_string(options);
    let flags = build_flags(options);

    let regex_string = format!(r"{}{}", flags, pattern);

    let regex = Regex::new(regex_string.as_str())?;

    Ok(regex)
}

/// Searches data of a source line by line, returns matches.
pub fn search_lines<'a>(regex: &'a Regex, data: &'a String) -> Vec<Matches<'a, 'a>> {
    let mut matches: Vec<Matches> = Vec::new();

    let lines = data.split("\n");
    for line in lines {
        matches.push(regex.find_iter(&line));
    }

    matches
}

/// Searches data of source and returns the number of matches found.
pub fn count_matching_lines(regex: &Regex, data: &String, invert_match: bool) -> usize {
    let mut matching_lines: usize = 0;

    let lines = data.split("\n");
    for line in lines {
        let mut match_iter = regex.find_iter(&line);
        let line_has_match = match_iter.next().is_some();

        if line_has_match && !invert_match {
            matching_lines += 1;
        }
        else if !line_has_match && invert_match {
            matching_lines += 1;
        }
    }

    matching_lines
}

/// Combines patterns into a single regex
fn build_pattern_string(options: &Options) -> String {
    let mut patterns = if options.line_match {
        apply_line_matching(&options.patterns)
    }
    else {
        options.patterns.clone()
    };

    patterns = if options.word_match && !options.line_match {
        apply_word_matching(&patterns)
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

/// Maps patterns to new patterns that only match entire lines.
fn apply_line_matching(patterns: &Vec<String>) -> Vec<String> {
    patterns
        .into_iter()
        .map(|pattern| {
            String::from("^(") + &pattern + ")$"
        })
        .collect::<Vec<String>>()
}

/// Maps patterns to patterns that only match whole words.
fn apply_word_matching(patterns: &Vec<String>) -> Vec<String> {
    patterns
        .into_iter()
        .map(|pattern| {
            String::from(r"\b") + &pattern + r"\b"
        })
        .collect::<Vec<String>>()
}