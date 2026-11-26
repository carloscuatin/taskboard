use taskboard_domain::{Email, Password, PasswordHash};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum AddUserError {
    EmailTaken,
    Unexpected(BoxError),
}

pub trait UserRepository: Send + Sync {
    /// The store detects a repeated email itself and answers `EmailTaken`; a lookup
    /// before saving cannot tell two simultaneous registrations apart.
    fn add(
        &self,
        email: &Email,
        password_hash: &PasswordHash,
    ) -> impl Future<Output = Result<(), AddUserError>> + Send;
}

/// The algorithm and its cost belong to the implementation, not to this crate.
pub trait PasswordHasher: Send + Sync {
    fn hash(
        &self,
        password: Password,
    ) -> impl Future<Output = Result<PasswordHash, BoxError>> + Send;
}
