mod test;

use crate::source;
use crate::options;

use source::Source;
use options::Options;
use regex::Matches;
use std::iter::{zip, Peekable};
use std::io::{stdout, Write};

/// Default output buffer size.
const BUFFER_SIZE: usize = 4096;

/// Contains methods for buffering and writing output.
pub struct OutputBuffer{
    /// Internal buffer for output content.
    buffer: String,
    /// A writable destination for content to be written to.
    destination: Box<dyn Write>,
}

impl OutputBuffer {
    /// Creates new instance of OutputBuffer with default
    /// buffer size and stdout as destination.
    pub fn default() -> OutputBuffer {
        OutputBuffer {
            buffer: String::with_capacity(BUFFER_SIZE),
            destination: Box::from(stdout()),
        }
    }

    /// Writes results of search on a [Source] to the
    /// internal output buffer.
    pub fn append_source_matches(
        &mut self,
        options: &Options,
        source: &Source,
        source_matches: Vec<Matches>
    ) {
        let source_lines = source.data.split("\n");
        let line_matches = zip(source_lines, source_matches);

        for (line, matches) in line_matches {
            let mut matches = matches.peekable();

            // non-inverted matching
            let line = if matches.peek().is_some() && !options.invert_match {
                if options.color_output {
                    Self::apply_match_color(line, &mut matches)
                } else {
                    String::from(line)
                }
            }
            // inverted matching
            else if matches.peek().is_none() && options.invert_match && line != "" {
                String::from(line)
            }
            else {
                String::from("")
            };

            // exit immediately if matching line is found
            // and silent mode is on
            if options.silent && line != "" {
                std::process::exit(0);
            }

            self.append_line(options, &source.path, &line);
        }
    }

    /// Appends results of matching line search to output buffer.
    pub fn append_source_counts(
        &mut self,
        options: &Options,
        source: &Source,
        matching_lines: usize,
    ) {
        let line = format!("{}\n", matching_lines);

        self.append_line(options, &source.path, &line);
    }

    /// Writes newline to buffer to separate source results.
    pub fn append_separator(&mut self) {
        self.buffer.push('\n');

        if self.buffer.len() >= BUFFER_SIZE {
            self.write_and_flush();
        }
    }

    /// Writes buffer to destination and flushes.
    pub fn write_and_flush(&mut self) {
        write!(
            self.destination,
            "{}",
            self.buffer
        ).expect("grepRS: Could not write to destination!");

        self.destination.flush()
            .expect("grepRS: Could not flush output buffer!");

        self.buffer = String::with_capacity(BUFFER_SIZE);
    }

    /// Writes a single line to buffer. If `line` doesn't end
    /// with a newline char a newline will be added to the buffer.
    fn append_line(&mut self, options: &Options, path: &str, line: &str) {
        if options.file_prefix {
            self.append_file_prefix(path, options.color_output)
        }

        self.buffer.push_str(line);

        if !self.buffer.ends_with("\n") {
            self.buffer.push_str("\n");
        }

        if self.buffer.len() >= BUFFER_SIZE {
            self.write_and_flush();
        }
    }

    /// Adds a file prefix to output buffer for current line.
    fn append_file_prefix(&mut self, path: &str, color: bool) {
        let path = if path != "-" {
            path
        } else {
            "(standard input)"
        };

        let mut prefix = String::from(path) + ":\t";

        if color {
            prefix = String::from("\x1b[38;5;3m") + &prefix + "\x1b[39;49m"
        }

        self.buffer.push_str(prefix.as_str());
    }

    /// Applies color to matches inside a line. Only supports default color currently.
    fn apply_match_color(line: &str, matches: &mut Peekable<Matches>) -> String {
        let mut colored_line = String::new();

        let mut previous = 0;
        while let Some(match_obj) = matches.next() {
            let start = match_obj.start();
            let end = match_obj.end();

            colored_line.push_str(&line[previous..start]);
            colored_line.push_str("\x1b[38;5;3m");
            colored_line.push_str(&line[start..end]);
            colored_line.push_str("\x1b[39;49m");

            previous = end;
        }

        if line.len() != previous {
            colored_line.push_str(&line[previous..]);
        }

        colored_line
    }
}
