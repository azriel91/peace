use peace::{cmd_ctx::type_reg::untagged::TypeMap, rt_model::params::WorkspaceParams};

#[test]
fn new() {
    let workspace_params = WorkspaceParams::<()>::new();

    assert_eq!(0, workspace_params.len());
    assert_eq!(0, workspace_params.capacity());
}

#[test]
fn with_capacity() {
    let workspace_params = WorkspaceParams::<()>::with_capacity(10);

    assert_eq!(0, workspace_params.len());
    assert_eq!(10, workspace_params.capacity());
}

#[test]
fn into_inner() {
    let workspace_params = WorkspaceParams::<()>::with_capacity(10);
    let type_map = workspace_params.into_inner();

    assert_eq!(0, type_map.len());
    assert_eq!(10, type_map.capacity());
}

#[test]
fn deref() {
    let workspace_params = WorkspaceParams::<()>::with_capacity(10);

    assert_eq!(0, (*workspace_params).len());
}

#[test]
fn deref_mut() {
    let mut workspace_params = WorkspaceParams::<()>::with_capacity(10);

    assert_eq!(0, (*workspace_params).as_mut_slice().len());
}

#[test]
fn from_type_map() {
    let workspace_params = WorkspaceParams::from(TypeMap::<()>::new());

    assert_eq!(0, workspace_params.len());
}
