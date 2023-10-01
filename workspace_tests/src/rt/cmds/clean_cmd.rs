use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt::cmds::{
        ApplyStoredStateSync, CleanCmd, EnsureCmd, StatesCurrentReadCmd, StatesDiscoverCmd,
        StatesGoalReadCmd,
    },
    rt_model::{
        outcomes::CmdOutcome, ApplyCmdError, Error as PeaceRtError, Flow, ItemGraphBuilder,
        StateStoredAndDiscovered, Workspace, WorkspaceSpec,
    },
};

use crate::{
    mock_item::{MockItem, MockItemError, MockSrc, MockState},
    vec_copy_item::VecB,
    NoOpOutput, PeaceTestError, VecA, VecCopyError, VecCopyItem, VecCopyState,
};

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state_when_state_not_ensured()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current states to disk.
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

    // Dry-clean states
    let CmdOutcome {
        value: states_cleaned_dry,
        errors,
    } = CleanCmd::exec_dry(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_cleaned_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState::default()).as_ref(),
        states_current.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState::default()).as_ref(),
        states_cleaned_dry.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert!(errors.is_empty());

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state_when_state_ensured()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current and goal states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Ensure states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    // Dry-clean states.
    let CmdOutcome {
        value: states_clean_dry,
        errors: _,
    } = CleanCmd::exec_dry(&mut cmd_ctx).await?;

    // Re-read states from disk.
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;
    let CmdOutcome {
        value: states_current,
        errors: _,
    } = StatesDiscoverCmd::current(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_clean_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );

    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_current.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_current_stored.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState::default()).as_ref(),
        states_clean_dry.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_when_state_not_ensured()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    StatesDiscoverCmd::current(&mut cmd_ctx).await?;

    // Clean states.
    let CmdOutcome {
        value: states_cleaned,
        errors: _,
    } = CleanCmd::exec(&mut cmd_ctx).await?;

    // Re-read states from disk.
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_when_state_ensured()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current and goal states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Ensure states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    // Clean states.
    let CmdOutcome {
        value: states_cleaned,
        errors: _,
    } = CleanCmd::exec(&mut cmd_ctx).await?;

    // Re-read states from disk.
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn exec_dry_returns_sync_error_when_current_state_out_of_sync()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current and goal states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(0).into())
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Alter states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    // Overwrite states current.
    cmd_ctx
        .resources_mut()
        .insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));

    // Dry-clean states.
    let exec_dry_result =
        CleanCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await;

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &exec_dry_result,
                    Err(PeaceTestError::PeaceRt(PeaceRtError::ApplyCmdError(
                        ApplyCmdError::StatesCurrentOutOfSync { items_state_stored_stale }
                    )))
                    if items_state_stored_stale.len() == 1
                    && matches!(
                        items_state_stored_stale.iter().next(),
                        Some((item_id, state_stored_and_discovered))
                        if item_id == VecCopyItem::ID_DEFAULT
                        && matches!(
                            state_stored_and_discovered,
                            StateStoredAndDiscovered::ValuesDiffer { state_stored, state_discovered }
                            if matches!(
                                BoxDataTypeDowncast::<VecCopyState>::downcast_ref(state_stored),
                                Some(state_stored)
                                if **state_stored == [0, 1, 2, 3]
                            )
                            && matches!(
                                BoxDataTypeDowncast::<VecCopyState>::downcast_ref(state_discovered),
                                Some(state_discovered)
                                if **state_discovered == [0, 1, 2, 3, 4, 5, 6, 7]
                            )
                        ),
                    )
                ),
                "Expected `exec_dry_result` to be \
                `Err(.. {{ ApplyCmdError::StatesCurrentOutOfSync {{ .. }} }})`,\n\
                but was {exec_dry_result:?}",
            );
        }
    })();

    Ok(())
}

