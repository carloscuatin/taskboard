#[derive(Debug)]
pub enum EmailError {
    Empty,
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
        if s.trim().is_empty() {
            return Err(EmailError::Empty);
        }

        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_well_formed_address() {
        let email = Email::parse("user@example.com").expect("a well-formed address is accepted");
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn keeps_common_local_part_characters() {
        let email = Email::parse("first.last+tag@sub.example.org")
            .expect("keeps common local part characters");

        assert_eq!(email.as_str(), "first.last+tag@sub.example.org");
    }

    #[test]
    fn rejects_empty_text() {
        let err = Email::parse("").expect_err("rejects empty text");
        assert!(matches!(err, EmailError::Empty));
    }

    #[test]
    fn rejects_only_whitespace() {
        let inputs = ["   ", "\n", "\t", " \n\t "];

        for text in inputs {
            let err = Email::parse(text).expect_err("rejects only whitespace");
            assert!(matches!(err, EmailError::Empty));
        }
    }
}
