use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    cmd_rt::{CmdBlockWrapper, CmdExecution},
    resources::states::{
        ts::{Current, Goal},
        StateDiffs, StatesCurrent,
    },
    rt::cmd_blocks::{DiffCmdBlock, StatesDiscoverCmdBlock},
    rt_model::{outcomes::CmdOutcome, Flow, ItemGraphBuilder, Workspace, WorkspaceSpec},
};
use tempfile::TempDir;

use crate::{
    mock_item::{MockItem, MockSrc},
    no_op_output::NoOpOutput,
    peace_test_error::PeaceTestError,
    VecA, VecCopyItem,
};

mod cmd_execution_error_builder;

#[tokio::test]
async fn runs_one_cmd_block() -> Result<(), PeaceTestError> {
    let mut cmd_execution = CmdExecution::builder()
        .with_cmd_block(CmdBlockWrapper::new(
            StatesDiscoverCmdBlock::current(),
            StatesCurrent::from,
        ))
        .build();

    let TestCtx {
        tempdir: _tempdir,
        workspace,
        flow,
    } = test_ctx_init().await?;

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    let cmd_outcome = cmd_execution.exec(&mut cmd_ctx).await?;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_outcome,
                    CmdOutcome::Complete {
                        value: states_current,
                    }
                    if states_current.len() == 2
                ),
                "Expected states_current to have 2 items,\n\
                but cmd_outcome was: {cmd_outcome:?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn chains_multiple_cmd_blocks() -> Result<(), PeaceTestError> {
    let mut cmd_execution = CmdExecution::<StateDiffs, _, _>::builder()
        .with_cmd_block(CmdBlockWrapper::new(
            StatesDiscoverCmdBlock::current_and_goal(),
            // Should we support diffing the accumulated states?
            // Requires passing through `cmd_view` to here.
            |_states_current_and_goal_mut| StateDiffs::new(),
        ))
        .with_cmd_block(CmdBlockWrapper::new(
            DiffCmdBlock::<_, _, Current, Goal>::new(),
            |_state_diffs_ts0_and_ts1| StateDiffs::new(),
        ))
        .build();

    let TestCtx {
        tempdir: _tempdir,
        workspace,
        flow,
    } = test_ctx_init().await?;

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        .with_profile(profile!("test_profile"))
        .with_flow(&flow)
        .with_item_params::<VecCopyItem>(
            VecCopyItem::ID_DEFAULT.clone(),
            VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]).into(),
        )
        .with_item_params::<MockItem<()>>(MockItem::<()>::ID_DEFAULT.clone(), MockSrc(1).into())
        .await?;

    let cmd_outcome = cmd_execution.exec(&mut cmd_ctx).await?;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_outcome,
                    CmdOutcome::Complete {
                        value: state_diffs,
                    }
                    if state_diffs.len() == 2
                ),
                "Expected state_diffs to have 2 items,\n\
                but cmd_outcome was: {cmd_outcome:?}"
            );
        }
    })();

    Ok(())
}

async fn test_ctx_init() -> Result<TestCtx, PeaceTestError> {
    let tempdir = tempfile::tempdir().map_err(PeaceTestError::TempDir)?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let flow = {
        let graph = {
            let mut graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
            graph_builder.add_fn(VecCopyItem::default().into());
            graph_builder.add_fn(MockItem::<()>::default().into());
            graph_builder.build()
        };
        Flow::new(FlowId::new(crate::fn_name_short!())?, graph)
    };

    Ok(TestCtx {
        tempdir,
        workspace,
        flow,
    })
}

struct TestCtx {
    tempdir: TempDir,
    workspace: Workspace,
    flow: Flow<PeaceTestError>,
}
