use diff::{Diff, VecDiff, VecDiffType};
use peace::{
    cfg::{ItemSpec, State},
    resources::{Resources, StateDiffs, States, StatesDesired},
    rt::DiffCmd,
    rt_model::ItemSpecGraphBuilder,
};

use crate::{VecA, VecB, VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn contains_state_logical_diff_for_each_item_spec() -> Result<(), VecCopyError> {
    let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
    graph_builder.add_fn(VecCopyItemSpec.into());

    let graph = graph_builder.build();

    let resources = graph.setup(Resources::new()).await?;
    let resources = DiffCmd::exec(&graph, resources).await?;

    let states = resources.borrow::<States>();
    let states_desired = resources.borrow::<StatesDesired>();
    let state_diffs = resources.borrow::<StateDiffs>();
    let vec_diff = state_diffs.get::<<Vec<u8> as Diff>::Repr, _>(&VecCopyItemSpec.id());
    assert_eq!(
        Some(State::new(vec![], ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }]))
        .as_ref(),
        vec_diff
    );

    Ok(())
}

#[tokio::test]
async fn diff_with_multiple_changes() -> Result<(), VecCopyError> {
    let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
    graph_builder.add_fn(VecCopyItemSpec.into());

    let graph = graph_builder.build();

    let mut resources = graph.setup(Resources::new()).await?;
    // overwrite initial state
    #[rustfmt::skip]
    resources.insert(VecA(vec![0, 1, 2,    4, 5, 6, 8]));
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    let resources = DiffCmd::exec(&graph, resources).await?;

    let states = resources.borrow::<States>();
    let states_desired = resources.borrow::<StatesDesired>();
    let state_diffs = resources.borrow::<StateDiffs>();
    let vec_diff = state_diffs.get::<<Vec<u8> as Diff>::Repr, _>(&VecCopyItemSpec.id());
    assert_eq!(
        Some(State::new(vec![0, 1, 2, 3, 4, 5, 6, 7], ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(vec![0u8, 1, 2, 4, 5, 6, 8]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecDiff(vec![
            VecDiffType::Removed { index: 3, len: 1 },
            VecDiffType::Altered {
                index: 7,
                changes: vec![1] // 8 - 7 = 1
            }
        ]))
        .as_ref(),
        vec_diff
    );

    Ok(())
}
