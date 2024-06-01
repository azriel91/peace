use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores steps' states.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/states_current.yaml`.
///
/// See `StatesCurrentFile::from<&FlowDir>` if you want to construct a
/// `StatesCurrentFile` with the conventional `$flow_dir/states_current.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatesCurrentFile(PathBuf);

crate::paths::pathbuf_newtype!(StatesCurrentFile);

impl StatesCurrentFile {
    /// File name of the states file.
    pub const NAME: &'static str = "states_current.yaml";
}

impl From<&FlowDir> for StatesCurrentFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
