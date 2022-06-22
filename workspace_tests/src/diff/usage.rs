use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use peace::diff::Diff;

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct Unit;

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct StdImpls {
    pub items: HashMap<u16, Vec<u8>>,
    pub items_b: BTreeMap<u16, Vec<u8>>,
    pub tuple: (u32, u16),
    pub arc: Arc<String>,
    pub unit: Unit,
}

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub enum Enum {
    VarUnit,
    VarNamed { a: u32, b: u32 },
    VarUnnamed(u32, StdImpls),
}

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct NamedGenerics<A, B> {
    pub a: A,
    pub b: B,
}

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct UnnamedGenerics<A, B>(A, B);

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub enum EnumGenerics<A, B>
where
    A: PartialEq,
    B: PartialEq,
{
    Named { a: A, b: B },
    Unnamed(A, B),
    Unit,
}

pub trait Bound1 {}
pub trait Bound2 {}

impl Bound1 for u32 {}
impl Bound2 for u32 {}

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct NamedGenericsBoundsInternal<A: Bound1 + Bound2, B> {
    pub a: A,
    pub b: B,
}

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct NamedGenericsBoundsExternal<A, B>
where
    A: Bound1 + Bound2,
{
    pub a: A,
    pub b: B,
}

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct NamedGenericsBoundsMixed<A: Bound1, B>
where
    A: Bound2,
{
    pub a: A,
    pub b: B,
}

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct UnnamedGenericsBoundsInternal<A: Bound1 + Bound2, B>(A, B);

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct UnnamedGenericsBoundsExternal<A, B>(A, B)
where
    A: Bound1 + Bound2;

#[derive(Clone, Diff, Eq, PartialEq, Debug)]
pub struct UnnamedGenericsBoundsMixed<A: Bound1, B>(A, B)
where
    A: Bound2;

fn a_diff_b<T>(a: &mut T, b: &T) -> bool
where
    T: Diff + Eq,
{
    let diff = a.diff(&b);
    a.apply(&diff);
    a == b
}

#[test]
fn test_std_impls() {
    let mut a = {
        let mut items = HashMap::new();
        items.insert(1, vec![1, 2]);
        items.insert(2, vec![3, 4]);
        let mut items_b = BTreeMap::new();
        items_b.insert(2, vec![3, 4]);
        let tuple = (5u32, 6u16);
        let arc = Arc::new(String::from("hello"));
        let unit = Unit;

        StdImpls {
            items,
            items_b,
            tuple,
            arc,
            unit,
        }
    };

    let b = {
        let mut items = HashMap::new();
        items.insert(2, vec![1, 2]);
        let mut items_b = BTreeMap::new();
        items_b.insert(3, vec![3, 4]);
        let tuple = (7u32, 6u16);
        let arc = Arc::new(String::from("bye"));
        let unit = Unit;

        StdImpls {
            items,
            items_b,
            tuple,
            arc,
            unit,
        }
    };

    assert!(a_diff_b(&mut a, &b));
}

#[test]
fn test_enum() {
    let unit = Enum::VarUnit;
    let named_0 = Enum::VarNamed { a: 1u32, b: 2u32 };
    let named_1 = Enum::VarNamed { a: 3u32, b: 4u32 };
    let unnamed_0 = Enum::VarUnnamed(1u32, {
        let mut items = HashMap::new();
        items.insert(1, vec![1, 2]);
        items.insert(2, vec![3, 4]);
        let mut items_b = BTreeMap::new();
        items_b.insert(2, vec![3, 4]);
        let tuple = (5u32, 6u16);
        let arc = Arc::new(String::from("hello"));
        let unit = Unit;

        StdImpls {
            items,
            items_b,
            tuple,
            arc,
            unit,
        }
    });
    let unnamed_1 = Enum::VarUnnamed(3u32, {
        let mut items = HashMap::new();
        items.insert(2, vec![1, 2]);
        let mut items_b = BTreeMap::new();
        items_b.insert(3, vec![3, 4]);
        let tuple = (7u32, 6u16);
        let arc = Arc::new(String::from("bye"));
        let unit = Unit;

        StdImpls {
            items,
            items_b,
            tuple,
            arc,
            unit,
        }
    });

    [
        (&unit, &unit),
        (&unit, &named_0),
        (&unit, &unnamed_0),
        (&named_0, &unit),
        (&named_0, &named_0),
        (&named_0, &named_1),
        (&named_0, &unnamed_0),
        (&unnamed_0, &unit),
        (&unnamed_0, &named_0),
        (&unnamed_0, &unnamed_1),
    ]
    .into_iter()
    .for_each(|(a, b)| assert!(a_diff_b(&mut a.clone(), b)));
}

#[test]
fn test_named_generics() {
    let mut a = NamedGenerics { a: 1u32, b: 2u64 };
    let b = NamedGenerics { a: 3u32, b: 4u64 };

    assert!(a_diff_b(&mut a, &b));
}

#[test]
fn test_unnamed_generics() {
    let mut a = UnnamedGenerics(1u32, 2u64);
    let b = UnnamedGenerics(3u32, 4u64);

    assert!(a_diff_b(&mut a, &b));
}

#[test]
fn test_enum_generics() {
    let unit = EnumGenerics::Unit;
    let named_0 = EnumGenerics::Named { a: 1u32, b: 2u32 };
    let named_1 = EnumGenerics::Named { a: 3u32, b: 4u32 };
    let unnamed_0 = EnumGenerics::Unnamed(1u32, 2u32);
    let unnamed_1 = EnumGenerics::Unnamed(3u32, 3u32);

    [
        (&unit, &unit),
        (&unit, &named_0),
        (&unit, &unnamed_0),
        (&named_0, &unit),
        (&named_0, &named_0),
        (&named_0, &named_1),
        (&named_0, &unnamed_0),
        (&unnamed_0, &unit),
        (&unnamed_0, &named_0),
        (&unnamed_0, &unnamed_1),
    ]
    .into_iter()
    .for_each(|(a, b)| assert!(a_diff_b(&mut a.clone(), b)));
}
