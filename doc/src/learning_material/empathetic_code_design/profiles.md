# Profiles

Profiles are a way of logically separating environments, aka namespacing.

First execution / init:

```rust ,ignore
let cmd_ctx_builder = CmdCtx::builder_single_profile_no_flow
    ::<EnvManError, _>(output, &workspace)
    .with_workspace_param_value(
        WorkspaceParamsKey::Profile,
        Some(profile!("demo")),
    );
```

Subsequent executions:

```rust ,ignore
let cmd_ctx_builder = CmdCtx::builder_single_profile_single_flow
    ::<EnvManError, _>(output, workspace)
    .with_profile_from_workspace_param(WorkspaceParamsKey::Profile)
    .with_flow(flow);
```
