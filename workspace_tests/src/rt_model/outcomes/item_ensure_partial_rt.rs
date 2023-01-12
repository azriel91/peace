use peace::{
    cfg::{
        state::{Nothing, Placeholder},
        OpCheckStatus, State,
    },
    rt_model::outcomes::{ItemEnsurePartial, ItemEnsurePartialRt},
};

#[test]
fn item_ensure_rt_state_saved_returns_state_saved() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };
    let item_ensure_partial_boxed = Box::new(item_ensure_partial) as Box<dyn ItemEnsurePartialRt>;

    let state_saved = ItemEnsurePartialRt::state_saved(&item_ensure_partial_boxed).unwrap();

    assert_eq!(
        State::new(Nothing, 456u32),
        *state_saved.downcast::<State<Nothing, u32>>().unwrap()
    );
    Ok(())
}

#[test]
fn item_ensure_rt_state_current_returns_state_current() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };
    let item_ensure_partial_boxed = Box::new(item_ensure_partial) as Box<dyn ItemEnsurePartialRt>;

    let state_current = ItemEnsurePartialRt::state_current(&item_ensure_partial_boxed).unwrap();

    assert_eq!(
        State::new(Nothing, 123u32),
        *state_current.downcast::<State<Nothing, u32>>().unwrap()
    );
    Ok(())
}

#[test]
fn item_ensure_rt_state_desired_returns_state_desired() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };
    let item_ensure_partial_boxed = Box::new(item_ensure_partial) as Box<dyn ItemEnsurePartialRt>;

    let state_desired = ItemEnsurePartialRt::state_desired(&item_ensure_partial_boxed).unwrap();

    assert_eq!(
        State::new(Nothing, Placeholder::tbd()),
        *state_desired
            .downcast::<State<Nothing, Placeholder>>()
            .unwrap()
    );
    Ok(())
}

#[test]
fn item_ensure_rt_state_diff_returns_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };
    let item_ensure_partial_boxed = Box::new(item_ensure_partial) as Box<dyn ItemEnsurePartialRt>;

    let state_diff = ItemEnsurePartialRt::state_diff(&item_ensure_partial_boxed).unwrap();

    assert_eq!(8u8, *state_diff.downcast::<u8>().unwrap());
    Ok(())
}

#[test]
fn item_ensure_rt_op_check_status_returns_op_check_status() -> Result<(), Box<dyn std::error::Error>>
{
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };
    let item_ensure_partial_boxed = Box::new(item_ensure_partial) as Box<dyn ItemEnsurePartialRt>;

    let op_check_status = ItemEnsurePartialRt::op_check_status(&item_ensure_partial_boxed).unwrap();

    assert_eq!(OpCheckStatus::ExecNotRequired, op_check_status);
    Ok(())
}

#[test]
fn item_ensure_rt_as_data_type_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure_partial_clone = item_ensure_partial.clone();
    let item_ensure_partial_clone_boxed =
        Box::new(item_ensure_partial_clone) as Box<dyn ItemEnsurePartialRt>;
    let data_type = ItemEnsurePartialRt::as_data_type(&item_ensure_partial_clone_boxed);

    assert_eq!(
        item_ensure_partial,
        *data_type
            .downcast_ref::<ItemEnsurePartial<Nothing, u32, u8>>()
            .unwrap()
    );
    Ok(())
}

#[test]
fn item_ensure_rt_as_data_type_mut_returns_self() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };

    let item_ensure_partial_clone = item_ensure_partial.clone();
    let mut item_ensure_partial_clone_boxed =
        Box::new(item_ensure_partial_clone) as Box<dyn ItemEnsurePartialRt>;
    let data_type = ItemEnsurePartialRt::as_data_type_mut(&mut item_ensure_partial_clone_boxed);

    assert_eq!(
        item_ensure_partial,
        *data_type
            .downcast_mut::<ItemEnsurePartial<Nothing, u32, u8>>()
            .unwrap()
    );
    Ok(())
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    let item_ensure_partial = ItemEnsurePartial {
        state_saved: Some(State::new(Nothing, 456u32)),
        state_current: Some(State::new(Nothing, 123u32)),
        state_desired: Some(State::new(Nothing, Placeholder::tbd())),
        state_diff: Some(8u8),
        op_check_status: Some(OpCheckStatus::ExecNotRequired),
    };
    let item_ensure_partial_boxed = Box::new(item_ensure_partial) as Box<dyn ItemEnsurePartialRt>;

    let serialized = serde_yaml::to_string(&item_ensure_partial_boxed)?;
    assert_eq!(
        r#"state_saved:
  logical: null
  physical: 456
state_current:
  logical: null
  physical: 123
state_desired:
  logical: null
  physical: !Tbd null
state_diff: 8
op_check_status: ExecNotRequired
"#,
        serialized
    );
    Ok(())
}
