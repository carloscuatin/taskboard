use std::error::Error;

/// Checks that the error keeps the failure that caused it, which is what lets a log
/// show the real reason behind a generic message.
pub fn assert_cause(error: &dyn Error, expected: &str) {
    let cause = error.source().expect("the error has a cause");

    assert_eq!(cause.to_string(), expected);
}
