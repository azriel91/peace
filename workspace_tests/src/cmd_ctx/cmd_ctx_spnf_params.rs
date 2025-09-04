use peace::{
    cfg::{app_name, profile},
    cmd_ctx::{type_reg::untagged::TypeReg, CmdCtxSpnf, CmdCtxTypes, ProfileSelection},
    profile_model::Profile,
    resource_rt::paths::{ProfileDir, ProfileHistoryDir},
};

use crate::{no_op_output::NoOpOutput, test_support::workspace, PeaceTestError};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_from_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            String::from("profile").into(),
        ))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            String::from("profile").into(),
        ))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_none_provided(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let mut output = NoOpOutput;
    let cmd_ctx_save = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some(String::from("ws_param_1_value")),
        )
        .with_workspace_param(String::from("ws_param_2"), None::<u8>)
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .await?;
    drop(cmd_ctx_save);

    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            String::from("profile").into(),
        ))
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&String::from("ws_param_1_value")),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(None::<u8>, workspace_params.get("ws_param_2").copied());
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    Ok(())
}

#[derive(Debug)]
pub struct TestCctCmdCtxSpnf;

impl CmdCtxTypes for TestCctCmdCtxSpnf {
    type AppError = PeaceTestError;
    type FlowParamsKey = ();
    type MappingFns = ();
    type Output = NoOpOutput;
    type ProfileParamsKey = String;
    type WorkspaceParamsKey = String;

    fn workspace_params_register(type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {
        type_reg.register::<Profile>(String::from("profile"));
        type_reg.register::<String>(String::from("ws_param_1"));
        type_reg.register::<u8>(String::from("ws_param_2"));
    }

    fn profile_params_register(type_reg: &mut TypeReg<Self::ProfileParamsKey>) {
        type_reg.register::<u32>(String::from("profile_param_0"));
        type_reg.register::<u64>(String::from("profile_param_1"));
    }

    fn flow_params_register(_type_reg: &mut TypeReg<Self::FlowParamsKey>) {}
}
