use peace::{
    cfg::{flow_id, profile, FlowId, ItemSpec, ItemSpecId, Profile},
    resources::{paths::StatesDesiredFile, states::StatesDesired, type_reg::untagged::TypeReg},
    rt::StatesDesiredDiscoverCmd,
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_desired_for_each_item_spec() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::init(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        flow_id!("test_flow"),
    )
    .await?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let cmd_context = CmdContext::init(&workspace, &graph).await?;

    let CmdContext { resources, .. } = StatesDesiredDiscoverCmd::exec(cmd_context).await?;

    let states_desired = resources.borrow::<StatesDesired>();
    let vec_copy_desired_state = states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id());
    let states_desired_on_disk = {
        let states_desired_file = resources.borrow::<StatesDesiredFile>();
        let states_slice = std::fs::read(&*states_desired_file)?;

        let mut type_reg = TypeReg::<ItemSpecId>::new();
        type_reg.register::<<VecCopyItemSpec as ItemSpec>::StateLogical>(VecCopyItemSpec.id());

        let deserializer = serde_yaml::Deserializer::from_slice(&states_slice);
        StatesDesired::from(type_reg.deserialize_map(deserializer)?)
    };
    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        vec_copy_desired_state
    );
    assert_eq!(
        states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id()),
        states_desired_on_disk.get::<Vec<u8>, _>(&VecCopyItemSpec.id())
    );

    Ok(())
}
