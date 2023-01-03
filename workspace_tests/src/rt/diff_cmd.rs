use diff::{VecDiff, VecDiffType};
use peace::{
    cfg::{
        profile,
        state::{Nothing, Placeholder},
        FlowId, ItemSpec, Profile, State,
    },
    resources::states::{StateDiffs, StatesDesired, StatesSaved},
    rt::cmds::{DiffCmd, StatesDiscoverCmd},
    rt_model::{CliOutput, CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecA, VecB, VecCopyDiff, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn contains_state_logical_diff_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>>
{
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
    let mut output = NoOpOutput;

    // Write current and desired states to disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext { resources, .. } = DiffCmd::exec(cmd_context).await?;

    let states_saved = resources.borrow::<StatesSaved>();
    let states_desired = resources.borrow::<StatesDesired>();
    let state_diffs = resources.borrow::<StateDiffs>();
    let vec_diff = state_diffs.get::<VecCopyDiff, _>(&VecCopyItemSpec.id());
    assert_eq!(
        Some(State::new(VecCopyState::new(), Nothing)).as_ref(),
        states_saved.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        states_desired
            .get::<State<VecCopyState, Placeholder>, _>(&VecCopyItemSpec.id())
            .map(|state_desired| &state_desired.logical)
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
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut buffer = Vec::with_capacity(256);
    let mut cli_output = CliOutput::new_with_writer(&mut buffer);

    // Write current and desired states to disk.
    let mut cmd_context = CmdContext::builder(&workspace, &graph, &mut cli_output).await?;
    // overwrite initial state
    let resources = cmd_context.resources_mut();
    #[rustfmt::skip]
    resources.insert(VecA(vec![0, 1, 2,    4, 5, 6, 8, 9]));
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Re-read states from disk.
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut cli_output).await?;
    let CmdContext { resources, .. } = DiffCmd::exec(cmd_context).await?;

    let states_saved = resources.borrow::<StatesSaved>();
    let states_desired = resources.borrow::<StatesDesired>();
    let state_diffs = resources.borrow::<StateDiffs>();
    let vec_diff = state_diffs.get::<VecCopyDiff, _>(&VecCopyItemSpec.id());
    assert_eq!(
        Some(State::new(
            VecCopyState::from(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            Nothing
        ))
        .as_ref(),
        states_saved.get::<State<VecCopyState, Nothing>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 4, 5, 6, 8, 9])).as_ref(),
        states_desired
            .get::<State<VecCopyState, Placeholder>, _>(&VecCopyItemSpec.id())
            .map(|state_desired| &state_desired.logical)
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
    assert_eq!(
        "vec_copy: [(-)3..4, (~)7;1, (+)8;9, ]\n",
        String::from_utf8(buffer)?
    );

    Ok(())
}
