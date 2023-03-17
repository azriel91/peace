use std::ops::{Deref, DerefMut};

use peace::{
    cfg::{state::External, OpCheckStatus, State},
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemApply, ItemApplyBoxed, ItemApplyPartial},
};
use pretty_assertions::assert_eq;

#[test]
fn clone() {
    let item_apply_boxed = ItemApplyBoxed::from(item_apply());
    let mut item_apply_boxed_clone = Clone::clone(&item_apply_boxed);

    *BoxDataTypeDowncast::<ItemApply<State<u32, External<u32>>, u32>>::downcast_mut(
        &mut item_apply_boxed_clone,
    )
    .unwrap() = item_apply();

    assert_eq!(
        Some(item_apply()),
        BoxDataTypeDowncast::<ItemApply<State<u32, External<u32>>, u32>>::downcast_ref(
            &item_apply_boxed
        )
        .cloned()
    );
    assert_eq!(
        Some(item_apply()),
        BoxDataTypeDowncast::<ItemApply<State<u32, External<u32>>, u32>>::downcast_ref(
            &item_apply_boxed_clone
        )
        .cloned()
    );
}

#[test]
fn debug() {
    let item_apply_boxed = ItemApplyBoxed::from(item_apply());

    assert_eq!(
        r#"ItemApplyBoxed(
    ItemApply {
        state_saved: None,
        state_current: State {
            logical: 1,
            physical: Value(
                0,
            ),
        },
        state_desired: State {
            logical: 3,
            physical: Tbd,
        },
        state_diff: 2,
        op_check_status: ExecNotRequired,
        state_applied: None,
    },
)"#,
        format!("{item_apply_boxed:#?}")
    );
}

#[test]
fn deref() {
    let item_apply_boxed = ItemApplyBoxed::from(item_apply());
    let _data_type = Deref::deref(&item_apply_boxed);
}

#[test]
fn deref_mut() {
    let mut item_apply_boxed = ItemApplyBoxed::from(item_apply());
    let _data_type = DerefMut::deref_mut(&mut item_apply_boxed);
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let item_apply_boxed = ItemApplyBoxed::from(item_apply());
    let data_type_wrapper = &item_apply_boxed;

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
state_applied: null
"#,
        serde_yaml::to_string(data_type_wrapper)?
    );
    Ok(())
}

fn item_apply() -> ItemApply<State<u32, External<u32>>, u32> {
    let mut item_apply_partial = ItemApplyPartial::new();
    item_apply_partial.state_current = Some(State::new(1, External::Value(0)));
    item_apply_partial.state_desired = Some(State::new(3, External::Tbd));
    item_apply_partial.state_diff = Some(2);
    item_apply_partial.op_check_status = Some(OpCheckStatus::ExecNotRequired);
    ItemApply::try_from((item_apply_partial, None)).unwrap()
}
