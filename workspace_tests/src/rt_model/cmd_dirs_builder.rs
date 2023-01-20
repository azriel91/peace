use std::path::Path;

use peace::{
    cfg::{profile, FlowId, Profile},
    resources::paths::PeaceAppDir,
    rt_model::cmd::CmdDirsBuilder,
};

#[test]
fn returns_profile_history_dir_relative_to_profile_dir() -> Result<(), Box<dyn std::error::Error>> {
    let peace_app_dir = PeaceAppDir::new(Path::new("peace_app_dir").to_path_buf());
    let cmd_dirs = CmdDirsBuilder::build(
        &peace_app_dir,
        &profile!("test_profile"),
        &FlowId::new(crate::fn_name_short!())?,
    );

    let profile_history_dir = cmd_dirs.profile_history_dir();

    assert!(
        profile_history_dir.ends_with("peace_app_dir/test_profile/.history"),
        "Expected `{}` to end with `peace_app_dir/test_profile/.history`",
        profile_history_dir.display()
    );
    Ok(())
}

#[test]
fn returns_profile_dir_relative_to_peace_app_dir() -> Result<(), Box<dyn std::error::Error>> {
    let peace_app_dir = PeaceAppDir::new(Path::new("peace_app_dir").to_path_buf());
    let cmd_dirs = CmdDirsBuilder::build(
        &peace_app_dir,
        &profile!("test_profile"),
        &FlowId::new(crate::fn_name_short!())?,
    );

    let profile_dir = cmd_dirs.profile_dir();

    assert!(
        profile_dir.ends_with(Path::new("peace_app_dir/test_profile")),
        "Expected profile directory `{}` to end with `peace_app_dir/test_profile`",
        profile_dir.display()
    );
    Ok(())
}

#[test]
fn returns_flow_dir_relative_to_profile_dir() -> Result<(), Box<dyn std::error::Error>> {
    let peace_app_dir = PeaceAppDir::new(Path::new("peace_app_dir").to_path_buf());
    let cmd_dirs = CmdDirsBuilder::build(
        &peace_app_dir,
        &profile!("test_profile"),
        &FlowId::new(crate::fn_name_short!())?,
    );

    let flow_dir = cmd_dirs.flow_dir();

    assert!(
        flow_dir.ends_with("peace_app_dir/test_profile/returns_flow_dir_relative_to_profile_dir"),
        "Expected `{}` to end with `peace_app_dir/test_profile/returns_flow_dir_relative_to_profile_dir`",
        flow_dir.display()
    );
    Ok(())
}

#[test]
fn debug() {
    let cmd_dirs_builder = CmdDirsBuilder;
    assert_eq!("CmdDirsBuilder", format!("{cmd_dirs_builder:?}"));
}
