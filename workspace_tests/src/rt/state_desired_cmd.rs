use peace::{
    cfg::{profile, ItemSpec, Profile},
    resources::StatesDesired,
    rt::StateDesiredCmd,
    rt_model::{ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_desired_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
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
    let workspace = StateDesiredCmd::exec(workspace).await?;
    let resources = workspace.resources();

    let states_desired = resources.borrow::<StatesDesired>();
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id())
    );

    Ok(())
}
