use peace::{
    cfg::{profile, ItemSpec, Profile, State},
    resources::{States, StatesDesired, StatesEnsured},
    rt::EnsureCmd,
    rt_model::{ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn contains_state_ensured_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
    // given
    let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
    graph_builder.add_fn(VecCopyItemSpec.into());

    let graph = graph_builder.build();

    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let workspace = Workspace::init(
        &WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile,
        graph,
    )
    .await?;

    // when
    let workspace = EnsureCmd::exec(workspace).await?;
    let resources = workspace.resources();

    // then
    let states = resources.borrow::<States>();
    let states_desired = resources.borrow::<StatesDesired>();
    let states_ensured = resources.borrow::<StatesEnsured>();
    assert_eq!(
        Some(State::new(vec![], ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_ensured
            .get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
            .map(|state| &state.logical)
    ); // states_ensured.logical should be the same as states desired, if all went well.

    Ok(())
}
