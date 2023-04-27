use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    resources::states::StatesSaved,
    rt::cmds::{sub::StatesSavedReadCmd, EnsureCmd, StatesDiscoverCmd},
    rt_model::{outcomes::CmdOutcome, Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, PeaceTestError, VecA, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn resources_ensured_dry_does_not_alter_state() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let (states_current, _states_desired) =
        StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
    let states_saved = StatesSaved::from(states_current);

    // Dry-ensured states.
    // The returned states are currently the same as `StatesSaved`, but it would be
    // useful to return simulated ensured states.
    let CmdOutcome {
        value: states_ensured_dry,
        errors: _,
    } = EnsureCmd::exec_dry(&mut cmd_ctx, &states_saved).await?;

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let states = resources.borrow::<StatesCurrent>();
    // let states_desired = resources.borrow::<StatesDesired>();
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     states.get::<VecCopyState, _>(&VecCopyItemSpec::ID_DEFAULT)
    // );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     states_desired
    //         .get::<VecCopyState, _>(&VecCopyItemSpec::ID_DEFAULT)
    //
    // );
    // ```

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItemSpec::ID_DEFAULT)
    ); // states_ensured_dry should be the same as the beginning.

    Ok(())
}

#[tokio::test]
async fn resources_ensured_contains_state_ensured_for_each_item_spec_when_state_not_yet_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let (states_current, _states_desired) =
        StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
    let states_saved = StatesSaved::from(states_current);

    // Alter states.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let CmdOutcome {
        value: ensured_states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx, &states_saved).await?;

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let ensured_states_before = resources_ensured.borrow::<StatesCurrent>();
    // let ensured_states_desired = resources_ensured.borrow::<StatesDesired>();
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     ensured_states_before.get::<VecCopyState, _>(&VecCopyItemSpec::ID_DEFAULT)
    // );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     ensured_states_desired
    //         .get::<VecCopyState, _>(&VecCopyItemSpec::ID_DEFAULT)
    //
    // );
    // ```

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured.get::<VecCopyState, _>(VecCopyItemSpec::ID_DEFAULT)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec::ID_DEFAULT)
    );

    Ok(())
}

#[tokio::test]
async fn resources_ensured_contains_state_ensured_for_each_item_spec_when_state_already_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let (states_current, _states_desired) =
        StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
    let states_saved = StatesSaved::from(states_current);

    // Alter states.
    let CmdOutcome {
        value: ensured_states_ensured,
        errors: _,
    } = EnsureCmd::exec(&mut cmd_ctx, &states_saved).await?;
    let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

    // Dry ensure states.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let CmdOutcome {
        value: ensured_states_ensured_dry,
        errors: _,
    } = EnsureCmd::exec_dry(&mut cmd_ctx, &states_saved).await?;

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_spec_params::<VecCopyItemSpec>(
            VecCopyItemSpec::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let ensured_states_before = // StatesCurrent passed in(?) to EnsureCmd
    // let ensured_states_desired = // StatesDesired passed in(?) to EnsureCmd
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     ensured_states_before.get::<VecCopyState,
    // _>(&VecCopyItemSpec::ID_DEFAULT) );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     ensured_states_desired
    //         .get::<VecCopyState, _>(&VecCopyItemSpec::ID_DEFAULT)
    //
    // );
    // ```
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured.get::<VecCopyState, _>(VecCopyItemSpec::ID_DEFAULT)
    ); // states_ensured.logical should be the same as states desired, if all went well.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured_dry.get::<VecCopyState, _>(VecCopyItemSpec::ID_DEFAULT)
    ); // TODO: EnsureDry state should simulate the actual states, not return the actual current state
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec::ID_DEFAULT)
    );

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!("{:?}", EnsureCmd::<VecCopyError, NoOpOutput, ()>::default());
    assert!(
        debug_str
            == r#"EnsureCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#
            || debug_str == r#"EnsureCmd(PhantomData)"#
    );
}
