use std::{fmt, path::PathBuf};

use crate::paths::FlowDir;

/// Path to the file that stores the flow initialization parameters.
///
/// Typically `$workspace_dir/.peace/$app/$profile/$flow_id/flow_params.yaml`.
///
/// See `FlowParamsFile::from<&FlowDir>` if you want to construct a
/// `FlowParamsFile` with the conventional `$flow_dir/flow_params.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(FlowParamsFile);

impl FlowParamsFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "flow_params.yaml";
}

impl From<&FlowDir> for FlowParamsFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}

impl fmt::Display for FlowParamsFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
