use std::fmt;

use serde::{Deserialize, Serialize};

/// State of the contents of the tar to extract.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TarXState {}

impl fmt::Display for TarXState {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
