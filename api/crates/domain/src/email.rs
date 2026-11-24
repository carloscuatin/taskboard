#[derive(Debug)]
pub enum EmailError {
    Empty,
    NonAscii,
    InvalidCharacter,
}

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

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests;
