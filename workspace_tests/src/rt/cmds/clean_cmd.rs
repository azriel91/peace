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
    vec_copy_item::VecB, NoOpOutput, PeaceTestError, VecA, VecCopyError, VecCopyItem, VecCopyState,
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
    ); // states_cleaned_dry should be the same as the beginning.
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
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Ensure states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    // Dry-clean states.
    CleanCmd::exec_dry(&mut cmd_ctx).await?;

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
        .await?;
    StatesDiscoverCmd::current(&mut cmd_ctx).await?;

    // Clean states.
    let CmdOutcome {
        value: cleaned_states_cleaned,
        errors: _,
    } = CleanCmd::exec(&mut cmd_ctx).await?;

    // Re-read states from disk.
    let states_current_stored = StatesCurrentReadCmd::exec(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        cleaned_states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    ); // states_cleaned.logical should be empty, if all went well.
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
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Ensure states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx).await?;

    // Clean states.
    let CmdOutcome {
        value: cleaned_states_cleaned,
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
        cleaned_states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    ); // states_cleaned.logical should be empty, if all went well.
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
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Alter states.
    let CmdOutcome {
        value: ensured_states_ensured,
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
        .await?;
    // Overwrite states current.
    cmd_ctx
        .resources_mut()
        .insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));

    // Dry-clean states.
    let exec_dry_result =
        CleanCmd::exec_dry_with(&mut cmd_ctx.as_standalone(), ApplyStoredStateSync::Current).await;

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3])).as_ref(),
        ensured_states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
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
        "Expected `exec_dry_result` to be `Err(.. {{ ApplyCmdError::StatesCurrentOutOfSync {{ .. }} }})`,\n\
        but was {exec_dry_result:?}",
    );

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
        .await?;

    // Dry-clean states.
    let CmdOutcome {
        value: states_cleaned_dry,
        errors: _,
    } = CleanCmd::exec_dry_with(&mut cmd_ctx.as_standalone(), ApplyStoredStateSync::Goal).await?;

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
        .await?;
    StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;

    // Alter states.
    let CmdOutcome {
        value: ensured_states_ensured,
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
        .await?;
    // Overwrite states current.
    cmd_ctx
        .resources_mut()
        .insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));

    // Clean states.
    let exec_result =
        CleanCmd::exec_with(&mut cmd_ctx.as_standalone(), ApplyStoredStateSync::Current).await;

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3])).as_ref(),
        ensured_states_ensured.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    );
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
        "Expected `exec_result` to be `Err(.. {{ ApplyCmdError::StatesCurrentOutOfSync {{ .. }} }})`,\n\
        but was {exec_result:?}",
    );

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
        .await?;

    // Clean states.
    let CmdOutcome {
        value: states_cleaned,
        errors: _,
    } = CleanCmd::exec_with(&mut cmd_ctx.as_standalone(), ApplyStoredStateSync::Goal).await?;

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

#[test]
fn debug() {
    let debug_str = format!("{:?}", CleanCmd::<VecCopyError, NoOpOutput, ()>::default());
    assert!(
        debug_str
            == r#"CleanCmd(PhantomData<(workspace_tests::vec_copy_item::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#
            || debug_str == r#"CleanCmd(PhantomData)"#
    );
}
