use std::fmt;

use serde::{Deserialize, Serialize};

/// Placeholder for physical state to be computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Placeholder;

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "placeholder".fmt(f)
    }
}
