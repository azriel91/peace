use std::ops::{Deref, DerefMut};

use peace::rt_model::ItemSpecGraph;

use crate::FileDownloadError;

/// Graph of [`ItemSpec`]s for the `download` example.
/// `ItemSpecGraph<FileDownloadError>` newtype.
///
/// This is useful for WASM support -- `wasm_bindgen` doesn't currently support
/// exporting types with type parameters, so if the type is to be exported to
/// JS, we have to make a type wrapper with all the parameters specified.
///
/// [`ItemSpec`]: peace::cfg::ItemSpec
#[derive(Debug)]
#[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]
pub struct FileDownloadItemSpecGraph(ItemSpecGraph<FileDownloadError>);

impl FileDownloadItemSpecGraph {
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> ItemSpecGraph<FileDownloadError> {
        self.0
    }
}

impl Deref for FileDownloadItemSpecGraph {
    type Target = ItemSpecGraph<FileDownloadError>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FileDownloadItemSpecGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ItemSpecGraph<FileDownloadError>> for FileDownloadItemSpecGraph {
    fn from(graph: ItemSpecGraph<FileDownloadError>) -> Self {
        Self(graph)
    }
}
