use peace::diff::{Changeable, Equality, Tracked};

#[test]
fn equality_returns_from_cmp_to() {
    let changeable = Changeable::new(Tracked::<u8>::Unknown, Tracked::<u8>::Known(1));

    assert_eq!(Equality::Unknown, changeable.equality());
}

// for coverage
#[test]
fn clone() {
    let changeable = Changeable::known(1, 1);

    assert_eq!(changeable, changeable.clone());
}

#[test]
fn debug() {
    let changeable = Changeable::new(Tracked::<u8>::Unknown, Tracked::<u8>::Known(1));

    assert_eq!(
        "Changeable { from: Unknown, to: Known(1) }",
        format!("{changeable:?}")
    );
}

#[test]
fn partial_eq() {
    let changeable_0 = Changeable::new(Tracked::<u8>::Unknown, Tracked::<u8>::Known(1));
    let changeable_1 = Changeable::known(1, 1);

    assert!(changeable_0 != changeable_0);
    assert!(changeable_0 != changeable_1);
    assert!(changeable_1 == changeable_1);
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let changeable = Changeable::new(Tracked::<u8>::Unknown, Tracked::<u8>::Known(1));

    assert_eq!(
        "from: Unknown\n\
        to: !Known 1\n\
        ",
        serde_yaml::to_string(&changeable)?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert!(matches!(
        serde_yaml::from_str(
            "from: Unknown\n\
            to: !Known 1\n\
            "
        )?,
        Changeable {
            from: Tracked::<u8>::Unknown,
            to: Tracked::<u8>::Known(1),
        }
    ));
    Ok(())
}
