use std::fmt;

use serde::{Deserialize, Serialize};

/// Represent no data.
///
/// This is used to represent no separate physical state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nothing;

impl fmt::Display for Nothing {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
