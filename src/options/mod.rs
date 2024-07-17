mod test;

use crate::source;

use source::Source;
use std::error::Error;
use std::env::var_os;
use std::io::{stderr, Write};

/// #### Options for a run of GrepRS.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Options {
    pub patterns: Vec<String>,
    pub color_output: bool,
    pub file_prefix: bool,
    pub ignore_case: bool,
    pub invert_match: bool,
}

impl Options {
    /// Returns default Options.
    pub fn default() -> Options {
        Options {
            patterns: Vec::new(),
            color_output: Self::supports_color(),
            file_prefix: false,
            ignore_case: false,
            invert_match: false,
        }
    }

    /// Parses option argument and applies option to current options struct.
    pub fn parse_option(&mut self, arg: String) -> Result<(), Box<dyn Error>> {
        let split_arg = Self::split_option(&arg);
        let (option, value) = split_arg;

        if option == "-e" || option == "--regexp" {
            self.handle_pattern(value);
        }
        else if option == "-f" || option == "--file" {
            self.handle_pattern_file(value);
        }
        else if option == "-h" || option == "--no-filename" ||
                option == "-H" || option == "--with-filename" {
            self.handle_prefix(option);
        }
        else if option == "-i" || option == "-y" ||
                option == "--ignore-case" || option == "--no-ignore-case" {
            self.handle_ignore_case(option);
        }
        else if option == "-v" || option == "--invert-match" {
            self.invert_match = true;
        }
        else {
            panic!("Invalid option: {}", option);
        }

        Ok(())
    }

    /// Adds pattern(s) from explicit pattern argument.
    /// **value** is the pattern string
    fn handle_pattern(&mut self, patterns: &str) {
        let mut patterns = patterns;
        if patterns.starts_with("\"") && patterns.ends_with("\"") {
            // remove enclosing double quotes
            patterns = &patterns[1..patterns.len() - 1];
        }

        for pattern in patterns.split("\n") {
            self.patterns.push(String::from(pattern));
        }
    }

    /// Reads patterns from a file, or stdin. Prints message to stderr if
    /// an io error is encountered.
    fn handle_pattern_file(&mut self, path: &str) {
        let mut pattern_source: Source = Source::new(path.to_string());
        if let Err(msg) = pattern_source.read_content(){
            writeln!(
                &mut stderr(),
                "{} {}",
                "grepRS:",
                msg
            ).expect("Could not write to stderr.");
        };

        let patterns = pattern_source.content;

        for pattern in patterns.split("\n") {
            self.patterns.push(String::from(pattern));
        }
    }

    /// Updates self.file_prefix. This function will only ever
    /// be called with one of "-h" "--no-filename", "-H", or
    /// "--with-filename" as **option**.
    fn handle_prefix(&mut self, option: &str) {
        match option {
            "-h" | "--no-filename" => {
                self.file_prefix = false;
            },
            _ => {
                self.file_prefix = true;
            }
        }
    }

    /// Updates ignore case option. **option** is restricted to
    /// "-i", "-y", "--ignore-case", and "--no-ignore-case".
    fn handle_ignore_case(&mut self, option: &str) {
        match option {
            "-i" | "-y" | "--ignore-case" => {
                self.ignore_case = true;
            },
            _ => {
                self.ignore_case = false;
            }
        }
    }

    /// Check if running in a terminal that supports color
    /// ANSI color.
    // TODO: Add support for more environments.
    fn supports_color() -> bool {
        match var_os("COLORTERM") {
            Some(_) => {
                true
            },
            None => {
                false
            }
        }
    }

    /// Attempts to split option argument at "=" and return the option and value.
    /// Otherwise, returns option argument and an empty value string.
    fn split_option<'a>(arg: &'a String) -> (&'a str, &'a str) {
        let split_arg = arg.split_once("=");
        if let Some((option, value)) = split_arg {
            (option, value)
        }
        else {
            (arg.as_str(), "")
        }
    }
}