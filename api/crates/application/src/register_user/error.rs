use std::error::Error;
use std::fmt;

use crate::BoxError;

#[derive(Debug)]
pub enum RegisterUserError {
    Unexpected(BoxError),
    EmailAlreadyRegistered,
}

impl fmt::Display for RegisterUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterUserError::Unexpected(_) => write!(f, "the registration failed unexpectedly"),
            RegisterUserError::EmailAlreadyRegistered => {
                write!(f, "the email is already registered")
            }
        }
    }
}

impl Error for RegisterUserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RegisterUserError::Unexpected(cause) => Some(cause.as_ref()),
            RegisterUserError::EmailAlreadyRegistered => None,
        }
    }
}
