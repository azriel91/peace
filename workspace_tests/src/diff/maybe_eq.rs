use std::collections::{HashMap, HashSet};

use peace::diff::{Equality, MaybeEq, Tracked};

#[test]
fn vec_eq() {
    assert_eq!(Equality::Equal, vec![1, 2, 3].maybe_eq(&vec![1, 2, 3]));
    assert_eq!(
        Equality::Equal,
        vec![Tracked::Known(Value(1))].maybe_eq(&vec![Tracked::Known(Value(1))])
    );
}

#[test]
fn vec_not_eq() {
    assert_eq!(Equality::NotEqual, vec![1, 2].maybe_eq(&vec![1, 2, 3]));

    assert_eq!(
        Equality::NotEqual,
        vec![Tracked::Known(Value(1)), Tracked::Known(Value(2))]
            .maybe_eq(&vec![Tracked::Known(Value(1))])
    );

    // Note: Currently we do not do element-wise comparisons.
    assert_eq!(
        Equality::NotEqual,
        vec![Tracked::<u8>::Unknown].maybe_eq(&vec![Tracked::<u8>::Unknown])
    );
}

#[test]
fn set_eq() {
    let mut set_0 = HashSet::new();
    set_0.insert(1);
    let mut set_1 = HashSet::new();
    set_1.insert(1);

    assert_eq!(Equality::Equal, set_0.maybe_eq(&set_1));

    let mut set_0 = HashSet::new();
    set_0.insert(Tracked::Known(Value(1)));
    let mut set_1 = HashSet::new();
    set_1.insert(Tracked::Known(Value(1)));
    assert_eq!(Equality::Equal, set_0.maybe_eq(&set_1));
}

#[test]
fn set_not_eq() {
    let mut set_0 = HashSet::new();
    set_0.insert(1);
    let mut set_1 = HashSet::new();
    set_1.insert(2);

    assert_eq!(Equality::NotEqual, set_0.maybe_eq(&set_1));

    let mut set_0 = HashSet::new();
    set_0.insert(Tracked::Known(Value(1)));
    let mut set_1 = HashSet::new();
    set_1.insert(Tracked::Known(Value(2)));
    assert_eq!(Equality::NotEqual, set_0.maybe_eq(&set_1));

    // Note: Currently we do not do element-wise comparisons.
    let mut set_0 = HashSet::new();
    set_0.insert(Tracked::<u8>::Unknown);
    let mut set_1 = HashSet::new();
    set_1.insert(Tracked::<u8>::Unknown);
    assert_eq!(Equality::NotEqual, set_0.maybe_eq(&set_1));
}

#[test]
fn map_eq() {
    let mut map_0 = HashMap::new();
    map_0.insert(1, 1);
    let mut map_1 = HashMap::new();
    map_1.insert(1, 1);

    assert_eq!(Equality::Equal, map_0.maybe_eq(&map_1));

    let mut map_0 = HashMap::new();
    map_0.insert(1, Tracked::Known(Value(1)));
    let mut map_1 = HashMap::new();
    map_1.insert(1, Tracked::Known(Value(1)));
    assert_eq!(Equality::Equal, map_0.maybe_eq(&map_1));
}

#[test]
fn map_not_eq() {
    let mut map_0 = HashMap::new();
    map_0.insert(0, 1);
    map_0.insert(1, 1);
    map_0.insert(2, 1);
    let mut map_1 = HashMap::new();
    map_1.insert(0, 1);
    map_1.insert(1, 2);
    map_1.insert(2, 1);
    assert_eq!(Equality::NotEqual, map_0.maybe_eq(&map_1));

    // different length
    let mut map_0 = HashMap::new();
    map_0.insert(0, 1);
    map_0.insert(1, 1);
    let mut map_1 = HashMap::new();
    map_1.insert(0, 1);
    assert_eq!(Equality::NotEqual, map_0.maybe_eq(&map_1));

    let mut map_0 = HashMap::new();
    map_0.insert(0, Tracked::Known(Value(1)));
    map_0.insert(1, Tracked::Known(Value(1)));
    map_0.insert(2, Tracked::Known(Value(1)));
    let mut map_1 = HashMap::new();
    map_1.insert(0, Tracked::Known(Value(1)));
    map_1.insert(1, Tracked::Known(Value(2)));
    map_1.insert(2, Tracked::Known(Value(1)));
    assert_eq!(Equality::NotEqual, map_0.maybe_eq(&map_1));
}

#[test]
fn map_unknown() {
    let mut map_0 = HashMap::new();
    map_0.insert(0, Tracked::<u8>::Known(1));
    map_0.insert(1, Tracked::<u8>::Unknown);
    map_0.insert(2, Tracked::<u8>::Known(1));
    let mut map_1 = HashMap::new();
    map_1.insert(0, Tracked::<u8>::Known(1));
    map_1.insert(1, Tracked::<u8>::Unknown);
    map_1.insert(2, Tracked::<u8>::Known(1));
    assert_eq!(Equality::Unknown, map_0.maybe_eq(&map_1));
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Value(u8);

impl MaybeEq for Value {
    fn maybe_eq(&self, other: &Self) -> Equality {
        Equality::from(self == other)
    }
}
