use std::error::Error;
use std::fmt;

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

/// A parsed and normalized email address.
///
/// The field is private, so outside this module an `Email` can only come from `parse`:
/// building one from raw text does not compile.
///
/// ```compile_fail
/// use taskboard_domain::Email;
///
/// let email = Email("user@example.com".to_string());
/// ```
#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub const MAX_LEN: usize = 254;
    pub const MAX_LOCAL_LEN: usize = 64;

    pub fn parse(s: &str) -> Result<Self, EmailError> {
        let normalized = s.trim().to_ascii_lowercase();

        if normalized.is_empty() {
            return Err(EmailError::Empty);
        }

        if !normalized.is_ascii() {
            return Err(EmailError::NonAscii);
        }

        if normalized
            .chars()
            .any(|c| c.is_whitespace() || c.is_ascii_control())
        {
            return Err(EmailError::InvalidCharacter);
        }

        if normalized.len() > Self::MAX_LEN {
            return Err(EmailError::TooLong);
        }

        let Some((local, domain)) = normalized.split_once('@') else {
            return Err(EmailError::MissingAt);
        };

        if domain.contains('@') {
            return Err(EmailError::MultipleAt);
        }

        if local.len() > Self::MAX_LOCAL_LEN {
            return Err(EmailError::LocalPartTooLong);
        }

        if local.is_empty() {
            return Err(EmailError::EmptyLocalPart);
        }

        if domain.is_empty() {
            return Err(EmailError::EmptyDomain);
        }

        if !domain.contains('.') {
            return Err(EmailError::DomainWithoutDot);
        }

        if domain.starts_with('.') || domain.ends_with('.') {
            return Err(EmailError::DomainDotAtEdge);
        }

        if domain.contains("..") {
            return Err(EmailError::DomainConsecutiveDots);
        }

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests;
