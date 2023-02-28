use peace::{
    cfg::{app_name, profile, AppName, Profile},
    cmd::ctx::CmdCtx,
    resources::paths::{ProfileDir, ProfileHistoryDir},
};

use super::workspace;
use crate::PeaceTestError;

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtx::builder_single_profile_no_flow::<PeaceTestError>(&workspace)
        .with_profile(profile.clone())
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let scope = cmd_ctx.scope();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtx::builder_single_profile_no_flow::<PeaceTestError>(&workspace)
        .with_profile(profile.clone())
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
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

    let cmd_ctx = CmdCtx::builder_single_profile_no_flow::<PeaceTestError>(&workspace)
        .with_profile_param_value(String::from("profile_param_0"), Some(1u32))
        .with_profile_param_value(String::from("profile_param_1"), Some(2u64))
        .with_profile(profile.clone())
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let scope = cmd_ctx.scope();
    let profile_params = scope.profile_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
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

    let cmd_ctx = CmdCtx::builder_single_profile_no_flow::<PeaceTestError>(&workspace)
        .with_profile(profile.clone())
        .with_profile_param_value(String::from("profile_param_0"), Some(1u32))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_profile_param_value(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
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
async fn build_with_workspace_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtx::builder_single_profile_no_flow::<PeaceTestError>(&workspace)
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_from_workspace_param(&String::from("profile"))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtx::builder_single_profile_no_flow::<PeaceTestError>(&workspace)
        .with_profile_param_value(String::from("profile_param_0"), Some(1u32))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_profile_param_value(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_from_workspace_param(&String::from("profile"))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    Ok(())
}
