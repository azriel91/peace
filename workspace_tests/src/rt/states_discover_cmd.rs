use peace::{
    cfg::{app_name, profile, AppName, FlowId, ItemSpec, ItemSpecId, Profile},
    resources::{
        paths::{StatesDesiredFile, StatesSavedFile},
        states::{StatesCurrent, StatesDesired},
        type_reg::untagged::{BoxDtDisplay, TypeReg},
    },
    rt::cmds::StatesDiscoverCmd,
    rt_model::{cmd::CmdContext, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};

use crate::{NoOpOutput, VecCopyError, VecCopyItemSpec, VecCopyState};

#[tokio::test]
async fn runs_state_current_and_state_desired() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<VecCopyError>::new();
        graph_builder.add_fn(VecCopyItemSpec.into());
        graph_builder.build()
    };
    let mut output = NoOpOutput;
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output)
        .with_profile(profile!("test_profile"))
        .with_flow_id(FlowId::new(crate::fn_name_short!())?)
        .await?;

    let CmdContext { resources, .. } = StatesDiscoverCmd::exec(cmd_context).await?;

    let states_current = resources.borrow::<StatesCurrent>();
    let states_desired = resources.borrow::<StatesDesired>();
    let vec_copy_state = states_current.get::<VecCopyState, _>(VecCopyItemSpec.id());
    let states_on_disk = {
        let states_saved_file = resources.borrow::<StatesSavedFile>();
        let states_slice = std::fs::read(&*states_saved_file)?;

        let mut type_reg = TypeReg::<ItemSpecId, BoxDtDisplay>::new_typed();
        type_reg.register::<VecCopyState>(VecCopyItemSpec.id().clone());

        let deserializer = serde_yaml::Deserializer::from_slice(&states_slice);
        StatesCurrent::from(type_reg.deserialize_map(deserializer)?)
    };
    let vec_copy_desired_state = states_desired.get::<VecCopyState, _>(VecCopyItemSpec.id());
    let states_desired_on_disk = {
        let states_desired_file = resources.borrow::<StatesDesiredFile>();
        let states_slice = std::fs::read(&*states_desired_file)?;

        let mut type_reg = TypeReg::<ItemSpecId, BoxDtDisplay>::new_typed();
        type_reg.register::<VecCopyState>(VecCopyItemSpec.id().clone());

        let deserializer = serde_yaml::Deserializer::from_slice(&states_slice);
        StatesDesired::from(type_reg.deserialize_map(deserializer)?)
    };
    assert_eq!(Some(VecCopyState::new()).as_ref(), vec_copy_state);
    assert_eq!(
        states_current.get::<VecCopyState, _>(VecCopyItemSpec.id()),
        states_on_disk.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        vec_copy_desired_state
    );
    assert_eq!(
        states_desired.get::<VecCopyState, _>(VecCopyItemSpec.id()),
        states_desired_on_disk.get::<VecCopyState, _>(VecCopyItemSpec.id())
    );

    Ok(())
}

#[test]
fn debug() {
    let debug_str = format!(
        "{:?}",
        StatesDiscoverCmd::<VecCopyError, NoOpOutput, (), (), ()>::default()
    );
    assert!(
        debug_str
            == r#"StatesDiscoverCmd(PhantomData<(workspace_tests::vec_copy_item_spec::VecCopyError, workspace_tests::no_op_output::NoOpOutput, (), (), ())>)"#
            || debug_str == r#"StatesDiscoverCmd(PhantomData)"#
    );
}
