mod login_user;
mod ports;
mod register_user;

#[cfg(test)]
mod test_support;

pub use login_user::{LoginUser, LoginUserError};
pub use ports::{AddUserError, BoxError, PasswordHasher, PasswordVerifier, UserRepository};
pub use register_user::{RegisterUser, RegisterUserError};

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "taskboard-application");
    }
}
