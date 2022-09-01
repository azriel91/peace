use std::path::PathBuf;

use peace_core::FlowId;

use crate::dir::ProfileDir;

/// Directory to store all data produced by the current flow's execution.
///
/// Typically `$workspace_dir/.peace/$profile/$flow_id`.
///
/// This is the directory that contains all information produced and used during
/// a `peace` tool invocation. Exceptions include authentication information
/// stored in their respective directories on the file system, such as
/// application credentials stored in `~/${app}/credentials`.
///
/// See `FlowDir::from<(&ProfileDir, &FlowId)>` if you want to
/// construct a `FlowDir` with the default `$peace_dir/.peace/$profile/$flow_id`
/// name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowDir(PathBuf);

crate::dir::pathbuf_newtype!(FlowDir);

impl From<(&ProfileDir, &FlowId)> for FlowDir {
    fn from((peace_dir, flow_id): (&ProfileDir, &FlowId)) -> Self {
        let mut path = peace_dir.to_path_buf();
        path.push(flow_id.as_ref());

        Self(path)
    }
}
