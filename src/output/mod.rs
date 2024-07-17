use crate::source;
use crate::options;

use source::Source;
use options::Options;
use regex::Matches;
use std::iter::zip;

/// Displays results of searching a [Source].
pub fn display(options: &Options, source: &Source, source_matches: Vec<Matches>) {
    let source_lines = source.content.split("\n");
    let line_matches = zip(source_lines, source_matches);

    for (line, matches) in line_matches {
        let mut matches = matches.peekable();

        if matches.peek().is_some() && !options.invert_match {
            display_line(options, source, line);
        }
        else if matches.peek().is_none() && options.invert_match {
            display_line(options, source, line);
        }
    }

    println!();
}

/// Displays line of source.
fn display_line(options: &Options, source: &Source, line: &str) {
    let mut output = String::new();

    if options.file_prefix {
        let prefix = String::from(&source.path) + ":\t";
        output.push_str(prefix.as_str());
    }

    output.push_str(line);

    if !output.ends_with("\n") {
        output.push_str("\n");
    }

    print!("{}", output);
}