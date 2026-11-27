mod error;

pub use error::LoginUserError;

use taskboard_domain::{Email, Password, PasswordHash, User};

use crate::{PasswordVerifier, UserRepository};

pub struct LoginUser<R, V> {
    repository: R,
    verifier: V,
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

    /// An unknown email is verified against the decoy hash and the result discarded, so
    /// it does not answer faster than a known one.
    pub async fn execute(&self, email: Email, password: Password) -> Result<User, LoginUserError> {
        let found = self
            .repository
            .find_by_email(&email)
            .await
            .map_err(LoginUserError::Unexpected)?;
        let hash = found.as_ref().map_or(&self.decoy_hash, User::password_hash);

        let matches = self
            .verifier
            .verify(password, hash)
            .await
            .map_err(LoginUserError::Unexpected)?;

        match found {
            Some(user) if matches => Ok(user),
            _ => Err(LoginUserError::InvalidCredentials),
        }
    }
}

#[cfg(test)]
mod tests;
