use peace::{
    cfg::{profile, ItemSpec, Profile, State},
    resources::{States, StatesDesired, StatesEnsured},
    rt::EnsureCmd,
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn contains_state_ensured_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::init(
        &WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
    )
    .await?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let cmd_context = CmdContext::init(&workspace, &graph).await?;

    let CmdContext { resources, .. } = EnsureCmd::exec(cmd_context).await?;

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
