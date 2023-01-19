use peace::{
    cfg::{app_name, flow_id, profile, AppName, FlowId, Profile},
    rt_model::{workspace::WorkspaceBuilder, Workspace, WorkspaceSpec},
};

#[test]
fn profile_defaults_to_workspace_init() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::builder(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )
    .build()?;

    assert_eq!(&profile!("workspace_init"), workspace.profile());
    Ok(())
}

#[test]
fn flow_id_defaults_to_workspace_init() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = WorkspaceBuilder::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )
    .build()?;

    assert_eq!(&flow_id!("workspace_init"), workspace.flow_id());
    Ok(())
}

#[test]
fn flow_id_defaults_to_profile_init_when_with_profile() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = WorkspaceBuilder::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )
    .with_profile(profile!("test_profile"))
    .build()?;

    assert_eq!(&flow_id!("profile_init"), workspace.flow_id());
    Ok(())
}

#[test]
fn passes_all_params_to_workspace() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = WorkspaceBuilder::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )
    .with_profile(profile!("test_profile"))
    .with_flow_id(flow_id!("test_flow_id"))
    .build()?;

    assert_eq!(&profile!("test_profile"), workspace.profile());
    assert_eq!(&flow_id!("test_flow_id"), workspace.flow_id());
    Ok(())
}
