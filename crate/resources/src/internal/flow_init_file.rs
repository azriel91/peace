use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores the flow initialization parameters.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/init.yaml`.
///
/// See `FlowInitFile::from<&FlowDir>` if you want to construct a
/// `FlowInitFile` with the conventional `$flow_dir/init.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowInitFile(PathBuf);

crate::paths::pathbuf_newtype!(FlowInitFile);

impl FlowInitFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "init.yaml";
}

impl From<&FlowDir> for FlowInitFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
