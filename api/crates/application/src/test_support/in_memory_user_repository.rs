use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use taskboard_domain::{Email, PasswordHash, User, UserId};

use crate::test_support::EventLog;
use crate::{AddUserError, BoxError, UserRepository};

#[derive(Default)]
pub struct InMemoryUserRepository {
    users: Mutex<Vec<User>>,
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
        self.users()
            .into_iter()
            .map(|user| (user.email().clone(), user.password_hash().clone()))
            .collect()
    }

    pub fn users(&self) -> Vec<User> {
        self.users.lock().expect("users lock").clone()
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

        let mut users = self.users.lock().expect("users lock");
        if self.email_taken || users.iter().any(|user| user.email() == email) {
            return Err(AddUserError::EmailTaken);
        }

        let id = UserId::from_u128(users.len() as u128 + 1);
        users.push(User::new(id, email.clone(), password_hash.clone()));
        Ok(())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, BoxError> {
        if let Some(log) = &self.log {
            log.record("find");
        }

        if let Some(message) = &self.failure {
            return Err(message.clone().into());
        }

        let users = self.users.lock().expect("users lock");
        Ok(users.iter().find(|user| user.email() == email).cloned())
    }
}
