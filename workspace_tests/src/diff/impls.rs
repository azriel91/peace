use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use peace::diff::{Diff, HashMapDiff, OptionDiff, VecDiff, VecDiffType};

fn identity_test<D: Diff + Debug + PartialEq>(s: D) {
    assert_eq!(D::identity().apply_new(&D::identity().diff(&s)), s);
}

fn generate_map<K: Eq + Hash, V>(parts: Vec<(K, V)>) -> HashMap<K, V> {
    parts.into_iter().collect::<HashMap<_, _>>()
}

#[test]
fn numeric_diffs() {
    identity_test(true);
    identity_test(42_u8);
    identity_test(42_i8);
    identity_test(42_u16);
    identity_test(42_i16);
    identity_test(42_u32);
    identity_test(42_i32);
    identity_test(42.0_f32);
    identity_test(42.0_f64);
}

#[test]
fn test_char_string() {
    identity_test('b');
    identity_test(String::from("42"));
    assert_eq!('b'.diff(&'c'), Some('c'));
    assert_eq!('b'.diff(&'b'), None);
    assert_eq!(
        String::from("42").diff(&String::from("asdf")),
        Some(String::from("asdf"))
    );
    assert_eq!(String::from("42").diff(&String::from("42")), None);
}

#[test]
fn test_opt() {
    assert_eq!(Some(10).diff(&Some(15)), OptionDiff::Some(5));
    assert_eq!(None.apply_new(&OptionDiff::Some(5)), Some(5));
    assert_eq!(Some(100).apply_new(&OptionDiff::None), None);
    identity_test(Some(42))
}

#[test]
fn test_maps() {
    let a = generate_map(vec![("a", 1), ("b", 2), ("x", 42)]);
    let b = generate_map(vec![("b", 3), ("c", 4), ("x", 42)]);
    let expected = HashMapDiff {
        altered: generate_map(vec![("b", 1), ("c", 4)]),
        removed: vec!["a"].into_iter().collect::<HashSet<_>>(),
    };
    assert_eq!(a.diff(&b), expected);
    identity_test(a);
}

#[derive(Debug, PartialEq, Diff)]
#[diff(attr(
    #[derive(Debug, PartialEq)]
))]
#[diff(name(TestDiff))]
struct TestStruct {
    a: bool,
    b: u32,
}

#[test]
fn test_derive() {
    let a = TestStruct { a: false, b: 42 };

    let b = TestStruct { a: true, b: 43 };

    let diff = TestDiff {
        a: true.into(),
        b: 1,
    };
    assert_eq!(a.diff(&b), diff);

    identity_test(a);
}

#[derive(Debug, PartialEq, Diff)]
#[diff(attr(
    #[derive(Debug, PartialEq)]
))]
struct TestTupleStruct(i32);

#[test]
fn test_tuple_derive() {
    let a = TestTupleStruct(10);
    let b = TestTupleStruct(30);
    let diff = TestTupleStructDiff(20);
    assert_eq!(a.diff(&b), diff);
}

#[derive(Debug, Default, PartialEq, Diff)]
#[diff(visibility(pub))]
struct ProjectMeta {
    contributors: Vec<String>,
    combined_work_hours: usize,
}

#[test]
fn test_apply() {
    let mut base = ProjectMeta::default();
    let contribution_a = ProjectMeta {
        contributors: vec!["Alice".into()],
        combined_work_hours: 3,
    };
    let contribution_b = ProjectMeta {
        contributors: vec!["Bob".into(), "Candice".into()],
        combined_work_hours: 10,
    };
    let expected = ProjectMeta {
        contributors: vec!["Bob".into(), "Candice".into(), "Alice".into()],
        combined_work_hours: 13,
    };
    let diff_a = base.diff(&contribution_a);
    let diff_b = base.diff(&contribution_b);
    base.apply(&diff_a);
    base.apply(&diff_b);
    assert_eq!(base, expected);
}

#[test]
fn test_vecs() {
    let a = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let b = vec![0, /* 1, 2 */ 3, 4, 42, 5, /* 6 -> */ 10, 7];
    let diff = VecDiff(vec![
        VecDiffType::Removed { index: 1, len: 2 },
        VecDiffType::Inserted {
            index: 5,
            changes: vec![42],
        },
        VecDiffType::Altered {
            index: 6,
            changes: vec![4], // add 4 to 6
        },
    ]);
    assert_eq!(diff, a.diff(&b));
    assert_eq!(a.apply_new(&diff), b);
}
use serde::Serialize;

#[derive(Default, PartialEq, Serialize, Diff)]
#[diff(name(SpecialName))]
#[diff(visibility(pub))]
#[diff(attr(
    #[derive(Default, PartialEq, Serialize)]
))]
/// A struct with a lot of attributes
struct MyTestStruct {
    #[diff(name(special_field_name))]
    #[diff(visibility(pub))]
    #[diff(attr(
        #[serde(rename = "name")]
    ))]
    /// This field has a lot of attributes too
    test_field: u32,
}

#[test]
fn test_full_struct() {
    let base = MyTestStruct::default();
    let other = MyTestStruct { test_field: 1 };

    let diff = base.diff(&other);
    assert_eq!(diff.special_field_name, 1);
}
