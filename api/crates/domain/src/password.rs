mod error;

pub use error::PasswordError;

pub struct Password(String);

impl Password {
    pub const MIN_LEN: usize = 15;

    pub fn parse(s: String) -> Result<Self, PasswordError> {
        if s.chars().count() < Self::MIN_LEN {
            return Err(PasswordError::TooShort);
        }

        Ok(Self(s))
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests;
