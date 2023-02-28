use std::collections::BTreeMap;

use peace::{
    cfg::{app_name, flow_id, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    rt_model::{Flow, ItemSpecGraphBuilder},
};

use super::workspace_with;
use crate::PeaceTestError;

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_flow(flow)
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
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
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_profile_params_k::<String>()
        .with_profile_param::<u32>(String::from("profile_param_0"))
        .with_profile_param::<u64>(String::from("profile_param_1"))
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
    Ok(())
}

#[tokio::test]
async fn build_with_flow_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_flow(flow)
        .with_flow_params_k::<String>()
        .with_flow_param::<String>(String::from("flow_param_0"))
        .with_flow_param::<u32>(String::from("flow_param_1"))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    let profile_to_flow_params = scope.profile_to_flow_params();
    let flow_params = profile_to_flow_params
        .get(&profile)
        .expect("Expected flow params to exist.");
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
    assert_eq!(
        Some(&"flow_param_0_value".to_string()),
        flow_params.get("flow_param_0")
    );
    assert_eq!(Some(&456u32), flow_params.get("flow_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_flow(flow)
        .with_profile_params_k::<String>()
        .with_profile_param::<u32>(String::from("profile_param_0"))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_profile_param::<u64>(String::from("profile_param_1"))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_to_profile_params = scope.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
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
async fn build_with_workspace_params_with_profile_params_with_flow_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_flow(flow)
        .with_profile_params_k::<String>()
        .with_profile_param::<u32>(String::from("profile_param_0"))
        .with_flow_params_k::<String>()
        .with_flow_param::<String>(String::from("flow_param_0"))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_flow_param::<u32>(String::from("flow_param_1"))
        .with_profile_param::<u64>(String::from("profile_param_1"))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_to_profile_params = scope.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    let profile_to_flow_params = scope.profile_to_flow_params();
    let flow_params = profile_to_flow_params
        .get(&profile)
        .expect("Expected flow params to exist.");
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(
        Some(&"flow_param_0_value".to_string()),
        flow_params.get("flow_param_0")
    );
    assert_eq!(Some(&456u32), flow_params.get("flow_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_filter() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_filter(|profile| **profile == "test_profile")
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone()], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_filter()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), ItemSpecGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(&flow_id),
    )
    .await?;

    let cmd_ctx = CmdCtx::builder_multi_profile_single_flow::<PeaceTestError>(&workspace)
        .with_profile_params_k::<String>()
        .with_profile_param::<u32>(String::from("profile_param_0"))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_profile_param::<u64>(String::from("profile_param_1"))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_flow_params_k::<String>()
        .with_flow_param::<String>(String::from("flow_param_0"))
        .with_profile_filter(|profile| **profile == "test_profile")
        .with_flow_param::<u32>(String::from("flow_param_1"))
        .with_flow(flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, &flow_id));

        profile_dirs.insert(profile.clone(), profile_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_to_profile_params = scope.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    let profile_to_flow_params = scope.profile_to_flow_params();
    let flow_params = profile_to_flow_params
        .get(&profile)
        .expect("Expected flow params to exist.");
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone()], scope.profiles());
    assert_eq!(&profile_dirs, scope.profile_dirs());
    assert_eq!(&profile_history_dirs, scope.profile_history_dirs());
    assert_eq!(&flow_id, scope.flow().flow_id());
    assert_eq!(&flow_dirs, scope.flow_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(
        Some(&"flow_param_0_value".to_string()),
        flow_params.get("flow_param_0")
    );
    assert_eq!(Some(&456u32), flow_params.get("flow_param_1"));
    Ok(())
}
