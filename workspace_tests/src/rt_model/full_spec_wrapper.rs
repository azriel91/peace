use peace::{
    cfg::State,
    resources::{resources_type_state::SetUp, Resources},
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
    let state = full_spec_wrapper.state_now_fn_exec(&resources).await?;

    assert_eq!(
        Some(State::new(vec![], ())).as_ref(),
        state.downcast_ref::<State<Vec<u8>, ()>>()
    );

    Ok(())
}

#[tokio::test]
async fn state_desired_fn_exec() -> Result<(), VecCopyError> {
    let full_spec_wrapper = FullSpecWrapper::from(VecCopyFullSpec);
    let mut resources = Resources::new();
    full_spec_wrapper.setup(&mut resources).await?;

    let resources = Resources::<SetUp>::from(resources);
    let state_desired = full_spec_wrapper.state_desired_fn_exec(&resources).await?;

    assert_eq!(
        Some(vec![0u8, 1, 2, 3, 4, 5, 6, 7]).as_ref(),
        state_desired.downcast_ref::<Vec<u8>>()
    );

    Ok(())
}
