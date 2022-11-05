use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between the current and desired file extraction.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TarXStateDiff {}

impl fmt::Display for TarXStateDiff {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
