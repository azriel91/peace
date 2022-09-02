use peace::{
    cfg::{flow_id, profile, FlowId, ItemSpec, ItemSpecId, Profile, State},
    resources::{
        dir::FlowDir,
        states::{StatesCurrent, StatesDesired},
        type_reg::untagged::TypeReg,
    },
    rt::{StateCurrentCmd, StateDesiredCmd, StateDiscoverCmd},
    rt_model::{CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{VecCopyError, VecCopyItemSpec};

#[tokio::test]
async fn runs_state_current_and_state_desired() -> Result<(), Box<dyn std::error::Error>> {
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

    let CmdContext { resources, .. } = StateDiscoverCmd::exec(cmd_context).await?;

    let states = resources.borrow::<StatesCurrent>();
    let states_desired = resources.borrow::<StatesDesired>();
    let vec_copy_state = states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id());
    let states_on_disk = {
        let flow_dir = resources.borrow::<FlowDir>();
        let states_file = flow_dir.join(StateCurrentCmd::<VecCopyError>::STATES_CURRENT_FILE);
        let states_slice = std::fs::read(states_file)?;

        let mut type_reg = TypeReg::<ItemSpecId>::new();
        type_reg.register::<State<Vec<u8>, ()>>(VecCopyItemSpec.id());

        let deserializer = serde_yaml::Deserializer::from_slice(&states_slice);
        StatesCurrent::from(type_reg.deserialize_map(deserializer)?)
    };
    let vec_copy_desired_state = states_desired.get::<Vec<u8>, _>(&VecCopyItemSpec.id());
    let states_desired_on_disk = {
        let flow_dir = resources.borrow::<FlowDir>();
        let states_file = flow_dir.join(StateDesiredCmd::<VecCopyError>::STATES_DESIRED_FILE);
        let states_slice = std::fs::read(states_file)?;

        let mut type_reg = TypeReg::<ItemSpecId>::new();
        type_reg.register::<<VecCopyItemSpec as ItemSpec>::StateLogical>(VecCopyItemSpec.id());

        let deserializer = serde_yaml::Deserializer::from_slice(&states_slice);
        StatesDesired::from(type_reg.deserialize_map(deserializer)?)
    };
    assert_eq!(
        Some(State::new(Vec::<u8>::new(), ())).as_ref(),
        vec_copy_state
    );
    assert_eq!(
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id()),
        states_on_disk.get::<State<Vec<u8>, ()>, _>(&VecCopyItemSpec.id())
    );
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
