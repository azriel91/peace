use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    cmd_model::CmdOutcome,
    rt::cmds::{StatesDiscoverCmd, StatesGoalDisplayCmd},
    rt_model::{Error, Flow, ItemGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{
    FnInvocation, FnTrackerOutput, NoOpOutput, PeaceTestError, VecA, VecCopyError, VecCopyItem,
    VecCopyState,
};

#[tokio::test]
async fn reads_states_goal_from_disk_when_present() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Write goal states to disk.
    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .await?;
    let CmdOutcome::Complete {
        value: states_goal_from_discover,
        cmd_blocks_processed: _,
    } = StatesDiscoverCmd::goal(&mut cmd_ctx).await?
    else {
        panic!("Expected `StatesDiscoverCmd::goal` to complete successfully.");
    };

    // Re-read states from disk in a new set of resources.
    let mut cmd_ctx =
        CmdCtx::builder_single_profile_single_flow(&mut fn_tracker_output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(&flow)
            .with_item_params::<VecCopyItem>(
                VecCopyItem::ID_DEFAULT.clone(),
                VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
            )
            .await?;
    let CmdOutcome::Complete {
        value: states_goal_from_read,
        cmd_blocks_processed: _,
    } = StatesGoalDisplayCmd::exec(&mut cmd_ctx).await?
    else {
        panic!("Expected `StatesGoalDisplayCmd::exec` to complete successfully.");
    };
    let fn_tracker_output = cmd_ctx.output();

    let vec_copy_state_from_discover =
        states_goal_from_discover.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    let states_goal_from_read = &*states_goal_from_read;
    let vec_copy_state_from_read =
        states_goal_from_read.get::<VecCopyState, _>(VecCopyItem::ID_DEFAULT);
    assert_eq!(vec_copy_state_from_discover, vec_copy_state_from_read);
    assert_eq!(
        vec![FnInvocation::new(
            "present",
            vec![Some(serde_yaml::to_string(states_goal_from_read)?)],
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
        let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItem::default().into());
        graph_builder.build()
    };
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut fn_tracker_output = FnTrackerOutput::new();

    // Try and display states from disk.
    let mut cmd_ctx =
        CmdCtx::builder_single_profile_single_flow(&mut fn_tracker_output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(&flow)
            .with_item_params::<VecCopyItem>(
                VecCopyItem::ID_DEFAULT.clone(),
                VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
            )
            .await?;
    let exec_result = StatesGoalDisplayCmd::exec(&mut cmd_ctx).await;
    drop(cmd_ctx);

    assert!(matches!(
        exec_result,
        Err(PeaceTestError::PeaceRt(Error::StatesGoalDiscoverRequired))
    ));
    let err = PeaceTestError::PeaceRt(Error::StatesGoalDiscoverRequired);
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
        StatesGoalDisplayCmd::<VecCopyError, NoOpOutput, ()>::default()
    );
    assert_eq!(
        r#"StatesGoalDisplayCmd(PhantomData<(workspace_tests::vec_copy_item::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#,
        debug_str,
    );
}
