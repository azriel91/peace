use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores the flow initialization parameters.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/init.yaml`.
///
/// See `FlowParamsFile::from<&FlowDir>` if you want to construct a
/// `FlowParamsFile` with the conventional `$flow_dir/init.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(FlowParamsFile);

impl FlowParamsFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "init.yaml";
}

impl From<&FlowDir> for FlowParamsFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
