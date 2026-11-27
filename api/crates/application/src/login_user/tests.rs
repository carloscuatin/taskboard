use taskboard_domain::PasswordHash;

use super::*;
use crate::UserRepository;
use crate::test_support::{
    FakeVerifier, InMemoryUserRepository, block_on, email, fake_hash, password,
};

fn decoy() -> PasswordHash {
    PasswordHash::new("decoy-hash".to_string())
}

/// A use case whose store already holds one user per `(email, password)` pair.
fn use_case(users: &[(&str, &str)]) -> LoginUser<InMemoryUserRepository, FakeVerifier> {
    let repository = InMemoryUserRepository::default();
    for (address, secret) in users {
        block_on(repository.add(&email(address), &fake_hash(secret))).expect("saves the user");
    }
    LoginUser::new(repository, FakeVerifier::default(), decoy())
}

#[test]
fn returns_the_stored_user_for_the_right_password() {
    let use_case = use_case(&[("user@example.com", "horse-battery-staple")]);

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
    let use_case = use_case(&[("user@example.com", "horse-battery-staple")]);

    let user = block_on(use_case.execute(
        email(" User@Example.COM "),
        password("horse-battery-staple"),
    ))
    .expect("logs in");

    assert_eq!(user.email().as_str(), "user@example.com");
}

#[test]
fn returns_only_the_user_of_the_given_email() {
    let use_case = use_case(&[
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

#[test]
fn rejects_a_wrong_password() {
    let use_case = use_case(&[("user@example.com", "horse-battery-staple")]);

    let result =
        block_on(use_case.execute(email("user@example.com"), password("correct-horse-battery")));

    assert!(matches!(result, Err(LoginUserError::InvalidCredentials)));
}

#[test]
fn rejects_an_unknown_email() {
    let use_case = use_case(&[]);

    let result =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")));

    assert!(matches!(result, Err(LoginUserError::InvalidCredentials)));
}

#[test]
fn rejects_the_password_of_another_user() {
    let use_case = use_case(&[
        ("first@example.com", "horse-battery-staple"),
        ("second@example.com", "correct-horse-battery"),
    ]);

    let result = block_on(use_case.execute(
        email("first@example.com"),
        password("correct-horse-battery"),
    ));

    assert!(matches!(result, Err(LoginUserError::InvalidCredentials)));
}

#[test]
fn unknown_email_and_wrong_password_look_the_same() {
    let use_case = use_case(&[("user@example.com", "horse-battery-staple")]);

    let unknown = block_on(use_case.execute(
        email("nobody@example.com"),
        password("horse-battery-staple"),
    ))
    .expect_err("rejects the unknown email");
    let wrong =
        block_on(use_case.execute(email("user@example.com"), password("correct-horse-battery")))
            .expect_err("rejects the wrong password");

    assert_eq!(unknown.to_string(), wrong.to_string());
    assert_eq!(format!("{unknown:?}"), format!("{wrong:?}"));
}

#[test]
fn verifies_an_unknown_email_against_the_decoy() {
    let use_case = use_case(&[]);

    let _ = block_on(use_case.execute(
        email("nobody@example.com"),
        password("horse-battery-staple"),
    ));

    assert_eq!(
        use_case.verifier.received(),
        vec![("horse-battery-staple".to_string(), decoy())]
    );
}

#[test]
fn a_decoy_that_matches_never_logs_anyone_in() {
    let use_case = LoginUser::new(
        InMemoryUserRepository::default(),
        FakeVerifier::default().matching_every_hash(),
        decoy(),
    );

    let result = block_on(use_case.execute(
        email("nobody@example.com"),
        password("horse-battery-staple"),
    ));

    assert!(matches!(result, Err(LoginUserError::InvalidCredentials)));
    assert_eq!(use_case.verifier.calls(), 1);
}
