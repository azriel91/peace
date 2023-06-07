use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores items' states.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/states_goal.yaml`.
///
/// See `StatesGoalFile::from<&FlowDir>` if you want to construct a
/// `StatesGoalFile` with the conventional `$flow_dir/states_goal.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatesGoalFile(PathBuf);

crate::paths::pathbuf_newtype!(StatesGoalFile);

impl StatesGoalFile {
    /// File name of the goal states file.
    pub const NAME: &'static str = "states_goal.yaml";
}

impl From<&FlowDir> for StatesGoalFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
