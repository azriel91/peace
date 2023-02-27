# No Profile No Flow

This scope is for a command that only works with workspace parameters.

```bash
path/to/repo/.peace/envman
|- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
|
|- 🌏 ..                       # ❌ cannot read or write `Profile` information
```

## Capabilities

This kind of command can:

* Read or write workspace parameters.

This kind of command cannot:

* Read or write profile parameters -- see `SingleProfileNoFlow` or
  `MultiProfileNoFlow`.
* Read or write flow parameters -- see `MultiProfileNoFlow`.
* Read or write flow state -- see `SingleProfileSingleFlow` or
  `MultiProfileSingleFlow`.
