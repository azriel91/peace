use peace::params::Params;
use serde::{Deserialize, Serialize};

/// Source for blank state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Params)]
pub struct BlankSrc(pub u32);

impl std::ops::Deref for BlankSrc {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BlankSrc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
