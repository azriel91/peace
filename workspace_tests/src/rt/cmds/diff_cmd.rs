use diff::{VecDiff, VecDiffType};
use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    params::ParamsSpec,
    rt::cmds::{
        DiffCmd, DiffStateSpec, StatesCurrentReadCmd, StatesDiscoverCmd, StatesGoalReadCmd,
    },
    rt_model::{
        outcomes::CmdOutcome,
        output::{CliOutput, OutputWrite},
        Flow, ItemGraphBuilder, Workspace, WorkspaceSpec,
    },
};

use crate::{
    mock_item::{MockDest, MockDiff, MockItem, MockSrc, MockState},
    NoOpOutput, PeaceTestError, VecA, VecB, VecCopyDiff, VecCopyError, VecCopyItem, VecCopyState,
};

#[tokio::test]
async fn diff_stored_contains_state_diff_for_each_item() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(MockItem::<()>::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Discover current and goal states.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    let CmdOutcome {
        value: (states_current, states_goal),
        errors: _,
    } = StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Diff current and goal states.
    let state_diffs = DiffCmd::diff_stored(&mut cmd_ctx).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_current_state = states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_goal_state = states_goal.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(Some(VecCopyState::new()).as_ref(), vec_copy_current_state);
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        vec_copy_goal_state
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    let mock_diff = state_diffs.get::<MockDiff, _>(MockItem::<()>::ID_DEFAULT);
    let mock_current_state = states_current.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);
    let mock_goal_state = states_goal.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);

    assert_eq!(Some(MockState(0)).as_ref(), mock_current_state);
    assert_eq!(Some(MockState(1)).as_ref(), mock_goal_state);
    assert_eq!(Some(MockDiff(1)).as_ref(), mock_diff);

    Ok(())
}

#[tokio::test]
async fn diff_discover_current_on_demand() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(MockItem::<()>::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Discover goal state.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    let CmdOutcome {
        value: states_goal,
        errors: _,
    } = StatesDiscoverCmd::goal(&mut cmd_ctx).await?;

    // Diff current and stored goal states.
    let state_diffs = DiffCmd::diff(
        &mut cmd_ctx,
        DiffStateSpec::Current,
        DiffStateSpec::GoalStored,
    )
    .await?
    .value;

    let states_current = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_current_state = states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_goal_state = states_goal.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(Some(VecCopyState::new()).as_ref(), vec_copy_current_state);
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        vec_copy_goal_state
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    let mock_diff = state_diffs.get::<MockDiff, _>(MockItem::<()>::ID_DEFAULT);
    let mock_current_state = states_current.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);
    let mock_goal_state = states_goal.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);

    assert_eq!(Some(MockState(0)).as_ref(), mock_current_state);
    assert_eq!(Some(MockState(1)).as_ref(), mock_goal_state);
    assert_eq!(Some(MockDiff(1)).as_ref(), mock_diff);

    Ok(())
}

#[tokio::test]
async fn diff_discover_goal_on_demand() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(MockItem::<()>::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Discover current state.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    let CmdOutcome {
        value: states_current,
        errors: _,
    } = StatesDiscoverCmd::current(&mut cmd_ctx).await?;

    // Diff current and stored goal states.
    let state_diffs = DiffCmd::diff(
        &mut cmd_ctx,
        DiffStateSpec::CurrentStored,
        DiffStateSpec::Goal,
    )
    .await?
    .value;

    let states_goal = StatesGoalReadCmd::exec(&mut cmd_ctx).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_current_state = states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_goal_state = states_goal.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(Some(VecCopyState::new()).as_ref(), vec_copy_current_state);
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        vec_copy_goal_state
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    let mock_diff = state_diffs.get::<MockDiff, _>(MockItem::<()>::ID_DEFAULT);
    let mock_current_state = states_current.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);
    let mock_goal_state = states_goal.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);

    assert_eq!(Some(MockState(0)).as_ref(), mock_current_state);
    assert_eq!(Some(MockState(1)).as_ref(), mock_goal_state);
    assert_eq!(Some(MockDiff(1)).as_ref(), mock_diff);

    Ok(())
}

