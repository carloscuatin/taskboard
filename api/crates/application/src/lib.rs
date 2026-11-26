mod ports;
#[cfg(test)]
mod test_support;

pub use ports::{AddUserError, BoxError, PasswordHasher, UserRepository};

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "taskboard-application");
    }
}
