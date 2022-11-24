use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between the current and desired file extraction.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShCmdStateDiff(String);

impl From<String> for ShCmdStateDiff {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl std::ops::Deref for ShCmdStateDiff {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ShCmdStateDiff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for ShCmdStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
