use std::fmt;

mod error;
mod rules;

pub use error::EmailError;
use rules::{check_characters, check_domain, check_length, check_local_part, split_at_sign};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Email {
    pub const MAX_LEN: usize = 254;
    pub const MAX_LOCAL_LEN: usize = 64;

    pub fn parse(s: &str) -> Result<Self, EmailError> {
        let normalized = s.trim().to_ascii_lowercase();

        check_characters(&normalized)?;
        check_length(&normalized)?;
        let (local, domain) = split_at_sign(&normalized)?;
        check_local_part(local)?;
        check_domain(domain)?;

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests;
