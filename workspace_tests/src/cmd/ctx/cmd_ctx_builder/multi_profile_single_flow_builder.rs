use std::collections::BTreeMap;

use peace::{
    cfg::{app_name, profile},
    cmd_ctx::CmdCtxMpsf,
    flow_model::flow_id,
    flow_rt::{Flow, ItemGraphBuilder},
    resource_rt::{
        paths::{FlowDir, ProfileDir, ProfileHistoryDir},
        type_reg::untagged::TypeReg,
    },
    rt_model::{ParamsSpecsTypeReg, StatesTypeReg},
};

use crate::{
    no_op_output::NoOpOutput, peace_cmd_ctx_types::TestCctNoOpOutput, test_support::workspace_with,
    PeaceTestError,
};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
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
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(123))
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
    Ok(())
}

#[tokio::test]
async fn build_with_flow_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_flow_param::<bool>(&profile, String::from("flow_param_0"), Some(true))
        .with_flow_param::<u16>(&profile, String::from("flow_param_1"), None)
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    let profile_to_flow_params = fields.profile_to_flow_params();
    let flow_params = profile_to_flow_params
        .get(&profile)
        .expect("Expected flow params to exist.");
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(123))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_to_profile_params = fields.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
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
async fn build_with_workspace_params_with_profile_params_with_flow_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(123))
        .with_flow_param::<bool>(&profile, String::from("flow_param_0"), Some(true))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_flow_param::<u16>(&profile, String::from("flow_param_1"), None)
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);
        let profile_other_flow_dir = FlowDir::from((&profile_other_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);
        flow_dirs.insert(profile_other.clone(), profile_other_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_to_profile_params = fields.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    let profile_to_flow_params = fields.profile_to_flow_params();
    let flow_params = profile_to_flow_params
        .get(&profile)
        .expect("Expected flow params to exist.");
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_filter() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_filter_fn(|profile| **profile == "test_profile")
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone()], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_filter(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(123))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_flow_param::<bool>(&profile, String::from("flow_param_0"), Some(true))
        .with_profile_filter_fn(|profile| **profile == "test_profile")
        .with_flow_param::<u16>(&profile, String::from("flow_param_1"), None)
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs, flow_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();
        let mut flow_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let profile_flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        profile_dirs.insert(profile.clone(), profile_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);

        flow_dirs.insert(profile.clone(), profile_flow_dir);

        (profile_dirs, profile_history_dirs, flow_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_to_profile_params = fields.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    let profile_to_flow_params = fields.profile_to_flow_params();
    let flow_params = profile_to_flow_params
        .get(&profile)
        .expect("Expected flow params to exist.");
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone()], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dirs, fields.flow_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));
    Ok(())
}

#[tokio::test]
async fn getters() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let mut cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .await?;

    assert!(matches!(cmd_ctx.output(), NoOpOutput));
    assert!(matches!(cmd_ctx.output_mut(), NoOpOutput));
    let fields = cmd_ctx.fields_mut();
    assert_eq!(workspace.dirs().workspace_dir(), fields.workspace_dir());
    assert_eq!(workspace.dirs().peace_dir(), fields.peace_dir());
    assert_eq!(workspace.dirs().peace_app_dir(), fields.peace_app_dir());
    assert_eq!(2, fields.profile_to_states_current_stored().len());
    assert_eq!(2, fields.profile_to_params_specs().len());
    assert!(matches!(fields.workspace_params_type_reg(), TypeReg { .. }));
    assert!(matches!(
        fields.workspace_params_type_reg_mut(),
        TypeReg { .. }
    ));
    assert!(matches!(fields.profile_params_type_reg(), TypeReg { .. }));
    assert!(matches!(
        fields.profile_params_type_reg_mut(),
        TypeReg { .. }
    ));
    assert!(matches!(fields.flow_params_type_reg(), TypeReg { .. }));
    assert!(matches!(fields.flow_params_type_reg_mut(), TypeReg { .. }));
    assert!(matches!(
        fields.params_specs_type_reg(),
        ParamsSpecsTypeReg { .. }
    ));
    assert!(matches!(fields.states_type_reg(), StatesTypeReg { .. }));
    assert!(!fields.resources().is_empty());
    assert!(!fields.resources_mut().is_empty());

    Ok(())
}

#[tokio::test]
async fn debug() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_single_flow"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .build()
        .await?;

    assert!(format!("{cmd_ctx:?}").contains("CmdCtxMpsf {"));

    let fields = cmd_ctx.fields();
    assert!(format!("{fields:?}").contains("CmdCtxMpsfFields {"));

    Ok(())
}
