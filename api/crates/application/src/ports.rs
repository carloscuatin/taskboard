use taskboard_domain::{Email, Password, PasswordHash, User};

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

    fn find_by_email(
        &self,
        email: &Email,
    ) -> impl Future<Output = Result<Option<User>, BoxError>> + Send;
}

/// The algorithm and its cost belong to the implementation, not to this crate.
pub trait PasswordHasher: Send + Sync {
    fn hash(
        &self,
        password: Password,
    ) -> impl Future<Output = Result<PasswordHash, BoxError>> + Send;
}

/// `Ok(false)` means the password does not match; a stored hash that cannot be read
/// is an `Err`, because that is a fault and not a wrong password.
pub trait PasswordVerifier: Send + Sync {
    fn verify(
        &self,
        password: Password,
        hash: &PasswordHash,
    ) -> impl Future<Output = Result<bool, BoxError>> + Send;
}
