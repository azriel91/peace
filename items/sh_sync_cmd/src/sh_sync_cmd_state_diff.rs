use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between the current and desired file extraction.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ShSyncCmdStateDiff {}

impl fmt::Display for ShSyncCmdStateDiff {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
