use std::path::PathBuf;

use peace::resource_rt::{
    internal::WorkspaceDirs,
    paths::{PeaceAppDir, PeaceDir, WorkspaceDir},
};

#[test]
fn into_inner() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from("workspace_dir"));
    let peace_dir = PeaceDir::new(PathBuf::from("peace_dir"));
    let peace_app_dir = PeaceAppDir::new(PathBuf::from("peace_app_dir"));

    let workspace_dirs = WorkspaceDirs::new(
        workspace_dir.clone(),
        peace_dir.clone(),
        peace_app_dir.clone(),
    );

    let (workspace_dir_inner, peace_dir_inner, peace_app_dir_inner) = workspace_dirs.into_inner();

    assert_eq!(workspace_dir, workspace_dir_inner);
    assert_eq!(peace_dir, peace_dir_inner);
    assert_eq!(peace_app_dir, peace_app_dir_inner);
}

#[test]
fn clone() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from("workspace_dir"));
    let peace_dir = PeaceDir::new(PathBuf::from("peace_dir"));
    let peace_app_dir = PeaceAppDir::new(PathBuf::from("peace_app_dir"));

    let workspace_dirs_0 = WorkspaceDirs::new(workspace_dir, peace_dir, peace_app_dir);

    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let workspace_dirs_1 = workspace_dirs_0.clone();

    assert_eq!(workspace_dirs_0, workspace_dirs_1);
}

#[test]
fn debug() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from("workspace_dir"));
    let peace_dir = PeaceDir::new(PathBuf::from("peace_dir"));
    let peace_app_dir = PeaceAppDir::new(PathBuf::from("peace_app_dir"));

    let workspace_dirs = WorkspaceDirs::new(workspace_dir, peace_dir, peace_app_dir);

    assert_eq!(
        "WorkspaceDirs { \
            workspace_dir: WorkspaceDir(\"workspace_dir\"), \
            peace_dir: PeaceDir(\"peace_dir\"), \
            peace_app_dir: PeaceAppDir(\"peace_app_dir\") \
        }",
        format!("{workspace_dirs:?}")
    );
}
