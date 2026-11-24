use std::error::Error;
use std::fmt;

use super::Email;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    NonAscii,
    InvalidCharacter,
    TooLong,
    MissingAt,
    MultipleAt,
    EmptyLocalPart,
    LocalPartTooLong,
    EmptyDomain,
    DomainWithoutDot,
    DomainDotAtEdge,
    DomainConsecutiveDots,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::Empty => write!(f, "the email is empty"),
            EmailError::NonAscii => write!(f, "the email contains non-ASCII characters"),
            EmailError::InvalidCharacter => {
                write!(f, "the email contains a space or a control character")
            }
            EmailError::TooLong => {
                write!(f, "the email is longer than {} characters", Email::MAX_LEN)
            }
            EmailError::MissingAt => write!(f, "the email has no at sign"),
            EmailError::MultipleAt => write!(f, "the email has more than one at sign"),
            EmailError::EmptyLocalPart => write!(f, "the part before the at sign is empty"),
            EmailError::LocalPartTooLong => {
                write!(
                    f,
                    "the part before the at sign is longer than {} characters",
                    Email::MAX_LOCAL_LEN
                )
            }
            EmailError::EmptyDomain => write!(f, "the part after the at sign is empty"),
            EmailError::DomainWithoutDot => write!(f, "the domain has no dot"),
            EmailError::DomainDotAtEdge => write!(f, "the domain starts or ends with a dot"),
            EmailError::DomainConsecutiveDots => {
                write!(f, "the domain has two dots in a row")
            }
        }
    }
}

impl Error for EmailError {}
