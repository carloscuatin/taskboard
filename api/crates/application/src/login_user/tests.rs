use std::error::Error;

use taskboard_domain::PasswordHash;

use super::*;
use crate::test_support::{
    FakeVerifier, InMemoryUserRepository, assert_cause, block_on, email, fake_hash, password,
};
use crate::{PasswordVerifier, UserRepository};

fn decoy() -> PasswordHash {
    PasswordHash::new("decoy-hash".to_string())
}

/// A store that already holds one user per `(email, password)` pair.
fn repository_with(users: &[(&str, &str)]) -> InMemoryUserRepository {
    let repository = InMemoryUserRepository::default();
    for (address, secret) in users {
        block_on(repository.add(&email(address), &fake_hash(secret))).expect("saves the user");
    }
    repository
}

fn use_case(users: &[(&str, &str)]) -> LoginUser<InMemoryUserRepository, FakeVerifier> {
    LoginUser::new(repository_with(users), FakeVerifier::default(), decoy())
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

#[test]
fn store_failure_keeps_its_cause_and_skips_the_verifier() {
    let use_case = LoginUser::new(
        InMemoryUserRepository::default().failing_with("store is down"),
        FakeVerifier::default(),
        decoy(),
    );

    let error =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("the store fails");

    assert!(matches!(error, LoginUserError::Unexpected(_)));
    assert_cause(&error, "store is down");
    assert_eq!(use_case.verifier.calls(), 0);
}

#[test]
fn verifier_failure_keeps_its_cause_for_a_known_email() {
    let use_case = LoginUser::new(
        repository_with(&[("user@example.com", "horse-battery-staple")]),
        FakeVerifier::default().failing_with("verifier is down"),
        decoy(),
    );

    let error =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("the verifier fails");

    assert!(matches!(error, LoginUserError::Unexpected(_)));
    assert_cause(&error, "verifier is down");
}

#[test]
fn verifier_failure_is_unexpected_for_an_unknown_email_too() {
    let use_case = LoginUser::new(
        InMemoryUserRepository::default(),
        FakeVerifier::default().failing_with("verifier is down"),
        decoy(),
    );

    let error = block_on(use_case.execute(
        email("nobody@example.com"),
        password("horse-battery-staple"),
    ))
    .expect_err("the verifier fails");

    assert!(matches!(error, LoginUserError::Unexpected(_)));
    assert_cause(&error, "verifier is down");
}

#[test]
fn errors_do_not_carry_the_password() {
    let wrong = use_case(&[("user@example.com", "correct-horse-battery")]);
    let broken = LoginUser::new(
        InMemoryUserRepository::default(),
        FakeVerifier::default().failing_with("verifier is down"),
        decoy(),
    );

    let rejected =
        block_on(wrong.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("rejects the password");
    let failed =
        block_on(broken.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("the verifier fails");

    for error in [rejected, failed] {
        assert!(!format!("{error:?}").contains("horse-battery-staple"));
        assert!(!format!("{error}").contains("horse-battery-staple"));
    }
}

#[test]
fn error_propagates_as_std_error() {
    fn log_in_with_question_mark(
        use_case: &LoginUser<InMemoryUserRepository, FakeVerifier>,
    ) -> Result<(), Box<dyn Error>> {
        block_on(use_case.execute(
            email("nobody@example.com"),
            password("horse-battery-staple"),
        ))?;
        Ok(())
    }

    let error = log_in_with_question_mark(&use_case(&[])).expect_err("propagates the rejection");

    assert_eq!(
        error.to_string(),
        LoginUserError::InvalidCredentials.to_string()
    );
}

#[test]
fn execute_future_is_send() {
    fn assert_send<T: Send>(_: T) {}

    fn with_any_ports<R: UserRepository, V: PasswordVerifier>(use_case: &LoginUser<R, V>) {
        assert_send(use_case.execute(email("user@example.com"), password("horse-battery-staple")));
    }

    with_any_ports(&use_case(&[]));
}
