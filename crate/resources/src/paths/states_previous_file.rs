use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores item specs' states.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/states_previous.yaml`.
///
/// See `StatesPreviousFile::from<&FlowDir>` if you want to construct a
/// `StatesPreviousFile` with the conventional `$flow_dir/states_previous.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatesPreviousFile(PathBuf);

crate::paths::pathbuf_newtype!(StatesPreviousFile);

impl StatesPreviousFile {
    /// File name of the states file.
    pub const NAME: &'static str = "states_previous.yaml";
}

impl From<&FlowDir> for StatesPreviousFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
