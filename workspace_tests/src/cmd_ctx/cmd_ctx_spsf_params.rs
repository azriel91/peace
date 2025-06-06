use peace::{
    cfg::{app_name, Item},
    cmd_ctx::{type_reg::untagged::TypeReg, CmdCtxSpsf, CmdCtxTypes, ProfileSelection},
    flow_model::flow_id,
    flow_rt::{Flow, ItemGraphBuilder},
    item_model::item_id,
    params::{Params, ParamsSpec, ValueResolutionCtx, ValueResolutionMode, ValueSpec},
    profile_model::{profile, Profile},
    resource_rt::{
        internal::WorkspaceParamsFile,
        paths::{FlowDir, ProfileDir, ProfileHistoryDir},
        type_reg::untagged::BoxDataTypeDowncast,
    },
};

use crate::{
    no_op_output::NoOpOutput,
    test_support::{assert_flow_params, assert_profile_params, assert_workspace_params, workspace},
    vec_copy_item::{VecA, VecAFieldWise, VecCopyItem},
    PeaceTestError,
};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );

    let resources = fields.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());

    let resources = fields.resources();
    assert_profile_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_flow_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_flow_param(String::from("flow_param_0"), Some(true))
        .with_flow_param(String::from("flow_param_1"), Some(456u16))
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    let flow_params = fields.flow_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));

    let resources = fields.resources();
    assert_flow_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));

    let resources = fields.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    assert_profile_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_flow_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_flow_param(String::from("flow_param_0"), Some(true))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_flow_param(String::from("flow_param_1"), Some(456u16))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    let flow_params = fields.flow_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));

    let resources = fields.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    assert_profile_params(resources).await?;
    assert_flow_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_flow_params_none_provided(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx_save = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_workspace_param(String::from("ws_param_2"), None::<u8>)
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_flow_param(String::from("flow_param_0"), Some(true))
        .with_flow_param(String::from("flow_param_1"), Some(456u16))
        .await?;
    drop(cmd_ctx_save);

    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            String::from("profile").into(),
        ))
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    let flow_params = fields.flow_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&String::from("ws_param_1_value")),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(None::<u8>, workspace_params.get("ws_param_2").copied());
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));

    let resources = fields.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    assert_profile_params(resources).await?;
    assert_flow_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_from_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            String::from("profile").into(),
        ))
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );

    let resources = fields.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_param(String::from("profile_param_0"), Some(1u32))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param(String::from("profile_param_1"), Some(2u64))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_flow_param(String::from("flow_param_0"), Some(true))
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            String::from("profile").into(),
        ))
        .with_flow_param(String::from("flow_param_1"), Some(456u16))
        .with_flow((&flow).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
    let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    let flow_params = fields.flow_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(flow.flow_id(), fields.flow().flow_id());
    assert_eq!(&flow_dir, fields.flow_dir());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    assert_eq!(Some(true), flow_params.get("flow_param_0").copied());
    assert_eq!(Some(&456u16), flow_params.get("flow_param_1"));

    let resources = fields.resources();
    let res_profile = &*resources.borrow::<Profile>();
    assert_eq!(&profile, res_profile);
    assert_workspace_params(resources).await?;
    assert_profile_params(resources).await?;
    assert_flow_params(resources).await?;
    Ok(())
}

