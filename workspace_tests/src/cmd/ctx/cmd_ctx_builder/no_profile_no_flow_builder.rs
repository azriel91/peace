use peace::{
    cfg::{app_name, profile, AppName, Profile},
    cmd::ctx::CmdCtx,
};

use crate::{no_op_output::NoOpOutput, test_support::workspace, PeaceTestError};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_no_profile_no_flow"))?;

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_no_profile_no_flow::<PeaceTestError, _>(&mut output, &workspace)
        .build()
        .await?;

    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_no_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_no_profile_no_flow::<PeaceTestError, _>(&mut output, &workspace)
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}
