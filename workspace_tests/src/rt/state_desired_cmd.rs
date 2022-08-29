use peace::{
    cfg::{profile, ItemSpec, Profile},
    resources::StatesDesired,
    rt::StateDesiredCmd,
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_desired_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
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

    let CmdContext { resources, .. } = StateDesiredCmd::exec(cmd_context).await?;

    let states_desired = resources.borrow::<StatesDesired>();
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id())
    );

    Ok(())
}
