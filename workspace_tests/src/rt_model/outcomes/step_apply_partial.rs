use peace::{
    cfg::ApplyCheck,
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemApplyPartial, ItemApplyPartialRt},
};

#[test]
fn item_apply_rt_state_current_stored_returns_state_current_stored()
-> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let state_current_stored =
        ItemApplyPartialRt::state_current_stored(&item_apply_partial).unwrap();
    dbg!(&state_current_stored);

    assert_eq!(
        456u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_current_stored).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_current_returns_state_current() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let state_current = ItemApplyPartialRt::state_current(&item_apply_partial).unwrap();

    assert_eq!(
        123u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_current).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_target_returns_state_target() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let state_target = ItemApplyPartialRt::state_target(&item_apply_partial).unwrap();

    assert_eq!(
        789u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_target).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_diff_returns_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let state_diff = ItemApplyPartialRt::state_diff(&item_apply_partial).unwrap();

    assert_eq!(
        8u8,
        *BoxDataTypeDowncast::<u8>::downcast_ref(&state_diff).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_apply_check_returns_apply_check() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let apply_check = ItemApplyPartialRt::apply_check(&item_apply_partial).unwrap();

    assert_eq!(ApplyCheck::ExecNotRequired, apply_check);
    Ok(())
}

#[test]
fn item_apply_rt_as_data_type_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let item_apply_partial_clone = item_apply_partial.clone();
    let data_type = ItemApplyPartialRt::as_data_type(&item_apply_partial_clone);

    assert_eq!(
        item_apply_partial,
        *data_type
            .downcast_ref::<ItemApplyPartial<u32, u8>>()
            .unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_as_data_type_mut_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let mut item_apply_partial_clone = item_apply_partial.clone();
    let data_type = ItemApplyPartialRt::as_data_type_mut(&mut item_apply_partial_clone);

    assert_eq!(
        item_apply_partial,
        *data_type
            .downcast_mut::<ItemApplyPartial<u32, u8>>()
            .unwrap()
    );
    Ok(())
}
