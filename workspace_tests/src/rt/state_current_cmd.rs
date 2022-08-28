use peace::{
    cfg::{profile, ItemSpec, Profile, State},
    resources::States,
    rt::StateCurrentCmd,
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_current_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::try_new(
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

    let CmdContext { resources, .. } = StateCurrentCmd::exec(cmd_context).await?;

    let states = resources.borrow::<States>();
    assert_eq!(
        Some(State::new(Vec::<u8>::new(), ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );

    Ok(())
}
