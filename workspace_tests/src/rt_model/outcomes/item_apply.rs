use peace::{
    cfg::OpCheckStatus,
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemApply, ItemApplyPartial, ItemApplyRt},
};

#[test]
fn try_from_returns_ok_when_state_saved_is_none_and_others_are_some()
-> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: None,
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();

    assert_eq!(
        ItemApply {
            state_saved: None,
            state_current: 123u32,
            state_target: 789u32,
            state_diff: 8u8,
            op_check_status: OpCheckStatus::ExecNotRequired,
            state_applied: None,
        },
        item_apply
    );
    Ok(())
}

#[test]
fn try_from_returns_ok_when_all_fields_are_some() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();

    assert_eq!(
        ItemApply {
            state_saved: Some(456u32),
            state_current: 123u32,
            state_target: 789u32,
            state_diff: 8u8,
            op_check_status: OpCheckStatus::ExecNotRequired,
            state_applied: None,
        },
        item_apply
    );
    Ok(())
}

#[test]
fn try_from_passes_through_state_applied() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, Some(789))).unwrap();

    assert_eq!(
        ItemApply {
            state_saved: Some(456u32),
            state_current: 123u32,
            state_target: 789u32,
            state_diff: 8u8,
            op_check_status: OpCheckStatus::ExecNotRequired,
            state_applied: Some(789),
        },
        item_apply
    );
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_current_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: None,
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_saved: Some(456u32),
            state_current: None,
            state_target: Some(789u32),
            state_diff: Some(8u8),
            op_check_status: Some(OpCheckStatus::ExecNotRequired),
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_target_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: None,
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_saved: Some(456u32),
            state_current: Some(123u32),
            state_target: None,
            state_diff: Some(8u8),
            op_check_status: Some(OpCheckStatus::ExecNotRequired),
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_diff_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: None::<u8>,
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_saved: Some(456u32),
            state_current: Some(123u32),
            state_target: Some(789u32),
            state_diff: None,
            op_check_status: Some(OpCheckStatus::ExecNotRequired),
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_op_check_status_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: None,
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_saved: Some(456u32),
            state_current: Some(123u32),
            state_target: Some(789u32),
            state_diff: Some(8u8),
            op_check_status: None,
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

#[test]
fn item_apply_rt_state_saved_returns_state_saved() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let state_saved = ItemApplyRt::state_saved(&item_apply).unwrap();

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
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let state_current = ItemApplyRt::state_current(&item_apply);

    assert_eq!(
        123u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_current).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_target_returns_state_target() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let state_target = ItemApplyRt::state_target(&item_apply);

    assert_eq!(
        789u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_target).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_state_diff_returns_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let state_diff = ItemApplyRt::state_diff(&item_apply);

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
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let op_check_status = ItemApplyRt::op_check_status(&item_apply);

    assert_eq!(OpCheckStatus::ExecNotRequired, op_check_status);
    Ok(())
}

#[test]
fn item_apply_rt_state_applied_returns_state_applied() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, Some(456u32))).unwrap();
    let state_applied = ItemApplyRt::state_applied(&item_apply).unwrap();

    assert_eq!(
        456u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_applied).unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_as_data_type_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_clone = item_apply.clone();
    let data_type = ItemApplyRt::as_data_type(&item_apply_clone);

    assert_eq!(
        item_apply,
        *data_type.downcast_ref::<ItemApply<u32, u8>>().unwrap()
    );
    Ok(())
}

#[test]
fn item_apply_rt_as_data_type_mut_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let mut item_apply_clone = item_apply.clone();
    let data_type = ItemApplyRt::as_data_type_mut(&mut item_apply_clone);

    assert_eq!(
        item_apply,
        *data_type.downcast_mut::<ItemApply<u32, u8>>().unwrap()
    );
    Ok(())
}
