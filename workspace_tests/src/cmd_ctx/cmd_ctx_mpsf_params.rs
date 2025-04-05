use std::collections::BTreeMap;

use peace::{
    cfg::{app_name, profile},
    cmd_ctx::{CmdCtxMpsf, CmdCtxTypes},
    flow_model::flow_id,
    flow_rt::{Flow, ItemGraphBuilder},
    params::ParamsSpec,
    profile_model::Profile,
    resource_rt::{
        paths::{FlowDir, ParamsSpecsFile, ProfileDir, ProfileHistoryDir},
        type_reg::untagged::TypeReg,
    },
    rt_model::{ParamsSpecsTypeReg, StatesTypeReg},
};

use crate::{
    no_op_output::NoOpOutput, test_support::workspace_with, PeaceTestError, VecA, VecCopyItem,
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        // Overwrite existing value
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(3u32))
        // Erase existing value
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        // Add new value
        .with_profile_param::<i64>(&profile, String::from("profile_param_2"), Some(-4i64))
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
    assert_eq!(Some(3u32), profile_params.get("profile_param_0").copied());
    assert_eq!(None::<u64>, profile_params.get("profile_param_1").copied());
    assert_eq!(Some(-4i64), profile_params.get("profile_param_2").copied());
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        // Overwrite existing value
        .with_flow_param::<bool>(&profile, String::from("flow_param_0"), Some(true))
        // Erase existing value
        .with_flow_param::<u16>(&profile, String::from("flow_param_1"), None)
        // Add new value
        .with_flow_param::<i16>(&profile, String::from("flow_param_2"), Some(-5i16))
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
    assert_eq!(None::<u16>, flow_params.get("flow_param_1").copied());
    assert_eq!(Some(-5i16), flow_params.get("flow_param_2").copied());
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        // Overwrite existing value
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(3u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        // Erase existing value
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        // Add new value
        .with_profile_param::<i64>(&profile, String::from("profile_param_2"), Some(-4i64))
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
    assert_eq!(Some(3u32), profile_params.get("profile_param_0").copied());
    assert_eq!(None::<u64>, profile_params.get("profile_param_1").copied());
    assert_eq!(Some(-4i64), profile_params.get("profile_param_2").copied());
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        // Overwrite existing value
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(3u32))
        // Erase existing value
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        // Add new value
        .with_profile_param::<i64>(&profile, String::from("profile_param_2"), Some(-4i64))
        // Overwrite existing value
        .with_flow_param::<bool>(&profile, String::from("flow_param_0"), Some(true))
        // Erase existing value
        .with_flow_param::<u16>(&profile, String::from("flow_param_1"), None)
        // Add new value
        .with_flow_param::<i16>(&profile, String::from("flow_param_2"), Some(-5i16))
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
    assert_eq!(Some(3u32), profile_params.get("profile_param_0").copied());
    assert_eq!(None::<u64>, profile_params.get("profile_param_1").copied());
    assert_eq!(Some(-4i64), profile_params.get("profile_param_2").copied());
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(None::<u16>, flow_params.get("flow_param_1").copied());
    assert_eq!(Some(-5i16), flow_params.get("flow_param_2").copied());
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_profile_filter_fn(|profile| **profile == "test_profile")
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        // Overwrite existing value
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(3u32))
        // Erase existing value
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        // Add new value
        .with_profile_param::<i64>(&profile, String::from("profile_param_2"), Some(-4i64))
        // Overwrite existing value
        .with_flow_param::<bool>(&profile, String::from("flow_param_0"), Some(true))
        // Erase existing value
        .with_flow_param::<u16>(&profile, String::from("flow_param_1"), None)
        // Add new value
        .with_flow_param::<i16>(&profile, String::from("flow_param_2"), Some(-5i16))
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
    assert_eq!(Some(3u32), profile_params.get("profile_param_0").copied());
    assert_eq!(None::<u64>, profile_params.get("profile_param_1").copied());
    assert_eq!(Some(-4i64), profile_params.get("profile_param_2").copied());
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(None::<u16>, flow_params.get("flow_param_1").copied());
    assert_eq!(Some(-5i16), flow_params.get("flow_param_2").copied());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_filter_with_flow_params_none_specified(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx_save = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_profile_filter_fn(|profile| **profile == "test_profile")
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some(String::from("ws_param_1_value")),
        )
        .with_workspace_param(String::from("ws_param_2"), None::<u8>)
        // Overwrite existing value
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(3u32))
        // Erase existing value
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        // Add new value
        .with_profile_param::<i64>(&profile, String::from("profile_param_2"), Some(-4i64))
        // Overwrite existing value
        .with_flow_param::<bool>(&profile, String::from("flow_param_0"), Some(true))
        // Erase existing value
        .with_flow_param::<u16>(&profile, String::from("flow_param_1"), None)
        // Add new value
        .with_flow_param::<i16>(&profile, String::from("flow_param_2"), Some(-5i16))
        .await?;
    drop(cmd_ctx_save);

    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(NoOpOutput.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_profile_filter_fn(|profile| **profile == "test_profile")
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
        Some(&String::from("ws_param_1_value")),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(None::<u8>, workspace_params.get("ws_param_2").copied());
    assert_eq!(Some(3u32), profile_params.get("profile_param_0").copied());
    assert_eq!(None::<u64>, profile_params.get("profile_param_1").copied());
    assert_eq!(Some(-4i64), profile_params.get("profile_param_2").copied());
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(None::<u16>, flow_params.get("flow_param_1").copied());
    assert_eq!(Some(-5i16), flow_params.get("flow_param_2").copied());
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let mut cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
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
async fn build_with_item_params_with_resource() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(
            &profile,
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0u8]).into(),
        )
        // Overwriting should work.
        .with_item_params::<VecCopyItem>(
            &profile,
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![1u8]).into(),
        )
        .with_item_params::<VecCopyItem>(
            &profile_other,
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![2u8]).into(),
        )
        .with_resource("Adding &'static str")
        .await?;

    let fields = cmd_ctx.fields();
    let profile_to_params_specs = fields.profile_to_params_specs();
    let params_specs = profile_to_params_specs
        .get(&profile)
        .expect("Expected params_specs to exist for test_profile.");
    assert!(matches!(
        params_specs.get::<ParamsSpec<VecA>, _>(VecCopyItem::ID_DEFAULT),
        Some(params_spec)
            if matches!(
                params_spec,
                ParamsSpec::Value { value } if value == &VecA(vec![1u8])
            )
    ));
    let params_specs = profile_to_params_specs
        .get(&profile_other)
        .expect("Expected params_specs to exist for test_profile_other.");
    assert!(matches!(
        params_specs.get::<ParamsSpec<VecA>, _>(VecCopyItem::ID_DEFAULT),
        Some(params_spec)
            if matches!(
                params_spec,
                ParamsSpec::Value { value } if value == &VecA(vec![2u8])
            )
    ));
    let resources = fields.resources();
    let s = resources.try_borrow::<&'static str>();
    assert_eq!(Ok("Adding &'static str"), s.as_deref().copied());
    Ok(())
}

