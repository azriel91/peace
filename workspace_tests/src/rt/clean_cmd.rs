use peace::{
    cfg::{flow_id, profile, FlowId, ItemSpec, Profile, State},
    resources::states::{StatesCleaned, StatesCleanedDry, StatesCurrent},
    rt::cmds::{
        sub::{StatesCurrentDiscoverCmd, StatesCurrentReadCmd},
        CleanCmd,
    },
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        flow_id!("test_flow"),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut no_op_output = NoOpOutput;

    // Write current states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    StatesCurrentDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext { resources, .. } = CleanCmd::exec_dry(cmd_context).await?;

    let states = resources.borrow::<StatesCurrent>();
    let states_cleaned_dry = resources.borrow::<StatesCleanedDry>();
    assert_eq!(
        Some(State::new(vec![], ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(State::new(vec![], ())).as_ref(),
        states_cleaned_dry.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    ); // states_cleaned_dry should be the same as the beginning.

    Ok(())
}

#[tokio::test]
async fn resources_cleaned_contains_state_cleaned_for_each_item_spec()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        flow_id!("test_flow"),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut no_op_output = NoOpOutput;

    // Write current states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    StatesCurrentDiscoverCmd::exec(cmd_context).await?;

    // Alter states.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_cleaned,
        ..
    } = CleanCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_reread,
        ..
    } = StatesCurrentReadCmd::exec(cmd_context).await?;

    let cleaned_states = resources_cleaned.borrow::<StatesCurrent>();
    let cleaned_states_cleaned = resources_cleaned.borrow::<StatesCleaned>();
    let reread_states = resources_reread.borrow::<StatesCurrent>();
    assert_eq!(
        Some(State::new(vec![], ())).as_ref(),
        cleaned_states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(vec![]).as_ref(),
        cleaned_states_cleaned
            .get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    ); // states_cleaned.logical should be empty, if all went well.
    assert_eq!(
        Some(vec![]).as_ref(),
        reread_states
            .get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    );

    Ok(())
}
