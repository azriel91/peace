use peace::{
    cfg::{
        profile,
        state::{Nothing, Placeholder},
        FlowId, ItemSpec, Profile, State,
    },
    resources::states::{StatesCurrent, StatesDesired, StatesEnsured, StatesEnsuredDry},
    rt::cmds::{sub::StatesCurrentReadCmd, EnsureCmd, StatesDiscoverCmd},
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn resources_ensured_dry_does_not_alter_state() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut no_op_output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext { resources, .. } = EnsureCmd::exec_dry(cmd_context).await?;

    let states = resources.borrow::<StatesCurrent>();
    let states_desired = resources.borrow::<StatesDesired>();
    let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        states.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_desired
            .get::<State<VecCopyState, Placeholder>, _>(&VecCopyItemSpec.id())
            .map(|state_desired| &state_desired.logical)
    );
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        states_ensured_dry.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    ); // states_ensured_dry should be the same as the beginning.

    Ok(())
}

#[tokio::test]
async fn resources_ensured_contains_state_ensured_for_each_item_spec_when_state_not_yet_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut no_op_output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Alter states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_ensured,
        ..
    } = EnsureCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesCurrentReadCmd::exec(cmd_context).await?;

    let ensured_states_before = resources_ensured.borrow::<StatesCurrent>();
    let ensured_states_desired = resources_ensured.borrow::<StatesDesired>();
    let ensured_states_ensured = resources_ensured.borrow::<StatesEnsured>();
    let reread_states = resources_reread.borrow::<StatesCurrent>();
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        ensured_states_before.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_desired
            .get::<State<VecCopyState, Placeholder>, _>(&VecCopyItemSpec.id())
            .map(|state_desired| &state_desired.logical)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    ); // states_ensured.logical should be the same as states desired, if all went well.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        reread_states
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    );

    Ok(())
}

#[tokio::test]
async fn resources_ensured_contains_state_ensured_for_each_item_spec_when_state_already_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut no_op_output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Alter states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_ensured,
        ..
    } = EnsureCmd::exec(cmd_context).await?;

    // Dry ensure states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_ensured_dry,
        ..
    } = EnsureCmd::exec_dry(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesCurrentReadCmd::exec(cmd_context).await?;

    let ensured_states_before = resources_ensured.borrow::<StatesCurrent>();
    let ensured_states_desired = resources_ensured.borrow::<StatesDesired>();
    let ensured_states_ensured = resources_ensured.borrow::<StatesEnsured>();
    let ensured_states_ensured_dry = resources_ensured_dry.borrow::<StatesEnsuredDry>();
    let reread_states = resources_reread.borrow::<StatesCurrent>();
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        ensured_states_before.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_desired
            .get::<State<VecCopyState, Placeholder>, _>(&VecCopyItemSpec.id())
            .map(|state_desired| &state_desired.logical)
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    ); // states_ensured.logical should be the same as states desired, if all went well.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states_ensured_dry
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    ); // TODO: EnsureDry state should simulate the actual states, not return the actual current state
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        reread_states
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    );

    Ok(())
}