#[tokio::test]
async fn build_with_missing_item_params_returns_error() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    // Delete the item params specs file.
    let item_params_specs_file = {
        let peace_app_dir = workspace.dirs().peace_app_dir();
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));
        ParamsSpecsFile::from(&flow_dir)
    };
    tokio::fs::remove_file(item_params_specs_file).await?;

    let output = NoOpOutput;
    let error = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_flow((&flow).into())
        .await
        .unwrap_err();

    if let PeaceTestError::PeaceRt(peace::rt_model::Error::ItemParamsSpecsFileNotFound {
        app_name,
        profile,
        flow_id,
    }) = error
    {
        assert_eq!(app_name.as_str(), "test_cmd_ctx_mpsf_params");
        assert_eq!(profile.as_str(), "test_profile");
        assert_eq!(flow_id.as_str(), "test_flow_id");
    } else {
        panic!(
            "Expected error to be `PeaceTestError::PeaceRt(\
            peace::rt_model::Error::ItemParamsSpecsFileNotFound {{ .. }})`, \
            but it was  {error:?}"
        );
    }
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
        app_name!("test_cmd_ctx_mpsf_params"),
        &[profile.clone(), profile_other.clone()],
        Some(flow.flow_id()),
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpsf::<TestCctCmdCtxMpsf>::builder()
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

#[derive(Debug)]
pub struct TestCctCmdCtxMpsf;

impl CmdCtxTypes for TestCctCmdCtxMpsf {
    type AppError = PeaceTestError;
    type FlowParamsKey = String;
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
        type_reg.register::<i64>(String::from("profile_param_2"));
    }

    fn flow_params_register(type_reg: &mut TypeReg<Self::FlowParamsKey>) {
        type_reg.register::<bool>(String::from("flow_param_0"));
        type_reg.register::<u16>(String::from("flow_param_1"));
        type_reg.register::<i16>(String::from("flow_param_2"));
    }
}
