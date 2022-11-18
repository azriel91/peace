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
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShCmdParams<Id> {
    /// Path of the file to extract.
    dest: PathBuf,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> ShCmdParams<Id> {
    /// Returns new `ShCmdParams`.
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

impl<Id> fmt::Debug for ShCmdParams<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShCmdParams")
            .field("dest", &self.dest)
            .finish()
    }
}
