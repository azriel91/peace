use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores item specs' states.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/states_saved.yaml`.
///
/// See `StatesSavedFile::from<&FlowDir>` if you want to construct a
/// `StatesSavedFile` with the conventional `$flow_dir/states_saved.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatesSavedFile(PathBuf);

crate::paths::pathbuf_newtype!(StatesSavedFile);

impl StatesSavedFile {
    /// File name of the states file.
    pub const NAME: &'static str = "states_saved.yaml";
}

impl From<&FlowDir> for StatesSavedFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
