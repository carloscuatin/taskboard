use taskboard_domain::{Email, Password, PasswordHash};

use super::*;
use crate::UserRepository;
use crate::test_support::{FakeVerifier, InMemoryUserRepository, block_on, fake_hash};

fn email(text: &str) -> Email {
    Email::parse(text).expect("a valid email")
}

fn password(text: &str) -> Password {
    Password::parse(text.to_string()).expect("a valid password")
}

fn decoy() -> PasswordHash {
    PasswordHash::new("decoy-hash".to_string())
}

/// A use case whose store already holds one user per `(email, password)` pair.
fn use_case_with(users: &[(&str, &str)]) -> LoginUser<InMemoryUserRepository, FakeVerifier> {
    let repository = InMemoryUserRepository::default();
    for (address, secret) in users {
        block_on(repository.add(&email(address), &fake_hash(secret))).expect("saves the user");
    }
    LoginUser::new(repository, FakeVerifier::default(), decoy())
}

#[test]
fn returns_the_stored_user_for_the_right_password() {
    let use_case = use_case_with(&[("user@example.com", "horse-battery-staple")]);

    let user =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect("logs in");

    assert_eq!(user, use_case.repository.users()[0]);
    assert_eq!(
        use_case.verifier.received(),
        vec![(
            "horse-battery-staple".to_string(),
            fake_hash("horse-battery-staple")
        )]
    );
}

#[test]
fn finds_the_user_by_the_normalized_email() {
    let use_case = use_case_with(&[("user@example.com", "horse-battery-staple")]);

    let user = block_on(use_case.execute(
        email(" User@Example.COM "),
        password("horse-battery-staple"),
    ))
    .expect("logs in");

    assert_eq!(user.email().as_str(), "user@example.com");
}

#[test]
fn returns_only_the_user_of_the_given_email() {
    let use_case = use_case_with(&[
        ("first@example.com", "horse-battery-staple"),
        ("second@example.com", "correct-horse-battery"),
    ]);

    let user = block_on(use_case.execute(
        email("second@example.com"),
        password("correct-horse-battery"),
    ))
    .expect("logs in");

    assert_eq!(user, use_case.repository.users()[1]);
    assert_ne!(user.id(), use_case.repository.users()[0].id());
}
