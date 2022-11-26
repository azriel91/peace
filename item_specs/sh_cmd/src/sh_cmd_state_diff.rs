use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between the current and desired file extraction.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShCmdStateDiff {
    /// stdout output.
    stdout: String,
    /// stderr output.
    stderr: String,
}

impl ShCmdStateDiff {
    /// Returns a new `ShCmdStateDiff`.
    pub fn new(stdout: String, stderr: String) -> Self {
        Self { stdout, stderr }
    }
}

impl std::ops::Deref for ShCmdStateDiff {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.stdout
    }
}

impl std::ops::DerefMut for ShCmdStateDiff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stdout
    }
}

impl fmt::Display for ShCmdStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.stderr.fmt(f)
    }
}
