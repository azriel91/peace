use diff::{VecDiff, VecDiffType};
use peace::{
    cfg::{ApplyCheck, FnCtx},
    data::marker::{ApplyDry, Clean, Current, Desired},
    params::{ParamsSpecs, ValueSpec},
    resources::{
        internal::StatesMut,
        resources::ts::SetUp,
        states::{self, StatesCurrent, StatesDesired, StatesSaved},
        type_reg::untagged::BoxDataTypeDowncast,
        Resources,
    },
    rt_model::{ItemSpecRt, ItemSpecWrapper},
};
cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace::cfg::progress::{ProgressLimit, ProgressSender};
        use tokio::sync::mpsc;
    }
}

use crate::{
    VecA, VecASpec, VecB, VecCopyDiff, VecCopyError, VecCopyItemSpec, VecCopyItemSpecWrapper,
    VecCopyState,
};

#[tokio::test]
async fn deref_to_dyn_item_spec_rt() {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec.clone());
    let item_spec_rt: &dyn ItemSpecRt<_> = &item_spec_wrapper;

    assert_eq!(
        format!("{vec_copy_item_spec:?}"),
        format!("{item_spec_rt:?}")
    );
}

#[tokio::test]
async fn deref_mut_to_dyn_item_spec_rt() {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec.clone());
    let item_spec_rt: &dyn ItemSpecRt<_> = &item_spec_wrapper;

    assert_eq!(
        format!("{vec_copy_item_spec:?}"),
        format!("{item_spec_rt:?}")
    );
}

#[tokio::test]
async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let mut resources = Resources::new();
    <dyn ItemSpecRt<_>>::setup(&item_spec_wrapper, &mut resources).await?;
    resources.insert(VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]));

    assert!(resources.try_borrow::<VecA>().is_ok());
    // Automatic `Current<State>` and `Desired<State>` insertion.
    assert!(resources.try_borrow::<Current<VecCopyState>>().is_ok());
    assert!(resources.borrow::<Current<VecCopyState>>().is_none());
    assert!(resources.try_borrow::<Desired<VecCopyState>>().is_ok());
    assert!(resources.borrow::<Desired<VecCopyState>>().is_none());

    Ok(())
}

#[tokio::test]
async fn state_current_try_exec() -> Result<(), Box<dyn std::error::Error>> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    let state = item_spec_wrapper
        .state_current_try_exec(&params_specs, &resources, fn_ctx)
        .await?
        .unwrap();

    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        BoxDataTypeDowncast::<VecCopyState>::downcast_ref(&state)
    );
    // Automatic `Current<State>` insertion.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        resources.borrow::<Current<VecCopyState>>().as_ref()
    );

    Ok(())
}

#[tokio::test]
async fn state_desired_try_exec() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    let state_desired = item_spec_wrapper
        .state_desired_try_exec(&params_specs, &resources, fn_ctx)
        .await?
        .unwrap();

    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        BoxDataTypeDowncast::<VecCopyState>::downcast_ref(&state_desired)
    );
    // Automatic `Desired<State>` insertion.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        resources.borrow::<Desired<VecCopyState>>().as_ref()
    );

    Ok(())
}

#[tokio::test]
async fn state_diff_exec() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);

    let (params_specs, resources, states_saved, states_desired) =
        resources_and_states_saved_and_desired(&item_spec_wrapper).await?;

    let state_diff = item_spec_wrapper
        .state_diff_exec(&params_specs, &resources, &states_saved, &states_desired)
        .await?
        .expect("Expected state_diff to be Some when state_saved and state_desired both exist.");

    assert_eq!(
        Some(VecCopyDiff::from(VecDiff(vec![VecDiffType::Inserted {
            index: 0,
            changes: vec![0u8, 1, 2, 3, 4, 5, 6, 7]
        }])))
        .as_ref(),
        BoxDataTypeDowncast::<VecCopyDiff>::downcast_ref(&state_diff)
    );

    Ok(())
}

