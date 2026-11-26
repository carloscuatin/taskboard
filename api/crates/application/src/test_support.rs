use std::pin::pin;
use std::task::{Context, Poll, Waker};

mod event_log;
mod fake_hasher;
mod in_memory_user_repository;

pub use event_log::EventLog;
pub use fake_hasher::FakeHasher;
pub use in_memory_user_repository::InMemoryUserRepository;

/// Runs a future that must finish on its first poll, which every double here does:
/// none of them waits on a network, a disk or a timer.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("the future is waiting; a test double must not wait"),
    }
}

#[cfg(test)]
mod tests {
    use taskboard_domain::{Email, Password, PasswordHash};

    use super::*;
    use crate::{AddUserError, PasswordHasher, UserRepository};

    fn email(text: &str) -> Email {
        Email::parse(text).expect("a valid email")
    }

    fn hash(text: &str) -> PasswordHash {
        PasswordHash::new(text.to_string())
    }

    fn password(text: &str) -> Password {
        Password::parse(text.to_string()).expect("a valid password")
    }

    #[test]
    fn block_on_returns_the_value_of_a_ready_future() {
        assert_eq!(block_on(async { 42 }), 42);
    }

    #[test]
    #[should_panic(expected = "must not wait")]
    fn block_on_panics_when_the_future_is_pending() {
        block_on(std::future::pending::<()>());
    }

    #[test]
    fn repository_keeps_what_it_is_given() {
        let repository = InMemoryUserRepository::default();

        block_on(repository.add(&email("ana@example.com"), &hash("hash-a"))).expect("saves");
        block_on(repository.add(&email("ben@example.com"), &hash("hash-b"))).expect("saves");

        assert_eq!(
            repository.saved(),
            vec![
                (email("ana@example.com"), hash("hash-a")),
                (email("ben@example.com"), hash("hash-b")),
            ]
        );
        assert_eq!(repository.attempts(), 2);
    }

    #[test]
    fn repository_refuses_a_repeated_email() {
        let repository = InMemoryUserRepository::default();
        block_on(repository.add(&email("ana@example.com"), &hash("hash-a"))).expect("saves");

        let result = block_on(repository.add(&email("ana@example.com"), &hash("hash-b")));

        assert!(matches!(result, Err(AddUserError::EmailTaken)));
        assert_eq!(repository.saved().len(), 1);
        assert_eq!(repository.attempts(), 2);
    }

    #[test]
    fn repository_can_be_told_the_email_is_taken() {
        let repository = InMemoryUserRepository::default().taking_every_email();

        let result = block_on(repository.add(&email("ana@example.com"), &hash("hash-a")));

        assert!(matches!(result, Err(AddUserError::EmailTaken)));
        assert!(repository.saved().is_empty());
    }

    #[test]
    fn repository_can_be_told_to_fail() {
        let repository = InMemoryUserRepository::default().failing_with("store is down");

        let result = block_on(repository.add(&email("ana@example.com"), &hash("hash-a")));

        assert!(
            matches!(result, Err(AddUserError::Unexpected(cause)) if cause.to_string() == "store is down")
        );
        assert!(repository.saved().is_empty());
    }

    #[test]
    fn hasher_derives_the_hash_from_the_password_and_remembers_the_calls() {
        let hasher = FakeHasher::default();

        let first = block_on(hasher.hash(password("horse-battery-staple"))).expect("hashes");
        let second = block_on(hasher.hash(password("correct-horse-battery"))).expect("hashes");
        let again = block_on(hasher.hash(password("horse-battery-staple"))).expect("hashes");

        assert_eq!(first, again);
        assert_ne!(first, second);
        assert!(!first.as_str().contains("horse"));
        assert_eq!(hasher.calls(), 3);
        assert_eq!(
            hasher.received(),
            vec![
                "horse-battery-staple",
                "correct-horse-battery",
                "horse-battery-staple"
            ]
        );
    }

    #[test]
    fn hasher_can_be_told_to_fail() {
        let hasher = FakeHasher::default().failing_with("hasher is down");

        let result = block_on(hasher.hash(password("horse-battery-staple")));

        assert!(matches!(result, Err(cause) if cause.to_string() == "hasher is down"));
        assert_eq!(hasher.calls(), 1);
    }
}
