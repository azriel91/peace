use peace::{
    cfg::{app_name, profile, AppName},
    cmd_ctx::{CmdCtxSpsf, ProfileSelection},
    flow_model::flow_id,
    flow_rt::{Flow, ItemGraphBuilder},
    resource_rt::paths::{FlowDir, ProfileDir},
    rt_model::Workspace,
};

use crate::{no_op_output::NoOpOutput, peace_cmd_ctx_types::TestCctNoOpOutput, PeaceTestError};

#[tokio::test]
async fn single_profile_single_flow_getters() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemGraphBuilder::new().build());

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .await?;

    let fields = cmd_ctx.fields();
    let profile_dir = ProfileDir::from((workspace.dirs().peace_app_dir(), &profile));
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(workspace.dirs().workspace_dir(), fields.workspace_dir());
    assert_eq!(workspace.dirs().peace_dir(), fields.peace_dir());
    assert_eq!(workspace.dirs().peace_app_dir(), fields.peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&flow_id, fields.flow().flow_id());
    assert_eq!(&FlowDir::from((&profile_dir, &flow_id)), fields.flow_dir());
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
