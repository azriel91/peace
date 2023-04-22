use std::path::PathBuf;

use crate::paths::FlowDir;

/// Path to the file that stores item specs' states.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id/item_spec_params.yaml`.
///
/// See `ItemSpecParamsFile::from<&FlowDir>` if you want to construct a
/// `ItemSpecParamsFile` with the conventional `$flow_dir/item_spec_params.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemSpecParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(ItemSpecParamsFile);

impl ItemSpecParamsFile {
    /// File name of the states file.
    pub const NAME: &'static str = "item_spec_params.yaml";
}

impl From<&FlowDir> for ItemSpecParamsFile {
    fn from(flow_dir: &FlowDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
