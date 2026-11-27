use crate::{Email, PasswordHash, UserId};

/// An account holder: who they are and how they prove it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
}

impl User {
    pub fn new(id: UserId, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id,
            email,
            password_hash,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
}

#[cfg(test)]
mod tests;
