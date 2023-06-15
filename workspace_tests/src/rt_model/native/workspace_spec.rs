use peace::rt_model::WorkspaceSpec;

#[test]
fn clone() {
    let workspace_spec = WorkspaceSpec::Path(".git".into());

    assert_eq!(workspace_spec.clone(), workspace_spec);
}

#[test]
fn debug() {
    let workspace_spec = WorkspaceSpec::WorkingDir;

    assert_eq!("WorkingDir", format!("{workspace_spec:?}"));
}

#[test]
fn partial_eq() {
    assert_eq!(WorkspaceSpec::WorkingDir, WorkspaceSpec::WorkingDir);
    assert_ne!(
        WorkspaceSpec::FirstDirWithFile(".git".into()),
        WorkspaceSpec::FirstDirWithFile(".peace".into())
    );
}
