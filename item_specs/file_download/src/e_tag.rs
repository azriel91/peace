use serde::{Deserialize, Serialize};

/// Represents an ETag returned from a server.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ETag(String);

impl ETag {
    /// Returns a new `ETag`.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

impl std::ops::Deref for ETag {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ETag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for ETag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
