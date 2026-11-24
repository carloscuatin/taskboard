mod email;

pub use email::Email;
pub use email::EmailError;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "taskboard-domain");
    }
}
