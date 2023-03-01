use diff::{VecDiff, VecDiffType};
use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    cmd::ctx::CmdCtx,
    resources::states::{StateDiffs, StatesDesired, StatesSaved},
    rt::cmds::{DiffCmd, StatesDiscoverCmd},
    rt_model::{output::CliOutput, Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{
    NoOpOutput, PeaceTestError, VecA, VecB, VecCopyDiff, VecCopyError, VecCopyItemSpec,
    VecCopyState,
};

#[tokio::test]
async fn contains_state_logical_diff_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>>
{
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
    let cmd_ctx =
        CmdCtx::builder_single_profile_single_flow::<PeaceTestError>(&mut output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(Flow::new(
                FlowId::new(crate::fn_name_short!())?,
                graph.clone(),
            ))
            .await?;
    StatesDiscoverCmd::exec_v2(cmd_ctx).await?;

    // Re-read states from disk.
    let cmd_ctx =
        CmdCtx::builder_single_profile_single_flow::<PeaceTestError>(&mut output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
            .await?;
    let cmd_ctx = DiffCmd::exec_v2(cmd_ctx).await?;
    let resources = cmd_ctx.resources();

    let states_saved = resources.borrow::<StatesSaved>();
    let states_desired = resources.borrow::<StatesDesired>();
    let state_diffs = resources.borrow::<StateDiffs>();
    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItemSpec.id());
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_desired.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        vec_diff
    );

    Ok(())
}

#[tokio::test]
async fn diff_with_multiple_changes() -> Result<(), Box<dyn std::error::Error>> {
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
    let mut buffer = Vec::with_capacity(256);
    let mut cli_output = CliOutput::new_with_writer(&mut buffer);

    // Write current and desired states to disk.
    let mut cmd_ctx =
        CmdCtx::builder_single_profile_single_flow::<PeaceTestError>(&mut cli_output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(Flow::new(
                FlowId::new(crate::fn_name_short!())?,
                graph.clone(),
            ))
            .await?;
    // overwrite initial state
    let resources = cmd_ctx.resources_mut();
    #[rustfmt::skip]
    resources.insert(VecA(vec![0, 1, 2,    4, 5, 6, 8, 9]));
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    StatesDiscoverCmd::exec_v2(cmd_ctx).await?;

    // Re-read states from disk.
    let cmd_ctx =
        CmdCtx::builder_single_profile_single_flow::<PeaceTestError>(&mut cli_output, &workspace)
            .with_profile(profile!("test_profile"))
            .with_flow(Flow::new(FlowId::new(crate::fn_name_short!())?, graph))
            .await?;
    let cmd_ctx = DiffCmd::exec_v2(cmd_ctx).await?;
    let resources = cmd_ctx.resources();

    // Separate scope drops borrowed resources.
    {
        let states_saved = resources.borrow::<StatesSaved>();
        let states_desired = resources.borrow::<StatesDesired>();
        let state_diffs = resources.borrow::<StateDiffs>();
        let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItemSpec.id());
        assert_eq!(
            Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),).as_ref(),
            states_saved.get::<VecCopyState, _>(VecCopyItemSpec.id())
        );
        assert_eq!(
            Some(VecCopyState::from(vec![0u8, 1, 2, 4, 5, 6, 8, 9])).as_ref(),
            states_desired.get::<VecCopyState, _>(VecCopyItemSpec.id())
        );
        assert_eq!(
            Some(VecCopyDiff::from(VecDiff(vec![
                VecDiffType::Removed { index: 3, len: 1 },
                VecDiffType::Altered {
                    index: 7,
                    changes: vec![1] // 8 - 7 = 1
                },
                VecDiffType::Inserted {
                    index: 8,
                    changes: vec![9]
                },
            ])))
            .as_ref(),
            vec_diff
        );
    }
    assert_eq!(
        "1. `vec_copy`: [(-)3..4, (~)7;1, (+)8;9, ]\n",
        String::from_utf8(buffer)?
    );

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!("{:?}", DiffCmd::<VecCopyError, NoOpOutput, ()>::default());
    assert!(
        debug_str
            == r#"DiffCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, ())>)"#
            || debug_str == r#"DiffCmd(PhantomData)"#
    );
}
