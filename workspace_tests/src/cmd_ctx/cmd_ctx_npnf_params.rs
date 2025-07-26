use peace::{
    cfg::{app_name, profile},
    cmd_ctx::{type_reg::untagged::TypeReg, CmdCtxNpnf, CmdCtxTypes},
    profile_model::Profile,
};

use crate::{no_op_output::NoOpOutput, test_support::workspace, PeaceTestError};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_npnf_params")).await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxNpnf::<TestCctCmdCtxNpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .await?;

    assert!(std::ptr::eq(&workspace, cmd_ctx.fields().workspace()));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_npnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxNpnf::<TestCctCmdCtxNpnf>::builder()
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

#[tokio::test]
async fn build_with_workspace_params_none_specified() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_npnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx_save = CmdCtxNpnf::<TestCctCmdCtxNpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some(String::from("ws_param_1_value")),
        )
        .with_workspace_param(String::from("ws_param_2"), None::<u8>)
        .await?;
    drop(cmd_ctx_save);

    let cmd_ctx = CmdCtxNpnf::<TestCctCmdCtxNpnf>::builder()
        .with_output(NoOpOutput.into())
        .with_workspace((&workspace).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&String::from("ws_param_1_value")),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(None::<u8>, workspace_params.get("ws_param_2").copied());
    Ok(())
}

#[derive(Debug)]
pub struct TestCctCmdCtxNpnf;

impl CmdCtxTypes for TestCctCmdCtxNpnf {
    type AppError = PeaceTestError;
    type FlowParamsKey = ();
    type MappingFns = ();
    type Output = NoOpOutput;
    type ProfileParamsKey = ();
    type WorkspaceParamsKey = String;

    fn workspace_params_register(type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {
        type_reg.register::<Profile>(String::from("profile"));
        type_reg.register::<String>(String::from("ws_param_1"));
        type_reg.register::<u8>(String::from("ws_param_2"));
    }

    fn profile_params_register(_type_reg: &mut TypeReg<Self::ProfileParamsKey>) {}

    fn flow_params_register(_type_reg: &mut TypeReg<Self::FlowParamsKey>) {}
}
