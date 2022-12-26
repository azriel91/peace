use std::path::PathBuf;

use crate::paths::ProfileDir;

/// Path to the file that stores the profile initialization parameters.
///
/// Typically `$workspace_dir/.peace/$profile/init.yaml`.
///
/// See `ProfileParamsFile::from<&ProfileDir>` if you want to construct a
/// `ProfileParamsFile` with the conventional `$profile_dir/init.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(ProfileParamsFile);

impl ProfileParamsFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "init.yaml";
}

impl From<&ProfileDir> for ProfileParamsFile {
    fn from(flow_dir: &ProfileDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
