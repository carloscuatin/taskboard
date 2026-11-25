mod email;
mod password;

pub use email::Email;
pub use email::EmailError;

pub use password::Password;
pub use password::PasswordError;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "taskboard-domain");
    }
}
