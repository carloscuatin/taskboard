use std::sync::Mutex;

use taskboard_domain::{Password, PasswordHash};

use crate::{BoxError, PasswordHasher};

#[derive(Default)]
pub struct FakeHasher {
    received: Mutex<Vec<String>>,
    failure: Option<String>,
}

impl FakeHasher {
    pub fn failing_with(mut self, message: &str) -> Self {
        self.failure = Some(message.to_string());
        self
    }

    pub fn calls(&self) -> usize {
        self.received.lock().expect("received lock").len()
    }

    pub fn received(&self) -> Vec<String> {
        self.received.lock().expect("received lock").clone()
    }
}

impl PasswordHasher for FakeHasher {
    async fn hash(&self, password: Password) -> Result<PasswordHash, BoxError> {
        self.received
            .lock()
            .expect("received lock")
            .push(password.expose().to_string());

        if let Some(message) = &self.failure {
            return Err(message.clone().into());
        }

        Ok(PasswordHash::new("fake-hash-value".to_string()))
    }
}
