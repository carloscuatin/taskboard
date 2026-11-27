use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use taskboard_domain::{Email, PasswordHash};

use crate::test_support::EventLog;
use crate::{AddUserError, UserRepository};

#[derive(Default)]
pub struct InMemoryUserRepository {
    saved: Mutex<Vec<(Email, PasswordHash)>>,
    attempts: AtomicUsize,
    email_taken: bool,
    failure: Option<String>,
    log: Option<EventLog>,
}

impl InMemoryUserRepository {
    pub fn taking_every_email(mut self) -> Self {
        self.email_taken = true;
        self
    }

    pub fn failing_with(mut self, message: &str) -> Self {
        self.failure = Some(message.to_string());
        self
    }

    pub fn recording_in(mut self, log: &EventLog) -> Self {
        self.log = Some(log.clone());
        self
    }

    pub fn saved(&self) -> Vec<(Email, PasswordHash)> {
        self.saved.lock().expect("saved lock").clone()
    }

    pub fn attempts(&self) -> usize {
        self.attempts.load(Ordering::SeqCst)
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn add(&self, email: &Email, password_hash: &PasswordHash) -> Result<(), AddUserError> {
        self.attempts.fetch_add(1, Ordering::SeqCst);
        if let Some(log) = &self.log {
            log.record("add");
        }

        if let Some(message) = &self.failure {
            return Err(AddUserError::Unexpected(message.clone().into()));
        }

        let mut saved = self.saved.lock().expect("saved lock");
        if self.email_taken || saved.iter().any(|(existing, _)| existing == email) {
            return Err(AddUserError::EmailTaken);
        }

        saved.push((email.clone(), password_hash.clone()));
        Ok(())
    }
}
