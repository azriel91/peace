use peace::{
    cfg::{ItemSpec, State},
    resources::{Resources, States},
    rt::StateCurrentCmd,
    rt_model::ItemSpecGraphBuilder,
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_current_for_each_item_spec() -> Result<(), VecCopyError> {
    let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
    graph_builder.add_fn(VecCopyItemSpec.into());

    let graph = graph_builder.build();

    let resources = graph.setup(Resources::new()).await?;
    let resources = StateCurrentCmd::exec(&graph, resources).await?;

    let states = resources.borrow::<States>();
    assert_eq!(
        Some(State::new(Vec::<u8>::new(), ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );

    Ok(())
}
