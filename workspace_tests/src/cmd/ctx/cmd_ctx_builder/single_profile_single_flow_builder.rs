use peace::{
    cfg::{app_name, flow_id, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    rt_model::{Flow, ItemSpecGraphBuilder},
};

use super::workspace;
use crate::PeaceTestError;

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow(flow)
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(String::from("something_else"), Some("a string".to_string()))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"a string".to_string()),
        workspace_params.get("something_else")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_profile_param_value(String::from("profile_param"), Some(1u32))
        .with_profile_param_value(String::from("profile_param_other"), Some(2u64))
        .with_profile(profile.clone())
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    Ok(())
}

#[tokio::test]
async fn build_with_flow_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow(flow)
        .with_flow_param_value(
            String::from("flow_param"),
            Some("flow param value".to_string()),
        )
        .with_flow_param_value(String::from("flow_param_other"), Some(456u32))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    let flow_params = scope.flow_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(
        Some(&"flow param value".to_string()),
        flow_params.get("flow_param")
    );
    assert_eq!(Some(&456u32), flow_params.get("flow_param_other"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow(flow)
        .with_profile_param_value(String::from("profile_param"), Some(1u32))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_profile_param_value(String::from("profile_param_other"), Some(2u64))
        .with_workspace_param_value(String::from("something_else"), Some("a string".to_string()))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"a string".to_string()),
        workspace_params.get("something_else")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_other"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_flow_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_profile(profile.clone())
        .with_flow(flow)
        .with_profile_param_value(String::from("profile_param"), Some(1u32))
        .with_flow_param_value(
            String::from("flow_param"),
            Some("flow param value".to_string()),
        )
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_flow_param_value(String::from("flow_param_other"), Some(456u32))
        .with_profile_param_value(String::from("profile_param_other"), Some(2u64))
        .with_workspace_param_value(String::from("something_else"), Some("a string".to_string()))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    let flow_params = scope.flow_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"a string".to_string()),
        workspace_params.get("something_else")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_other"));
    assert_eq!(
        Some(&"flow param value".to_string()),
        flow_params.get("flow_param")
    );
    assert_eq!(Some(&456u32), flow_params.get("flow_param_other"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(String::from("something_else"), Some("a string".to_string()))
        .with_profile_from_workspace_param(&String::from("profile"))
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"a string".to_string()),
        workspace_params.get("something_else")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&workspace)
        .with_profile_param_value(String::from("profile_param"), Some(1u32))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_profile_param_value(String::from("profile_param_other"), Some(2u64))
        .with_workspace_param_value(String::from("something_else"), Some("a string".to_string()))
        .with_flow_param_value(
            String::from("flow_param"),
            Some("flow param value".to_string()),
        )
        .with_profile_from_workspace_param(&String::from("profile"))
        .with_flow_param_value(String::from("flow_param_other"), Some(456u32))
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, &flow_id));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    let flow_params = scope.flow_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"a string".to_string()),
        workspace_params.get("something_else")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_other"));
    assert_eq!(
        Some(&"flow param value".to_string()),
        flow_params.get("flow_param")
    );
    assert_eq!(Some(&456u32), flow_params.get("flow_param_other"));
    Ok(())
}
