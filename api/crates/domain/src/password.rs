use std::fmt;

mod error;

pub use error::PasswordError;

/// A password that has passed the length policy.
///
/// It has no `Display`, so it cannot be printed by accident:
///
/// ```compile_fail
/// use taskboard_domain::Password;
///
/// let password = Password::parse("horse-battery-staple".to_string()).unwrap();
/// println!("{password}");
/// ```
pub struct Password(String);

impl Password {
    pub const MIN_LEN: usize = 15;
    pub const MAX_LEN: usize = 128;

    pub fn parse(s: String) -> Result<Self, PasswordError> {
        let len = s.chars().count();

        if len < Self::MIN_LEN {
            return Err(PasswordError::TooShort);
        }

        if len > Self::MAX_LEN {
            return Err(PasswordError::TooLong);
        }

        Ok(Self(s))
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Password(<redacted>)")
    }
}

#[cfg(test)]
mod tests;
