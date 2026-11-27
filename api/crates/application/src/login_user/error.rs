use std::error::Error;
use std::fmt;

use crate::BoxError;

#[derive(Debug)]
pub enum LoginUserError {
    Unexpected(BoxError),
    InvalidCredentials,
}

impl fmt::Display for LoginUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginUserError::Unexpected(_) => write!(f, "the login failed unexpectedly"),
            LoginUserError::InvalidCredentials => write!(f, "the email or the password is wrong"),
        }
    }
}

impl Error for LoginUserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoginUserError::Unexpected(cause) => Some(cause.as_ref()),
            LoginUserError::InvalidCredentials => None,
        }
    }
}
