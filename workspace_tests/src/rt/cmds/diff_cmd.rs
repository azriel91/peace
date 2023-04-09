use diff::{VecDiff, VecDiffType};
use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlowView},
    rt::cmds::{DiffCmd, StatesDiscoverCmd},
    rt_model::{
        output::{CliOutput, OutputWrite},
        Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
    },
};

use crate::{
    NoOpOutput, PeaceTestError, VecA, VecB, VecCopyDiff, VecCopyError, VecCopyItemSpec,
    VecCopyState,
};

#[tokio::test]
async fn contains_state_diff_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
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
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut output = NoOpOutput;

    // Discover current and desired states.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    let (states_current, states_desired) =
        StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
    let SingleProfileSingleFlowView {
        flow, resources, ..
    } = cmd_ctx.view();

    // Diff current and desired states.
    let state_diffs = DiffCmd::diff_any(flow, resources, &states_current, &states_desired).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItemSpec.id());
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
    let flow = Flow::new(FlowId::new(crate::fn_name_short!())?, graph);
    let mut buffer = Vec::with_capacity(256);
    let mut output = CliOutput::new_with_writer(&mut buffer);

    // Discover current and desired states.
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .await?;
    // overwrite initial state
    let resources = cmd_ctx.resources_mut();
    #[rustfmt::skip]
    resources.insert(VecA(vec![0, 1, 2,    4, 5, 6, 8, 9]));
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    let (states_current, states_desired) =
        StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
    let SingleProfileSingleFlowView {
        flow, resources, ..
    } = cmd_ctx.view();

    // Diff current and desired states.
    let state_diffs = DiffCmd::diff_any(flow, resources, &states_current, &states_desired).await?;
    <_ as OutputWrite<PeaceTestError>>::present(cmd_ctx.output_mut(), &state_diffs).await?;

    let vec_diff = state_diffs.get::<VecCopyDiff, _>(VecCopyItemSpec.id());
    assert_eq!(
        Some(VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),).as_ref(),
        states_current.get::<VecCopyState, _>(VecCopyItemSpec.id())
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
    // `CliOutput` writes `\n\n`s in `progress_end` for better spacing with
    // progress bars.
    #[cfg(feature = "output_progress")]
    assert_eq!(
        "\n\n1. `vec_copy`: [(-)3..4, (~)7;1, (+)8;9, ]\n",
        String::from_utf8(buffer)?
    );
    #[cfg(not(feature = "output_progress"))]
    assert_eq!(
        "1. `vec_copy`: [(-)3..4, (~)7;1, (+)8;9, ]\n",
        String::from_utf8(buffer)?
    );

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!("{:?}", DiffCmd::<VecCopyError>::default());
    assert!(
        debug_str == r#"DiffCmd(PhantomData<workspace_tests::vec_copy_item_spec::VecCopyError>)"#
            || debug_str == r#"DiffCmd(PhantomData)"#
    );
}
