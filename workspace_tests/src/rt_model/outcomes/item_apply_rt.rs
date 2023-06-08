use peace::{
    cfg::ApplyCheck,
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemApply, ItemApplyPartial, ItemApplyRt},
};

#[test]
fn state_current_stored_returns_state_current_stored() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_boxed = Box::new(item_apply) as Box<dyn ItemApplyRt>;
    let state_current_stored = ItemApplyRt::state_current_stored(&item_apply_boxed).unwrap();

    assert_eq!(
        456u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_current_stored).unwrap()
    );
    Ok(())
}

#[test]
fn state_current_returns_state_current() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_boxed = Box::new(item_apply) as Box<dyn ItemApplyRt>;
    let state_current = ItemApplyRt::state_current(&item_apply_boxed);

    assert_eq!(
        123u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_current).unwrap()
    );
    Ok(())
}

#[test]
fn state_target_returns_state_target() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_boxed = Box::new(item_apply) as Box<dyn ItemApplyRt>;
    let state_target = ItemApplyRt::state_target(&item_apply_boxed);

    assert_eq!(
        789u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_target).unwrap()
    );
    Ok(())
}

#[test]
fn state_diff_returns_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_boxed = Box::new(item_apply) as Box<dyn ItemApplyRt>;
    let state_diff = ItemApplyRt::state_diff(&item_apply_boxed);

    assert_eq!(
        8u8,
        *BoxDataTypeDowncast::<u8>::downcast_ref(&state_diff).unwrap()
    );
    Ok(())
}

#[test]
fn apply_check_returns_apply_check() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_boxed = Box::new(item_apply) as Box<dyn ItemApplyRt>;
    let apply_check = ItemApplyRt::apply_check(&item_apply_boxed);

    assert_eq!(ApplyCheck::ExecNotRequired, apply_check);
    Ok(())
}

#[test]
fn state_applied_returns_state_applied() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, Some(456u32))).unwrap();
    let item_apply_boxed = Box::new(item_apply) as Box<dyn ItemApplyRt>;
    let state_applied = ItemApplyRt::state_applied(&item_apply_boxed).unwrap();

    assert_eq!(
        456u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_applied).unwrap()
    );
    Ok(())
}

#[test]
fn as_data_type_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_clone = item_apply.clone();
    let item_apply_clone_boxed = Box::new(item_apply_clone) as Box<dyn ItemApplyRt>;
    let data_type = ItemApplyRt::as_data_type(&item_apply_clone_boxed);

    assert_eq!(
        item_apply,
        *data_type.downcast_ref::<ItemApply<u32, u8>>().unwrap()
    );
    Ok(())
}

#[test]
fn as_data_type_mut_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, None)).unwrap();
    let item_apply_clone = item_apply.clone();
    let mut item_apply_clone_boxed = Box::new(item_apply_clone) as Box<dyn ItemApplyRt>;
    let data_type = ItemApplyRt::as_data_type_mut(&mut item_apply_clone_boxed);

    assert_eq!(
        item_apply,
        *data_type.downcast_mut::<ItemApply<u32, u8>>().unwrap()
    );
    Ok(())
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    let item_apply_partial = ItemApplyPartial {
        state_current_stored: Some(456u32),
        state_current: Some(123u32),
        state_target: Some(789u32),
        state_diff: Some(8u8),
        apply_check: Some(ApplyCheck::ExecNotRequired),
    };

    let item_apply = ItemApply::try_from((item_apply_partial, Some(456u32))).unwrap();
    let item_apply_boxed = Box::new(item_apply) as Box<dyn ItemApplyRt>;

    let serialized = serde_yaml::to_string(&item_apply_boxed)?;
    assert_eq!(
        r#"state_current_stored: 456
state_current: 123
state_target: 789
state_diff: 8
apply_check: ExecNotRequired
state_applied: 456
"#,
        serialized
    );
    Ok(())
}
