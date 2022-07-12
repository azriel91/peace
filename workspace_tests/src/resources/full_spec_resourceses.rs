use peace::{
    data::Resources,
    resources::{rt_vec::RtVec, FullSpecResourceses, FullSpecRtId},
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let resourceses = FullSpecResourceses::with_capacity(100);
    assert!(resourceses.capacity() >= 100);
}

#[test]
fn into_inner() {
    let resourceses = test_resourceses();

    let rt_vec = resourceses.into_inner();

    assert_eq!(2, rt_vec.len())
}

#[test]
fn deref_and_deref_mut() {
    let mut resourceses = FullSpecResourceses::new();
    let resources_0 = Resources::new();
    let mut resources_1 = Resources::new();
    resources_1.insert(1u32);

    // deref_mut
    resourceses.push(resources_0);
    resourceses.push(resources_1);

    // deref
    assert_eq!(2, resourceses.len())
}

#[test]
fn try_borrow() {
    let resourceses = test_resourceses();

    let rt_id_0 = FullSpecRtId::from(0);

    let borrow_0 = resourceses.try_borrow(rt_id_0);
    let borrow_1 = resourceses.try_borrow(rt_id_0);
    let borrow_2 = resourceses.try_borrow_mut(rt_id_0);

    assert!(borrow_0.is_ok());
    assert!(borrow_1.is_ok());
    assert!(borrow_2.is_err());
}

#[test]
fn borrow() {
    let resourceses = test_resourceses();

    let rt_id_0 = FullSpecRtId::from(0);
    let rt_id_1 = FullSpecRtId::from(1);

    assert_eq!(0, resourceses.borrow(rt_id_0).len());
    assert_eq!(1, resourceses.borrow(rt_id_1).len());
}

#[test]
fn try_borrow_mut() {
    let resourceses = test_resourceses();

    let rt_id_0 = FullSpecRtId::from(0);

    let borrow_0 = resourceses.try_borrow_mut(rt_id_0);
    let borrow_1 = resourceses.try_borrow(rt_id_0);
    let borrow_2 = resourceses.try_borrow_mut(rt_id_0);

    assert!(borrow_0.is_ok());
    assert!(borrow_1.is_err());
    assert!(borrow_2.is_err());
}

#[test]
fn borrow_mut() {
    let resourceses = test_resourceses();

    let mut resources = resourceses.borrow_mut(FullSpecRtId::from(0));
    resources.insert(1u32);
    resources.insert(2u64);
    resources.insert(true);
    drop(resources);

    assert_eq!(3, resourceses.borrow(FullSpecRtId::from(0)).len());
}

#[test]
fn get() {
    let resourceses = test_resourceses();

    let rt_id_0 = FullSpecRtId::from(0);
    let rt_id_1 = FullSpecRtId::from(1);

    assert_eq!(
        Some(0),
        resourceses
            .get(rt_id_0)
            .as_deref()
            .map(|resources| resources.len())
    );
    assert_eq!(
        Some(1),
        resourceses
            .get(rt_id_1)
            .as_deref()
            .map(|resources| resources.len())
    );
}

#[test]
fn get_mut() {
    let mut resourceses = test_resourceses();

    if let Some(resources) = resourceses.get_mut(FullSpecRtId::from(0)) {
        resources.insert(1u32);
        resources.insert(2u64);
        resources.insert(true);
    }

    assert_eq!(3, resourceses.borrow(FullSpecRtId::from(0)).len());
}

#[test]
fn from_rt_vec() {
    let mut resourceses = FullSpecResourceses::from(RtVec::new());

    let resources_0 = Resources::new();
    let mut resources_1 = Resources::new();
    resources_1.insert(1u32);

    resourceses.push(resources_0);
    resourceses.push(resources_1);

    assert_eq!(2, resourceses.len())
}

#[test]
fn debug() {
    let resourceses = test_resourceses();

    // TODO: in `rt_ref`, manually impl `Debug` for `Cell`, so that we don't just
    // get the non-exhaustive `".."` from `std::cell::UnsafeCell`
    assert_eq!(
        r#"FullSpecResourceses(RtVec([Cell { flag: 0, inner: UnsafeCell { .. } }, Cell { flag: 0, inner: UnsafeCell { .. } }]))"#,
        format!("{resourceses:?}")
    )
}

fn test_resourceses() -> FullSpecResourceses {
    let mut resourceses = FullSpecResourceses::new();
    let resources_0 = Resources::new();
    let mut resources_1 = Resources::new();
    resources_1.insert(1u32);

    resourceses.push(resources_0);
    resourceses.push(resources_1);

    resourceses
}
