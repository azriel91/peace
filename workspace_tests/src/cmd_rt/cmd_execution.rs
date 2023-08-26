use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow},
    cmd_rt::{CmdBlockWrapper, CmdExecution},
    resources::{
        resources::ts::SetUp,
        states::{StateDiffs, StatesCurrent, StatesGoal},
    },
    rt::cmds::{DiffCmd, StatesDiscoverCmd},
    rt_model::{
        outcomes::CmdOutcome,
        params::{KeyUnknown, ParamsKeysImpl},
        Flow, ItemGraphBuilder, Workspace, WorkspaceSpec,
    },
};
use tempfile::TempDir;

use crate::{
    mock_item::{MockItem, MockSrc},
    no_op_output::NoOpOutput,
    peace_test_error::PeaceTestError,
    VecA, VecCopyItem,
};

#[tokio::test]
async fn runs_one_cmd_block() -> Result<(), PeaceTestError> {
    let mut cmd_execution = CmdExecution::builder()
        .with_cmd_block(CmdBlockWrapper::new(
            StatesDiscoverCmd::<
                PeaceTestError,
                NoOpOutput,
                ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
            >::default(),
            |states_current_and_goal_mut| {
                let (states_current_mut, states_goal_mut) =
                    (states_current_and_goal_mut.0, states_current_and_goal_mut.1);
                let states_current = StatesCurrent::from(states_current_mut);
                let states_goal = StatesGoal::from(states_goal_mut);
                (states_current, states_goal)
            },
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
        #[cfg_attr(coverage_nightly, no_coverage)]
        || {
            assert!(
                matches!(
                    &cmd_outcome,
                    CmdOutcome {
                        value,
                        errors,
                    }
                    if {
                        let (states_current, states_goal) = (&value.0, &value.1);
                        states_current.len() == 2 && states_goal.len() == 2
                    }
                    && errors.is_empty()
                ),
                "Expected states_current and states_goal to have 2 items,\n\
                but cmd_outcome was: {cmd_outcome:?}"
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn chains_multiple_cmd_blocks() -> Result<(), PeaceTestError> {
    let mut cmd_execution = CmdExecution::builder()
        .with_cmd_block(CmdBlockWrapper::new(
            StatesDiscoverCmd::<
                PeaceTestError,
                NoOpOutput,
                ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
            >::default(),
            |states_current_and_goal_mut| {
                let (states_current_mut, states_goal_mut) =
                    (states_current_and_goal_mut.0, states_current_and_goal_mut.1);
                let states_current = StatesCurrent::from(states_current_mut);
                let states_goal = StatesGoal::from(states_goal_mut);
                (states_current, states_goal)
            },
        ))
        .with_cmd_block(CmdBlockWrapper::new(
            DiffCmd::<
                '_,
                PeaceTestError,
                NoOpOutput,
                ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
                SingleProfileSingleFlow<
                    '_,
                    PeaceTestError,
                    NoOpOutput,
                    ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
                    SetUp,
                >,
            >::default(),
            |state_diffs| -> StateDiffs { *state_diffs },
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
        #[cfg_attr(coverage_nightly, no_coverage)]
        || {
            assert!(
                matches!(
                    &cmd_outcome,
                    CmdOutcome {
                        value: state_diffs,
                        errors,
                    }
                    if state_diffs.len() == 2
                    && errors.is_empty()
                ),
                "Expected states_current and states_goal to have 2 items,\n\
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
