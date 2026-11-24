#[derive(Debug)]
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

        if normalized.len() > 254 {
            return Err(EmailError::TooLong);
        }

        let Some((local, domain)) = normalized.split_once('@') else {
            return Err(EmailError::MissingAt);
        };

        if domain.contains('@') {
            return Err(EmailError::MultipleAt);
        }

        if local.len() > 64 {
            return Err(EmailError::LocalPartTooLong);
        }

        if local.is_empty() {
            return Err(EmailError::EmptyLocalPart);
        }

        if domain.is_empty() {
            return Err(EmailError::EmptyDomain);
        }

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests;
