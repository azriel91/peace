use peace::{
    cfg::ApplyCheck,
    resource_rt::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemApply, ItemApplyPartial, ItemApplyRt},
};

#[test]
fn try_from_returns_ok_when_state_current_stored_is_none_and_others_are_some()
-> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: None,
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();

    assert_eq!(
        ItemApply {
            state_current_stored: None,
            state_current: 123u32,
            state_target: 789u32,
            state_diff: 8u8,
            apply_check: ApplyCheck::ExecNotRequired,
            state_applied: None,
        },
        item_apply
    );
    Ok(())
}

#[test]
fn try_from_returns_ok_when_all_fields_are_some() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();

    assert_eq!(
        ItemApply {
            state_current_stored: Some(456u32),
            state_current: 123u32,
            state_target: 789u32,
            state_diff: 8u8,
            apply_check: ApplyCheck::ExecNotRequired,
            state_applied: None,
        },
        item_apply
    );
    Ok(())
}

#[test]
fn try_from_passes_through_state_applied() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, Some(789))).unwrap();

    assert_eq!(
        ItemApply {
            state_current_stored: Some(456u32),
            state_current: 123u32,
            state_target: 789u32,
            state_diff: 8u8,
            apply_check: ApplyCheck::ExecNotRequired,
            state_applied: Some(789),
        },
        item_apply
    );
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_current_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: None,
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_current_stored: Some(456u32),
            state_current: None,
            state_target: Some(789u32),
            state_diff: Some(8u8),
            apply_check: Some(ApplyCheck::ExecNotRequired),
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_target_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: None,
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_current_stored: Some(456u32),
            state_current: Some(123u32),
            state_target: None,
            state_diff: Some(8u8),
            apply_check: Some(ApplyCheck::ExecNotRequired),
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_diff_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: None::<u8>,
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_current_stored: Some(456u32),
            state_current: Some(123u32),
            state_target: Some(789u32),
            state_diff: None,
            apply_check: Some(ApplyCheck::ExecNotRequired),
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_apply_check_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: None,
    };

    let (item_apply_partial, state_applied) =
        ItemApply::try_from((item_apply_partial, None)).unwrap_err();

    assert_eq!(
        ItemApplyPartial {
            state_current_stored: Some(456u32),
            state_current: Some(123u32),
            state_target: Some(789u32),
            state_diff: Some(8u8),
            apply_check: None,
        },
        item_apply_partial
    );
    assert!(state_applied.is_none());
    Ok(())
}

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

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let state_current_stored = ItemApplyRt::state_current_stored(&item_apply).unwrap();

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
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
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
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
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
fn item_apply_rt_apply_check_returns_apply_check() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let apply_check = ItemApplyRt::apply_check(&item_apply);

    assert_eq!(ApplyCheck::ExecNotRequired, apply_check);
    Ok(())
}

#[test]
fn item_apply_rt_state_applied_returns_state_applied() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
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
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let data_type = ItemApplyRt::as_data_type(&item_apply);

    assert_eq!(
        item_apply,
        *data_type.downcast_ref::<ItemApply<u32, u8>>().unwrap()
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

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let mut item_apply_clone = item_apply.clone();
    let data_type = ItemApplyRt::as_data_type_mut(&mut item_apply_clone);

    assert_eq!(
        item_apply,
        *data_type.downcast_mut::<ItemApply<u32, u8>>().unwrap()
    );
    Ok(())
}
