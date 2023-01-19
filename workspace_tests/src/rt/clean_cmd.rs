use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    resources::states::{
        StatesCleaned, StatesCleanedDry, StatesCurrent, StatesEnsured, StatesSaved,
    },
    rt::cmds::{
        sub::{StatesCurrentDiscoverCmd, StatesSavedReadCmd},
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
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    StatesCurrentDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext { resources, .. } = CleanCmd::exec_dry(cmd_context).await?;

    let states = resources.borrow::<StatesCurrent>();
    let states_cleaned_dry = resources.borrow::<StatesCleanedDry>();
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_cleaned_dry.get::<VecCopyState, _>(VecCopyItemSpec.id())
    ); // states_cleaned_dry should be the same as the beginning.

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state_when_state_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Ensure states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_ensured,
        ..
    } = EnsureCmd::exec(cmd_context).await?;

    // Clean states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_cleaned,
        ..
    } = CleanCmd::exec_dry(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesSavedReadCmd::exec(cmd_context).await?;

    let ensured_states = resources_ensured.borrow::<StatesEnsured>();
    let cleaned_states_before = resources_cleaned.borrow::<StatesCurrent>();
    let states_saved = resources_reread.borrow::<StatesSaved>();

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        ensured_states.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        cleaned_states_before.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    StatesCurrentDiscoverCmd::exec(cmd_context).await?;

    // Clean states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_cleaned,
        ..
    } = CleanCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesSavedReadCmd::exec(cmd_context).await?;

    let cleaned_states = resources_cleaned.borrow::<StatesCurrent>();
    let cleaned_states_cleaned = resources_cleaned.borrow::<StatesCleaned>();
    let states_saved = resources_reread.borrow::<StatesSaved>();
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        cleaned_states.get::<VecCopyState, _>(VecCopyItemSpec.id())
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

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_spec_when_state_ensured()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Ensure states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_ensured,
        ..
    } = EnsureCmd::exec(cmd_context).await?;

    // Clean states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_cleaned,
        ..
    } = CleanCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesSavedReadCmd::exec(cmd_context).await?;

    let ensured_states = resources_ensured.borrow::<StatesEnsured>();
    let cleaned_states_before = resources_cleaned.borrow::<StatesCurrent>();
    let cleaned_states_cleaned = resources_cleaned.borrow::<StatesCleaned>();
    let states_saved = resources_reread.borrow::<StatesSaved>();

    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),).as_ref(),
        ensured_states.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),).as_ref(),
        cleaned_states_before.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
