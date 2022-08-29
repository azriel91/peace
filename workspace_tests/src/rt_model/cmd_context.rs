use std::path::PathBuf;

use peace::{
    cfg::{profile, Profile},
    resources::dir::{PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir},
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecA, VecB, VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn init_inserts_workspace_dirs_into_resources_for_workspace_spec_working_dir()
-> Result<(), VecCopyError> {
    let workspace = Workspace::init(&WorkspaceSpec::WorkingDir, profile!("test_profile")).await?;
    let item_spec_graph = {
        let mut builder = ItemSpecGraphBuilder::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };

    let cmd_context = CmdContext::init(&workspace, &item_spec_graph).await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<PeaceDir>().is_ok());
    assert!(resources.try_borrow::<ProfileDir>().is_ok());
    assert!(resources.try_borrow::<ProfileHistoryDir>().is_ok());
    assert!(resources.try_borrow::<WorkspaceDir>().is_ok());
    Ok(())
}

#[tokio::test]
async fn init_inserts_workspace_dirs_into_resources_for_workspace_spec_path()
-> Result<(), VecCopyError> {
    let workspace = Workspace::init(
        &WorkspaceSpec::Path(PathBuf::from(".")),
        profile!("test_profile"),
    )
    .await?;
    let item_spec_graph = {
        let mut builder = ItemSpecGraphBuilder::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };

    let cmd_context = CmdContext::init(&workspace, &item_spec_graph).await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<PeaceDir>().is_ok());
    assert!(resources.try_borrow::<ProfileDir>().is_ok());
    assert!(resources.try_borrow::<ProfileHistoryDir>().is_ok());
    assert!(resources.try_borrow::<WorkspaceDir>().is_ok());
    Ok(())
}

#[tokio::test]
async fn init_inserts_workspace_dirs_into_resources_for_workspace_spec_first_dir_with_file()
-> Result<(), VecCopyError> {
    let workspace = Workspace::init(
        &WorkspaceSpec::FirstDirWithFile("Cargo.lock".into()),
        profile!("test_profile"),
    )
    .await?;
    let item_spec_graph = {
        let mut builder = ItemSpecGraphBuilder::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };

    let cmd_context = CmdContext::init(&workspace, &item_spec_graph).await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<PeaceDir>().is_ok());
    assert!(resources.try_borrow::<ProfileDir>().is_ok());
    assert!(resources.try_borrow::<ProfileHistoryDir>().is_ok());
    assert!(resources.try_borrow::<WorkspaceDir>().is_ok());
    Ok(())
}

#[tokio::test]
async fn init_runs_graph_setup() -> Result<(), VecCopyError> {
    let workspace = Workspace::init(&WorkspaceSpec::WorkingDir, profile!("test_profile")).await?;
    let item_spec_graph = {
        let mut builder = ItemSpecGraphBuilder::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };

    let cmd_context = CmdContext::init(&workspace, &item_spec_graph).await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<VecA>().is_ok());
    assert!(resources.try_borrow::<VecB>().is_ok());
    Ok(())
}
