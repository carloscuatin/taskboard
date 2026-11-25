use std::fmt;

/// The stored form of a password. The domain only carries it; the adapter that
/// computes it decides the algorithm.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PasswordHash(<redacted>)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_back_hash_text() {
        let hash = PasswordHash::new("fake-hash-value".to_string());

        assert_eq!(hash.as_str(), "fake-hash-value");
    }

    #[test]
    fn debug_hides_hash() {
        let hash = PasswordHash::new("fake-hash-value".to_string());

        assert!(!format!("{hash:?}").contains("fake-hash-value"));
    }
}
