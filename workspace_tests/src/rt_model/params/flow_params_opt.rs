use peace::{cmd_ctx::type_reg::untagged::TypeMapOpt, rt_model::params::FlowParamsOpt};

#[test]
fn new() {
    let flow_params_opt = FlowParamsOpt::<()>::new();

    assert_eq!(0, flow_params_opt.len());
    assert_eq!(0, flow_params_opt.capacity());
}

#[test]
fn with_capacity() {
    let flow_params_opt = FlowParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, flow_params_opt.len());
    assert_eq!(10, flow_params_opt.capacity());
}

#[test]
fn into_inner() {
    let flow_params_opt = FlowParamsOpt::<()>::with_capacity(10);
    let type_map_opt = flow_params_opt.into_inner();

    assert_eq!(0, type_map_opt.len());
    assert_eq!(10, type_map_opt.capacity());
}

#[test]
fn deref() {
    let flow_params_opt = FlowParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, (*flow_params_opt).len());
}

#[test]
fn deref_mut() {
    let mut flow_params_opt = FlowParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, (*flow_params_opt).as_mut_slice().len());
}

#[test]
fn from_type_map_opt() {
    let flow_params_opt = FlowParamsOpt::from(TypeMapOpt::<()>::new());

    assert_eq!(0, flow_params_opt.len());
}
