use peace::{
    cfg::FullSpec,
    resources::{Resources, StatesDesired},
    rt::StateDesiredCmd,
    rt_model::FullSpecGraphBuilder,
};

use crate::{VecCopyError, VecCopyFullSpec};

#[tokio::test]
async fn runs_state_desired_for_each_full_spec() -> Result<(), VecCopyError> {
    let mut graph_builder = FullSpecGraphBuilder::<VecCopyError>::new();
    graph_builder.add_fn(VecCopyFullSpec.into());

    let graph = graph_builder.build();

    let resources = graph.setup(Resources::new()).await?;
    let resources = StateDesiredCmd::exec(&graph, resources).await?;

    let states_desired = resources.borrow::<StatesDesired>();
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyFullSpec.id())
    );

    Ok(())
}
