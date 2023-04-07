use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    cmd::ctx::CmdCtx,
    resources::states::StatesSaved,
    rt::cmds::{sub::StatesSavedReadCmd, CleanCmd, EnsureCmd, StatesDiscoverCmd},
    rt_model::{outcomes::CmdOutcome, Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, PeaceTestError, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state_when_state_not_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Write current states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    let states_current = StatesDiscoverCmd::current(&mut cmd_ctx).await?;
    let states_saved = StatesSaved::from(states_current);

    // Dry-clean states
    let CmdOutcome {
        value: states_cleaned_dry,
        errors,
    } = CleanCmd::exec_dry(&mut cmd_ctx, &states_saved).await?;

    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_cleaned_dry.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    let (states_current, _states_desired) =
        StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
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
    let states_current = StatesDiscoverCmd::current(&mut cmd_ctx).await?;
    let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_spec_when_state_not_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Write current states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    let states_current = StatesDiscoverCmd::current(&mut cmd_ctx).await?;
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
        cleaned_states_cleaned.get::<VecCopyState, _>(VecCopyItemSpec.id())
    ); // states_cleaned.logical should be empty, if all went well.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_spec_when_state_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    let (states_current, _states_desired) =
        StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
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
        states_ensured.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        cleaned_states_cleaned.get::<VecCopyState, _>(VecCopyItemSpec.id())
    ); // states_cleaned.logical should be empty, if all went well.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!("{:?}", CleanCmd::<VecCopyError, NoOpOutput, ()>::default());
    assert!(
        debug_str
            == r#"CleanCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#
            || debug_str == r#"CleanCmd(PhantomData)"#
    );
}
