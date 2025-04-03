use peace::{cmd_ctx::type_reg::untagged::TypeMap, rt_model::params::ProfileParams};

#[test]
fn new() {
    let profile_params = ProfileParams::<()>::new();

    assert_eq!(0, profile_params.len());
    assert_eq!(0, profile_params.capacity());
}

#[test]
fn with_capacity() {
    let profile_params = ProfileParams::<()>::with_capacity(10);

    assert_eq!(0, profile_params.len());
    assert_eq!(10, profile_params.capacity());
}

#[test]
fn into_inner() {
    let profile_params = ProfileParams::<()>::with_capacity(10);
    let type_map = profile_params.into_inner();

    assert_eq!(0, type_map.len());
    assert_eq!(10, type_map.capacity());
}

#[test]
fn deref() {
    let profile_params = ProfileParams::<()>::with_capacity(10);

    assert_eq!(0, (*profile_params).len());
}

#[test]
fn deref_mut() {
    let mut profile_params = ProfileParams::<()>::with_capacity(10);

    assert_eq!(0, (*profile_params).as_mut_slice().len());
}

#[test]
fn from_type_map() {
    let profile_params = ProfileParams::from(TypeMap::<()>::new());

    assert_eq!(0, profile_params.len());
}
