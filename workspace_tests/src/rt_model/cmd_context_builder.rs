use std::path::Path;

use peace::{
    cfg::{profile, FlowId, Profile},
    resources::internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    rt_model::{
        CmdContext, CmdContextBuilder, ItemSpecGraph, ItemSpecGraphBuilder, Workspace,
        WorkspaceSpec,
    },
};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

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
    let workspace_params_file = resources.borrow::<WorkspaceParamsFile>();
    let workspace_init = resources.borrow::<String>();

    assert!(workspace_params_file.exists());
    assert_eq!(
        "param: workspace_init\n",
        tokio::fs::read_to_string(&*workspace_params_file).await?
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
        .with_workspace_param("param2".to_string(), Option::<u32>::None)
        .await?;
    let workspace_params_file = resources.borrow::<WorkspaceParamsFile>();
    let workspace_init = resources.borrow::<String>();

    assert!(workspace_params_file.exists());
    assert_eq!(
        "param1: workspace_init\n",
        tokio::fs::read_to_string(&*workspace_params_file).await?
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
    let profile_params_file = resources.borrow::<ProfileParamsFile>();
    let profile_init = resources.borrow::<String>();

    assert!(profile_params_file.exists());
    assert_eq!(
        "param: profile_init\n",
        tokio::fs::read_to_string(&*profile_params_file).await?
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
    let profile_params_file = resources.borrow::<ProfileParamsFile>();
    let profile_init = resources.borrow::<String>();

    assert!(profile_params_file.exists());
    assert_eq!(
        "param1: profile_init\n",
        tokio::fs::read_to_string(&*profile_params_file).await?
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
    let flow_params_file = resources.borrow::<FlowParamsFile>();
    let flow_init = resources.borrow::<String>();

    assert!(flow_params_file.exists());
    assert_eq!(
        "param: flow_init\n",
        tokio::fs::read_to_string(&*flow_params_file).await?
    );
    assert_eq!("flow_init", flow_init.as_str());
    Ok(())
}

#[tokio::test]
async fn build_inserts_flow_init_params_from_storage() -> Result<(), Box<dyn std::error::Error>> {
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;
    let _cmd_context = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_flow_param("param1".to_string(), Some("flow_init".to_string()))
        .with_flow_param("param2".to_string(), None::<String>)
        .await?;

    // Create another CmdContext, this time using no parameter.
    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_flow_param("param1".to_string(), None::<String>)
        .with_flow_param("param2".to_string(), None::<String>)
        .await?;
    let flow_params_file = resources.borrow::<FlowParamsFile>();
    let flow_init = resources.borrow::<String>();

    assert!(flow_params_file.exists());
    assert_eq!(
        "param1: flow_init\n",
        tokio::fs::read_to_string(&*flow_params_file).await?
    );
    assert_eq!("flow_init", flow_init.as_str());
    Ok(())
}

#[tokio::test]
async fn build_inserts_mix_params_from_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let (_tempdir, workspace, graph, mut output) = test_setup().await?;

    let CmdContext { resources, .. } = CmdContextBuilder::new(&workspace, &graph, &mut output)
        .with_flow_param(FlowKey::F1, Some("flow_1".to_string()))
        .with_workspace_param(WorkspaceKey::W1, Some(true))
        .with_profile_param(ProfileKey::P1, Some(1u32))
        .with_workspace_param(WorkspaceKey::W2, Some(2u8))
        .with_flow_param(FlowKey::F2, Some(vec!["flow_2".to_string()]))
        .with_profile_param(ProfileKey::P2, Some(2u64))
        .await?;
    let workspace_params_file = resources.borrow::<WorkspaceParamsFile>();
    let workspace_1 = resources.borrow::<bool>();
    let workspace_2 = resources.borrow::<u8>();
    let profile_params_file = resources.borrow::<ProfileParamsFile>();
    let profile_1 = resources.borrow::<u32>();
    let profile_2 = resources.borrow::<u64>();
    let flow_params_file = resources.borrow::<FlowParamsFile>();
    let flow_1 = resources.borrow::<String>();
    let flow_2 = resources.borrow::<Vec<String>>();

    assert!(workspace_params_file.exists());
    assert_eq!(
        r#"W1: true
W2: 2
"#,
        tokio::fs::read_to_string(&*workspace_params_file).await?
    );
    assert_eq!(true, *workspace_1);
    assert_eq!(2u8, *workspace_2);

    assert!(profile_params_file.exists());
    assert_eq!(
        r#"P1: 1
P2: 2
"#,
        tokio::fs::read_to_string(&*profile_params_file).await?
    );
    assert_eq!(1u32, *profile_1);
    assert_eq!(2u64, *profile_2);

    assert!(flow_params_file.exists());
    assert_eq!(
        r#"F1: flow_1
F2:
- flow_2
"#,
        tokio::fs::read_to_string(&*flow_params_file).await?
    );
    assert_eq!("flow_1", flow_1.as_str());
    assert_eq!(vec!["flow_2".to_string()], *flow_2);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum WorkspaceKey {
    W1,
    W2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum ProfileKey {
    P1,
    P2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum FlowKey {
    F1,
    F2,
}
