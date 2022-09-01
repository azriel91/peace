use std::path::PathBuf;

use peace::resources::{
    dir::{FlowDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir},
    internal::WorkspaceDirs,
};

#[test]
fn into_inner() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from(""));
    let peace_dir = PeaceDir::new(PathBuf::from(""));
    let profile_dir = ProfileDir::new(PathBuf::from(""));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from(""));
    let flow_dir = FlowDir::new(PathBuf::from(""));

    let workspace_dirs = WorkspaceDirs::new(
        workspace_dir.clone(),
        peace_dir.clone(),
        profile_dir.clone(),
        profile_history_dir.clone(),
        flow_dir.clone(),
    );

    let (
        workspace_dir_inner,
        peace_dir_inner,
        profile_dir_inner,
        profile_history_dir_inner,
        flow_dir_inner,
    ) = workspace_dirs.into_inner();

    assert_eq!(workspace_dir, workspace_dir_inner);
    assert_eq!(peace_dir, peace_dir_inner);
    assert_eq!(profile_dir, profile_dir_inner);
    assert_eq!(profile_history_dir, profile_history_dir_inner);
    assert_eq!(flow_dir, flow_dir_inner);
}

#[test]
fn clone() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from(""));
    let peace_dir = PeaceDir::new(PathBuf::from(""));
    let profile_dir = ProfileDir::new(PathBuf::from(""));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from(""));
    let flow_dir = FlowDir::new(PathBuf::from(""));

    let workspace_dirs_0 = WorkspaceDirs::new(
        workspace_dir,
        peace_dir,
        profile_dir,
        profile_history_dir,
        flow_dir,
    );

    let workspace_dirs_1 = workspace_dirs_0.clone();

    assert_eq!(workspace_dirs_0, workspace_dirs_1);
}

#[test]
fn debug() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from(""));
    let peace_dir = PeaceDir::new(PathBuf::from(""));
    let profile_dir = ProfileDir::new(PathBuf::from(""));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from(""));
    let flow_dir = FlowDir::new(PathBuf::from(""));

    let workspace_dirs = WorkspaceDirs::new(
        workspace_dir,
        peace_dir,
        profile_dir,
        profile_history_dir,
        flow_dir,
    );

    assert_eq!(
        "WorkspaceDirs { \
            workspace_dir: WorkspaceDir(\"\"), \
            peace_dir: PeaceDir(\"\"), \
            profile_dir: ProfileDir(\"\"), \
            profile_history_dir: ProfileHistoryDir(\"\"), \
            flow_dir: FlowDir(\"\") \
        }",
        format!("{workspace_dirs:?}")
    );
}
