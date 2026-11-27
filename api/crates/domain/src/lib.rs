mod email;
mod password;
mod password_hash;
mod user;
mod user_id;

pub use email::Email;
pub use email::EmailError;

pub use password::Password;
pub use password::PasswordError;
pub use password_hash::PasswordHash;

pub use user::User;
pub use user_id::UserId;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "taskboard-domain");
    }
}
