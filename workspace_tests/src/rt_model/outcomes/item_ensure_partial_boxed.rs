use std::ops::{Deref, DerefMut};

use peace::{
    cfg::{state::External, OpCheckStatus, State},
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemEnsurePartial, ItemEnsurePartialBoxed},
};
use pretty_assertions::assert_eq;

#[test]
fn clone() {
    let item_ensure_partial_boxed = ItemEnsurePartialBoxed::from(item_ensure_partial());
    let mut item_ensure_partial_boxed_clone = Clone::clone(&item_ensure_partial_boxed);

    *BoxDataTypeDowncast::<ItemEnsurePartial<u32, External<u32>, u32>>::downcast_mut(
        &mut item_ensure_partial_boxed_clone,
    )
    .unwrap() = item_ensure_partial();

    assert_eq!(
        Some(item_ensure_partial()),
        BoxDataTypeDowncast::<ItemEnsurePartial<u32, External<u32>, u32>>::downcast_ref(
            &item_ensure_partial_boxed
        )
        .cloned()
    );
    assert_eq!(
        Some(item_ensure_partial()),
        BoxDataTypeDowncast::<ItemEnsurePartial<u32, External<u32>, u32>>::downcast_ref(
            &item_ensure_partial_boxed_clone
        )
        .cloned()
    );
}

#[test]
fn debug() {
    let item_ensure_partial_boxed = ItemEnsurePartialBoxed::from(item_ensure_partial());

    assert_eq!(
        r#"ItemEnsurePartialBoxed(
    ItemEnsurePartial {
        state_saved: None,
        state_current: Some(
            State {
                logical: 1,
                physical: Value(
                    0,
                ),
            },
        ),
        state_desired: Some(
            State {
                logical: 3,
                physical: Tbd(
                    (),
                ),
            },
        ),
        state_diff: Some(
            2,
        ),
        op_check_status: Some(
            ExecNotRequired,
        ),
    },
)"#,
        format!("{item_ensure_partial_boxed:#?}")
    );
}

#[test]
fn deref() {
    let item_ensure_partial_boxed = ItemEnsurePartialBoxed::from(item_ensure_partial());
    let _data_type = Deref::deref(&item_ensure_partial_boxed);
}

#[test]
fn deref_mut() {
    let mut item_ensure_partial_boxed = ItemEnsurePartialBoxed::from(item_ensure_partial());
    let _data_type = DerefMut::deref_mut(&mut item_ensure_partial_boxed);
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let item_ensure_partial_boxed = ItemEnsurePartialBoxed::from(item_ensure_partial());
    let data_type_wrapper = &item_ensure_partial_boxed;

    assert_eq!(
        r#"state_saved: null
state_current:
  logical: 1
  physical: !Value 0
state_desired:
  logical: 3
  physical: !Tbd null
state_diff: 2
op_check_status: ExecNotRequired
"#,
        serde_yaml::to_string(data_type_wrapper)?
    );
    Ok(())
}

fn item_ensure_partial() -> ItemEnsurePartial<u32, External<u32>, u32> {
    let mut item_ensure_partial = ItemEnsurePartial::new();
    item_ensure_partial.state_current = Some(State::new(1, External::Value(0)));
    item_ensure_partial.state_desired = Some(State::new(3, External::tbd()));
    item_ensure_partial.state_diff = Some(2);
    item_ensure_partial.op_check_status = Some(OpCheckStatus::ExecNotRequired);
    item_ensure_partial
}
