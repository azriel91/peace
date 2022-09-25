use std::path::PathBuf;

use crate::paths::ProfileDir;

/// Path to the file that stores the profile initialization parameters.
///
/// Typically `$workspace_dir/.peace/$profile/init.yaml`.
///
/// See `ProfileInitFile::from<&ProfileDir>` if you want to construct a
/// `ProfileInitFile` with the conventional `$profile_dir/init.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileInitFile(PathBuf);

crate::paths::pathbuf_newtype!(ProfileInitFile);

impl ProfileInitFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "init.yaml";
}

impl From<&ProfileDir> for ProfileInitFile {
    fn from(flow_dir: &ProfileDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