/// This should not return an error, because the target state for cleaning is
/// not `state_goal`, but `state_clean`.
#[tokio::test]
async fn exec_dry_does_not_return_sync_error_when_goal_state_out_of_sync()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current and goal states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(0).into())
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Alter states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Dry-clean states.
    let CmdOutcome {
        value: states_cleaned_dry,
        errors: _,
    } = CleanCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Goal).await?;

    // Re-read states from disk.
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;
    let states_goal_stored = StatesGoalReadCmd::exec(&mut cmd_ctx).await?;
    let CmdOutcome {
        value: (states_current, states_goal),
        errors: _,
    } = StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_goal_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_goal.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_cleaned_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn exec_returns_sync_error_when_current_state_out_of_sync()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current and goal states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(0).into())
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Alter states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;
    // Overwrite states current.
    cmd_ctx
        .resources_mut()
        .insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));

    // Clean states.
    let exec_result = CleanCmd::exec_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await;

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &exec_result,
                    Err(PeaceTestError::PeaceRt(PeaceRtError::ApplyCmdError(
                        ApplyCmdError::StatesCurrentOutOfSync { items_state_stored_stale }
                    )))
                    if items_state_stored_stale.len() == 1
                    && matches!(
                        items_state_stored_stale.iter().next(),
                        Some((item_id, state_stored_and_discovered))
                        if item_id == VecCopyItem::ID_DEFAULT
                        && matches!(
                            state_stored_and_discovered,
                            StateStoredAndDiscovered::ValuesDiffer { state_stored, state_discovered }
                            if matches!(
                                BoxDataTypeDowncast::<VecCopyState>::downcast_ref(state_stored),
                                Some(state_stored)
                                if **state_stored == [0, 1, 2, 3]
                            )
                            && matches!(
                                BoxDataTypeDowncast::<VecCopyState>::downcast_ref(state_discovered),
                                Some(state_discovered)
                                if **state_discovered == [0, 1, 2, 3, 4, 5, 6, 7]
                            )
                        ),
                    )
                ),
                "Expected `exec_result` to be \
                `Err(.. {{ ApplyCmdError::StatesCurrentOutOfSync {{ .. }} }})`,\n\
                but was {exec_result:?}",
            );
        }
    })();

    Ok(())
}

/// This should not return an error, because the target state for cleaning is
/// not `state_goal`, but `state_clean`.
#[tokio::test]
async fn exec_does_not_return_sync_error_when_goal_state_out_of_sync()
-> Result<(), Box<dyn std::error::Error>> {
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

    // Write current and goal states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(0).into())
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Alter states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Clean states.
    let CmdOutcome {
        value: states_cleaned,
        errors: _,
    } = CleanCmd::exec_with(&mut cmd_ctx, ApplyStoredStateSync::Goal).await?;

    // Re-read states from disk.
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;
    let states_goal_stored = StatesGoalReadCmd::exec(&mut cmd_ctx).await?;
    let CmdOutcome {
        value: (states_current, states_goal),
        errors: _,
    } = StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_goal_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_goal.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn states_current_not_serialized_on_states_clean_insert_cmd_block_fail()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(
            MockItem::<()>::default()
                .with_state_clean(|_, _| {
                    Err(MockItemError::Synthetic(String::from("state_clean_err")))
                })
                .into(),
        );
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Write current and goal states to disk.
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    let CmdOutcome {
        value: states_cleaned,
        errors,
    } = CleanCmd::exec(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        None,
        states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        None,
        states_cleaned.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_current_stored.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    let mock_error = errors.get(MockItem::<()>::ID_DEFAULT);
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    mock_error,
                    Some(PeaceTestError::Mock(MockItemError::Synthetic(s)))
                    if s == "state_clean_err"
                ),
                "Expected `mock_error` to be \
                `Err(.. {{ MockItemError::Synthetic {{ \"state_clean_err\" }} }})`,\n\
                but was `{mock_error:?}`",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn states_current_not_serialized_on_states_discover_cmd_block_fail()
-> Result<(), Box<dyn std::error::Error>> {
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

    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Write current and goal states to disk.
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    // Reinitialize graph with failing discover.
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(
            MockItem::<()>::default()
                .with_try_state_current(|_, _, _| {
                    Err(MockItemError::Synthetic(String::from(
                        "try_state_current_err",
                    )))
                })
                .into(),
        );
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;

    let CmdOutcome {
        value: states_cleaned,
        errors,
    } = CleanCmd::exec(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        None,
        states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        None,
        states_cleaned.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_current_stored.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    let mock_error = errors.get(MockItem::<()>::ID_DEFAULT);
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    mock_error,
                    Some(PeaceTestError::Mock(MockItemError::Synthetic(s)))
                    if s == "try_state_current_err"
                ),
                "Expected `mock_error` to be \
                `Err(.. {{ MockItemError::Synthetic {{ \"try_state_current_err\" }} }})`,\n\
                but was `{mock_error:?}`",
            );
        }
    })();

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!("{:?}", CleanCmd::<VecCopyError, NoOpOutput, ()>::default());
    assert_eq!(
        r#"CleanCmd(PhantomData<(workspace_tests::vec_copy_item::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#,
        debug_str,
    );
}
