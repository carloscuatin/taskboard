#[derive(Debug, PartialEq)]
pub enum PasswordError {
    TooShort,
    TooLong,
}
