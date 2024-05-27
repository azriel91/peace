use std::ops::{Deref, DerefMut};

use peace::{
    cfg::{state::External, ApplyCheck, State},
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemApplyPartial, ItemApplyPartialBoxed},
};
use pretty_assertions::assert_eq;

#[test]
fn clone() {
    let item_apply_partial_boxed = ItemApplyPartialBoxed::from(item_apply_partial());
    let mut item_apply_partial_boxed_clone = Clone::clone(&item_apply_partial_boxed);

    *BoxDataTypeDowncast::<ItemApplyPartial<State<u32, External<u32>>, u32>>::downcast_mut(
        &mut item_apply_partial_boxed_clone,
    )
    .unwrap() = item_apply_partial();

    assert_eq!(
        Some(item_apply_partial()),
        BoxDataTypeDowncast::<ItemApplyPartial<State<u32, External<u32>>, u32>>::downcast_ref(
            &item_apply_partial_boxed
        )
        .cloned()
    );
    assert_eq!(
        Some(item_apply_partial()),
        BoxDataTypeDowncast::<ItemApplyPartial<State<u32, External<u32>>, u32>>::downcast_ref(
            &item_apply_partial_boxed_clone
        )
        .cloned()
    );
}

#[test]
fn debug() {
    let item_apply_partial_boxed = ItemApplyPartialBoxed::from(item_apply_partial());

    assert_eq!(
        r#"ItemApplyPartialBoxed(
    ItemApplyPartial {
        state_current_stored: None,
        state_current: Some(
            State {
                logical: 1,
                physical: Value(
                    0,
                ),
            },
        ),
        state_target: Some(
            State {
                logical: 3,
                physical: Tbd,
            },
        ),
        state_diff: Some(
            2,
        ),
        apply_check: Some(
            ExecNotRequired,
        ),
    },
)"#,
        format!("{item_apply_partial_boxed:#?}")
    );
}

#[test]
fn deref() {
    let item_apply_partial_boxed = ItemApplyPartialBoxed::from(item_apply_partial());
    let _data_type = Deref::deref(&item_apply_partial_boxed);
}

#[test]
fn deref_mut() {
    let mut item_apply_partial_boxed = ItemApplyPartialBoxed::from(item_apply_partial());
    let _data_type = DerefMut::deref_mut(&mut item_apply_partial_boxed);
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let item_apply_partial_boxed = ItemApplyPartialBoxed::from(item_apply_partial());
    let data_type_wrapper = &item_apply_partial_boxed;

    assert_eq!(
        r#"state_current_stored: null
state_current:
  logical: 1
  physical: !Value 0
state_target:
  logical: 3
  physical: !Tbd null
state_diff: 2
apply_check: ExecNotRequired
"#,
        serde_yaml::to_string(data_type_wrapper)?
    );
    Ok(())
}

fn item_apply_partial() -> ItemApplyPartial<State<u32, External<u32>>, u32> {
    let mut item_apply_partial = ItemApplyPartial::new();
    item_apply_partial.state_current = Some(State::new(1, External::Value(0)));
    item_apply_partial.state_target = Some(State::new(3, External::Tbd));
    item_apply_partial.state_diff = Some(2);
    item_apply_partial.apply_check = Some(ApplyCheck::ExecNotRequired);
    item_apply_partial
}
