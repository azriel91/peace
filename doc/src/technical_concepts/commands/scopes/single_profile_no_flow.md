# Single Profile No Flow

This scope is for a command that works with a single profile, without any items.

```bash
path/to/repo/.peace/envman
|- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
|
|- 🌏 internal_dev_a           # ✅ can read `Profile`
|   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
|   |
|   |- 🌊 ..                   # ❌ cannot read or write Flow information
|
|- 🌏 ..                       # ❌ cannot read or write other `Profile` information
```

## Capabilities

This kind of command can:

* Read or write workspace parameters.
* Read or write a single profile's parameters. For multiple profiles, see
  `MultiProfileNoFlow`.

This kind of command cannot:

* Read or write flow parameters -- see `SingleProfileSingleFlow` or
  `MultiProfileSingleFlow`.
* Read or write flow state -- see `SingleProfileSingleFlow` or
  `MultiProfileSingleFlow`.
