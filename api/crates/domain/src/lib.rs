mod email;
mod password;
mod password_hash;

pub use email::Email;
pub use email::EmailError;

pub use password::Password;
pub use password::PasswordError;
pub use password_hash::PasswordHash;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "taskboard-domain");
    }
}
