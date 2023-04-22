use peace::{
    cfg::{
        app_name, flow_id, item_spec_id, profile, AppName, FlowId, ItemSpec, ItemSpecId, Profile,
    },
    cmd::ctx::CmdCtx,
    resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    rt_model::{Flow, ItemSpecGraphBuilder},
};

use crate::{
    cmd::ctx::cmd_ctx_builder::{
        assert_flow_params, assert_profile_params, assert_workspace_params, workspace,
    },
    no_op_output::NoOpOutput,
    vec_copy_item_spec::{VecA, VecCopyItemSpec},
    PeaceTestError,
};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
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
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );

    let resources = cmd_ctx.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile_param_value(String::from("profile_param_0"), Some(1u32))
        .with_profile_param_value(String::from("profile_param_1"), Some(2u64))
        .with_profile(profile.clone())
        .with_flow(&flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());

    let resources = cmd_ctx.resources();
    assert_profile_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_flow_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_flow_param_value(String::from("flow_param_0"), Some(true))
        .with_flow_param_value(String::from("flow_param_1"), Some(456u16))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    let flow_params = scope.flow_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));

    let resources = cmd_ctx.resources();
    assert_flow_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
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
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));

    let resources = cmd_ctx.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    assert_profile_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_flow_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_profile_param_value(String::from("profile_param_0"), Some(1u32))
        .with_flow_param_value(String::from("flow_param_0"), Some(true))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_flow_param_value(String::from("flow_param_1"), Some(456u16))
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
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    let flow_params = scope.flow_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));

    let resources = cmd_ctx.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    assert_profile_params(resources).await?;
    assert_flow_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_from_workspace_param(&String::from("profile"))
        .with_flow(&flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );

    let resources = cmd_ctx.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemSpecGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile_param_value(String::from("profile_param_0"), Some(1u32))
        .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
        .with_profile_param_value(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param_value(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_flow_param_value(String::from("flow_param_0"), Some(true))
        .with_profile_from_workspace_param(&String::from("profile"))
        .with_flow_param_value(String::from("flow_param_1"), Some(456u16))
        .with_flow(&flow)
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let scope = cmd_ctx.scope();
    let workspace_params = scope.workspace_params();
    let profile_params = scope.profile_params();
    let flow_params = scope.flow_params();
    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(peace_app_dir, cmd_ctx.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, scope.profile());
    assert_eq!(&profile_dir, scope.profile_dir());
    assert_eq!(&profile_history_dir, scope.profile_history_dir());
    assert_eq!(flow.flow_id(), scope.flow().flow_id());
    assert_eq!(&flow_dir, scope.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));

    let resources = cmd_ctx.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    assert_profile_params(resources).await?;
    assert_flow_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_item_spec_params_returns_ok_when_params_provided()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::new();
        item_spec_graph_builder.add_fn(VecCopyItemSpec::default().into());
        item_spec_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_spec_graph);

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![1u8]),
        )
        .build()
        .await?;

    let scope = cmd_ctx.scope();
    let item_spec_params = scope.item_spec_params();
    let resources = scope.resources();
    assert_eq!(
        Some(VecA(vec![1u8])).as_ref().map(|vec_a| &vec_a.0),
        item_spec_params
            .get::<<VecCopyItemSpec as ItemSpec>::Params<'_>, _>(VecCopyItemSpec::ID_DEFAULT)
            .map(|vec_a| &vec_a.0)
    );
    assert_eq!(
        Some(VecA(vec![1u8])).as_ref().map(|vec_a| &vec_a.0),
        resources
            .try_borrow::<<VecCopyItemSpec as ItemSpec>::Params<'_>>()
            .ok()
            .as_ref()
            .map(|vec_a| &vec_a.0),
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_spec_params_returns_err_when_params_not_provided_and_not_stored()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::new();
        item_spec_graph_builder.add_fn(VecCopyItemSpec::default().into());
        item_spec_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_spec_graph);

    let mut output = NoOpOutput;
    let cmd_ctx_result = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .build()
        .await;

    assert!(
        matches!(
            &cmd_ctx_result,
            Err(PeaceTestError::PeaceRtError(
                peace::rt_model::Error::ItemSpecParamsMismatch {
                    item_spec_ids_with_no_params,
                    provided_item_spec_params_mismatches,
                    stored_item_spec_params_mismatches
                }
            ))
            if item_spec_ids_with_no_params == &vec![VecCopyItemSpec::ID_DEFAULT.clone()]
            && provided_item_spec_params_mismatches.is_empty()
            && stored_item_spec_params_mismatches.is_none(),
        ),
        "was {cmd_ctx_result:#?}"
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_spec_params_returns_ok_when_params_not_provided_but_are_stored()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::new();
        item_spec_graph_builder.add_fn(VecCopyItemSpec::default().into());
        item_spec_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_spec_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![1u8]),
        )
        .build()
        .await?;

    let cmd_ctx_from_stored = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .build()
        .await?;

    let scope = cmd_ctx_from_stored.scope();
    let item_spec_params = scope.item_spec_params();
    let resources = scope.resources();
    assert_eq!(
        Some(VecA(vec![1u8])).as_ref().map(|vec_a| &vec_a.0),
        item_spec_params
            .get::<<VecCopyItemSpec as ItemSpec>::Params<'_>, _>(VecCopyItemSpec::ID_DEFAULT)
            .map(|vec_a| &vec_a.0)
    );
    assert_eq!(
        Some(VecA(vec![1u8])).as_ref().map(|vec_a| &vec_a.0),
        resources
            .try_borrow::<<VecCopyItemSpec as ItemSpec>::Params<'_>>()
            .ok()
            .as_ref()
            .map(|vec_a| &vec_a.0),
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_spec_params_returns_ok_and_uses_params_provided_when_params_provided_and_stored()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::new();
        item_spec_graph_builder.add_fn(VecCopyItemSpec::default().into());
        item_spec_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_spec_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![1u8]),
        )
        .build()
        .await?;

    let cmd_ctx_from_stored = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![2u8]),
        )
        .build()
        .await?;

    let scope = cmd_ctx_from_stored.scope();
    let item_spec_params = scope.item_spec_params();
    let resources = scope.resources();
    assert_eq!(
        Some(VecA(vec![2u8])).as_ref().map(|vec_a| &vec_a.0),
        item_spec_params
            .get::<<VecCopyItemSpec as ItemSpec>::Params<'_>, _>(VecCopyItemSpec::ID_DEFAULT)
            .map(|vec_a| &vec_a.0)
    );
    assert_eq!(
        Some(VecA(vec![2u8])).as_ref().map(|vec_a| &vec_a.0),
        resources
            .try_borrow::<<VecCopyItemSpec as ItemSpec>::Params<'_>>()
            .ok()
            .as_ref()
            .map(|vec_a| &vec_a.0),
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_spec_params_returns_err_when_params_mismatch()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_single_profile_single_flow"))?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::new();
        item_spec_graph_builder.add_fn(VecCopyItemSpec::default().into());
        item_spec_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_spec_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![1u8]),
        )
        .build()
        .await?;

    let cmd_ctx_result = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile.clone())
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(item_spec_id!("mismatch_id"), VecA(vec![2u8]))
        .build()
        .await;

    assert!(
        matches!(
            &cmd_ctx_result,
            Err(PeaceTestError::PeaceRtError(
                peace::rt_model::Error::ItemSpecParamsMismatch {
                    item_spec_ids_with_no_params,
                    provided_item_spec_params_mismatches,
                    stored_item_spec_params_mismatches
                }
            ))
            if item_spec_ids_with_no_params.is_empty()
            && provided_item_spec_params_mismatches.get(&item_spec_id!("mismatch_id"))
                == Some(&VecA(vec![2u8]))
            && matches!(
                stored_item_spec_params_mismatches,
                Some(stored_item_spec_params_mismatches)
                if stored_item_spec_params_mismatches.is_empty()
            ),
        ),
        "was {cmd_ctx_result:#?}"
    );

    Ok(())
}
