use std::path::PathBuf;

/// Base directory of the workspace.
///
/// Given a workspace lives in `workspace_dir`, it is natural for users to
/// execute a `peace` tool in any sub directory of `workspace_dir`, in which
/// case execution should be consistent with invocations in `workspace_dir`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceDir(PathBuf);

crate::paths::pathbuf_newtype!(WorkspaceDir);
