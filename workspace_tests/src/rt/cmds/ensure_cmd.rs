use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    cmd::ctx::CmdCtx,
    resources::states::{StatesEnsured, StatesEnsuredDry, StatesSaved},
    rt::cmds::{sub::StatesSavedReadCmd, EnsureCmd, StatesDiscoverCmd},
    rt_model::{Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, PeaceTestError, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn resources_ensured_dry_does_not_alter_state() -> Result<(), Box<dyn std::error::Error>> {
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
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    StatesDiscoverCmd::exec(cmd_ctx).await?;

    // Re-read states from disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = EnsureCmd::exec_dry(cmd_ctx).await?;
    let resources = cmd_ctx.resources();

    let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let states = resources.borrow::<StatesCurrent>();
    // let states_desired = resources.borrow::<StatesDesired>();
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     states.get::<VecCopyState, _>(&VecCopyItemSpec.id())
    // );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     states_desired
    //         .get::<VecCopyState, _>(&VecCopyItemSpec.id())
    //
    // );
    // ```

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_ensured_dry.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    StatesDiscoverCmd::exec(cmd_ctx).await?;

    // Alter states.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = EnsureCmd::exec(cmd_ctx).await?;
    let resources_ensured = cmd_ctx.resources();

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let resources_reread = cmd_ctx.resources();

    let ensured_states_ensured = resources_ensured.borrow::<StatesEnsured>();
    let states_saved = resources_reread.borrow::<StatesSaved>();

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let ensured_states_before = resources_ensured.borrow::<StatesCurrent>();
    // let ensured_states_desired = resources_ensured.borrow::<StatesDesired>();
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     ensured_states_before.get::<VecCopyState, _>(&VecCopyItemSpec.id())
    // );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     ensured_states_desired
    //         .get::<VecCopyState, _>(&VecCopyItemSpec.id())
    //
    // );
    // ```

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    StatesDiscoverCmd::exec(cmd_ctx).await?;

    // Alter states.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = EnsureCmd::exec(cmd_ctx).await?;
    let resources_ensured = cmd_ctx.resources();

    // Dry ensure states.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = EnsureCmd::exec_dry(cmd_ctx).await?;
    let resources_ensured_dry = cmd_ctx.resources();

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let resources_reread = cmd_ctx.resources();

    let ensured_states_ensured = resources_ensured.borrow::<StatesEnsured>();
    let ensured_states_ensured_dry = resources_ensured_dry.borrow::<StatesEnsuredDry>();
    let states_saved = resources_reread.borrow::<StatesSaved>();

    // TODO: When EnsureCmd returns the execution report, assert on the state that
    // was discovered.
    //
    // ```rust,ignore
    // let ensured_states_before = resources_ensured.borrow::<StatesCurrent>();
    // let ensured_states_desired = resources_ensured.borrow::<StatesDesired>();
    // assert_eq!(
    //     Some(VecCopyState::new()).as_ref(),
    //     ensured_states_before.get::<VecCopyState,
    // _>(&VecCopyItemSpec.id()) );
    // assert_eq!(
    //     Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
    //     ensured_states_desired
    //         .get::<VecCopyState, _>(&VecCopyItemSpec.id())
    //
    // );
    // ```
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured.get::<VecCopyState, _>(VecCopyItemSpec.id())
    ); // states_ensured.logical should be the same as states desired, if all went well.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured_dry.get::<VecCopyState, _>(VecCopyItemSpec.id())
    ); // TODO: EnsureDry state should simulate the actual states, not return the actual current state
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
