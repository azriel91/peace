use std::{fmt, path::PathBuf};

use crate::paths::ProfileDir;

/// Path to the file that stores the profile initialization parameters.
///
/// Typically `$workspace_dir/.peace/$app/$profile/profile_params.yaml`.
///
/// See `ProfileParamsFile::from<&ProfileDir>` if you want to construct a
/// `ProfileParamsFile` with the conventional `$profile_dir/profile_params.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(ProfileParamsFile);

impl ProfileParamsFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "profile_params.yaml";
}

impl From<&ProfileDir> for ProfileParamsFile {
    fn from(profile_dir: &ProfileDir) -> Self {
        let path = profile_dir.join(Self::NAME);

        Self(path)
    }
}

impl fmt::Display for ProfileParamsFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
