use peace::{cmd_ctx::type_reg::untagged::TypeMapOpt, rt_model::params::WorkspaceParamsOpt};

#[test]
fn new() {
    let workspace_params_opt = WorkspaceParamsOpt::<()>::new();

    assert_eq!(0, workspace_params_opt.len());
    assert_eq!(0, workspace_params_opt.capacity());
}

#[test]
fn with_capacity() {
    let workspace_params_opt = WorkspaceParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, workspace_params_opt.len());
    assert_eq!(10, workspace_params_opt.capacity());
}

#[test]
fn into_inner() {
    let workspace_params_opt = WorkspaceParamsOpt::<()>::with_capacity(10);
    let type_map_opt = workspace_params_opt.into_inner();

    assert_eq!(0, type_map_opt.len());
    assert_eq!(10, type_map_opt.capacity());
}

#[test]
fn deref() {
    let workspace_params_opt = WorkspaceParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, (*workspace_params_opt).len());
}

#[test]
fn deref_mut() {
    let mut workspace_params_opt = WorkspaceParamsOpt::<()>::with_capacity(10);

    assert_eq!(0, (*workspace_params_opt).as_mut_slice().len());
}

#[test]
fn from_type_map_opt() {
    let workspace_params_opt = WorkspaceParamsOpt::from(TypeMapOpt::<()>::new());

    assert_eq!(0, workspace_params_opt.len());
}
