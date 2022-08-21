use std::ops::{Deref, DerefMut};

use peace::rt_model::ItemSpecGraph;

use crate::DownloadError;

/// Graph of [`ItemSpec`]s for the `download` example.
/// `ItemSpecGraph<DownloadError>` newtype.
///
/// This is useful for WASM support -- `wasm_bindgen` doesn't currently support
/// exporting types with type parameters, so if the type is to be exported to
/// JS, we have to make a type wrapper with all the parameters specified.
///
/// [`ItemSpec`]: peace::cfg::ItemSpec
#[derive(Debug)]
#[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]
pub struct DownloadItemSpecGraph(ItemSpecGraph<DownloadError>);

impl DownloadItemSpecGraph {
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> ItemSpecGraph<DownloadError> {
        self.0
    }
}

impl Deref for DownloadItemSpecGraph {
    type Target = ItemSpecGraph<DownloadError>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DownloadItemSpecGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ItemSpecGraph<DownloadError>> for DownloadItemSpecGraph {
    fn from(graph: ItemSpecGraph<DownloadError>) -> Self {
        Self(graph)
    }
}
