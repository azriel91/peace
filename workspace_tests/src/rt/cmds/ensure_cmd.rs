use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt::cmds::{ApplyStoredStateSync, EnsureCmd, StatesCurrentReadCmd, StatesDiscoverCmd},
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
async fn resources_ensured_dry_does_not_alter_state() -> Result<(), Box<dyn std::error::Error>> {
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

    // Dry-ensured states.
    // The returned states are currently the same as `StatesCurrentStored`, but it
    // would be useful to return simulated ensured states.
    let CmdOutcome {
        value: states_ensured_dry,
        errors: _,
    } = EnsureCmd::exec_dry(&mut cmd_ctx).await?;

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let states = resources.borrow::<StatesCurrent>();
    // let states_goal = resources.borrow::<StatesGoal>();
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     states.get::<VecCopyState, _>(&VecCopyItem::ID_DEFAULT)
    // );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     states_goal
    //         .get::<VecCopyState, _>(&VecCopyItem::ID_DEFAULT)
    //
    // );
    // ```

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_ensured_dry.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn resources_ensured_contains_state_ensured_for_each_item_when_state_not_yet_ensured()
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

    // Alter states.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let ensured_states_before = resources_ensured.borrow::<StatesCurrent>();
    // let ensured_states_goal = resources_ensured.borrow::<StatesGoal>();
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     ensured_states_before.get::<VecCopyState, _>(&VecCopyItem::ID_DEFAULT)
    // );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     ensured_states_goal
    //         .get::<VecCopyState, _>(&VecCopyItem::ID_DEFAULT)
    //
    // );
    // ```

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_current_stored.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn resources_ensured_contains_state_ensured_for_each_item_when_state_already_ensured()
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

    // Alter states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    // Dry ensure states.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(0).into())
        .await?;
    // Changing params changes VecCopyItem state_goal
    StatesDiscoverCmd::goal(&mut cmd_ctx).await?;
    let CmdOutcome {
        value: states_ensured_dry,
        errors: _,
    } = EnsureCmd::exec_dry(&mut cmd_ctx).await?;

    // Re-read states from disk.
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
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let ensured_states_before = // StatesCurrent passed in(?) to EnsureCmd
    // let ensured_states_goal = // StatesGoal passed in(?) to EnsureCmd
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     ensured_states_before.get::<VecCopyState,
    // _>(&VecCopyItem::ID_DEFAULT) );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     ensured_states_goal
    //         .get::<VecCopyState, _>(&VecCopyItem::ID_DEFAULT)
    //
    // );
    // ```
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current_stored.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );

    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured_dry.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(1)).as_ref(),
        states_current_stored.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
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

    // Dry ensure states.
    let exec_dry_result =
        EnsureCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await;

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
                but was `{exec_dry_result:?}`",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn exec_dry_returns_sync_error_when_goal_state_out_of_sync()
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

    // Dry ensure states.
    let exec_dry_result = EnsureCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Goal).await;

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
                        ApplyCmdError::StatesGoalOutOfSync { items_state_stored_stale }
                    )))
                    if items_state_stored_stale.len() == 2
                    && matches!(
                        items_state_stored_stale.get(VecCopyItem::ID_DEFAULT),
                        Some(state_stored_and_discovered)
                        if matches!(
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
                    && matches!(
                        items_state_stored_stale.get(MockItem::<()>::ID_DEFAULT),
                        Some(state_stored_and_discovered)
                        if matches!(
                            state_stored_and_discovered,
                            StateStoredAndDiscovered::ValuesDiffer { state_stored, state_discovered }
                            if matches!(
                                BoxDataTypeDowncast::<MockState>::downcast_ref(state_stored),
                                Some(state_stored)
                                if **state_stored == 0
                            )
                            && matches!(
                                BoxDataTypeDowncast::<MockState>::downcast_ref(state_discovered),
                                Some(state_discovered)
                                if **state_discovered == 1
                            )
                        ),
                    )
                ),
                "Expected `exec_dry_result` to be \
                `Err(.. {{ ApplyCmdError::StatesGoalOutOfSync {{ .. }} }})`,\n\
                but was `{exec_dry_result:?}`",
            );
        }
    })();

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

    // Ensure states.
    let exec_result = EnsureCmd::exec_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await;

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

