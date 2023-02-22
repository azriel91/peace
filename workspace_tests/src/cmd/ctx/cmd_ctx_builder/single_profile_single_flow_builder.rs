use peace::{
    cfg::{app_name, flow_id, profile, AppName, FlowId, Profile},
    cmd::{ctx::CmdCtxBuilder, scopes::SingleProfileSingleFlow},
    resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir},
};

use super::workspace;

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow_id(flow_id.clone())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
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
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile_param(String::from("profile_param"), Some(1u32))
        .with_profile_param(String::from("profile_param_other"), Some(2u64))
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
    Ok(())
}

#[tokio::test]
async fn build_with_flow_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow_id(flow_id.clone())
        .with_flow_param(
            String::from("flow_param"),
            Some("flow param value".to_string()),
        )
        .with_flow_param(String::from("flow_param_other"), Some(456u32))
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
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow_id(flow_id.clone())
        .with_profile_param(String::from("profile_param"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_other"), Some(2u64))
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
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_flow_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow_id(flow_id.clone())
        .with_profile_param(String::from("profile_param"), Some(1u32))
        .with_flow_param(
            String::from("flow_param"),
            Some("flow param value".to_string()),
        )
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_flow_param(String::from("flow_param_other"), Some(456u32))
        .with_profile_param(String::from("profile_param_other"), Some(2u64))
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
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(String::from("something_else"), Some("a string".to_string()))
        .with_profile_from_workspace_param(&String::from("profile"))
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
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    let cmd_ctx = CmdCtxBuilder::single_profile_single_flow(&workspace)
        .with_profile_param(String::from("profile_param"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_other"), Some(2u64))
        .with_workspace_param(String::from("something_else"), Some("a string".to_string()))
        .with_flow_param(
            String::from("flow_param"),
            Some("flow param value".to_string()),
        )
        .with_profile_from_workspace_param(&String::from("profile"))
        .with_flow_param(String::from("flow_param_other"), Some(456u32))
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
    Ok(())
}