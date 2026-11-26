use taskboard_domain::{Email, Password};

use crate::{AddUserError, BoxError, PasswordHasher, UserRepository};

#[derive(Debug)]
pub enum RegisterUserError {
    Unexpected(BoxError),
    EmailAlreadyRegistered,
}

pub struct RegisterUser<R, H> {
    repository: R,
    hasher: H,
}

impl<R: UserRepository, H: PasswordHasher> RegisterUser<R, H> {
    pub fn new(repository: R, hasher: H) -> Self {
        Self { repository, hasher }
    }

    /// The password is hashed even when the email turns out to be taken, so a registered
    /// email does not answer faster than a new one.
    pub async fn execute(&self, email: Email, password: Password) -> Result<(), RegisterUserError> {
        let hash = self
            .hasher
            .hash(password)
            .await
            .map_err(RegisterUserError::Unexpected)?;

        match self.repository.add(&email, &hash).await {
            Ok(()) => Ok(()),
            Err(AddUserError::Unexpected(cause)) => Err(RegisterUserError::Unexpected(cause)),
            Err(AddUserError::EmailTaken) => Err(RegisterUserError::EmailAlreadyRegistered),
        }
    }
}

#[cfg(test)]
mod tests;
