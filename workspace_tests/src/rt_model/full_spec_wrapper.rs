use peace::{
    cfg::{FullSpec, State},
    resources::{resources_type_state::SetUp, Resources, StatesDesiredRw, StatesRw},
    rt_model::{FullSpecRt, FullSpecWrapper},
};

use crate::{VecA, VecCopyError, VecCopyFullSpec};

#[tokio::test]
async fn deref_to_dyn_full_spec_rt() {
    let full_spec_wrapper = FullSpecWrapper::from(VecCopyFullSpec);
    let full_spec_rt: &dyn FullSpecRt<_> = &full_spec_wrapper;

    assert_eq!(
        format!("{:?}", VecCopyFullSpec),
        format!("{:?}", full_spec_rt)
    );
}

#[tokio::test]
async fn deref_mut_to_dyn_full_spec_rt() {
    let full_spec_wrapper = FullSpecWrapper::from(VecCopyFullSpec);
    let full_spec_rt: &dyn FullSpecRt<_> = &full_spec_wrapper;

    assert_eq!(
        format!("{:?}", VecCopyFullSpec),
        format!("{:?}", full_spec_rt)
    );
}

#[tokio::test]
async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let full_spec_wrapper = FullSpecWrapper::from(VecCopyFullSpec);
    let mut resources = Resources::new();
    full_spec_wrapper.setup(&mut resources).await?;

    assert!(resources.try_borrow::<VecA>().is_ok());

    Ok(())
}

#[tokio::test]
async fn state_now_fn_exec() -> Result<(), Box<dyn std::error::Error>> {
    let full_spec_wrapper = FullSpecWrapper::from(VecCopyFullSpec);
    let mut resources = Resources::new();
    full_spec_wrapper.setup(&mut resources).await?;

    let resources = Resources::<SetUp>::from(resources);
    full_spec_wrapper.state_now_fn_exec(&resources).await?;

    let states_rw = resources.borrow::<StatesRw>();
    let states = states_rw.read().await;

    assert_eq!(
        Some(State::new(vec![0, 1, 2, 3, 4, 5, 6, 7], ())).as_ref(),
        states.get::<State<Vec<u8>, ()>, _>(&VecCopyFullSpec.id())
    );

    Ok(())
}

#[tokio::test]
async fn state_desired_fn_exec() -> Result<(), VecCopyError> {
    let full_spec_wrapper = FullSpecWrapper::from(VecCopyFullSpec);
    let mut resources = Resources::new();
    full_spec_wrapper.setup(&mut resources).await?;

    let resources = Resources::<SetUp>::from(resources);
    full_spec_wrapper.state_desired_fn_exec(&resources).await?;

    let states_desired_rw = resources.borrow::<StatesDesiredRw>();
    let states_desired = states_desired_rw.read().await;

    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        states_desired.get::<Vec<u8>, _>(&VecCopyFullSpec.id())
    );

    Ok(())
}
