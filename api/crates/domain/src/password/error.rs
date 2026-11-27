use std::error::Error;
use std::fmt;

use super::Password;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordError {
    TooShort,
    TooLong,
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::TooShort => {
                write!(
                    f,
                    "the password is shorter than {} characters",
                    Password::MIN_LEN
                )
            }
            PasswordError::TooLong => {
                write!(
                    f,
                    "the password is longer than {} characters",
                    Password::MAX_LEN
                )
            }
        }
    }
}

impl Error for PasswordError {}
