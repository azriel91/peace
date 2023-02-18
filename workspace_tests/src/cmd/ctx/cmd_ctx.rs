use peace::{
    cfg::{app_name, flow_id, profile, AppName, FlowId, Profile},
    cmd::{ctx::CmdCtxBuilder, scopes::SingleProfileSingleFlow},
    resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    rt_model::Workspace,
};

#[tokio::test]
async fn single_profile_single_flow_getters() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow_id(flow_id.clone())
        .build()
        .await?;

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

        SingleProfileSingleFlow::new(profile, profile_dir, profile_history_dir, flow_id, flow_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    assert_eq!(workspace.dirs().workspace_dir(), cmd_ctx.workspace_dir());
    assert_eq!(workspace.dirs().peace_dir(), cmd_ctx.peace_dir());
    assert_eq!(workspace.dirs().peace_app_dir(), cmd_ctx.peace_app_dir());
    assert_eq!(scope.profile(), cmd_ctx.profile());
    assert_eq!(scope.profile_dir(), cmd_ctx.profile_dir());
    assert_eq!(scope.flow_id(), cmd_ctx.flow_id());
    assert_eq!(scope.flow_dir(), cmd_ctx.flow_dir());
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
