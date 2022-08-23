use std::{
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
};

use peace_core::Profile;

use crate::dir::PeaceDir;

/// Directory to store all data produced by the current profile's execution.
///
/// Typically `$workspace_dir/.peace/$profile`.
///
/// This is the directory that contains all information produced and used during
/// a `peace` tool invocation. Exceptions include authentication information
/// stored in their respective directories on the file system, such as
/// application credentials stored in `~/${app}/credentials`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileDir(PathBuf);

impl ProfileDir {
    /// Returns a new [`ProfileDir`].
    ///
    /// See `ProfileDir::from<(&PeaceDir, &Profile)>` if you want to
    /// construct a `ProfileDir` with the default `$peace_dir/.peace` name.
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    /// Returns the inner [`PathBuf`].
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl From<(&PeaceDir, &Profile)> for ProfileDir {
    fn from((peace_dir, profile): (&PeaceDir, &Profile)) -> Self {
        let mut path = peace_dir.to_path_buf();
        path.push(profile.as_ref());

        Self(path)
    }
}

impl From<PathBuf> for ProfileDir {
    fn from(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }
}

impl AsRef<OsStr> for ProfileDir {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl AsRef<Path> for ProfileDir {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for ProfileDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
