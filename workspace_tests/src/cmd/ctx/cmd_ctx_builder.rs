use peace::{
    cfg::{app_name, flow_id, profile, AppName, FlowId, Profile},
    cmd::{
        ctx::CmdCtxBuilder,
        scopes::{NoProfileNoFlow, SingleProfileSingleFlow},
    },
    resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    rt_model::Workspace,
};

#[test]
fn builds_no_profile_no_flow() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_no_profile_no_flow"))?;

    let cmd_ctx_builder = CmdCtxBuilder::<NoProfileNoFlow>::new(&workspace);
    let cmd_ctx = cmd_ctx_builder.build();

    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&NoProfileNoFlow, cmd_ctx.scope());
    Ok(())
}

#[test]
fn builds_single_profile_single_flow() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow_id(flow_id.clone())
        .build();

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

        SingleProfileSingleFlow::new(profile, profile_dir, profile_history_dir, flow_id, flow_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    Ok(())
}

fn workspace(
    tempdir: tempfile::TempDir,
    app_name: AppName,
) -> Result<Workspace, Box<dyn std::error::Error>> {
    let workspace = {
        let workspace_spec = peace::rt_model::WorkspaceSpec::Path(tempdir.path().to_path_buf());
        Workspace::new(app_name, workspace_spec)?
    };
    Ok(workspace)
}
