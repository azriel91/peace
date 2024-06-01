use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores steps' states.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/params_specs.yaml`.
///
/// See `ParamsSpecsFile::from<&FlowDir>` if you want to construct a
/// `ParamsSpecsFile` with the conventional `$flow_dir/params_specs.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParamsSpecsFile(PathBuf);

crate::paths::pathbuf_newtype!(ParamsSpecsFile);

impl ParamsSpecsFile {
    /// File name of the states file.
    pub const NAME: &'static str = "params_specs.yaml";
}

impl From<&FlowDir> for ParamsSpecsFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
