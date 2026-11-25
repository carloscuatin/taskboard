mod error;

pub use error::PasswordError;

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

#[cfg(test)]
mod tests;
