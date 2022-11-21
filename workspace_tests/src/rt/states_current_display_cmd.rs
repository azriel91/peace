use peace::{
    cfg::{profile, state::Nothing, FlowId, ItemSpec, Profile, State},
    resources::states::StatesCurrent,
    rt::cmds::{sub::StatesCurrentDiscoverCmd, StatesCurrentDisplayCmd},
    rt_model::{CmdContext, Error, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{
    FnInvocation, FnTrackerOutput, NoOpOutput, VecCopyError, VecCopyItemSpec, VecCopyState,
};

#[tokio::test]
async fn reads_states_current_from_disk_when_present() -> Result<(), Box<dyn std::error::Error>> {
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
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Write current states to disk.
    let mut no_op_output = NoOpOutput;
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut no_op_output).await?;
    let CmdContext {
        resources: resources_from_discover,
        ..
    } = StatesCurrentDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk in a new set of resources.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut fn_tracker_output).await?;
    let CmdContext {
        resources: resources_from_read,
        ..
    } = StatesCurrentDisplayCmd::exec(cmd_context).await?;

    let states_from_discover = resources_from_discover.borrow::<StatesCurrent>();
    let vec_copy_state_from_discover =
        states_from_discover.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id());
    let states_from_read = resources_from_read.borrow::<StatesCurrent>();
    let states_from_read = &*states_from_read;
    let vec_copy_state_from_read =
        states_from_read.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id());
    assert_eq!(vec_copy_state_from_discover, vec_copy_state_from_read);
    assert_eq!(
        vec![FnInvocation::new(
            "write_states_current",
            vec![Some(format!("{states_from_read:?}"))],
        )],
        fn_tracker_output.fn_invocations()
    );
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
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Try and display states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut fn_tracker_output).await?;
    let exec_result = StatesCurrentDisplayCmd::exec(cmd_context).await;

    assert!(matches!(
        exec_result,
        Err(VecCopyError::PeaceRtError(
            Error::StatesCurrentDiscoverRequired
        ))
    ));
    let err = VecCopyError::PeaceRtError(Error::StatesCurrentDiscoverRequired);
    assert_eq!(
        vec![FnInvocation::new(
            "write_err",
            vec![Some(format!("{err:?}"))],
        )],
        fn_tracker_output.fn_invocations()
    );
    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!(
        "{:?}",
        StatesCurrentDisplayCmd::<VecCopyError, NoOpOutput>::default()
    );
    assert!(
        debug_str
            == r#"StatesCurrentDisplayCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput)>)"#
            || debug_str == r#"StatesCurrentDisplayCmd(PhantomData)"#
    );
}
