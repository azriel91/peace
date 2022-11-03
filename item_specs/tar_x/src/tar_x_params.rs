use std::{
    fmt,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

// TODO: params for:
//
// * keep or remove unknown files
// * force re-extraction
/// Tar extraction parameters.
///
/// The `Id` type parameter is needed for each tar extraction params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different tar extraction
///   parameters from each other.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TarXParams<Id> {
    /// Path of the file to extract.
    dest: PathBuf,
    /// Marker for unique tar extraction parameters type.
    marker: PhantomData<Id>,
}

impl<Id> TarXParams<Id> {
    /// Returns new `TarXParams`.
    pub fn new(dest: PathBuf) -> Self {
        Self {
            dest,
            marker: PhantomData,
        }
    }

    /// Returns the file path to write to.
    pub fn dest(&self) -> &Path {
        &self.dest
    }
}

impl<Id> fmt::Debug for TarXParams<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TarXParams")
            .field("dest", &self.dest)
            .finish()
    }
}
