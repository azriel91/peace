use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    cmd::ctx::CmdCtx,
    resources::states::{StatesCurrent, StatesSaved},
    rt::cmds::sub::{StatesCurrentDiscoverCmd, StatesSavedReadCmd},
    rt_model::{Error, Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, PeaceTestError, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn reads_states_saved_from_disk_when_present() -> Result<(), Box<dyn std::error::Error>> {
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
    let mut output = NoOpOutput;

    // Write current states to disk.
    let cmd_ctx =
        CmdCtx::builder_single_profile_single_flow::<PeaceTestError>(&mut output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(Flow::new(
                FlowId::new(crate::fn_name_short!())?,
                graph.clone(),
            ))
            .await?;
    let cmd_ctx = StatesCurrentDiscoverCmd::exec(cmd_ctx).await?;
    let resources_from_discover = cmd_ctx.resources();

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let cmd_ctx =
        CmdCtx::builder_single_profile_single_flow::<PeaceTestError>(&mut output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
            .await?;
    let cmd_ctx = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let resources_from_read = cmd_ctx.resources();

    let states_from_discover = resources_from_discover.borrow::<StatesCurrent>();
    let vec_copy_state_from_discover =
        states_from_discover.get::<VecCopyState, _>(VecCopyItemSpec.id());
    let states_from_read = resources_from_read.borrow::<StatesSaved>();
    let vec_copy_state_from_read = states_from_read.get::<VecCopyState, _>(VecCopyItemSpec.id());
    assert_eq!(vec_copy_state_from_discover, vec_copy_state_from_read);
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

    // Try and read states from disk.
    let mut output = NoOpOutput;
    let cmd_ctx =
        CmdCtx::builder_single_profile_single_flow::<PeaceTestError>(&mut output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
            .await?;
    let exec_result = StatesSavedReadCmd::exec(cmd_ctx).await;

    assert!(matches!(
        exec_result,
        Err(PeaceTestError::PeaceRtError(
            Error::StatesCurrentDiscoverRequired
        ))
    ));
    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!(
        "{:?}",
        StatesSavedReadCmd::<VecCopyError, NoOpOutput, ()>::default()
    );
    assert!(
        debug_str
            == r#"StatesSavedReadCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#
            || debug_str == r#"StatesSavedReadCmd(PhantomData)"#
    );
}
