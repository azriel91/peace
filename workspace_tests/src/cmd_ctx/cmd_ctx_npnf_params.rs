use peace::{
    cfg::{app_name, profile},
    cmd_ctx::CmdCtxNpnf,
};

use crate::{
    no_op_output::NoOpOutput, peace_cmd_ctx_types::TestCctNoOpOutput, test_support::workspace,
};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_no_profile_no_flow"))?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxNpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .await?;

    assert!(std::ptr::eq(&workspace, cmd_ctx.fields().workspace()));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_no_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxNpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}
