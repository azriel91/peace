use std::path::PathBuf;

use peace_core::Profile;

use crate::paths::PeaceDir;

/// Directory to store all data produced by the current profile's execution.
///
/// Typically `$workspace_dir/.peace/$profile`.
///
/// This is the directory that contains all information produced and used during
/// a `peace` tool invocation. Exceptions include authentication information
/// stored in their respective directories on the file system, such as
/// application credentials stored in `~/${app}/credentials`.
///
/// See `ProfileDir::from<(&PeaceDir, &Profile)>` if you want to
/// construct a `ProfileDir` with the default `$peace_dir/.peace/$profile` name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileDir(PathBuf);

crate::paths::pathbuf_newtype!(ProfileDir);

impl From<(&PeaceDir, &Profile)> for ProfileDir {
    fn from((peace_dir, profile): (&PeaceDir, &Profile)) -> Self {
        let mut path = peace_dir.to_path_buf();
        path.push(profile.as_ref());

        Self(path)
    }
}