#[tokio::test]
async fn exec_returns_sync_error_when_goal_state_out_of_sync()
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

    // Ensure states.
    let exec_result = EnsureCmd::exec_with(&mut cmd_ctx, ApplyStoredStateSync::Goal).await;

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
                        ApplyCmdError::StatesGoalOutOfSync { items_state_stored_stale }
                    )))
                    if items_state_stored_stale.len() == 2
                    && matches!(
                        items_state_stored_stale.get(VecCopyItem::ID_DEFAULT),
                        Some(state_stored_and_discovered)
                        if matches!(
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
                    && matches!(
                        items_state_stored_stale.get(MockItem::<()>::ID_DEFAULT),
                        Some(state_stored_and_discovered)
                        if matches!(
                            state_stored_and_discovered,
                            StateStoredAndDiscovered::ValuesDiffer { state_stored, state_discovered }
                            if matches!(
                                BoxDataTypeDowncast::<MockState>::downcast_ref(state_stored),
                                Some(state_stored)
                                if **state_stored == 0
                            )
                            && matches!(
                                BoxDataTypeDowncast::<MockState>::downcast_ref(state_discovered),
                                Some(state_discovered)
                                if **state_discovered == 1
                            )
                        ),
                    )
                ),
                "Expected `exec_result` to be \
                `Err(.. {{ ApplyCmdError::StatesGoalOutOfSync {{ .. }} }})`,\n\
                but was {exec_result:?}",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn exec_dry_returns_item_error_when_item_discover_current_returns_error()
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

    // Create new `cmd_ctx` with failing state current discovery.
    let mut output = NoOpOutput;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(
            MockItem::<()>::default()
                .with_state_current(|_fn_ctx, _mock_src, _data| {
                    Err(MockItemError::Synthetic(String::from("state_current_err")))
                })
                .into(),
        );
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Dry ensure states.
    let CmdOutcome {
        value: states_ensured_dry,
        errors,
    } = EnsureCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    let mock_error = errors.get(MockItem::<()>::ID_DEFAULT);
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    mock_error,
                    Some(PeaceTestError::Mock(MockItemError::Synthetic(s)))
                    if s == "state_current_err"
                ),
                "Expected `mock_error` to be \
                `Err(.. {{ MockItemError::Synthetic {{ \"state_current_err\" }} }})`,\n\
                but was `{mock_error:?}`",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn exec_dry_returns_item_error_when_item_discover_goal_returns_error()
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

    // Create new `cmd_ctx` with failing state current discovery.
    let mut output = NoOpOutput;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(
            MockItem::<()>::default()
                .with_state_goal(|_fn_ctx, _mock_src, _data| {
                    Err(MockItemError::Synthetic(String::from("state_goal_err")))
                })
                .into(),
        );
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Dry ensure states.
    let CmdOutcome {
        value: states_ensured_dry,
        errors,
    } = EnsureCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    let mock_error = errors.get(MockItem::<()>::ID_DEFAULT);
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    mock_error,
                    Some(PeaceTestError::Mock(MockItemError::Synthetic(s)))
                    if s == "state_goal_err"
                ),
                "Expected `mock_error` to be \
                `Err(.. {{ MockItemError::Synthetic {{ \"state_goal_err\" }} }})`,\n\
                but was `{mock_error:?}`",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn exec_dry_returns_item_error_when_item_apply_check_returns_error()
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

    // Create new `cmd_ctx` with failing state current discovery.
    let mut output = NoOpOutput;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(
            MockItem::<()>::default()
                .with_apply_check(|_, _, _, _, _| {
                    Err(MockItemError::Synthetic(String::from("apply_check_err")))
                })
                .into(),
        );
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Dry ensure states.
    let CmdOutcome {
        value: states_ensured_dry,
        errors,
    } = EnsureCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    let mock_error = errors.get(MockItem::<()>::ID_DEFAULT);
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    mock_error,
                    Some(PeaceTestError::Mock(MockItemError::Synthetic(s)))
                    if s == "apply_check_err"
                ),
                "Expected `mock_error` to be \
                `Err(.. {{ MockItemError::Synthetic {{ \"apply_check_err\" }} }})`,\n\
                but was `{mock_error:?}`",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn exec_dry_returns_item_error_when_item_apply_dry_returns_error()
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

    // Create new `cmd_ctx` with failing state current discovery.
    let mut output = NoOpOutput;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(
            MockItem::<()>::default()
                .with_apply_dry(|_, _, _, _, _, _| {
                    Err(MockItemError::Synthetic(String::from("apply_dry_err")))
                })
                .into(),
        );
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Dry ensure states.
    let CmdOutcome {
        value: states_ensured_dry,
        errors,
    } = EnsureCmd::exec_dry_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    let mock_error = errors.get(MockItem::<()>::ID_DEFAULT);
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    mock_error,
                    Some(PeaceTestError::Mock(MockItemError::Synthetic(s)))
                    if s == "apply_dry_err"
                ),
                "Expected `mock_error` to be \
                `Err(.. {{ MockItemError::Synthetic {{ \"apply_dry_err\" }} }})`,\n\
                but was `{mock_error:?}`",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn exec_returns_item_error_when_item_apply_returns_error()
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

    // Create new `cmd_ctx` with failing state current discovery.
    let mut output = NoOpOutput;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.add_fn(
            MockItem::<()>::default()
                .with_apply(|_, _, _, _, _, _| {
                    Err(MockItemError::Synthetic(String::from("apply_err")))
                })
                .into(),
        );
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    // Ensure states again.
    let CmdOutcome {
        value: states_ensured_again,
        errors,
    } = EnsureCmd::exec_with(&mut cmd_ctx, ApplyStoredStateSync::Current).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    assert_eq!(
        Some(MockState(0)).as_ref(),
        states_ensured.get::<MockState, _>(MockItem::<()>::ID_DEFAULT)
    );

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_again.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
    let mock_error = errors.get(MockItem::<()>::ID_DEFAULT);
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    mock_error,
                    Some(PeaceTestError::Mock(MockItemError::Synthetic(s)))
                    if s == "apply_err"
                ),
                "Expected `mock_error` to be \
                `Err(.. {{ MockItemError::Synthetic {{ \"apply_err\" }} }})`,\n\
                but was `{mock_error:?}`",
            );
        }
    })();

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!("{:?}", EnsureCmd::<VecCopyError, NoOpOutput, ()>::default());
    assert_eq!(
        r#"EnsureCmd(PhantomData<(workspace_tests::vec_copy_item::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#,
        debug_str,
    );
}
