#![allow(unused_imports)]
use regex::Regex;

use crate::matcher;

use super::*;

/// To prevent output to standard output

#[test]
fn default_buffer_capacity() {
    let default_output_buffer = OutputBuffer::default();
    assert_eq!(default_output_buffer.buffer.capacity(), BUFFER_SIZE);
}

#[test]
fn append_file_prefix_to_buffer() {
    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_file_path("file path", false, &Colors::default());
    assert_eq!(output_buffer.buffer, "file path");

    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_file_path("-", false, &Colors::default());
    assert_eq!(output_buffer.buffer, "(standard input)");

    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_file_path("file path", true, &Colors::default());
    assert_eq!(output_buffer.buffer, "\x1b[32mfile path\x1b[0m");

    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_file_path("-", true, &Colors::default());
    assert_eq!(output_buffer.buffer, "\x1b[32m(standard input)\x1b[0m");
}

#[test]
fn append_line_to_buffer() {
    let line = "output line\n";
    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_line(&Options::default(), "-", line);
    assert_eq!(output_buffer.buffer, "output line\n");

    let line = "output line";
    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_line(&Options::default(), "-", line);
    assert_eq!(output_buffer.buffer, "output line\n");

    let line = "output line\n";
    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };
    let mut options = Options::default();
    options.color_output = true;
    options.file_prefix = true;

    output_buffer.append_line(&options, "-", line);
    assert_eq!(output_buffer.buffer, "\u{1b}[32m(standard input)\u{1b}[0m\u{1b}[35m:\t\u{1b}[0moutput line\n");

    let line = "output line\n";
    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };
    let mut options = Options::default();
    options.color_output = false;
    options.file_prefix = true;

    output_buffer.append_line(&options, "-", line);
    assert_eq!(output_buffer.buffer, "(standard input):\toutput line\n");
}

#[test]
fn flush_buffer() {
    let line = "output_line\n";
    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_line(&Options::default(), "-", line);

    output_buffer.write_and_flush();
    assert_eq!(output_buffer.buffer, "");
}

#[test]
fn appends_line_count_to_buffer() {
    let matching_lines: usize = 10;
    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    let test_source = Source {
        path: String::new(),
        data: String::new(),
    };

    output_buffer.append_source_counts(&Options::default(), &test_source, matching_lines);

    assert_eq!(output_buffer.buffer, "10\n");
}

#[test]
fn appends_source_matches_to_buffer() {
    let mut source = Source::new(String::from("res/test/haiku.txt"));
    let mut options = Options::default();
    options.color_output = true;
    options.patterns = vec!(String::from("dew"));

    let regex: Regex = matcher::build_regex(&options).unwrap();

    source.read_data().unwrap();

    let source_matches = matcher::search_lines(&regex, &source.data);

    let mut output_buffer = OutputBuffer {
        buffer: String::with_capacity(BUFFER_SIZE),
        destination: Box::new(Vec::<u8>::new())
    };

    output_buffer.append_source_matches(&options, &source, source_matches);

    let expected_buffer = String::from("This world of \x1b[1;33mdew\x1b[0m,\nis a world of \x1b[1;33mdew\x1b[0m,\n");

    assert_eq!(output_buffer.buffer, expected_buffer);
}