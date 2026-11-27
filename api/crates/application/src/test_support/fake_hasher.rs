use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use taskboard_domain::{Password, PasswordHash};

use crate::test_support::EventLog;
use crate::{BoxError, PasswordHasher};

#[derive(Default)]
pub struct FakeHasher {
    received: Mutex<Vec<String>>,
    failure: Option<String>,
    log: Option<EventLog>,
}

impl FakeHasher {
    pub fn failing_with(mut self, message: &str) -> Self {
        self.failure = Some(message.to_string());
        self
    }

    pub fn recording_in(mut self, log: &EventLog) -> Self {
        self.log = Some(log.clone());
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
        if let Some(log) = &self.log {
            log.record("hash");
        }
        self.received
            .lock()
            .expect("received lock")
            .push(password.expose().to_string());

        if let Some(message) = &self.failure {
            return Err(message.clone().into());
        }

        Ok(fake_hash(password.expose()))
    }
}

/// The hash the fake hasher gives a password, so the fake verifier can check against it.
pub fn fake_hash(text: &str) -> PasswordHash {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    PasswordHash::new(format!("fake-hash-{:x}", hasher.finish()))
}