#[tokio::test]
async fn ensure_prepare() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    match <dyn ItemSpecRt<_>>::ensure_prepare(&item_spec_wrapper, &params_specs, &resources, fn_ctx)
        .await
    {
        Ok(item_apply) => {
            #[cfg(not(feature = "output_progress"))]
            assert_eq!(ApplyCheck::ExecRequired, item_apply.apply_check());
            #[cfg(feature = "output_progress")]
            assert_eq!(
                ApplyCheck::ExecRequired {
                    progress_limit: ProgressLimit::Bytes(8)
                },
                item_apply.apply_check()
            );

            Ok(())
        }
        Err((error, _item_apply_partial)) => Err(error),
    }
}

#[tokio::test]
async fn apply_exec_dry_for_ensure() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    let mut item_apply_boxed =
        <dyn ItemSpecRt<_>>::ensure_prepare(&item_spec_wrapper, &params_specs, &resources, fn_ctx)
            .await
            .map_err(|(error, _)| error)?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    <dyn ItemSpecRt<_>>::apply_exec_dry(
        &item_spec_wrapper,
        &params_specs,
        &resources,
        fn_ctx,
        &mut item_apply_boxed,
    )
    .await?;

    let vec_b = resources.borrow::<VecB>();
    assert_eq!(&[0u8; 0], &*vec_b.0);

    // Automatic `Current<State>` insertion.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        resources.borrow::<Current<VecCopyState>>().as_ref()
    );
    // Automatic `ApplyDry<State>` insertion.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        resources.borrow::<ApplyDry<VecCopyState>>().as_ref()
    );
    // Desired should also exist.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        resources.borrow::<Desired<VecCopyState>>().as_ref()
    );

    Ok(())
}

#[tokio::test]
async fn apply_exec_for_ensure() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    let mut item_apply_boxed =
        <dyn ItemSpecRt<_>>::ensure_prepare(&item_spec_wrapper, &params_specs, &resources, fn_ctx)
            .await
            .map_err(|(error, _)| error)?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    <dyn ItemSpecRt<_>>::apply_exec(
        &item_spec_wrapper,
        &params_specs,
        &resources,
        fn_ctx,
        &mut item_apply_boxed,
    )
    .await?;

    let vec_b = resources.borrow::<VecB>();
    assert_eq!(&[0u8, 1, 2, 3, 4, 5, 6, 7], &*vec_b.0);

    // Automatic `Current<State>` insertion.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        resources.borrow::<Current<VecCopyState>>().as_ref()
    );
    // Desired should also exist.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        resources.borrow::<Desired<VecCopyState>>().as_ref()
    );

    Ok(())
}

#[tokio::test]
async fn clean_prepare() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up_pre_saved(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    match <dyn ItemSpecRt<_>>::clean_prepare(&item_spec_wrapper, &params_specs, &resources, fn_ctx)
        .await
    {
        Ok(item_apply) => {
            #[cfg(not(feature = "output_progress"))]
            assert_eq!(ApplyCheck::ExecRequired, item_apply.apply_check());
            #[cfg(feature = "output_progress")]
            assert_eq!(
                ApplyCheck::ExecRequired {
                    progress_limit: ProgressLimit::Bytes(8)
                },
                item_apply.apply_check()
            );

            Ok(())
        }
        Err((error, _item_apply_partial)) => Err(error),
    }
}

#[tokio::test]
async fn apply_exec_dry_for_clean() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up_pre_saved(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    let mut item_apply_boxed =
        <dyn ItemSpecRt<_>>::clean_prepare(&item_spec_wrapper, &params_specs, &resources, fn_ctx)
            .await
            .map_err(|(error, _)| error)?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    <dyn ItemSpecRt<_>>::apply_exec_dry(
        &item_spec_wrapper,
        &params_specs,
        &resources,
        fn_ctx,
        &mut item_apply_boxed,
    )
    .await?;

    let vec_b = resources.borrow::<VecB>();
    assert_eq!(&[0u8, 1, 2, 3, 4, 5, 6, 7], &*vec_b.0);

    // Automatic `Current<State>` insertion.
    assert_eq!(
        Some(VecCopyState::from(vec![0u8, 1, 2, 3, 4, 5, 6, 7])).as_ref(),
        resources.borrow::<Current<VecCopyState>>().as_ref()
    );
    // Automatic `ApplyDry<State>` insertion.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        resources.borrow::<ApplyDry<VecCopyState>>().as_ref()
    );
    // Clean should also exist.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        resources.borrow::<Clean<VecCopyState>>().as_ref()
    );

    Ok(())
}

