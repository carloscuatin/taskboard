use taskboard_domain::{Email, Password, PasswordHash};

pub fn email(text: &str) -> Email {
    Email::parse(text).expect("a valid email")
}

pub fn password(text: &str) -> Password {
    Password::parse(text.to_string()).expect("a valid password")
}

pub fn hash(text: &str) -> PasswordHash {
    PasswordHash::new(text.to_string())
}
