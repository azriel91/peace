use peace::{
    cfg::{profile, ItemSpec, Profile, State},
    resources::States,
    rt::StateCurrentCmd,
    rt_model::{ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_current_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
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
    let workspace = StateCurrentCmd::exec(workspace).await?;
    let resources = workspace.resources();

    let states = resources.borrow::<States>();
    assert_eq!(
        Some(State::new(Vec::<u8>::new(), ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );

    Ok(())
}
