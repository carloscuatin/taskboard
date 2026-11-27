use std::sync::Mutex;

use taskboard_domain::{Password, PasswordHash};

use crate::test_support::fake_hasher::fake_hash;
use crate::{BoxError, PasswordVerifier};

#[derive(Default)]
pub struct FakeVerifier {
    received: Mutex<Vec<(String, PasswordHash)>>,
    matches_everything: bool,
    failure: Option<String>,
}

impl FakeVerifier {
    pub fn matching_every_hash(mut self) -> Self {
        self.matches_everything = true;
        self
    }

    pub fn failing_with(mut self, message: &str) -> Self {
        self.failure = Some(message.to_string());
        self
    }

    pub fn calls(&self) -> usize {
        self.received.lock().expect("received lock").len()
    }

    pub fn received(&self) -> Vec<(String, PasswordHash)> {
        self.received.lock().expect("received lock").clone()
    }
}

impl PasswordVerifier for FakeVerifier {
    async fn verify(&self, password: Password, hash: &PasswordHash) -> Result<bool, BoxError> {
        self.received
            .lock()
            .expect("received lock")
            .push((password.expose().to_string(), hash.clone()));

        if let Some(message) = &self.failure {
            return Err(message.clone().into());
        }

        Ok(self.matches_everything || fake_hash(password.expose()) == *hash)
    }
}
