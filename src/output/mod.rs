use crate::source;
use crate::options;

use source::Source;
use options::Options;
use regex::Matches;
use std::iter::zip;
use std::io::{stdout, Write};

/// Default output buffer size.
const BUFFER_SIZE: usize = 4096;

/// Contains methods for buffering and writing output.
pub struct OutputBuffer {
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
    pub fn write_to_buffer(
        &mut self,
        options: &Options,
        source: &Source,
        source_matches: Vec<Matches>
    ) {
        let source_lines = source.data.split("\n");
        let line_matches = zip(source_lines, source_matches);

        for (line, matches) in line_matches {
            let mut matches = matches.peekable();

            if matches.peek().is_some() && !options.invert_match {
                self.write_line(options, source, line);
            }
            else if matches.peek().is_none() && options.invert_match {
                self.write_line(options, source, line);
            }
        }

        if self.buffer.len() >= BUFFER_SIZE {
            self.write_and_flush();
        }
    }

    /// Writes newline to buffer to separate source results.
    pub fn write_separator(&mut self) {
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
        ).expect("grepRS: Could not write to stdout!");

        self.destination.flush()
            .expect("grepRS: Could not flush buffer!");

        self.buffer = String::with_capacity(BUFFER_SIZE);
    }

    /// Writes a single line to buffer. If `line` doesn't end
    /// with a newline char a newline will be added to the buffer.
    fn write_line(&mut self, options: &Options, source: &Source, line: &str) {
        if options.file_prefix {
            let path = if &source.path != "-" {
                &source.path
            } else {
                "(standard input)"
            };

            let prefix = String::from(path) + ":\t";
            self.buffer.push_str(prefix.as_str());
        }

        self.buffer.push_str(line);

        if !self.buffer.ends_with("\n") {
            self.buffer.push_str("\n");
        }
    }
}