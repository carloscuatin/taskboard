use super::*;
use crate::{Email, PasswordHash, UserId};

fn user() -> User {
    User::new(
        UserId::from_u128(7),
        Email::parse("user@example.com").expect("valid email"),
        PasswordHash::new("fake-hash-value".to_string()),
    )
}

#[test]
fn reads_back_its_parts() {
    let user = user();

    assert_eq!(user.id(), UserId::from_u128(7));
    assert_eq!(user.email().as_str(), "user@example.com");
    assert_eq!(user.password_hash().as_str(), "fake-hash-value");
}

#[test]
fn debug_hides_the_password_hash() {
    let user = user();

    assert!(!format!("{user:?}").contains("fake-hash-value"));
}
