use std::path::Path;

use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{CmdContextBuilder, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{no_op_output::NoOpOutput, VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn build_initializes_dirs_using_profile_and_physically_creates_dirs()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::init(
        WorkspaceSpec::Path(tempdir.path().into()),
        profile!("test_profile"),
        flow_id!("test_flow"),
    )
    .await?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut no_op_output = NoOpOutput;
    CmdContextBuilder::new(&workspace, &graph, &mut no_op_output).await?;

    let workspace_dirs = workspace.dirs();
    let workspace_dir = tempdir.path();
    let peace_dir = tempdir
        .path()
        .join(workspace_dirs.peace_dir().file_name().unwrap());
    let profile_dir = peace_dir.join("test_profile");
    let profile_history_dir = profile_dir.join(".history");

    assert_eq!(
        workspace_dir,
        AsRef::<Path>::as_ref(workspace_dirs.workspace_dir())
    );
    assert_eq!(
        profile_dir,
        AsRef::<Path>::as_ref(workspace_dirs.profile_dir())
    );
    [
        workspace_dir,
        &peace_dir,
        &profile_dir,
        &profile_history_dir,
    ]
    .iter()
    .for_each(|dir| assert!(dir.exists()));
    Ok(())
}
