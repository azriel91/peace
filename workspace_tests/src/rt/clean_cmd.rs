use peace::{
    cfg::{profile, state::Nothing, FlowId, ItemSpec, Profile, State},
    resources::states::{StatesCleaned, StatesCleanedDry, StatesCurrent, StatesEnsured},
    rt::cmds::{
        sub::{StatesCurrentDiscoverCmd, StatesCurrentReadCmd},
        CleanCmd, EnsureCmd, StatesDiscoverCmd,
    },
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state_when_state_not_ensured()
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

    // Write current states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::new()))
        .await?;
    StatesCurrentDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(None)
        .await?;
    let CmdContext { resources, .. } = CleanCmd::exec_dry(cmd_context).await?;

    let states = resources.borrow::<StatesCurrent>();
    let states_cleaned_dry = resources.borrow::<StatesCleanedDry>();
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        states.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        states_cleaned_dry.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    ); // states_cleaned_dry should be the same as the beginning.

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state_when_state_ensured()
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
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::new()))
        .await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Ensure states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(None)
        .await?;
    let CmdContext {
        resources: resources_ensured,
        ..
    } = EnsureCmd::exec(cmd_context).await?;

    // Clean states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])))
        .await?;
    let CmdContext {
        resources: resources_cleaned,
        ..
    } = CleanCmd::exec_dry(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(None)
        .await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesCurrentReadCmd::exec(cmd_context).await?;

    let ensured_states = resources_ensured.borrow::<StatesEnsured>();
    let cleaned_states_before = resources_cleaned.borrow::<StatesCurrent>();
    let reread_states = resources_reread.borrow::<StatesCurrent>();

    assert_eq!(
        Some(State::new(
            VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            Nothing
        ))
        .as_ref(),
        ensured_states.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(State::new(
            VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            Nothing
        ))
        .as_ref(),
        cleaned_states_before.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        reread_states
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    );

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_spec_when_state_not_ensured()
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

    // Write current states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::new()))
        .await?;
    StatesCurrentDiscoverCmd::exec(cmd_context).await?;

    // Clean states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::new()))
        .await?;
    let CmdContext {
        resources: resources_cleaned,
        ..
    } = CleanCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(None)
        .await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesCurrentReadCmd::exec(cmd_context).await?;

    let cleaned_states = resources_cleaned.borrow::<StatesCurrent>();
    let cleaned_states_cleaned = resources_cleaned.borrow::<StatesCleaned>();
    let reread_states = resources_reread.borrow::<StatesCurrent>();
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        cleaned_states.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        cleaned_states_cleaned
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    ); // states_cleaned.logical should be empty, if all went well.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        reread_states
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    );

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_spec_when_state_ensured()
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
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::new()))
        .await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Ensure states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(None)
        .await?;
    let CmdContext {
        resources: resources_ensured,
        ..
    } = EnsureCmd::exec(cmd_context).await?;

    // Clean states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])))
        .await?;
    let CmdContext {
        resources: resources_cleaned,
        ..
    } = CleanCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(None)
        .await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesCurrentReadCmd::exec(cmd_context).await?;

    let ensured_states = resources_ensured.borrow::<StatesEnsured>();
    let cleaned_states_before = resources_cleaned.borrow::<StatesCurrent>();
    let cleaned_states_cleaned = resources_cleaned.borrow::<StatesCleaned>();
    let reread_states = resources_reread.borrow::<StatesCurrent>();

    assert_eq!(
        Some(State::new(
            VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            Nothing
        ))
        .as_ref(),
        ensured_states.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(State::new(
            VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            Nothing
        ))
        .as_ref(),
        cleaned_states_before.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        cleaned_states_cleaned
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    ); // states_cleaned.logical should be empty, if all went well.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        reread_states
            .get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    );

    Ok(())
}
