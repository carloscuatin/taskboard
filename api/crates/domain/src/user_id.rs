/// Opaque identifier of a user. The store assigns it; the domain never generates one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u128);

impl UserId {
    pub fn from_u128(value: u128) -> Self {
        Self(value)
    }

    pub fn as_u128(&self) -> u128 {
        self.0
    }
}

#[cfg(test)]
mod tests;
