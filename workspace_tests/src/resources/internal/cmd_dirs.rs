use std::path::PathBuf;

use peace::resources::{
    internal::CmdDirs,
    paths::{FlowDir, ProfileDir, ProfileHistoryDir},
};

#[test]
fn into_inner() {
    let profile_dir = ProfileDir::new(PathBuf::from("profile_dir"));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from("profile_history_dir"));
    let flow_dir = FlowDir::new(PathBuf::from("flow_dir"));

    let cmd_dirs = CmdDirs::new(
        profile_dir.clone(),
        profile_history_dir.clone(),
        flow_dir.clone(),
    );

    let (profile_dir_inner, profile_history_dir_inner, flow_dir_inner) = cmd_dirs.into_inner();

    assert_eq!(profile_dir, profile_dir_inner);
    assert_eq!(profile_history_dir, profile_history_dir_inner);
    assert_eq!(flow_dir, flow_dir_inner);
}

#[test]
fn clone() {
    let profile_dir = ProfileDir::new(PathBuf::from("profile_dir"));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from("profile_history_dir"));
    let flow_dir = FlowDir::new(PathBuf::from("flow_dir"));

    let cmd_dirs_0 = CmdDirs::new(profile_dir, profile_history_dir, flow_dir);

    let cmd_dirs_1 = cmd_dirs_0.clone();

    assert_eq!(cmd_dirs_0, cmd_dirs_1);
}

#[test]
fn debug() {
    let profile_dir = ProfileDir::new(PathBuf::from("profile_dir"));
    let profile_history_dir = ProfileHistoryDir::new(PathBuf::from("profile_history_dir"));
    let flow_dir = FlowDir::new(PathBuf::from("flow_dir"));

    let cmd_dirs = CmdDirs::new(profile_dir, profile_history_dir, flow_dir);

    assert_eq!(
        "CmdDirs { \
            profile_dir: ProfileDir(\"profile_dir\"), \
            profile_history_dir: ProfileHistoryDir(\"profile_history_dir\"), \
            flow_dir: FlowDir(\"flow_dir\") \
        }",
        format!("{cmd_dirs:?}")
    );
}
