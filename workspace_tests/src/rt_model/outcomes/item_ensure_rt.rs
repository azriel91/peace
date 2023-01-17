use peace::{
    cfg::OpCheckStatus,
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemEnsure, ItemEnsurePartial, ItemEnsureRt},
};

#[test]
fn state_saved_returns_state_saved() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_boxed = Box::new(item_ensure) as Box<dyn ItemEnsureRt>;
    let state_saved = ItemEnsureRt::state_saved(&item_ensure_boxed).unwrap();

    assert_eq!(
        456u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_saved).unwrap()
    );
    Ok(())
}

#[test]
fn state_current_returns_state_current() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_boxed = Box::new(item_ensure) as Box<dyn ItemEnsureRt>;
    let state_current = ItemEnsureRt::state_current(&item_ensure_boxed);

    assert_eq!(
        123u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_current).unwrap()
    );
    Ok(())
}

#[test]
fn state_desired_returns_state_desired() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_boxed = Box::new(item_ensure) as Box<dyn ItemEnsureRt>;
    let state_desired = ItemEnsureRt::state_desired(&item_ensure_boxed);

    assert_eq!(
        789u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_desired).unwrap()
    );
    Ok(())
}

#[test]
fn state_diff_returns_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_boxed = Box::new(item_ensure) as Box<dyn ItemEnsureRt>;
    let state_diff = ItemEnsureRt::state_diff(&item_ensure_boxed);

    assert_eq!(
        8u8,
        *BoxDataTypeDowncast::<u8>::downcast_ref(&state_diff).unwrap()
    );
    Ok(())
}

#[test]
fn op_check_status_returns_op_check_status() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_boxed = Box::new(item_ensure) as Box<dyn ItemEnsureRt>;
    let op_check_status = ItemEnsureRt::op_check_status(&item_ensure_boxed);

    assert_eq!(OpCheckStatus::ExecNotRequired, op_check_status);
    Ok(())
}

#[test]
fn state_ensured_returns_state_ensured() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, Some(456u32))).unwrap();
    let item_ensure_boxed = Box::new(item_ensure) as Box<dyn ItemEnsureRt>;
    let state_ensured = ItemEnsureRt::state_ensured(&item_ensure_boxed).unwrap();

    assert_eq!(
        456u32,
        *BoxDataTypeDowncast::<u32>::downcast_ref(&state_ensured).unwrap()
    );
    Ok(())
}

#[test]
fn as_data_type_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_clone = item_ensure.clone();
    let item_ensure_clone_boxed = Box::new(item_ensure_clone) as Box<dyn ItemEnsureRt>;
    let data_type = ItemEnsureRt::as_data_type(&item_ensure_clone_boxed);

    assert_eq!(
        item_ensure,
        *data_type.downcast_ref::<ItemEnsure<u32, u8>>().unwrap()
    );
    Ok(())
}

#[test]
fn as_data_type_mut_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_clone = item_ensure.clone();
    let mut item_ensure_clone_boxed = Box::new(item_ensure_clone) as Box<dyn ItemEnsureRt>;
    let data_type = ItemEnsureRt::as_data_type_mut(&mut item_ensure_clone_boxed);

    assert_eq!(
        item_ensure,
        *data_type.downcast_mut::<ItemEnsure<u32, u8>>().unwrap()
    );
    Ok(())
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, Some(456u32))).unwrap();
    let item_ensure_boxed = Box::new(item_ensure) as Box<dyn ItemEnsureRt>;

    let serialized = serde_yaml::to_string(&item_ensure_boxed)?;
    assert_eq!(
        r#"state_saved: 456
state_current: 123
state_desired: 789
state_diff: 8
op_check_status: ExecNotRequired
state_ensured: 456
"#,
        serialized
    );
    Ok(())
}