#[tokio::test]
async fn build_with_with_profile_from_params_returns_error_when_profile_not_found(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let flow_id = flow_id!("test_flow_id");
    let flow = Flow::<PeaceTestError>::new(flow_id, ItemGraphBuilder::new().build());

    let mut output = NoOpOutput;
    let error = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        // Deliberately not setting workspace param.
        // .with_workspace_param(String::from("profile"), Some(profile!("test_profile")))
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            String::from("profile").into(),
        ))
        .with_flow((&flow).into())
        .await
        .unwrap_err();

    let workspace_params_file_expected =
        WorkspaceParamsFile::from(workspace.dirs().peace_app_dir());

    if let PeaceTestError::PeaceRt(peace::rt_model::Error::WorkspaceParamsProfileNone {
        profile_key,
        workspace_params_file,
        workspace_params_file_contents,
    }) = &error
    {
        assert_eq!(profile_key, "profile");
        assert_eq!(workspace_params_file, &workspace_params_file_expected);
        assert_eq!(workspace_params_file_contents, "");
    } else {
        panic!(
            "Expected error to be \
            `PeaceTestError::PeaceRt(Error::WorkspaceParamsProfileNone {{ .. }})`, \
            but it was: {error:?}"
        );
    }

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_ok_when_params_provided(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);

    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(VecCopyItem::ID_DEFAULT.clone(), VecA(vec![1u8]).into())
        .await?;

    let fields = cmd_ctx.fields();
    let params_specs = fields.params_specs();
    let resources = fields.resources();
    let vec_a_spec = params_specs
        .get::<ParamsSpec<<VecCopyItem as Item>::Params<'_>>, _>(VecCopyItem::ID_DEFAULT);
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        VecCopyItem::ID_DEFAULT.clone(),
        tynm::type_name::<VecA>(),
    );
    assert!(matches!(vec_a_spec,
        Some(ParamsSpec::Value { value: VecA(value) })
        if value == &[1u8]
    ));
    assert_eq!(
        Some(VecA(vec![1u8])),
        vec_a_spec.and_then(|vec_a_spec| vec_a_spec
            .resolve(resources, &mut value_resolution_ctx)
            .ok()),
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_err_when_params_not_provided_and_not_stored(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);

    let mut output = NoOpOutput;
    let cmd_ctx_result = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .build()
        .await;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_ctx_result,
                    Err(PeaceTestError::PeaceRt(
                        peace::rt_model::Error::ParamsSpecsMismatch {
                            item_ids_with_no_params_specs,
                            params_specs_provided_mismatches,
                            params_specs_stored_mismatches,
                            params_specs_not_usable,
                        }
                    ))
                    if item_ids_with_no_params_specs == &vec![VecCopyItem::ID_DEFAULT.clone()]
                    && params_specs_provided_mismatches.is_empty()
                    && params_specs_stored_mismatches.is_none()
                    && params_specs_not_usable.is_empty(),
                ),
                "was {cmd_ctx_result:#?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_ok_when_params_not_provided_but_are_stored(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(VecCopyItem::ID_DEFAULT.clone(), VecA(vec![1u8]).into())
        .await?;

    let cmd_ctx_from_stored = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .await?;

    let fields = cmd_ctx_from_stored.fields();
    let params_specs = fields.params_specs();
    let resources = fields.resources();
    let vec_a_spec = params_specs
        .get::<ParamsSpec<<VecCopyItem as Item>::Params<'_>>, _>(VecCopyItem::ID_DEFAULT);
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        VecCopyItem::ID_DEFAULT.clone(),
        tynm::type_name::<VecA>(),
    );
    assert!(matches!(vec_a_spec,
        Some(ParamsSpec::Value { value: VecA(value) })
        if value == &[1u8]
    ));
    assert_eq!(
        Some(VecA(vec![1u8])),
        vec_a_spec.and_then(|vec_a_spec| vec_a_spec
            .resolve(resources, &mut value_resolution_ctx)
            .ok()),
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_ok_and_uses_params_provided_when_params_provided_and_stored(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(VecCopyItem::ID_DEFAULT.clone(), VecA(vec![1u8]).into())
        .await?;

    let cmd_ctx_from_stored = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(VecCopyItem::ID_DEFAULT.clone(), VecA(vec![2u8]).into())
        .await?;

    let fields = cmd_ctx_from_stored.fields();
    let params_specs = fields.params_specs();
    let resources = fields.resources();
    let vec_a_spec = params_specs
        .get::<ParamsSpec<<VecCopyItem as Item>::Params<'_>>, _>(VecCopyItem::ID_DEFAULT);
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        VecCopyItem::ID_DEFAULT.clone(),
        tynm::type_name::<VecA>(),
    );
    assert!(matches!(vec_a_spec,
        Some(ParamsSpec::Value { value: VecA(value) })
        if value == &[2u8]
    ));
    assert_eq!(
        Some(VecA(vec![2u8])),
        vec_a_spec.and_then(|vec_a_spec| vec_a_spec
            .resolve(resources, &mut value_resolution_ctx)
            .ok()),
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_err_when_params_provided_mismatch(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(VecCopyItem::ID_DEFAULT.clone(), VecA(vec![1u8]).into())
        .await?;

    let cmd_ctx_result = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(item_id!("mismatch_id"), VecA(vec![2u8]).into())
        .build()
        .await;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_ctx_result,
                    Err(PeaceTestError::PeaceRt(
                        peace::rt_model::Error::ParamsSpecsMismatch {
                            item_ids_with_no_params_specs,
                            params_specs_provided_mismatches,
                            params_specs_stored_mismatches,
                            params_specs_not_usable,
                        }
                    ))
                    if item_ids_with_no_params_specs.is_empty()
                    && matches!(
                        params_specs_provided_mismatches.get(&item_id!("mismatch_id")),
                        Some(ParamsSpec::Value { value: VecA(value) })
                        if value == &vec![2u8]
                    )
                    && matches!(
                        params_specs_stored_mismatches.as_ref(),
                        Some(params_specs_stored_mismatches)
                        if params_specs_stored_mismatches.is_empty()
                    )
                    && params_specs_not_usable.is_empty(),
                ),
                "was {cmd_ctx_result:#?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_err_when_params_stored_mismatch(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(item_id!("original_id")).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(item_id!("original_id"), VecA(vec![1u8]).into())
        .await?;

    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        // Without the original ID also registered for the item,
        // item_params deserialization will fail before reaching the params merge
        // error.
        item_graph_builder.add_fn(VecCopyItem::new(item_id!("original_id")).into());
        item_graph_builder.add_fn(VecCopyItem::new(item_id!("new_id")).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let cmd_ctx_result = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(item_id!("mismatch_id"), VecA(vec![2u8]).into())
        .build()
        .await;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_ctx_result,
                    Err(PeaceTestError::PeaceRt(
                        peace::rt_model::Error::ParamsSpecsMismatch {
                            item_ids_with_no_params_specs,
                            params_specs_provided_mismatches,
                            params_specs_stored_mismatches,
                            params_specs_not_usable,
                        }
                    ))
                    if item_ids_with_no_params_specs == &vec![item_id!("new_id")]
                    && matches!(
                        params_specs_provided_mismatches.get(&item_id!("mismatch_id")),
                        Some(ParamsSpec::Value { value: VecA(value) })
                        if value == &vec![2u8]
                    )
                    && matches!(
                        params_specs_stored_mismatches.as_ref(),
                        Some(params_specs_stored_mismatches)
                        if params_specs_stored_mismatches.is_empty()
                    )
                    && params_specs_not_usable.is_empty(),
                ),
                "was {cmd_ctx_result:#?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_ok_when_spec_provided_for_previous_mapping_fn(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(VecCopyItem::ID_DEFAULT.clone()).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_resource(0u8)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA::field_wise_spec()
                .with_0_from_map(|_: &u8| Some(vec![1u8]))
                .build(),
        )
        .await?;

    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(VecCopyItem::ID_DEFAULT.clone()).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA::field_wise_spec()
                .with_0_from_map(|_: &u8| Some(vec![1u8]))
                .build(),
        )
        .with_flow_param(String::from("for_item_mapping"), Some(1u8))
        .await?;

    let fields = cmd_ctx.fields();
    let params_specs = fields.params_specs();
    let resources = fields.resources();
    let vec_a_spec = params_specs
        .get::<ParamsSpec<<VecCopyItem as Item>::Params<'_>>, _>(VecCopyItem::ID_DEFAULT);
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        VecCopyItem::ID_DEFAULT.clone(),
        tynm::type_name::<VecA>(),
    );
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(vec_a_spec,
                    Some(ParamsSpec::FieldWise {
                        field_wise_spec: VecAFieldWise(ValueSpec::<Vec<u8>>::MappingFn(mapping_fn)),
                    })
                    if mapping_fn.is_valued()
                ),
                "was {vec_a_spec:?}"
            );
        }
    })();
    assert_eq!(
        Some(VecA(vec![1u8])),
        vec_a_spec.and_then(|vec_a_spec| vec_a_spec
            .resolve(resources, &mut value_resolution_ctx)
            .ok()),
    );

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_err_when_spec_fully_not_provided_for_previous_mapping_fn(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(VecCopyItem::ID_DEFAULT.clone()).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_resource(0u8)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA::field_wise_spec()
                .with_0_from_map(|_: &u8| Some(vec![1u8]))
                .build(),
        )
        .await?;

    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(VecCopyItem::ID_DEFAULT.clone()).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let cmd_ctx_result = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        // Note: no item_params for `VecCopyItem`
        .build()
        .await;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_ctx_result,
                    Err(PeaceTestError::PeaceRt(
                        peace::rt_model::Error::ParamsSpecsMismatch {
                            item_ids_with_no_params_specs,
                            params_specs_provided_mismatches,
                            params_specs_stored_mismatches,
                            params_specs_not_usable,
                        }
                    ))
                    if item_ids_with_no_params_specs.is_empty()
                    && params_specs_provided_mismatches.is_empty()
                    && matches!(
                        params_specs_stored_mismatches.as_ref(),
                        Some(params_specs_stored_mismatches)
                        if params_specs_stored_mismatches.is_empty()
                    )
                    && params_specs_not_usable == &[VecCopyItem::ID_DEFAULT.clone()],
                ),
                "was {cmd_ctx_result:#?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_err_when_value_spec_not_provided_for_previous_mapping_fn(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(VecCopyItem::ID_DEFAULT.clone()).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_resource(0u8)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA::field_wise_spec()
                .with_0_from_map(|_: &u8| Some(vec![1u8]))
                .build(),
        )
        .await?;

    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(VecCopyItem::ID_DEFAULT.clone()).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let cmd_ctx_result = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        // Note: item_params provided, but not enough to replace mapping function.
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA::field_wise_spec().build(),
        )
        .build()
        .await;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_ctx_result,
                    Err(PeaceTestError::PeaceRt(
                        peace::rt_model::Error::ParamsSpecsMismatch {
                            item_ids_with_no_params_specs,
                            params_specs_provided_mismatches,
                            params_specs_stored_mismatches,
                            params_specs_not_usable,
                        }
                    ))
                    if item_ids_with_no_params_specs.is_empty()
                    && params_specs_provided_mismatches.is_empty()
                    && matches!(
                        params_specs_stored_mismatches.as_ref(),
                        Some(params_specs_stored_mismatches)
                        if params_specs_stored_mismatches.is_empty()
                    )
                    && params_specs_not_usable == &[VecCopyItem::ID_DEFAULT.clone()],
                ),
                "was {cmd_ctx_result:#?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_params_specs_mismatch_err_when_item_renamed(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::new(item_id!("original_id")).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(item_id!("original_id"), VecA(vec![1u8]).into())
        .await?;

    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        // Note: no `"original_id"` item.
        item_graph_builder.add_fn(VecCopyItem::new(item_id!("new_id")).into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let cmd_ctx_result = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(item_id!("mismatch_id"), VecA(vec![2u8]).into())
        .build()
        .await;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_ctx_result,
                    Err(PeaceTestError::PeaceRt(
                        peace::rt_model::Error::ParamsSpecsMismatch {
                            item_ids_with_no_params_specs,
                            params_specs_provided_mismatches,
                            params_specs_stored_mismatches,
                            params_specs_not_usable,
                        }
                    ))
                    if item_ids_with_no_params_specs == &[item_id!("new_id")]
                    && params_specs_provided_mismatches.len() == 1
                    && params_specs_provided_mismatches.iter()
                        .next()
                        .map(|(item_id, params_spec_boxed)| {
                            item_id == &item_id!("mismatch_id")
                            && matches!(
                                BoxDataTypeDowncast::<ParamsSpec<VecA>>::downcast_ref(params_spec_boxed),
                                Some(ParamsSpec::Value { value })
                                if value == &VecA(vec![2u8])
                            )
                        })
                        .unwrap_or(false)
                    && matches!(
                        params_specs_stored_mismatches.as_ref(),
                        Some(params_specs_stored_mismatches)
                        if params_specs_stored_mismatches.is_empty()
                    )
                    && params_specs_not_usable.is_empty()
                ),
                "was {cmd_ctx_result:#?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn build_with_item_params_returns_ok_when_new_item_added_with_params_provided(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spsf_params")).await?;
    let profile = profile!("test_profile");
    let flow_id = flow_id!("test_flow_id");

    // Build first `cmd_ctx` without item.
    let item_graph = ItemGraphBuilder::new().build();
    let flow = Flow::<PeaceTestError>::new(flow_id.clone(), item_graph);

    let mut output = NoOpOutput;
    let _cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .await?;

    // Build second `cmd_ctx` with item.
    let item_graph = {
        let mut item_graph_builder = ItemGraphBuilder::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let flow = Flow::<PeaceTestError>::new(flow_id, item_graph);
    let cmd_ctx = CmdCtxSpsf::<TestCctCmdCtxSpsf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_flow((&flow).into())
        .with_item_params::<VecCopyItem>(VecCopyItem::ID_DEFAULT.clone(), VecA(vec![1u8]).into())
        .await?;

    let fields = cmd_ctx.fields();
    let params_specs = fields.params_specs();
    let resources = fields.resources();
    let vec_a_spec = params_specs
        .get::<ParamsSpec<<VecCopyItem as Item>::Params<'_>>, _>(VecCopyItem::ID_DEFAULT);
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        VecCopyItem::ID_DEFAULT.clone(),
        tynm::type_name::<VecA>(),
    );
    assert!(matches!(vec_a_spec,
        Some(ParamsSpec::Value { value: VecA(value) })
        if value == &[1u8]
    ));
    assert_eq!(
        Some(VecA(vec![1u8])),
        vec_a_spec.and_then(|vec_a_spec| vec_a_spec
            .resolve(resources, &mut value_resolution_ctx)
            .ok()),
    );

    Ok(())
}

#[derive(Debug)]
pub struct TestCctCmdCtxSpsf;

impl CmdCtxTypes for TestCctCmdCtxSpsf {
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
    }

    fn flow_params_register(type_reg: &mut TypeReg<Self::FlowParamsKey>) {
        type_reg.register::<bool>(String::from("flow_param_0"));
        type_reg.register::<u16>(String::from("flow_param_1"));
    }
}
