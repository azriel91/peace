use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores items' parameter values.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/item_params.yaml`.
///
/// See `ItemParamsFile::from<&FlowDir>` if you want to construct an
/// `ItemParamsFile` with the conventional `$flow_dir/item_params.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(ItemParamsFile);

impl ItemParamsFile {
    /// File name of the states file.
    pub const NAME: &'static str = "item_params.yaml";
}

impl From<&FlowDir> for ItemParamsFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
