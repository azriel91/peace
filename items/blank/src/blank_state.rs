use std::fmt;

use serde::{Deserialize, Serialize};

/// Logical blank state.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BlankState(pub Option<u32>);

impl fmt::Display for BlankState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(n) => n.fmt(f),
            None => "<none>".fmt(f),
        }
    }
}

impl std::ops::Deref for BlankState {
    type Target = Option<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BlankState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
