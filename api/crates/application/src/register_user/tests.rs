use super::*;
use crate::test_support::{FakeHasher, InMemoryUserRepository, block_on};
use taskboard_domain::{Email, Password, PasswordHash};

fn email(text: &str) -> Email {
    Email::parse(text).expect("a valid email")
}

fn password(text: &str) -> Password {
    Password::parse(text.to_string()).expect("a valid password")
}

fn fake_hash() -> PasswordHash {
    PasswordHash::new("fake-hash-value".to_string())
}

fn use_case() -> RegisterUser<InMemoryUserRepository, FakeHasher> {
    RegisterUser::new(InMemoryUserRepository::default(), FakeHasher::default())
}

#[test]
fn registers_new_email() {
    let use_case = use_case();

    block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
        .expect("registers a new email");

    assert_eq!(use_case.hasher.received(), vec!["horse-battery-staple"]);
    assert_eq!(
        use_case.repository.saved(),
        vec![(email("user@example.com"), fake_hash())]
    );
}

#[test]
fn registers_different_emails() {
    let use_case = use_case();

    block_on(use_case.execute(email("first@example.com"), password("horse-battery-staple")))
        .expect("registers the first email");
    block_on(use_case.execute(
        email("second@example.com"),
        password("correct-horse-battery"),
    ))
    .expect("registers the second email");

    assert_eq!(
        use_case.repository.saved(),
        vec![
            (email("first@example.com"), fake_hash()),
            (email("second@example.com"), fake_hash()),
        ]
    );
}
