use std::path::PathBuf;

use crate::dir::ProfileDir;

/// Directory to store all data produced by the current profile's execution.
///
/// Typically `$workspace_dir/.peace/$profile/.history`.
///
/// This directory contains significant command execution summaries.
///
/// See `ProfileHistoryDir::from<&ProfileDir>` if you want to
/// construct a `ProfileHistoryDir` with the default
/// `$peace_dir/.peace/$profile/.history` name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileHistoryDir(PathBuf);

crate::dir::pathbuf_newtype!(ProfileHistoryDir);

impl From<&ProfileDir> for ProfileHistoryDir {
    fn from(profile_dir: &ProfileDir) -> Self {
        let mut path = profile_dir.to_path_buf();
        path.push(".history");

        Self(path)
    }
}
