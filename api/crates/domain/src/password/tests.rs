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
