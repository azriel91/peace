use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    cmd::ctx::CmdCtx,
    resources::states::StatesDesired,
    rt::cmds::{sub::StatesDesiredDiscoverCmd, StatesDesiredDisplayCmd},
    rt_model::{Error, Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{
    FnInvocation, FnTrackerOutput, NoOpOutput, PeaceTestError, VecCopyError, VecCopyItemSpec,
    VecCopyState,
};

#[tokio::test]
async fn reads_states_desired_from_disk_when_present() -> Result<(), Box<dyn std::error::Error>> {
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
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Write desired states to disk.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = StatesDesiredDiscoverCmd::exec(cmd_ctx).await?;
    let resources_from_discover = cmd_ctx.resources();

    // Re-read states from disk in a new set of resources.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut fn_tracker_output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = StatesDesiredDisplayCmd::exec(cmd_ctx).await?;
    let resources_from_read = cmd_ctx.resources();
    let fn_tracker_output = cmd_ctx.output();

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
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Try and display states from disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut fn_tracker_output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let exec_result = StatesDesiredDisplayCmd::exec(cmd_ctx).await;

    assert!(matches!(
        exec_result,
        Err(PeaceTestError::PeaceRtError(
            Error::StatesDesiredDiscoverRequired
        ))
    ));
    let err = PeaceTestError::PeaceRtError(Error::StatesDesiredDiscoverRequired);
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
        StatesDesiredDisplayCmd::<VecCopyError, NoOpOutput, ()>::default()
    );
    assert!(
        debug_str
            == r#"StatesDesiredDisplayCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#
            || debug_str == r#"StatesDesiredDisplayCmd(PhantomData)"#
    );
}
