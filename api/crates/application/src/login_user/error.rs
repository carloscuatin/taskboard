use std::error::Error;
use std::fmt;

use crate::BoxError;

#[derive(Debug)]
pub enum LoginUserError {
    Unexpected(BoxError),
}

impl fmt::Display for LoginUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginUserError::Unexpected(_) => write!(f, "the login failed unexpectedly"),
        }
    }
}

impl Error for LoginUserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoginUserError::Unexpected(cause) => Some(cause.as_ref()),
        }
    }
}
