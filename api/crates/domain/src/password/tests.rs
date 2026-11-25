use super::*;

#[test]
fn accepts_minimum_length() {
    let text = "a".repeat(15);
    assert_eq!(text.len(), 15);

    assert!(Password::parse(text).is_ok());
}

#[test]
fn rejects_one_below_minimum() {
    let text = "a".repeat(14);
    assert_eq!(text.len(), 14);

    assert_eq!(Password::parse(text).err(), Some(PasswordError::TooShort));
}

#[test]
fn rejects_empty_as_too_short() {
    assert_eq!(
        Password::parse(String::new()).err(),
        Some(PasswordError::TooShort)
    );
}

#[test]
fn exposes_plaintext_on_request() {
    let text = "clave con ñandú 123";
    assert!(text.chars().count() >= Password::MIN_LEN);

    let password = Password::parse(text.to_string()).expect("accepts a valid password");
    assert_eq!(password.expose(), text);
}

#[test]
fn accepts_maximum_length() {
    let text = "a".repeat(128);
    assert_eq!(text.len(), 128);

    assert!(Password::parse(text).is_ok());
}

#[test]
fn rejects_one_above_maximum() {
    let text = "a".repeat(129);
    assert_eq!(text.len(), 129);

    assert_eq!(Password::parse(text).err(), Some(PasswordError::TooLong));
}

#[test]
fn counts_characters_not_bytes() {
    let enough = "ñ".repeat(15);
    let one_short = "ñ".repeat(14);
    assert_eq!(enough.len(), 30);
    assert_eq!(one_short.len(), 28);

    assert!(Password::parse(enough).is_ok());
    assert_eq!(
        Password::parse(one_short).err(),
        Some(PasswordError::TooShort)
    );
}

#[test]
fn keeps_surrounding_spaces() {
    let text = format!(" {} ", "a".repeat(13));
    assert_eq!(text.chars().count(), 15);

    let password = Password::parse(text.clone()).expect("spaces count as characters");

    assert_eq!(password.expose(), text);
}

#[test]
fn applies_no_composition_rules() {
    let lowercase_only = "a".repeat(15);
    let mixed = "mi clave ñandú😀";
    assert_eq!(mixed.chars().count(), 15);

    assert!(Password::parse(lowercase_only).is_ok());
    assert!(Password::parse(mixed.to_string()).is_ok());
}

#[test]
fn debug_hides_plaintext() {
    let password = Password::parse("horse-battery-staple".to_string()).expect("valid password");

    assert!(!format!("{password:?}").contains("horse-battery-staple"));
}

#[test]
fn rejection_does_not_carry_password() {
    let error = Password::parse("tiny-secret".to_string()).expect_err("too short");

    assert!(!format!("{error:?}").contains("tiny-secret"));
    assert!(!format!("{error}").contains("tiny-secret"));
}

#[test]
fn every_error_has_its_own_description() {
    let errors = [PasswordError::TooShort, PasswordError::TooLong];

    let descriptions: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
    assert!(descriptions.iter().all(|d| !d.is_empty()));

    let unique: std::collections::HashSet<&String> = descriptions.iter().collect();
    assert_eq!(unique.len(), descriptions.len());
}

#[test]
fn errors_are_comparable() {
    let first = Password::parse(String::new()).expect_err("rejects empty text");
    let second = Password::parse("a".repeat(14)).expect_err("rejects a short text");
    let other = Password::parse("a".repeat(129)).expect_err("rejects a long text");

    assert_eq!(first, second);
    assert_ne!(first, other);
}

#[test]
fn error_propagates_as_std_error() {
    fn parse_with_question_mark() -> Result<(), Box<dyn std::error::Error>> {
        Password::parse("a".repeat(129))?;
        Ok(())
    }

    let err = parse_with_question_mark().expect_err("propagates the rejection");
    assert_eq!(err.to_string(), PasswordError::TooLong.to_string());
}