#[tokio::test]
async fn diff_discover_current_and_goal_on_demand() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(MockItem::<()>::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // No discovery of current or goal states before diffing.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Diff current and stored goal states.
    let state_diffs = DiffCmd::diff(&mut cmd_ctx, DiffStateSpec::Current, DiffStateSpec::Goal)
        .await?
        .value;

    let states_current = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;
    let states_goal = StatesGoalReadCmd::exec(&mut cmd_ctx).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_current_state = states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_goal_state = states_goal.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(Some(VecCopyState::new()).as_ref(), vec_copy_current_state);
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        vec_copy_goal_state
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    let mock_diff = state_diffs.get::<MockDiff, _>(MockItem::<()>::ID_DEFAULT);
    let mock_current_state = states_current.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);
    let mock_goal_state = states_goal.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);

    assert_eq!(Some(MockState(0)).as_ref(), mock_current_state);
    assert_eq!(Some(MockState(1)).as_ref(), mock_goal_state);
    assert_eq!(Some(MockDiff(1)).as_ref(), mock_diff);

    Ok(())
}

#[tokio::test]
async fn diff_stored_with_multiple_profiles() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(MockItem::<()>::default().into());
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
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    let resources = cmd_ctx_0.resources_mut();
    resources.insert(MockDest(1));
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
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(3).into())
        .await?;
    let resources = cmd_ctx_1.resources_mut();
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    resources.insert(MockDest(3));
    let CmdOutcome {
        value: states_current_1,
        errors: _,
    } = StatesDiscoverCmd::current(&mut cmd_ctx_1).await?;

    let mut cmd_ctx_multi = CmdCtx::builder_multi_profile_single_flow(&mut output, &workspace)
        .with_flow(&flow)
        .await?;

    // Diff current states for profile_0 and profile_1.
    let state_diffs =
        DiffCmd::diff_current_stored(&mut cmd_ctx_multi, &profile_0, &profile_1).await?;

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

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_0_current_state = states_current_0.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    let vec_copy_1_current_state = states_current_1.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(Some(VecCopyState::new()).as_ref(), vec_copy_0_current_state);
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        vec_copy_1_current_state
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    let mock_diff = state_diffs.get::<MockDiff, _>(MockItem::<()>::ID_DEFAULT);
    let mock_0_current_state = states_current_0.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);
    let mock_1_current_state = states_current_1.get::<MockState, _>(MockItem::<()>::ID_DEFAULT);

    assert_eq!(Some(MockState(1)).as_ref(), mock_0_current_state);
    assert_eq!(Some(MockState(3)).as_ref(), mock_1_current_state);
    assert_eq!(Some(MockDiff(2)).as_ref(), mock_diff);

    Ok(())
}

#[tokio::test]
async fn diff_stored_with_missing_profile_0() -> Result<(), Box<dyn std::error::Error>> {
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
        DiffCmd::diff_current_stored(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRt(
                peace::rt_model::Error::ProfileNotInScope { profile, profiles_in_scope }
            ))
            if profile == profile_0 && profiles_in_scope == vec![profile_1]));

    Ok(())
}

#[tokio::test]
async fn diff_stored_with_missing_profile_1() -> Result<(), Box<dyn std::error::Error>> {
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
        DiffCmd::diff_current_stored(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRt(
                peace::rt_model::Error::ProfileNotInScope { profile, profiles_in_scope }
            ))
            if profile == profile_1 && profiles_in_scope == vec![profile_0]));

    Ok(())
}

#[tokio::test]
async fn diff_stored_with_profile_0_missing_states_current()
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
    StatesDiscoverCmd::goal(&mut cmd_ctx_0).await?;

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
        DiffCmd::diff_current_stored(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRt(
                peace::rt_model::Error::ProfileStatesCurrentNotDiscovered { profile }
            ))
            if profile == profile_0));

    Ok(())
}

#[tokio::test]
async fn diff_stored_with_profile_1_missing_states_current()
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
    StatesDiscoverCmd::goal(&mut cmd_ctx_1).await?;

    let mut cmd_ctx_multi = CmdCtx::builder_multi_profile_single_flow(&mut output, &workspace)
        .with_flow(&flow)
        .await?;

    let diff_result =
        DiffCmd::diff_current_stored(&mut cmd_ctx_multi, &profile_0, &profile_1).await;

    assert!(matches!(
            diff_result,
            Err(PeaceTestError::PeaceRt(
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

    // Discover current and goal states.
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
        value: (states_current, states_goal),
        errors: _,
    } = StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Diff current and goal states.
    let state_diffs = DiffCmd::diff_stored(&mut cmd_ctx).await?;
    <_ as OutputWrite<PeaceTestError>>::present(cmd_ctx.output_mut(), &state_diffs).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 4, 5, 6, 8, 9])).as_ref(),
        states_goal.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
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
    let debug_str = format!("{:?}", DiffCmd::<VecCopyError, (), (), ()>::default());
    assert_eq!(
        r#"DiffCmd(PhantomData<(workspace_tests::vec_copy_item::VecCopyError, &(), (), ())>)"#,
        debug_str,
    );
}
