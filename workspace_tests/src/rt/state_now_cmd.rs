use peace::{
    cfg::{FullSpec, State},
    resources::{Resources, States},
    rt::StateNowCmd,
    rt_model::FullSpecGraphBuilder,
};

use crate::{VecCopyError, VecCopyFullSpec};

#[tokio::test]
async fn runs_state_now_for_each_full_spec() -> Result<(), VecCopyError> {
    let mut graph_builder = FullSpecGraphBuilder::<VecCopyError>::new();
    graph_builder.add_fn(VecCopyFullSpec.into());

    let graph = graph_builder.build();

    let resources = graph.setup(Resources::new()).await?;
    let resources = StateNowCmd::exec(&graph, resources).await?;

    let states = resources.borrow::<States>();
    assert_eq!(
        Some(State::new(vec![0u8, 1, 2, 3, 4, 5, 6, 7], ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyFullSpec.id())
    );

    Ok(())
}
