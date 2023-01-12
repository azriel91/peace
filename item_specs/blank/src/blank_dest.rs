use std::fmt;

use serde::{Deserialize, Serialize};

/// Destination for blank state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BlankDest(pub Option<u32>);

impl fmt::Display for BlankDest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(n) => n.fmt(f),
            None => "<none>".fmt(f),
        }
    }
}

impl std::ops::Deref for BlankDest {
    type Target = Option<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BlankDest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
