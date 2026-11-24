use super::{Email, EmailError};

pub(super) fn check_characters(text: &str) -> Result<(), EmailError> {
    if text.is_empty() {
        return Err(EmailError::Empty);
    }

    if !text.is_ascii() {
        return Err(EmailError::NonAscii);
    }

    if text
        .chars()
        .any(|c| c.is_whitespace() || c.is_ascii_control())
    {
        return Err(EmailError::InvalidCharacter);
    }

    Ok(())
}

pub(super) fn check_length(text: &str) -> Result<(), EmailError> {
    if text.len() > Email::MAX_LEN {
        return Err(EmailError::TooLong);
    }

    Ok(())
}

pub(super) fn split_at_sign(text: &str) -> Result<(&str, &str), EmailError> {
    let Some((local, domain)) = text.split_once('@') else {
        return Err(EmailError::MissingAt);
    };

    if domain.contains('@') {
        return Err(EmailError::MultipleAt);
    }

    Ok((local, domain))
}

pub(super) fn check_local_part(local: &str) -> Result<(), EmailError> {
    if local.len() > Email::MAX_LOCAL_LEN {
        return Err(EmailError::LocalPartTooLong);
    }

    if local.is_empty() {
        return Err(EmailError::EmptyLocalPart);
    }

    Ok(())
}

pub(super) fn check_domain(domain: &str) -> Result<(), EmailError> {
    if domain.is_empty() {
        return Err(EmailError::EmptyDomain);
    }

    if !domain.contains('.') {
        return Err(EmailError::DomainWithoutDot);
    }

    if domain.starts_with('.') || domain.ends_with('.') {
        return Err(EmailError::DomainDotAtEdge);
    }

    if domain.contains("..") {
        return Err(EmailError::DomainConsecutiveDots);
    }

    Ok(())
}
