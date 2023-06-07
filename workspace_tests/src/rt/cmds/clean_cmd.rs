use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    resources::states::StatesSaved,
    rt::cmds::{CleanCmd, EnsureCmd, StatesDiscoverCmd, StatesSavedReadCmd},
    rt_model::{outcomes::CmdOutcome, Flow, ItemGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, PeaceTestError, VecA, VecCopyError, VecCopyItem, VecCopyState};

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
    let states_saved = StatesSaved::from(states_current);

    // Dry-clean states
    let CmdOutcome {
        value: states_cleaned_dry,
        errors,
    } = CleanCmd::exec_dry(&mut cmd_ctx, &states_saved).await?;

    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
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
    let CmdOutcome {
        value: (states_current, _states_goal),
        errors: _,
    } = StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;
    let states_saved = StatesSaved::from(states_current);

    // Ensure states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx, &states_saved).await?;

    // Clean states.
    CleanCmd::exec_dry(&mut cmd_ctx, &states_saved).await?;

    // Re-read states from disk.
    CleanCmd::exec_dry(&mut cmd_ctx, &states_saved).await?;
    let CmdOutcome {
        value: states_current,
        errors: _,
    } = StatesDiscoverCmd::current(&mut cmd_ctx).await?;
    let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

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
        states_saved.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
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
    let CmdOutcome {
        value: states_current,
        errors: _,
    } = StatesDiscoverCmd::current(&mut cmd_ctx).await?;
    let states_saved = StatesSaved::from(states_current);

    // Clean states.
    let CmdOutcome {
        value: cleaned_states_cleaned,
        errors: _,
    } = CleanCmd::exec(&mut cmd_ctx, &states_saved).await?;

    // Re-read states from disk.
    let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        cleaned_states_cleaned.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
    ); // states_cleaned.logical should be empty, if all went well.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
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
    let CmdOutcome {
        value: (states_current, _states_goal),
        errors: _,
    } = StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;
    let states_saved = StatesSaved::from(states_current);

    // Ensure states.
    let CmdOutcome {
        value: states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx, &states_saved).await?;

    // Clean states.
    let CmdOutcome {
        value: cleaned_states_cleaned,
        errors: _,
    } = CleanCmd::exec(&mut cmd_ctx, &states_saved).await?;

    // Re-read states from disk.
    let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

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
        states_saved.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT)
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
