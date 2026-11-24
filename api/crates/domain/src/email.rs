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
        let normalized = s.trim().to_ascii_lowercase();

        if normalized.is_empty() {
            return Err(EmailError::Empty);
        }

        Ok(Self(normalized))
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

    #[test]
    fn trims_surrounding_spaces() {
        let email = Email::parse("  user@example.com  ").expect("trims surrounding spaces");
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn trims_surrounding_tabs_and_line_breaks() {
        let email =
            Email::parse("\tuser@example.com\n").expect("trims surrounding tabs and line breaks");
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn trims_surrounding_non_breaking_spaces() {
        let email = Email::parse("\u{00A0}user@example.com\u{00A0}")
            .expect("trims surrounding non-breaking spaces");
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn lowercases_both_parts() {
        let email = Email::parse("First.Last@Example.COM").expect("lowercases both parts");
        assert_eq!(email.as_str(), "first.last@example.com");
    }
}
