use std::ops::{Deref, DerefMut};

use peace::rt_model::FullSpecGraph;

use crate::DownloadError;

/// Graph of [`FullSpec`]s for the `download` example.
/// `FullSpecGraph<DownloadError>` newtype.
///
/// This is useful for WASM support -- `wasm_bindgen` doesn't currently support
/// exporting types with type parameters, so if the type is to be exported to
/// JS, we have to make a type wrapper with all the parameters specified.
///
/// [`FullSpec`]: peace::cfg::FullSpec
#[derive(Debug)]
#[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]
pub struct DownloadFullSpecGraph(FullSpecGraph<DownloadError>);

impl DownloadFullSpecGraph {
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> FullSpecGraph<DownloadError> {
        self.0
    }
}

impl Deref for DownloadFullSpecGraph {
    type Target = FullSpecGraph<DownloadError>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DownloadFullSpecGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FullSpecGraph<DownloadError>> for DownloadFullSpecGraph {
    fn from(graph: FullSpecGraph<DownloadError>) -> Self {
        Self(graph)
    }
}
