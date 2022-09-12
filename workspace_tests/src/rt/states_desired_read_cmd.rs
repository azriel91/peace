use peace::{
    cfg::{flow_id, profile, FlowId, ItemSpec, Profile, State},
    resources::states::StatesDesired,
    rt::{StatesDesiredDiscoverCmd, StatesDesiredReadCmd},
    rt_model::{CmdContext, Error, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn reads_states_desired_from_disk_when_present() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::init(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        flow_id!("test_flow"),
    )
    .await?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };

    // Write desired states to disk.
    let cmd_context = CmdContext::init(&workspace, &graph, &NoOpOutput).await?;
    let CmdContext {
        resources: resources_from_discover,
        ..
    } = StatesDesiredDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk in a new set of resources.
    let cmd_context = CmdContext::init(&workspace, &graph, &NoOpOutput).await?;
    let CmdContext {
        resources: resources_from_read,
        ..
    } = StatesDesiredReadCmd::exec(cmd_context).await?;

    let states_desired_from_discover = resources_from_discover.borrow::<StatesDesired>();
    let vec_copy_state_from_discover =
        states_desired_from_discover.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id());
    let states_desired_from_read = resources_from_read.borrow::<StatesDesired>();
    let vec_copy_state_from_read =
        states_desired_from_read.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id());
    assert_eq!(vec_copy_state_from_discover, vec_copy_state_from_read);
    Ok(())
}

#[tokio::test]
async fn returns_error_when_states_not_on_disk() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::init(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        flow_id!("test_flow"),
    )
    .await?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };

    // Try and read desired states from disk.
    let cmd_context = CmdContext::init(&workspace, &graph, &NoOpOutput).await?;
    let exec_result = StatesDesiredReadCmd::exec(cmd_context).await;

    assert!(matches!(
        exec_result,
        Err(VecCopyError::PeaceRtError(
            Error::StatesDesiredDiscoverRequired
        ))
    ));
    Ok(())
}
