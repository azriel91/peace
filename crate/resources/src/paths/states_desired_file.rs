use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores item specs' states.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/states_desired.yaml`.
///
/// See `StatesDesiredFile::from<&FlowDir>` if you want to construct a
/// `StatesDesiredFile` with the conventional `$flow_dir/states_desired.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatesDesiredFile(PathBuf);

crate::paths::pathbuf_newtype!(StatesDesiredFile);

impl StatesDesiredFile {
    /// File name of the desired states file.
    pub const NAME: &'static str = "states_desired.yaml";
}

impl From<&FlowDir> for StatesDesiredFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
