use peace::{
    cfg::ItemSpec,
    resources::{Resources, StatesDesired},
    rt::StateDesiredCmd,
    rt_model::ItemSpecGraphBuilder,
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_desired_for_each_item_spec() -> Result<(), VecCopyError> {
    let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
    graph_builder.add_fn(VecCopyItemSpec.into());

    let graph = graph_builder.build();

    let resources = graph.setup(Resources::new()).await?;
    let resources = StateDesiredCmd::exec(&graph, resources).await?;

    let states_desired = resources.borrow::<StatesDesired>();
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id())
    );

    Ok(())
}
