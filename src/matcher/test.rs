#![allow(unused_imports)]
use super::*;

#[test]
fn ignore_case_flag() {
    let mut options = Options::default();
    options.ignore_case = true;
    let regex_flags = build_flags(&options);
    assert_eq!(regex_flags, String::from("(?i)"));
}