#[tokio::test]
async fn apply_exec_for_clean() -> Result<(), VecCopyError> {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_wrapper = ItemSpecWrapper::<_, VecCopyError>::from(vec_copy_item_spec);
    let (params_specs, resources) = resources_set_up_pre_saved(&item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    let mut item_apply_boxed =
        <dyn ItemSpecRt<_>>::clean_prepare(&item_spec_wrapper, &params_specs, &resources, fn_ctx)
            .await
            .map_err(|(error, _)| error)?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    <dyn ItemSpecRt<_>>::apply_exec(
        &item_spec_wrapper,
        &params_specs,
        &resources,
        fn_ctx,
        &mut item_apply_boxed,
    )
    .await?;

    let vec_b = resources.borrow::<VecB>();
    assert_eq!(&[0u8; 0], &*vec_b.0);

    // Automatic `Current<State>` insertion.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        resources.borrow::<Current<VecCopyState>>().as_ref()
    );
    // Clean should also exist.
    assert_eq!(
        Some(VecCopyState::new()).as_ref(),
        resources.borrow::<Clean<VecCopyState>>().as_ref()
    );

    Ok(())
}

async fn resources_set_up(
    item_spec_wrapper: &VecCopyItemSpecWrapper,
) -> Result<(ParamsSpecs, Resources<SetUp>), VecCopyError> {
    let mut params_specs = ParamsSpecs::new();
    params_specs.insert(
        VecCopyItemSpec::ID_DEFAULT.clone(),
        VecASpec(ValueSpec::from_map(|vec_a: &VecA| vec_a.0.clone())),
    );

    let mut resources = Resources::new();
    <dyn ItemSpecRt<_>>::setup(item_spec_wrapper, &mut resources).await?;
    let mut resources = Resources::<SetUp>::from(resources);
    resources.insert(VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]));

    Ok((params_specs, resources))
}

async fn resources_set_up_pre_saved(
    item_spec_wrapper: &VecCopyItemSpecWrapper,
) -> Result<(ParamsSpecs, Resources<SetUp>), VecCopyError> {
    let (params_specs, mut resources) = resources_set_up(item_spec_wrapper).await?;
    resources.insert(VecB(vec![0, 1, 2, 3, 4, 5, 6, 7]));

    Ok((params_specs, resources))
}

async fn resources_and_states_saved_and_desired(
    item_spec_wrapper: &VecCopyItemSpecWrapper,
) -> Result<(ParamsSpecs, Resources<SetUp>, StatesSaved, StatesDesired), VecCopyError> {
    let (params_specs, resources) = resources_set_up(item_spec_wrapper).await?;
    cfg_if::cfg_if! {
        if #[cfg(feature = "output_progress")] {
            let (progress_tx, _progress_rx) = mpsc::channel(10);
            let progress_sender = ProgressSender::new(
                VecCopyItemSpec::ID_DEFAULT,
                &progress_tx,
            );
        }
    }
    let fn_ctx = FnCtx::new(
        VecCopyItemSpec::ID_DEFAULT,
        #[cfg(feature = "output_progress")]
        progress_sender,
    );

    let states_saved = {
        let mut states_mut = StatesMut::new();
        let state = <dyn ItemSpecRt<_>>::state_current_try_exec(
            item_spec_wrapper,
            &params_specs,
            &resources,
            fn_ctx,
        )
        .await?;
        if let Some(state) = state {
            states_mut.insert_raw(<dyn ItemSpecRt<_>>::id(item_spec_wrapper).clone(), state);
        }

        Into::<StatesSaved>::into(StatesCurrent::from(states_mut))
    };
    let states_desired = {
        let mut states_desired_mut = StatesMut::<states::ts::Desired>::new();
        let state_desired = item_spec_wrapper
            .state_desired_try_exec(&params_specs, &resources, fn_ctx)
            .await?
            .unwrap();
        states_desired_mut.insert_raw(
            <dyn ItemSpecRt<_>>::id(item_spec_wrapper).clone(),
            state_desired,
        );

        StatesDesired::from(states_desired_mut)
    };
    Ok((params_specs, resources, states_saved, states_desired))
}
