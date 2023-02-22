use peace::{
    cfg::{app_name, profile, AppName, Profile},
    cmd::{ctx::CmdCtxBuilder, scopes::SingleProfileNoFlow},
    resources::paths::{ProfileDir, ProfileHistoryDir},
};

use super::workspace;

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtxBuilder::single_profile_no_flow(&workspace)
        .with_profile(profile.clone())
        .build()
        .await?;

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        SingleProfileNoFlow::new(profile, profile_dir, profile_history_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtxBuilder::single_profile_no_flow(&workspace)
        .with_profile(profile.clone())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .build()
        .await?;

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        SingleProfileNoFlow::new(profile, profile_dir, profile_history_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtxBuilder::single_profile_no_flow(&workspace)
        .with_profile_param(String::from("profile_param"), Some(1u32))
        .with_profile_param(String::from("profile_param_other"), Some(2u64))
        .with_profile(profile.clone())
        .build()
        .await?;

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        SingleProfileNoFlow::new(profile, profile_dir, profile_history_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtxBuilder::single_profile_no_flow(&workspace)
        .with_profile(profile.clone())
        .with_profile_param(String::from("profile_param"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_other"), Some(2u64))
        .build()
        .await?;

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        SingleProfileNoFlow::new(profile, profile_dir, profile_history_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtxBuilder::single_profile_no_flow(&workspace)
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(String::from("something_else"), Some("a string".to_string()))
        .with_profile_from_workspace_param(&String::from("profile"))
        .build()
        .await?;

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        SingleProfileNoFlow::new(profile, profile_dir, profile_history_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_no_flow"))?;
    let profile = profile!("test_profile");

    let cmd_ctx = CmdCtxBuilder::single_profile_no_flow(&workspace)
        .with_profile_param(String::from("profile_param"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_other"), Some(2u64))
        .with_workspace_param(String::from("something_else"), Some("a string".to_string()))
        .with_profile_from_workspace_param(&String::from("profile"))
        .build()
        .await?;

    let scope = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        SingleProfileNoFlow::new(profile, profile_dir, profile_history_dir)
    };
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&scope, cmd_ctx.scope());
    Ok(())
}
