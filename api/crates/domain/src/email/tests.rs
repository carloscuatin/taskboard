use super::*;

#[test]
fn accepts_well_formed_address() {
    let email = Email::parse("user@example.com").expect("a well-formed address is accepted");
    assert_eq!(email.as_str(), "user@example.com");
}

#[test]
fn keeps_common_local_part_characters() {
    let email =
        Email::parse("first.last+tag@sub.example.org").expect("keeps common local part characters");

    assert_eq!(email.as_str(), "first.last+tag@sub.example.org");
}

#[test]
fn rejects_empty_text() {
    let err = Email::parse("").expect_err("rejects empty text");
    assert!(matches!(err, EmailError::Empty));
}

#[test]
fn rejects_only_whitespace() {
    let inputs = ["   ", "\n", "\t", " \n\t "];

    for text in inputs {
        let err = Email::parse(text).expect_err("rejects only whitespace");
        assert!(matches!(err, EmailError::Empty));
    }
}

#[test]
fn trims_surrounding_spaces() {
    let email = Email::parse("  user@example.com  ").expect("trims surrounding spaces");
    assert_eq!(email.as_str(), "user@example.com");
}

#[test]
fn trims_surrounding_tabs_and_line_breaks() {
    let email =
        Email::parse("\tuser@example.com\n").expect("trims surrounding tabs and line breaks");
    assert_eq!(email.as_str(), "user@example.com");
}

#[test]
fn trims_surrounding_non_breaking_spaces() {
    let email = Email::parse("\u{00A0}user@example.com\u{00A0}")
        .expect("trims surrounding non-breaking spaces");
    assert_eq!(email.as_str(), "user@example.com");
}

#[test]
fn lowercases_both_parts() {
    let email = Email::parse("First.Last@Example.COM").expect("lowercases both parts");
    assert_eq!(email.as_str(), "first.last@example.com");
}

#[test]
fn rejects_non_ascii_local_part() {
    let err = Email::parse("josé@example.com").expect_err("rejects non-ASCII local part");
    assert!(matches!(err, EmailError::NonAscii));
}

#[test]
fn rejects_non_ascii_domain() {
    let err = Email::parse("user@bücher.example").expect_err("rejects non-ASCII domain");
    assert!(matches!(err, EmailError::NonAscii));
}

#[test]
fn rejects_fullwidth_at_sign() {
    let err = Email::parse("user\u{FF20}example.com").expect_err("rejects fullwidth at sign");
    assert!(matches!(err, EmailError::NonAscii));
}

#[test]
fn accepts_ascii_internationalized_domain() {
    let email =
        Email::parse("user@xn--bcher-kva.example").expect("accepts ASCII internationalized domain");
    assert_eq!(email.as_str(), "user@xn--bcher-kva.example");
}
