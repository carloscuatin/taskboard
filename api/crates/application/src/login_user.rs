mod error;

pub use error::LoginUserError;

use taskboard_domain::{Email, Password, PasswordHash, User};

use crate::{PasswordVerifier, UserRepository};

pub struct LoginUser<R, V> {
    repository: R,
    verifier: V,
    #[allow(dead_code)]
    decoy_hash: PasswordHash,
}

impl<R: UserRepository, V: PasswordVerifier> LoginUser<R, V> {
    pub fn new(repository: R, verifier: V, decoy_hash: PasswordHash) -> Self {
        Self {
            repository,
            verifier,
            decoy_hash,
        }
    }

    pub async fn execute(&self, email: Email, password: Password) -> Result<User, LoginUserError> {
        let found = self
            .repository
            .find_by_email(&email)
            .await
            .map_err(LoginUserError::Unexpected)?;
        let Some(user) = found else {
            return Err(LoginUserError::InvalidCredentials);
        };

        let matches = self
            .verifier
            .verify(password, user.password_hash())
            .await
            .map_err(LoginUserError::Unexpected)?;
        if !matches {
            return Err(LoginUserError::InvalidCredentials);
        }

        Ok(user)
    }
}

#[cfg(test)]
mod tests;
