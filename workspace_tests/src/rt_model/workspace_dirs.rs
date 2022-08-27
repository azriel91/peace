use std::path::PathBuf;

use peace::{
    resources::dir::{PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir},
    rt_model::WorkspaceDirs,
};

#[test]
fn clone() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from(""));
    let peace_dir = PeaceDir::new(PathBuf::from(""));
    let profile_dir = ProfileDir::new(PathBuf::from(""));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from(""));

    let workspace_dirs_0 =
        WorkspaceDirs::new(workspace_dir, peace_dir, profile_dir, profile_history_dir);

    let workspace_dirs_1 = workspace_dirs_0.clone();

    assert_eq!(workspace_dirs_0, workspace_dirs_1);
}

#[test]
fn debug() {
    let workspace_dir = WorkspaceDir::new(PathBuf::from(""));
    let peace_dir = PeaceDir::new(PathBuf::from(""));
    let profile_dir = ProfileDir::new(PathBuf::from(""));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from(""));

    let workspace_dirs =
        WorkspaceDirs::new(workspace_dir, peace_dir, profile_dir, profile_history_dir);

    assert_eq!(
        r#"WorkspaceDirs { workspace_dir: WorkspaceDir(""), peace_dir: PeaceDir(""), profile_dir: ProfileDir(""), profile_history_dir: ProfileHistoryDir("") }"#,
        format!("{workspace_dirs:?}")
    );
}
