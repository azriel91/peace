use peace::{
    cfg::{app_name, profile},
    cmd_ctx::{CmdCtxSpnf, ProfileSelection},
    resource_rt::paths::{ProfileDir, ProfileHistoryDir},
};

use crate::{
    no_op_output::NoOpOutput, peace_cmd_ctx_types::TestCctNoOpOutput, test_support::workspace,
};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctNoOpOutput>::builder()
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
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctNoOpOutput>::builder()
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
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctNoOpOutput>::builder()
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
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctNoOpOutput>::builder()
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
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctNoOpOutput>::builder()
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
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctNoOpOutput>::builder()
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
