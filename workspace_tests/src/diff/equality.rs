use std::cmp::Ordering;

use peace::diff::Equality;

#[test]
fn display() {
    assert_eq!("!=", format!("{}", Equality::NotEqual));
    assert_eq!("==", format!("{}", Equality::Equal));
    assert_eq!("?=", format!("{}", Equality::Unknown));
}

#[test]
fn partial_ord() {
    assert_eq!(
        Some(Ordering::Equal),
        Equality::NotEqual.partial_cmp(&Equality::NotEqual)
    );
    assert_eq!(
        Some(Ordering::Less),
        Equality::NotEqual.partial_cmp(&Equality::Equal)
    );
    assert_eq!(
        Some(Ordering::Less),
        Equality::NotEqual.partial_cmp(&Equality::Unknown)
    );

    assert_eq!(
        Some(Ordering::Greater),
        Equality::Equal.partial_cmp(&Equality::NotEqual)
    );
    assert_eq!(
        Some(Ordering::Equal),
        Equality::Equal.partial_cmp(&Equality::Equal)
    );
    assert_eq!(
        Some(Ordering::Less),
        Equality::Equal.partial_cmp(&Equality::Unknown)
    );

    assert_eq!(
        Some(Ordering::Greater),
        Equality::Unknown.partial_cmp(&Equality::NotEqual)
    );
    assert_eq!(
        Some(Ordering::Greater),
        Equality::Unknown.partial_cmp(&Equality::Equal)
    );
    assert_eq!(None, Equality::Unknown.partial_cmp(&Equality::Unknown));
}

#[test]
fn bool_from() {
    assert!(!bool::from(Equality::NotEqual));
    assert!(bool::from(Equality::Equal));
    assert!(!bool::from(Equality::Unknown));
}

#[test]
fn from_bool() {
    assert_eq!(Equality::Equal, Equality::from(true));
    assert_eq!(Equality::NotEqual, Equality::from(false));
}
