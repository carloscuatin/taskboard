use super::*;

/// Builds a well-formed address with the given local part and total length.
fn address_with(local_len: usize, total_len: usize) -> String {
    let domain_len = total_len - local_len - 1;
    format!(
        "{}@{}.com",
        "a".repeat(local_len),
        "d".repeat(domain_len - ".com".len())
    )
}

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

#[test]
fn rejects_space_in_local_part() {
    let err = Email::parse("first last@example.com").expect_err("rejects space in local part");
    assert!(matches!(err, EmailError::InvalidCharacter));
}

#[test]
fn rejects_space_in_domain() {
    let err = Email::parse("user@exam ple.com").expect_err("rejects space in domain");
    assert!(matches!(err, EmailError::InvalidCharacter));
}

#[test]
fn rejects_tab_inside() {
    let err = Email::parse("user\t@example.com").expect_err("rejects tab inside");
    assert!(matches!(err, EmailError::InvalidCharacter));
}

#[test]
fn rejects_control_characters() {
    let inputs = [
        "user\u{0}@example.com",
        "user@example.com\u{0}",
        "user\u{7F}@example.com",
    ];

    for text in inputs {
        let err = Email::parse(text).expect_err("rejects control characters");
        assert!(matches!(err, EmailError::InvalidCharacter));
    }
}

#[test]
fn accepts_254_characters() {
    let text = address_with(64, 254);
    assert_eq!(text.len(), 254);

    let email = Email::parse(&text).expect("accepts 254 characters");
    assert_eq!(email.as_str(), text);
}

#[test]
fn rejects_255_characters() {
    let text = address_with(64, 255);
    assert_eq!(text.len(), 255);

    let err = Email::parse(&text).expect_err("rejects 255 characters");
    assert!(matches!(err, EmailError::TooLong));
}

#[test]
fn measures_length_after_trimming() {
    let text = address_with(64, 254);

    let email = Email::parse(&format!("  {text}  ")).expect("measures length after trimming");
    assert_eq!(email.as_str().len(), 254);
}

#[test]
fn accepts_64_character_local_part() {
    let text = address_with(64, 100);

    let email = Email::parse(&text).expect("accepts a 64 character local part");
    assert_eq!(email.as_str(), text);
}

#[test]
fn rejects_65_character_local_part() {
    let text = address_with(65, 100);

    let err = Email::parse(&text).expect_err("rejects a 65 character local part");
    assert!(matches!(err, EmailError::LocalPartTooLong));
}

#[test]
fn rejects_missing_at() {
    let err = Email::parse("user.example.com").expect_err("rejects a missing at sign");
    assert!(matches!(err, EmailError::MissingAt));
}

#[test]
fn rejects_consecutive_at_signs() {
    let err = Email::parse("user@@example.com").expect_err("rejects consecutive at signs");
    assert!(matches!(err, EmailError::MultipleAt));
}

#[test]
fn rejects_separate_at_signs() {
    let err = Email::parse("user@host@example.com").expect_err("rejects separate at signs");
    assert!(matches!(err, EmailError::MultipleAt));
}

#[test]
fn rejects_quoted_local_part() {
    let err = Email::parse("\"a@b\"@example.com").expect_err("rejects a quoted local part");
    assert!(matches!(err, EmailError::MultipleAt));
}
