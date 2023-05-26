use diff::{VecDiff, VecDiffType};
use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    params::ParamsSpec,
    rt::cmds::{DiffCmd, StatesDiscoverCmd},
    rt_model::{
        outcomes::CmdOutcome,
        output::{CliOutput, OutputWrite},
        Flow, ItemGraphBuilder, Workspace, WorkspaceSpec,
    },
};

use crate::{
    NoOpOutput, PeaceTestError, VecA, VecB, VecCopyDiff, VecCopyError, VecCopyItem, VecCopyState,
};

#[tokio::test]
async fn contains_state_diff_for_each_item() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Discover current and desired states.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let CmdOutcome {
        value: (states_current, states_desired),
        errors: _,
    } = StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;

    // Diff current and desired states.
    let state_diffs = DiffCmd::current_and_desired(&mut cmd_ctx).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_desired.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    Ok(())
}

#[tokio::test]
async fn diff_profiles_current_with_multiple_profiles() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // profile_0
    let profile_0 = profile!("test_profile_0");
    let mut cmd_ctx_0 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_0.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let CmdOutcome {
        value: states_current_0,
        errors: _,
    } = StatesDiscoverCmd::current(&mut cmd_ctx_0).await?;

    // profile_1
    let profile_1 = profile!("test_profile_1");
    let mut cmd_ctx_1 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_1.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let resources = cmd_ctx_1.resources_mut();
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    let CmdOutcome {
        value: states_current_1,
        errors: _,
    } = StatesDiscoverCmd::current(&mut cmd_ctx_1).await?;

    let mut cmd_ctx_multi = CmdCtx::builder_multi_profile_single_flow(&mut output, &workspace)
        .with_flow(&flow)
        .await?;

    // Diff current states for profile_0 and profile_1.
    let state_diffs =
        DiffCmd::diff_profiles_current(&mut cmd_ctx_multi, &profile_0, &profile_1).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current_0.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current_1.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    Ok(())
}

#[tokio::test]
async fn diff_profiles_current_with_missing_profile_0() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // profile_0
    let profile_0 = profile!("test_profile_0");

    // profile_1
    let profile_1 = profile!("test_profile_1");
    let mut cmd_ctx_1 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_1.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let resources = cmd_ctx_1.resources_mut();
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    StatesDiscoverCmd::current(&mut cmd_ctx_1).await?;

    let mut cmd_ctx_multi = CmdCtx::builder_multi_profile_single_flow(&mut output, &workspace)
        .with_flow(&flow)
        .await?;

    let diff_result =
        DiffCmd::diff_profiles_current(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRtError(
                peace::rt_model::Error::ProfileNotInScope { profile, profiles_in_scope }
            ))
            if profile == profile_0 && profiles_in_scope == vec![profile_1]));

    Ok(())
}

#[tokio::test]
async fn diff_profiles_current_with_missing_profile_1() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // profile_0
    let profile_0 = profile!("test_profile_0");
    let mut cmd_ctx_0 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_0.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    StatesDiscoverCmd::current(&mut cmd_ctx_0).await?;

    // profile_1
    let profile_1 = profile!("test_profile_1");

    let mut cmd_ctx_multi = CmdCtx::builder_multi_profile_single_flow(&mut output, &workspace)
        .with_flow(&flow)
        .await?;

    let diff_result =
        DiffCmd::diff_profiles_current(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRtError(
                peace::rt_model::Error::ProfileNotInScope { profile, profiles_in_scope }
            ))
            if profile == profile_1 && profiles_in_scope == vec![profile_0]));

    Ok(())
}

#[tokio::test]
async fn diff_profiles_current_with_profile_0_missing_states_current()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // profile_0
    let profile_0 = profile!("test_profile_0");
    let mut cmd_ctx_0 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_0.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    StatesDiscoverCmd::desired(&mut cmd_ctx_0).await?;

    // profile_1
    let profile_1 = profile!("test_profile_1");
    let mut cmd_ctx_1 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_1.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let resources = cmd_ctx_1.resources_mut();
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    StatesDiscoverCmd::current(&mut cmd_ctx_1).await?;

    let mut cmd_ctx_multi = CmdCtx::builder_multi_profile_single_flow(&mut output, &workspace)
        .with_flow(&flow)
        .await?;

    let diff_result =
        DiffCmd::diff_profiles_current(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRtError(
                peace::rt_model::Error::ProfileStatesCurrentNotDiscovered { profile }
            ))
            if profile == profile_0));

    Ok(())
}

#[tokio::test]
async fn diff_profiles_current_with_profile_1_missing_states_current()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // profile_0
    let profile_0 = profile!("test_profile_0");
    let mut cmd_ctx_0 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_0.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    StatesDiscoverCmd::current(&mut cmd_ctx_0).await?;

    // profile_1
    let profile_1 = profile!("test_profile_1");
    let mut cmd_ctx_1 = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile_1.clone())
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    StatesDiscoverCmd::desired(&mut cmd_ctx_1).await?;

    let mut cmd_ctx_multi = CmdCtx::builder_multi_profile_single_flow(&mut output, &workspace)
        .with_flow(&flow)
        .await?;

    let diff_result =
        DiffCmd::diff_profiles_current(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRtError(
                peace::rt_model::Error::ProfileStatesCurrentNotDiscovered { profile }
            ))
            if profile == profile_1));

    Ok(())
}

#[tokio::test]
async fn diff_with_multiple_changes() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut buffer = Vec::with_capacity(256);
    let mut output = CliOutput::new_with_writer(&mut buffer);

    // Discover current and desired states.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(VecCopyItem::ID_DEFAULT.clone(), ParamsSpec::InMemory)
        .await?;
    // overwrite initial state
    let resources = cmd_ctx.resources_mut();
    #[rustfmt::skip]
    resources.insert(VecA(vec![0, 1, 2,    4, 5, 6, 8, 9]));
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    let CmdOutcome {
        value: (states_current, states_desired),
        errors: _,
    } = StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;

    // Diff current and desired states.
    let state_diffs = DiffCmd::current_and_desired(&mut cmd_ctx).await?;
    <_ as OutputWrite<PeaceTestError>>::present(cmd_ctx.output_mut(), &state_diffs).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 4, 5, 6, 8, 9])).as_ref(),
        states_desired.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![
            VecDiffType::Removed { index: 3, len: 1 },
            VecDiffType::Altered {
                index: 7,
                changes: vec![1] // 8 - 7 = 1
            },
            VecDiffType::Inserted {
                index: 8,
                changes: vec![9]
            },
        ])))
        .as_ref(),
        vec_diff
    );
    assert_eq!(
        "1. `vec_copy`: [(-)3..4, (~)7;1, (+)8;9, ]\n",
        String::from_utf8(buffer)?
    );

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!("{:?}", DiffCmd::<VecCopyError>::default());
    assert!(
        debug_str == r#"DiffCmd(PhantomData<workspace_tests::vec_copy_item::VecCopyError>)"#
            || debug_str == r#"DiffCmd(PhantomData)"#
    );
}
