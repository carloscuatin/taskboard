use std::error::Error;

use taskboard_domain::{Email, Password, PasswordHash};

use super::*;
use crate::test_support::{EventLog, FakeHasher, InMemoryUserRepository, block_on};

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

#[test]
fn rejects_same_email_twice() {
    let use_case = use_case();
    block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
        .expect("registers the first time");

    let result =
        block_on(use_case.execute(email("user@example.com"), password("correct-horse-battery")));

    assert!(matches!(
        result,
        Err(RegisterUserError::EmailAlreadyRegistered)
    ));
    assert_eq!(
        use_case.repository.saved(),
        vec![(email("user@example.com"), fake_hash())]
    );
}

#[test]
fn rejects_email_that_normalizes_the_same() {
    let use_case = use_case();
    block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
        .expect("registers the first time");

    let result = block_on(use_case.execute(
        email(" User@Example.COM "),
        password("horse-battery-staple"),
    ));

    assert!(matches!(
        result,
        Err(RegisterUserError::EmailAlreadyRegistered)
    ));
}

#[test]
fn detects_duplicate_when_saving() {
    let use_case = RegisterUser::new(
        InMemoryUserRepository::default().taking_every_email(),
        FakeHasher::default(),
    );

    let result =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")));

    assert!(matches!(
        result,
        Err(RegisterUserError::EmailAlreadyRegistered)
    ));
    assert_eq!(use_case.repository.attempts(), 1);
    assert!(use_case.repository.saved().is_empty());
}

#[test]
fn hashes_before_saving_a_duplicate() {
    let events = EventLog::default();
    let use_case = RegisterUser::new(
        InMemoryUserRepository::default().recording_in(&events),
        FakeHasher::default().recording_in(&events),
    );
    block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
        .expect("registers the first time");

    let result =
        block_on(use_case.execute(email("user@example.com"), password("correct-horse-battery")));

    assert!(matches!(
        result,
        Err(RegisterUserError::EmailAlreadyRegistered)
    ));
    assert_eq!(events.events(), vec!["hash", "add", "hash", "add"]);
}

#[test]
fn hashing_failure_stops_before_saving() {
    let use_case = RegisterUser::new(
        InMemoryUserRepository::default(),
        FakeHasher::default().failing_with("hasher is down"),
    );

    let result =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")));

    assert!(
        matches!(result, Err(RegisterUserError::Unexpected(cause)) if cause.to_string() == "hasher is down")
    );
    assert_eq!(use_case.repository.attempts(), 0);
    assert!(use_case.repository.saved().is_empty());
}

#[test]
fn store_failure_keeps_its_cause() {
    let use_case = RegisterUser::new(
        InMemoryUserRepository::default().failing_with("connection refused"),
        FakeHasher::default(),
    );

    let error =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("the store fails");

    assert!(matches!(error, RegisterUserError::Unexpected(_)));
    let cause = error.source().expect("an unexpected error has a cause");
    assert_eq!(cause.to_string(), "connection refused");
}

#[test]
fn hashing_failure_keeps_its_cause() {
    let use_case = RegisterUser::new(
        InMemoryUserRepository::default(),
        FakeHasher::default().failing_with("hasher unavailable"),
    );

    let error =
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("the hasher fails");

    assert!(matches!(error, RegisterUserError::Unexpected(_)));
    let cause = error.source().expect("an unexpected error has a cause");
    assert_eq!(cause.to_string(), "hasher unavailable");
}

#[test]
fn errors_do_not_carry_password() {
    let duplicate = RegisterUser::new(
        InMemoryUserRepository::default().taking_every_email(),
        FakeHasher::default(),
    );
    let failing = RegisterUser::new(
        InMemoryUserRepository::default().failing_with("connection refused"),
        FakeHasher::default(),
    );

    let errors = [
        block_on(duplicate.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("a duplicate"),
        block_on(failing.execute(email("user@example.com"), password("horse-battery-staple")))
            .expect_err("a failing store"),
    ];

    for error in errors {
        assert!(!format!("{error:?}").contains("horse-battery-staple"));
        assert!(!format!("{error}").contains("horse-battery-staple"));
    }
}

#[test]
fn error_propagates_as_std_error() {
    fn register_with_question_mark() -> Result<(), Box<dyn Error>> {
        let use_case = RegisterUser::new(
            InMemoryUserRepository::default().taking_every_email(),
            FakeHasher::default(),
        );
        block_on(use_case.execute(email("user@example.com"), password("horse-battery-staple")))?;
        Ok(())
    }

    let error = register_with_question_mark().expect_err("propagates the duplicate");

    assert!(!error.to_string().is_empty());
    assert_eq!(
        error.to_string(),
        RegisterUserError::EmailAlreadyRegistered.to_string()
    );
}

#[test]
fn execute_future_is_send() {
    fn assert_send<T: Send>(_: T) {}

    let use_case = use_case();

    assert_send(use_case.execute(email("user@example.com"), password("horse-battery-staple")));
}
