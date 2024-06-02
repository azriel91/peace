use std::path::PathBuf;

use crate::paths::ProfileDir;

/// Directory to store all data produced by the current profile's execution.
///
/// Typically `$workspace_dir/.peace/$app/$profile/.history`.
///
/// This directory is intended to contain significant command execution
/// summaries. Currently it is not written to yet.
///
/// See `ProfileHistoryDir::from<&ProfileDir>` if you want to construct a
/// `ProfileHistoryDir` with the conventional `$profile_dir/.history` path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileHistoryDir(PathBuf);

crate::paths::pathbuf_newtype!(ProfileHistoryDir);

impl From<&ProfileDir> for ProfileHistoryDir {
    fn from(profile_dir: &ProfileDir) -> Self {
        let path = profile_dir.join(".history");

        Self(path)
    }
}
