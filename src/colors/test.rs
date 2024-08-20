#![allow(unused_imports)]
use super::*;

#[test]
fn default_colors() {
    let default = Colors::default();
    let expected_default = Colors {
        selected_match: String::from("1;33"),
        context_match: String::from("1:33"),
        selected_line: String::from(""),
        context_line: String::from(""),
        file_name: String::from("32"),
        line_number: String::from("31"),
        byte_offset: String::from("36"),
        separator: String::from("35"),
    };

    assert_eq!(default, expected_default);
}

#[test]
fn parse_valid_colors_string() {
    let colors_string = "ms=30:mc=91:fn=93:ln=95:se=107;91";

    let colors = Colors::parse_colors(colors_string.to_string());

    let expected_colors = Colors {
        selected_match: String::from("30"),
        context_match: String::from("91"),
        selected_line: String::from(""),
        context_line: String::from(""),
        file_name: String::from("93"),
        line_number: String::from("95"),
        byte_offset: String::from("36"),
        separator: String::from("107;91"),
    };

    assert_eq!(colors, expected_colors);

    let colors_string = "mt=30";

    let colors = Colors::parse_colors(colors_string.to_string());

    let expected_colors = Colors {
        selected_match: String::from("30"),
        context_match: String::from("30"),
        selected_line: String::from(""),
        context_line: String::from(""),
        file_name: String::from("32"),
        line_number: String::from("31"),
        byte_offset: String::from("36"),
        separator: String::from("35"),
    };

    assert_eq!(colors, expected_colors);
}

#[test]
fn parse_empty_colors_string() {
    let colors_string = "";

    let colors = Colors::parse_colors(colors_string.to_string());

    assert_eq!(colors, Colors::default());
}

#[test]
fn parse_invalid_ascii_colors_string() {
    let colors_string = "ms=30:\u{C398}";

    let colors = Colors::parse_colors(colors_string.to_string());

    assert_eq!(colors, Colors::default());
}