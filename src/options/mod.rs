mod test;

use crate::source;
use crate::colors;

use source::Source;
use std::error::Error;
use std::env::var_os;
use std::io::{stderr, Write};
use colors::Colors;

/// #### Options for a run of GrepRS.
///
/// See [parse_option](Options::parse_option) for logic used to parse options
/// from command line arguments.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Options {
    /// Patterns to find matches for.
    pub patterns: Vec<String>,
    /// If output should use color. Defaults to `true` if the current environment indicates
    /// color output support.
    pub color_output: bool,
    /// Contains the colors that should be used for highlighting different aspects of output.
    /// See the colors module for more information.
    pub colors: Colors,
    /// If output should be prefixed with the path name.
    /// Uses `(standard input)` as a prefix for stdin.
    /// Defaults to `true` for multiple source executions, otherwise `false`.
    pub file_prefix: bool,
    /// If pattern matching should be case sensitive. Defaults to `true`.
    pub ignore_case: bool,
    /// If matching logic should be inverted. i.e. non-matching lines will match. Defaults to `false`.
    pub invert_match: bool,
    /// If `true`, patterns can only match with an entire line. Defaults to `false`.
    pub line_match: bool,
    /// If `true`, a pattern can only match areas surrounded by non-word bytes. Defaults to `false`.
    pub word_match: bool,
    /// If `true`, don't write any output to stdout. Defaults to `false`.
    pub silent: bool,
    /// If `true`, don't write any error messages. Defaults to `false`.
    pub no_messages: bool,
    /// If `true`, counts lines with matches, respects invert_match. Defaults to `false`.
    pub count_lines: bool,
}

impl Options {
    /// Returns default Options.
    /// ```text
    /// Options {
    ///     patterns: Vec::new(),
    ///     color_output: Self::supports_color(),
    ///     colors: Colors::get_colors(),
    ///     file_prefix: false,
    ///     ignore_case: false,
    ///     invert_match: false,
    ///     line_match: false,
    ///     word_match: false,
    ///     silent: false,
    ///     no_messages: false,
    ///     count_lines: false,
    /// }
    /// ```
    pub fn default() -> Options {
        Options {
            patterns: Vec::new(),
            color_output: Self::supports_color(),
            colors: Colors::get_colors(),
            file_prefix: false,
            ignore_case: false,
            invert_match: false,
            line_match: false,
            word_match: false,
            silent: false,
            no_messages: false,
            count_lines: false,
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
        else if option == "-x" || option == "--line-regexp" {
            self.line_match = true;
        }
        else if option == "-w" || option == "--word-regexp" {
            self.word_match = true;
        }
        else if option == "-q" || option == "--quiet" || option == "--silent" {
            self.silent = true;
            // set no error messages if silent option is passed
            self.no_messages = true;
        }
        else if option == "-s" || option == "--no-messages" {
            self.no_messages = true;
        }
        else if option == "-c" || option == "--count" {
            self.count_lines = true;
        }
        else if option == "--color" || option == "--colour" {
            self.handle_color(value);
        }
        else {
            panic!("Invalid option: {}", option);
        }

        Ok(())
    }

    /// Adds pattern(s) from a pattern argument. Each string separated by a
    /// newline is considered as a unique pattern.
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

    /// Reads pattern(s) from a file, or stdin. Prints message to stderr if
    /// an io error is encountered.
    fn handle_pattern_file(&mut self, path: &str) {
        let mut pattern_source: Source = Source::new(path.to_string());
        if let Err(msg) = pattern_source.read_data(){
            writeln!(
                &mut stderr(),
                "{} {}",
                "grepRS:",
                msg
            ).expect("Could not write to stderr.");
        };

        let patterns = pattern_source.data;

        for pattern in patterns.split("\n") {
            self.patterns.push(String::from(pattern));
        }
    }

    /// Sets `file_prefix`. This function has no effect if not
    /// called with one of `-h` `--no-filename`, `-H`, or
    /// `--with-filename` as `option`.
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

    /// Updates `ignore_case` option. Has no effect when `option` isn't one of
    /// `-i`, `-y`, `--ignore-case`, and `--no-ignore-case`.
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

    fn handle_color(&mut self, value: &str) {
        match value {
            "always" => self.color_output = true,
            "never" => self.color_output = false,
            _ => {},
        }
    }

    /// Check if running in a environment that supports color output.
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

    /// Attempts to split an option argument that is associated with a
    /// and return the option and value. Otherwise, returns option
    /// argument and an empty value string.
    fn split_option<'a>(arg: &'a String) -> (&'a str, &'a str) {
        let split_arg: Option<(&str, &str)> = arg.split_once("=");
        if let Some((option, value)) = split_arg {
            // checks for "=" delimited values
            (option, value)
        }
        else if !arg.starts_with("--") && arg.len() > 2 {
            (&arg[0..2], &arg[2..])
        }
        else {
            (arg.as_str(), "")
        }
    }
}