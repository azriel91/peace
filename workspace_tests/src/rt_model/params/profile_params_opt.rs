use peace::{cmd_ctx::type_reg::untagged::TypeMapOpt, rt_model::params::ProfileParamsOpt};

#[test]
fn new() {
    let profile_params_opt = ProfileParamsOpt::<()>::new();

    assert_eq!(0, profile_params_opt.len());
    assert_eq!(0, profile_params_opt.capacity());
}

#[test]
fn with_capacity() {
    let profile_params_opt = ProfileParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, profile_params_opt.len());
    assert_eq!(10, profile_params_opt.capacity());
}

#[test]
fn into_inner() {
    let profile_params_opt = ProfileParamsOpt::<()>::with_capacity(10);
    let type_map_opt = profile_params_opt.into_inner();

    assert_eq!(0, type_map_opt.len());
    assert_eq!(10, type_map_opt.capacity());
}

#[test]
fn deref() {
    let profile_params_opt = ProfileParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, (*profile_params_opt).len());
}

#[test]
fn deref_mut() {
    let mut profile_params_opt = ProfileParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, (*profile_params_opt).as_mut_slice().len());
}

#[test]
fn from_type_map_opt() {
    let profile_params_opt = ProfileParamsOpt::from(TypeMapOpt::<()>::new());

    assert_eq!(0, profile_params_opt.len());
}
