use std::ops::{Deref, DerefMut};

use peace::cfg::type_reg::untagged::{BoxDataTypeDowncast, DataTypeWrapper};

use super::ItemEnsureBoxed;

#[test]
fn clone() {
    let box_dt = ItemEnsureBoxed::new(1u32);
    let mut box_dt_clone = Clone::clone(&box_dt);

    *BoxDataTypeDowncast::<u32>::downcast_mut(&mut box_dt_clone).unwrap() = 2;

    assert_eq!(
        Some(1u32),
        BoxDataTypeDowncast::<u32>::downcast_ref(&box_dt).copied()
    );
    assert_eq!(
        Some(2u32),
        BoxDataTypeDowncast::<u32>::downcast_ref(&box_dt_clone).copied()
    );
}

#[cfg(not(feature = "debug"))]
#[test]
fn debug() {
    let box_dt = ItemEnsureBoxed::new(1u32);

    assert_eq!(r#"ItemEnsureBoxed("..")"#, format!("{box_dt:?}"));
}

#[cfg(feature = "debug")]
#[test]
fn debug() {
    let box_dt = ItemEnsureBoxed::new(1u32);

    assert_eq!("ItemEnsureBoxed(1)", format!("{box_dt:?}"));
}

#[test]
fn deref() {
    let box_dt = ItemEnsureBoxed::new(1u32);
    let _data_type = Deref::deref(&box_dt);
}

#[test]
fn deref_mut() {
    let mut box_dt = ItemEnsureBoxed::new(1u32);
    let _data_type = DerefMut::deref_mut(&mut box_dt);
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let box_dt = ItemEnsureBoxed::new(1u32);
    let data_type_wrapper: &dyn DataTypeWrapper = &box_dt;

    assert_eq!("1\n", serde_yaml::to_string(data_type_wrapper)?);
    Ok(())
}
