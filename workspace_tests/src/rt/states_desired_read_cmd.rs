use peace::{
    cfg::{profile, state::Nothing, FlowId, ItemSpec, Profile, State},
    resources::states::StatesDesired,
    rt::cmds::sub::{StatesDesiredDiscoverCmd, StatesDesiredReadCmd},
    rt_model::{CmdContext, Error, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn reads_states_desired_from_disk_when_present() -> Result<(), Box<dyn std::error::Error>> {
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

    // Write desired states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::new()))
        .await?;
    let CmdContext {
        resources: resources_from_discover,
        ..
    } = StatesDesiredDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(None)
        .await?;
    let CmdContext {
        resources: resources_from_read,
        ..
    } = StatesDesiredReadCmd::exec(cmd_context).await?;

    let states_desired_from_discover = resources_from_discover.borrow::<StatesDesired>();
    let vec_copy_state_from_discover =
        states_desired_from_discover.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id());
    let states_desired_from_read = resources_from_read.borrow::<StatesDesired>();
    let vec_copy_state_from_read =
        states_desired_from_read.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id());
    assert_eq!(vec_copy_state_from_discover, vec_copy_state_from_read);
    Ok(())
}

#[tokio::test]
async fn returns_error_when_states_not_on_disk() -> Result<(), Box<dyn std::error::Error>> {
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

    // Try and read desired states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output)
        .with_profile_init::<VecCopyState>(Some(VecCopyState::new()))
        .await?;
    let exec_result = StatesDesiredReadCmd::exec(cmd_context).await;

    assert!(matches!(
        exec_result,
        Err(VecCopyError::PeaceRtError(
            Error::StatesDesiredDiscoverRequired
        ))
    ));
    Ok(())
}

#[test]
fn debug() {
    assert_eq!(
        "StatesDesiredReadCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput)>)",
        format!(
            "{:?}",
            StatesDesiredReadCmd::<VecCopyError, NoOpOutput>::default()
        )
    );
}
