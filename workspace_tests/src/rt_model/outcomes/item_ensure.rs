use peace::{
    cfg::OpCheckStatus,
    rt_model::outcomes::{ItemEnsure, ItemEnsurePartial, ItemEnsureRt},
};

#[test]
fn try_from_returns_ok_when_state_saved_is_none_and_others_are_some()
-> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: None,
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();

    assert_eq!(
        ItemEnsure {
            state_saved: None,
            state_current: 123u32,
            state_desired: 789u32,
            state_diff: 8u8,
            op_check_status: OpCheckStatus::ExecNotRequired,
            state_ensured: None,
        },
        item_ensure
    );
    Ok(())
}

#[test]
fn try_from_returns_ok_when_all_fields_are_some() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();

    assert_eq!(
        ItemEnsure {
            state_saved: Some(456u32),
            state_current: 123u32,
            state_desired: 789u32,
            state_diff: 8u8,
            op_check_status: OpCheckStatus::ExecNotRequired,
            state_ensured: None,
        },
        item_ensure
    );
    Ok(())
}

#[test]
fn try_from_passes_through_state_ensured() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, Some(789))).unwrap();

    assert_eq!(
        ItemEnsure {
            state_saved: Some(456u32),
            state_current: 123u32,
            state_desired: 789u32,
            state_diff: 8u8,
            op_check_status: OpCheckStatus::ExecNotRequired,
            state_ensured: Some(789),
        },
        item_ensure
    );
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_current_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: None,
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let (item_ensure_partial, state_ensured) =
        ItemEnsure::try_from((item_ensure_partial, None)).unwrap_err();

    assert_eq!(
        ItemEnsurePartial {
            state_saved: Some(456u32),
            state_current: None,
            state_desired: Some(789u32),
            state_diff: Some(8u8),
            op_check_status: Some(OpCheckStatus::ExecNotRequired),
        },
        item_ensure_partial
    );
    assert!(state_ensured.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_desired_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: None,
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let (item_ensure_partial, state_ensured) =
        ItemEnsure::try_from((item_ensure_partial, None)).unwrap_err();

    assert_eq!(
        ItemEnsurePartial {
            state_saved: Some(456u32),
            state_current: Some(123u32),
            state_desired: None,
            state_diff: Some(8u8),
            op_check_status: Some(OpCheckStatus::ExecNotRequired),
        },
        item_ensure_partial
    );
    assert!(state_ensured.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_state_diff_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: None::<u8>,
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let (item_ensure_partial, state_ensured) =
        ItemEnsure::try_from((item_ensure_partial, None)).unwrap_err();

    assert_eq!(
        ItemEnsurePartial {
            state_saved: Some(456u32),
            state_current: Some(123u32),
            state_desired: Some(789u32),
            state_diff: None,
            op_check_status: Some(OpCheckStatus::ExecNotRequired),
        },
        item_ensure_partial
    );
    assert!(state_ensured.is_none());
    Ok(())
}

#[test]
fn try_from_returns_err_when_op_check_status_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: None,
    };

    let (item_ensure_partial, state_ensured) =
        ItemEnsure::try_from((item_ensure_partial, None)).unwrap_err();

    assert_eq!(
        ItemEnsurePartial {
            state_saved: Some(456u32),
            state_current: Some(123u32),
            state_desired: Some(789u32),
            state_diff: Some(8u8),
            op_check_status: None,
        },
        item_ensure_partial
    );
    assert!(state_ensured.is_none());
    Ok(())
}

#[test]
fn item_ensure_rt_state_saved_returns_state_saved() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let state_saved = ItemEnsureRt::state_saved(&item_ensure).unwrap();

    assert_eq!(456u32, *state_saved.downcast::<u32>().unwrap());
    Ok(())
}

#[test]
fn item_ensure_rt_state_current_returns_state_current() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let state_current = ItemEnsureRt::state_current(&item_ensure);

    assert_eq!(123u32, *state_current.downcast::<u32>().unwrap());
    Ok(())
}

#[test]
fn item_ensure_rt_state_desired_returns_state_desired() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let state_desired = ItemEnsureRt::state_desired(&item_ensure);

    assert_eq!(789u32, *state_desired.downcast::<u32>().unwrap());
    Ok(())
}

#[test]
fn item_ensure_rt_state_diff_returns_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let state_diff = ItemEnsureRt::state_diff(&item_ensure);

    assert_eq!(8u8, *state_diff.downcast::<u8>().unwrap());
    Ok(())
}

#[test]
fn item_ensure_rt_op_check_status_returns_op_check_status() -> Result<(), Box<dyn std::error::Error>>
{
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let op_check_status = ItemEnsureRt::op_check_status(&item_ensure);

    assert_eq!(OpCheckStatus::ExecNotRequired, op_check_status);
    Ok(())
}

#[test]
fn item_ensure_rt_state_ensured_returns_state_ensured() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, Some(456u32))).unwrap();
    let state_ensured = ItemEnsureRt::state_ensured(&item_ensure).unwrap();

    assert_eq!(456u32, *state_ensured.downcast::<u32>().unwrap());
    Ok(())
}

#[test]
fn item_ensure_rt_as_data_type_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let item_ensure_clone = item_ensure.clone();
    let data_type = ItemEnsureRt::as_data_type(&item_ensure_clone);

    assert_eq!(
        item_ensure,
        *data_type.downcast_ref::<ItemEnsure<u32, u8>>().unwrap()
    );
    Ok(())
}

#[test]
fn item_ensure_rt_as_data_type_mut_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(456u32),
        state_current: Some(123u32),
        state_desired: Some(789u32),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure = ItemEnsure::try_from((item_ensure_partial, None)).unwrap();
    let mut item_ensure_clone = item_ensure.clone();
    let data_type = ItemEnsureRt::as_data_type_mut(&mut item_ensure_clone);

    assert_eq!(
        item_ensure,
        *data_type.downcast_mut::<ItemEnsure<u32, u8>>().unwrap()
    );
    Ok(())
}
