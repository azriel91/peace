use peace::{
    cfg::OpCheckStatus,
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemApplyPartial, ItemApplyPartialRt},
};

#[test]
fn item_apply_rt_state_saved_returns_state_saved() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let state_saved = ItemApplyPartialRt::state_saved(&item_apply_partial).unwrap();
    dbg!(&state_saved);

    assert_eq!(
        456u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_saved).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_current_returns_state_current() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let state_current = ItemApplyPartialRt::state_current(&item_apply_partial).unwrap();

    assert_eq!(
        123u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_current).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_desired_returns_state_desired() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let state_desired = ItemApplyPartialRt::state_desired(&item_apply_partial).unwrap();

    assert_eq!(
        789u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_desired).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_diff_returns_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let state_diff = ItemApplyPartialRt::state_diff(&item_apply_partial).unwrap();

    assert_eq!(
        8u8,
        *BoxDataTypeDowncast::<u8>::downcast_ref(&state_diff).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_op_check_status_returns_op_check_status() -> Result<(), Box<dyn std::error::Error>>
{
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let op_check_status = ItemApplyPartialRt::op_check_status(&item_apply_partial).unwrap();

    assert_eq!(OpCheckStatus::ExecNotRequired, op_check_status);
    Ok(())
}

#[test]
fn item_apply_rt_as_data_type_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

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
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
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
