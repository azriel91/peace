use diff::{VecDiff, VecDiffType};
use peace::{
    cfg::{OpCheckStatus, ProgressLimit, State},
    resources::{
        resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
        Resources, StateDiffs, StateDiffsMut, States, StatesDesired, StatesDesiredMut, StatesMut,
    },
    rt_model::{ItemSpecRt, ItemSpecWrapper},
};

use crate::{VecA, VecB, VecCopyError, VecCopyItemSpec, VecCopyItemSpecWrapper};

#[tokio::test]
async fn deref_to_dyn_item_spec_rt() {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let item_spec_rt: &dyn ItemSpecRt<_> = &item_spec_wrapper;

    assert_eq!(
        format!("{:?}", VecCopyItemSpec),
        format!("{:?}", item_spec_rt)
    );
}

#[tokio::test]
async fn deref_mut_to_dyn_item_spec_rt() {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let item_spec_rt: &dyn ItemSpecRt<_> = &item_spec_wrapper;

    assert_eq!(
        format!("{:?}", VecCopyItemSpec),
        format!("{:?}", item_spec_rt)
    );
}

#[tokio::test]
async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let mut resources = Resources::new();
    item_spec_wrapper.setup(&mut resources).await?;

    assert!(resources.try_borrow::<VecA>().is_ok());

    Ok(())
}

#[tokio::test]
async fn state_current_fn_exec() -> Result<(), Box<dyn std::error::Error>> {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let resources = resources_set_up(&item_spec_wrapper).await?;

    let state = item_spec_wrapper.state_current_fn_exec(&resources).await?;

    assert_eq!(
        Some(State::new(vec![], ())).as_ref(),
        state.downcast_ref::<State<Vec<u8>, ()>>()
    );

    Ok(())
}

#[tokio::test]
async fn state_desired_fn_exec() -> Result<(), VecCopyError> {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let resources = resources_set_up(&item_spec_wrapper).await?;

    let state_desired = item_spec_wrapper.state_desired_fn_exec(&resources).await?;

    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        state_desired.downcast_ref::<Vec<u8>>()
    );

    Ok(())
}

#[tokio::test]
async fn state_diff_fn_exec() -> Result<(), VecCopyError> {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);

    let resources = resources_with_states_now_and_desired(&item_spec_wrapper).await?;

    let state_diff = item_spec_wrapper.state_diff_fn_exec(&resources).await?;

    assert_eq!(
        Some(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }]))
        .as_ref(),
        state_diff.downcast_ref::<VecDiff<u8>>()
    );

    Ok(())
}

#[tokio::test]
async fn ensure_op_check() -> Result<(), VecCopyError> {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let resources = resources_with_state_diffs(&item_spec_wrapper).await?;

    let op_check_status = item_spec_wrapper.ensure_op_check(&resources).await?;

    assert_eq!(
        OpCheckStatus::ExecRequired {
            progress_limit: ProgressLimit::Bytes(8)
        },
        op_check_status
    );

    Ok(())
}

#[tokio::test]
async fn ensure_op_exec_dry() -> Result<(), VecCopyError> {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let resources = resources_with_state_diffs(&item_spec_wrapper).await?;

    item_spec_wrapper.ensure_op_exec_dry(&resources).await?;

    let vec_b = resources.borrow::<VecB>();
    assert_eq!(&[0u8; 0], &*vec_b.0);

    Ok(())
}

#[tokio::test]
async fn ensure_op_exec() -> Result<(), VecCopyError> {
    let item_spec_wrapper = ItemSpecWrapper::from(VecCopyItemSpec);
    let resources = resources_with_state_diffs(&item_spec_wrapper).await?;

    item_spec_wrapper.ensure_op_exec(&resources).await?;

    let vec_b = resources.borrow::<VecB>();
    assert_eq!(&[0u8, 1, 2, 3, 4, 5, 6, 7], &*vec_b.0);

    Ok(())
}

async fn resources_set_up(
    item_spec_wrapper: &VecCopyItemSpecWrapper,
) -> Result<Resources<SetUp>, VecCopyError> {
    let mut resources = Resources::new();
    item_spec_wrapper.setup(&mut resources).await?;
    let resources = Resources::<SetUp>::from(resources);

    Ok(resources)
}

async fn resources_with_states_now_and_desired(
    item_spec_wrapper: &VecCopyItemSpecWrapper,
) -> Result<Resources<WithStatesCurrentAndDesired>, VecCopyError> {
    let resources = resources_set_up(item_spec_wrapper).await?;

    let states_current = {
        let mut states_mut = StatesMut::new();
        let state = item_spec_wrapper.state_current_fn_exec(&resources).await?;
        states_mut.insert_raw(item_spec_wrapper.id(), state);

        States::from(states_mut)
    };
    let states_desired = {
        let mut states_desired_mut = StatesDesiredMut::new();
        let state_desired = item_spec_wrapper.state_desired_fn_exec(&resources).await?;
        states_desired_mut.insert_raw(item_spec_wrapper.id(), state_desired);

        StatesDesired::from(states_desired_mut)
    };
    let resources =
        Resources::<WithStatesCurrentAndDesired>::from((resources, states_current, states_desired));
    Ok(resources)
}

async fn resources_with_state_diffs(
    item_spec_wrapper: &VecCopyItemSpecWrapper,
) -> Result<Resources<WithStateDiffs>, VecCopyError> {
    let resources = resources_with_states_now_and_desired(item_spec_wrapper).await?;

    let state_diffs = {
        let mut state_diffs_mut = StateDiffsMut::new();
        let state_desired = item_spec_wrapper.state_diff_fn_exec(&resources).await?;
        state_diffs_mut.insert_raw(item_spec_wrapper.id(), state_desired);

        StateDiffs::from(state_diffs_mut)
    };
    let resources = Resources::<WithStateDiffs>::from((resources, state_diffs));
    Ok(resources)
}
