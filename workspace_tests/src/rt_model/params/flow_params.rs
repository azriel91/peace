use peace::{cmd_ctx::type_reg::untagged::TypeMap, rt_model::params::FlowParams};

#[test]
fn new() {
    let flow_params = FlowParams::<()>::new();

    assert_eq!(0, flow_params.len());
    assert_eq!(0, flow_params.capacity());
}

#[test]
fn with_capacity() {
    let flow_params = FlowParams::<()>::with_capacity(10);

    assert_eq!(0, flow_params.len());
    assert_eq!(10, flow_params.capacity());
}

#[test]
fn into_inner() {
    let flow_params = FlowParams::<()>::with_capacity(10);
    let type_map = flow_params.into_inner();

    assert_eq!(0, type_map.len());
    assert_eq!(10, type_map.capacity());
}

#[test]
fn deref() {
    let flow_params = FlowParams::<()>::with_capacity(10);

    assert_eq!(0, (*flow_params).len());
}

#[test]
fn deref_mut() {
    let mut flow_params = FlowParams::<()>::with_capacity(10);

    assert_eq!(0, (*flow_params).as_mut_slice().len());
}

#[test]
fn from_type_map() {
    let flow_params = FlowParams::from(TypeMap::<()>::new());

    assert_eq!(0, flow_params.len());
}
