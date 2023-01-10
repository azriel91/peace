use std::ops::{Deref, DerefMut};

use peace::{
    cfg::{state::Placeholder, OpCheckStatus, State},
    resources::type_reg::untagged::BoxDataTypeDowncast,
    rt_model::outcomes::{ItemEnsure, ItemEnsureBoxed, ItemEnsurePartial},
};
use pretty_assertions::assert_eq;

#[test]
fn clone() {
    let item_ensure_boxed = ItemEnsureBoxed::from(item_ensure());
    let mut item_ensure_boxed_clone = Clone::clone(&item_ensure_boxed);

    *BoxDataTypeDowncast::<ItemEnsure<u32, u32, u32>>::downcast_mut(&mut item_ensure_boxed_clone)
        .unwrap() = item_ensure();

    assert_eq!(
        Some(item_ensure()),
        BoxDataTypeDowncast::<ItemEnsure<u32, u32, u32>>::downcast_ref(&item_ensure_boxed).cloned()
    );
    assert_eq!(
        Some(item_ensure()),
        BoxDataTypeDowncast::<ItemEnsure<u32, u32, u32>>::downcast_ref(&item_ensure_boxed_clone)
            .cloned()
    );
}

#[test]
fn debug() {
    let item_ensure_boxed = ItemEnsureBoxed::from(item_ensure());

    assert_eq!(
        r#"ItemEnsureBoxed(
    ItemEnsure {
        state_saved: None,
        state_current: State {
            logical: 1,
            physical: 0,
        },
        state_desired: State {
            logical: 3,
            physical: Calculated(
                PhantomData<()>,
            ),
        },
        state_diff: 2,
        op_check_status: ExecNotRequired,
        state_ensured: None,
    },
)"#,
        format!("{item_ensure_boxed:#?}")
    );
}

#[test]
fn deref() {
    let item_ensure_boxed = ItemEnsureBoxed::from(item_ensure());
    let _data_type = Deref::deref(&item_ensure_boxed);
}

#[test]
fn deref_mut() {
    let mut item_ensure_boxed = ItemEnsureBoxed::from(item_ensure());
    let _data_type = DerefMut::deref_mut(&mut item_ensure_boxed);
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let item_ensure_boxed = ItemEnsureBoxed::from(item_ensure());
    let data_type_wrapper = &item_ensure_boxed;

    assert_eq!(
        r#"state_saved: null
state_current:
  logical: 1
  physical: 0
state_desired:
  logical: 3
  physical: !Calculated null
state_diff: 2
op_check_status: ExecNotRequired
state_ensured: null
"#,
        serde_yaml::to_string(data_type_wrapper)?
    );
    Ok(())
}

fn item_ensure() -> ItemEnsure<u32, u32, u32> {
    let mut item_ensure_partial = ItemEnsurePartial::new();
    item_ensure_partial.state_current = Some(State::new(1, 0));
    item_ensure_partial.state_desired = Some(State::new(3, Placeholder::calculated()));
    item_ensure_partial.state_diff = Some(2);
    item_ensure_partial.op_check_status = Some(OpCheckStatus::ExecNotRequired);
    ItemEnsure::try_from((item_ensure_partial, None)).unwrap()
}
