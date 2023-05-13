use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use derivative::Derivative;
use peace::params::Value;
use serde::{Deserialize, Serialize};

/// Tar extraction parameters.
///
/// The `Id` type parameter is needed for each tar extraction params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different tar extraction
///   parameters from each other.
// TODO: params for:
//
// * keep or remove unknown files
// * force re-extraction
#[derive(Derivative, Value, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
#[serde(bound = "")]
pub struct TarXParams<Id> {
    /// Path of the tar file to extract.
    tar_path: PathBuf,
    /// Directory path to extract the tar file to.
    dest: PathBuf,
    /// Marker for unique tar extraction parameters type.
    marker: PhantomData<Id>,
}

impl<Id> TarXParams<Id> {
    /// Returns new `TarXParams`.
    pub fn new(tar_path: PathBuf, dest: PathBuf) -> Self {
        Self {
            tar_path,
            dest,
            marker: PhantomData,
        }
    }

    /// Returns the path of the tar file to extract.
    pub fn tar_path(&self) -> &Path {
        &self.tar_path
    }

    /// Returns the directory path to extract the tar file to.
    pub fn dest(&self) -> &Path {
        &self.dest
    }
}
