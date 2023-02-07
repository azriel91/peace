use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    resources::states::StatesDesired,
    rt::cmds::{sub::StatesDesiredDiscoverCmd, StatesDesiredDisplayCmd},
    rt_model::{cmd::CmdContext, Error, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{
    FnInvocation, FnTrackerOutput, NoOpOutput, VecCopyError, VecCopyItemSpec, VecCopyState,
};

#[tokio::test]
async fn reads_states_desired_from_disk_when_present() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Write desired states to disk.
    let mut output = NoOpOutput;
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output)
        .with_profile(profile!("test_profile"))
        .with_flow_id(FlowId::new(crate::fn_name_short!())?)
        .await?;
    let CmdContext {
        resources: resources_from_discover,
        ..
    } = StatesDesiredDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk in a new set of resources.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut fn_tracker_output)
        .with_profile(profile!("test_profile"))
        .with_flow_id(FlowId::new(crate::fn_name_short!())?)
        .await?;
    let CmdContext {
        resources: resources_from_read,
        ..
    } = StatesDesiredDisplayCmd::exec(cmd_context).await?;

    let states_from_discover = resources_from_discover.borrow::<StatesDesired>();
    let vec_copy_state_from_discover =
        states_from_discover.get::<VecCopyState, _>(VecCopyItemSpec.id());
    let states_from_read = resources_from_read.borrow::<StatesDesired>();
    let states_from_read = &*states_from_read;
    let vec_copy_state_from_read = states_from_read.get::<VecCopyState, _>(VecCopyItemSpec.id());
    assert_eq!(vec_copy_state_from_discover, vec_copy_state_from_read);
    assert_eq!(
        vec![FnInvocation::new(
            "present",
            vec![Some(serde_yaml::to_string(states_from_read)?)],
        )],
        fn_tracker_output.fn_invocations()
    );
    Ok(())
}

#[tokio::test]
async fn returns_error_when_states_not_on_disk() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Try and display states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut fn_tracker_output).await?;
    let exec_result = StatesDesiredDisplayCmd::exec(cmd_context).await;

    assert!(matches!(
        exec_result,
        Err(VecCopyError::PeaceRtError(
            Error::StatesDesiredDiscoverRequired
        ))
    ));
    let err = VecCopyError::PeaceRtError(Error::StatesDesiredDiscoverRequired);
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
        StatesDesiredDisplayCmd::<VecCopyError, NoOpOutput, (), (), ()>::default()
    );
    assert!(
        debug_str
            == r#"StatesDesiredDisplayCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, (), (), ())>)"#
            || debug_str == r#"StatesDesiredDisplayCmd(PhantomData)"#
    );
}
