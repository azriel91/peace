use std::path::Path;

use peace::{
    cfg::{profile, FlowId, Profile},
    resources::internal::{FlowInitFile, ProfileInitFile, WorkspaceInitFile},
    rt_model::{
        CmdContext, CmdContextBuilder, ItemSpecGraph, ItemSpecGraphBuilder, Workspace,
        WorkspaceSpec,
    },
};

use crate::{no_op_output::NoOpOutput, VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn build_initializes_dirs_using_profile_and_physically_creates_dirs()
-> Result<(), Box<dyn std::error::Error>> {
    let (tempdir, workspace, graph, mut output) = test_setup().await?;

    CmdContextBuilder::new(&workspace, &graph, &mut output).await?;

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

#[tokio::test]
async fn build_inserts_workspace_init_params_from_parameter()
-> Result<(), Box<dyn std::error::Error>> {
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;

    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_workspace_param("param".to_string(), Some("workspace_init".to_string()))
        .await?;
    let workspace_init_file = resources.borrow::<WorkspaceInitFile>();
    let workspace_init = resources.borrow::<String>();

    assert!(workspace_init_file.exists());
    assert_eq!(
        "workspace_init\n",
        tokio::fs::read_to_string(&*workspace_init_file).await?
    );
    assert_eq!("workspace_init", workspace_init.as_str());
    Ok(())
}

#[tokio::test]
async fn build_inserts_workspace_init_params_from_storage() -> Result<(), Box<dyn std::error::Error>>
{
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;
    let _cmd_context = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_workspace_param("param1".to_string(), Some("workspace_init".to_string()))
        .await?;

    // Create another CmdContext, this time using no parameter.
    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_workspace_param("param1".to_string(), Option::<String>::None)
        .with_workspace_param("param2".to_string(), Option::<String>::None)
        .await?;
    let workspace_init_file = resources.borrow::<WorkspaceInitFile>();
    let workspace_init = resources.borrow::<String>();

    assert!(workspace_init_file.exists());
    assert_eq!(
        "workspace_init\n",
        tokio::fs::read_to_string(&*workspace_init_file).await?
    );
    assert_eq!("workspace_init", workspace_init.as_str());
    Ok(())
}

#[tokio::test]
async fn build_inserts_profile_init_params_from_parameter() -> Result<(), Box<dyn std::error::Error>>
{
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;

    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_profile_param("param".to_string(), Some("profile_init".to_string()))
        .await?;
    let profile_init_file = resources.borrow::<ProfileInitFile>();
    let profile_init = resources.borrow::<String>();

    assert!(profile_init_file.exists());
    assert_eq!(
        "profile_init\n",
        tokio::fs::read_to_string(&*profile_init_file).await?
    );
    assert_eq!("profile_init", profile_init.as_str());
    Ok(())
}

#[tokio::test]
async fn build_inserts_profile_init_params_from_storage() -> Result<(), Box<dyn std::error::Error>>
{
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;
    let _cmd_context = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_profile_param("param1".to_string(), Some("profile_init".to_string()))
        .await?;

    // Create another CmdContext, this time using no parameter.
    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_profile_param("param1".to_string(), None::<String>)
        .with_profile_param("param2".to_string(), None::<String>)
        .await?;
    let profile_init_file = resources.borrow::<ProfileInitFile>();
    let profile_init = resources.borrow::<String>();

    assert!(profile_init_file.exists());
    assert_eq!(
        "profile_init\n",
        tokio::fs::read_to_string(&*profile_init_file).await?
    );
    assert_eq!("profile_init", profile_init.as_str());
    Ok(())
}

#[tokio::test]
async fn build_inserts_flow_init_params_from_parameter() -> Result<(), Box<dyn std::error::Error>> {
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;

    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_flow_param("param".to_string(), Some("flow_init".to_string()))
        .await?;
    let flow_init_file = resources.borrow::<FlowInitFile>();
    let flow_init = resources.borrow::<String>();

    assert!(flow_init_file.exists());
    assert_eq!(
        "flow_init\n",
        tokio::fs::read_to_string(&*flow_init_file).await?
    );
    assert_eq!("flow_init", flow_init.as_str());
    Ok(())
}

#[tokio::test]
async fn build_inserts_flow_init_params_from_storage() -> Result<(), Box<dyn std::error::Error>> {
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;
    let _cmd_context = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_flow_param("param1".to_string(), Some("flow_init".to_string()))
        .await?;

    // Create another CmdContext, this time using no parameter.
    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_flow_param("param1".to_string(), None::<String>)
        .with_flow_param("param2".to_string(), None::<String>)
        .await?;
    let flow_init_file = resources.borrow::<FlowInitFile>();
    let flow_init = resources.borrow::<String>();

    assert!(flow_init_file.exists());
    assert_eq!(
        "flow_init\n",
        tokio::fs::read_to_string(&*flow_init_file).await?
    );
    assert_eq!("flow_init", flow_init.as_str());
    Ok(())
}

// Test fixture
async fn test_setup() -> Result<
    (
        tempfile::TempDir,
        Workspace,
        ItemSpecGraph<VecCopyError>,
        NoOpOutput,
    ),
    Box<dyn std::error::Error>,
> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().into()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };

    Ok((tempdir, workspace, graph, NoOpOutput))
}
