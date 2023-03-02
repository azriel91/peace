use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    cmd::ctx::CmdCtx,
    resources::states::{
        StatesCleaned, StatesCleanedDry, StatesCurrent, StatesEnsured, StatesSaved,
    },
    rt::cmds::{
        sub::{StatesCurrentDiscoverCmd, StatesSavedReadCmd},
        CleanCmd, EnsureCmd, StatesDiscoverCmd,
    },
    rt_model::{Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, PeaceTestError, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn resources_cleaned_dry_does_not_alter_state_when_state_not_ensured()
-> Result<(), Box<dyn std::error::Error>> {
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
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    StatesCurrentDiscoverCmd::exec(cmd_ctx).await?;

    // Re-read states from disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = CleanCmd::exec_dry(cmd_ctx).await?;
    let resources = cmd_ctx.resources();

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
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    StatesDiscoverCmd::exec(cmd_ctx).await?;

    // Ensure states.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = EnsureCmd::exec(cmd_ctx).await?;
    let resources_ensured = cmd_ctx.resources();

    // Clean states.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = CleanCmd::exec_dry(cmd_ctx).await?;
    let resources_cleaned = cmd_ctx.resources();

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let resources_reread = cmd_ctx.resources();

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
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current states to disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    StatesCurrentDiscoverCmd::exec(cmd_ctx).await?;

    // Clean states.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = CleanCmd::exec(cmd_ctx).await?;
    let resources_cleaned = cmd_ctx.resources();

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let resources_reread = cmd_ctx.resources();

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
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<PeaceTestError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    StatesDiscoverCmd::exec(cmd_ctx).await?;

    // Ensure states.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = EnsureCmd::exec(cmd_ctx).await?;
    let resources_ensured = cmd_ctx.resources();

    // Clean states.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(
            FlowId::new(crate::fn_name_short!())?,
            graph.clone(),
        ))
        .await?;
    let cmd_ctx = CleanCmd::exec(cmd_ctx).await?;
    let resources_cleaned = cmd_ctx.resources();

    // Re-read states from disk.
    let mut output = NoOpOutput;
    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
        .await?;
    let cmd_ctx = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let resources_reread = cmd_ctx.resources();

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

#[test]
fn debug() {
    let debug_str = format!("{:?}", CleanCmd::<VecCopyError, NoOpOutput, ()>::default());
    assert!(
        debug_str
            == r#"CleanCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#
            || debug_str == r#"CleanCmd(PhantomData)"#
    );
}
