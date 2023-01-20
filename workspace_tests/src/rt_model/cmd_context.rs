use peace::{
    cfg::{app_name, profile, AppName, FlowId, Profile},
    resources::paths::{FlowDir, PeaceDir, ProfileDir, ProfileHistoryDir},
    rt_model::{cmd::CmdContext, ItemSpecGraphBuilder, Storage, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecA, VecB, VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn init_inserts_workspace_dirs_into_resources_for_workspace_spec_working_dir()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(app_name!(), WorkspaceSpec::Path(tempdir.path().into()))?;
    let graph = {
        let mut builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };
    let mut output = NoOpOutput;

    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output)
        .with_profile(profile!("test_profile"))
        .with_flow_id(FlowId::new(crate::fn_name_short!())?)
        .await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<PeaceDir>().is_ok());
    assert!(resources.try_borrow::<ProfileDir>().is_ok());
    assert!(resources.try_borrow::<ProfileHistoryDir>().is_ok());
    assert!(resources.try_borrow::<FlowDir>().is_ok());
    assert_eq!(
        Ok(profile!("test_profile")).as_ref(),
        resources.try_borrow::<Profile>().as_deref()
    );
    assert_eq!(
        Ok(FlowId::new(crate::fn_name_short!())?).as_ref(),
        resources.try_borrow::<FlowId>().as_deref()
    );
    assert!(resources.try_borrow::<Storage>().is_ok());
    Ok(())
}

#[tokio::test]
async fn init_inserts_workspace_dirs_into_resources_for_workspace_spec_path()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let temp_path = tempdir.path();
    let workspace = Workspace::new(app_name!(), WorkspaceSpec::Path(temp_path.to_path_buf()))?;
    let graph = {
        let mut builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };
    let mut output = NoOpOutput;

    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output)
        .with_profile(profile!("test_profile"))
        .with_flow_id(FlowId::new(crate::fn_name_short!())?)
        .await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<PeaceDir>().is_ok());
    assert!(resources.try_borrow::<ProfileDir>().is_ok());
    assert!(resources.try_borrow::<ProfileHistoryDir>().is_ok());
    assert!(resources.try_borrow::<FlowDir>().is_ok());
    assert_eq!(
        Ok(profile!("test_profile")).as_ref(),
        resources.try_borrow::<Profile>().as_deref()
    );
    assert_eq!(
        Ok(FlowId::new(crate::fn_name_short!())?).as_ref(),
        resources.try_borrow::<FlowId>().as_deref()
    );
    assert!(resources.try_borrow::<Storage>().is_ok());
    Ok(())
}

#[tokio::test]
async fn init_inserts_workspace_dirs_into_resources_for_workspace_spec_first_dir_with_file()
-> Result<(), Box<dyn std::error::Error>> {
    // Prevent the test from polluting the actual repository.
    let tempdir = tempfile::tempdir()?;
    let subdir = tempdir.path().join("subdir");
    tokio::fs::write(tempdir.path().join("Cargo.lock"), "").await?;
    tokio::fs::create_dir(&subdir).await?;
    std::env::set_current_dir(&subdir)?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::FirstDirWithFile("Cargo.lock".into()),
    )?;
    let graph = {
        let mut builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };
    let mut output = NoOpOutput;

    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output)
        .with_profile(profile!("test_profile"))
        .with_flow_id(FlowId::new(crate::fn_name_short!())?)
        .await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<PeaceDir>().is_ok());
    assert!(resources.try_borrow::<ProfileDir>().is_ok());
    assert!(resources.try_borrow::<ProfileHistoryDir>().is_ok());
    assert!(resources.try_borrow::<FlowDir>().is_ok());
    assert_eq!(
        Ok(profile!("test_profile")).as_ref(),
        resources.try_borrow::<Profile>().as_deref()
    );
    assert_eq!(
        Ok(FlowId::new(crate::fn_name_short!())?).as_ref(),
        resources.try_borrow::<FlowId>().as_deref()
    );
    assert!(resources.try_borrow::<Storage>().is_ok());
    Ok(())
}

#[tokio::test]
async fn init_runs_graph_setup() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(app_name!(), WorkspaceSpec::Path(tempdir.path().into()))?;
    let graph = {
        let mut builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        builder.add_fn(VecCopyItemSpec.into());
        builder.build()
    };
    let mut output = NoOpOutput;

    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output)
        .with_profile(profile!("test_profile"))
        .with_flow_id(FlowId::new(crate::fn_name_short!())?)
        .await?;

    let resources = cmd_context.resources();
    assert!(resources.try_borrow::<VecA>().is_ok());
    assert!(resources.try_borrow::<VecB>().is_ok());
    Ok(())
}
