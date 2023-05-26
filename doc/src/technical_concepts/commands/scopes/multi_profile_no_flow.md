# Multi Profile No Flow

This scope is for a command that works with multiple profiles, without any items.

```bash
path/to/repo/.peace/envman
|- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
|
|- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
|   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
|   |
|   |- ..                      # ❌ cannot read or write `Flow` information
|
|- 🌏 customer_a_dev           # ✅
|   |- 📝 profile_params.yaml  # ✅
|
|- 🌏 customer_a_prod          # ✅
|   |- 📝 profile_params.yaml  # ✅
|
|- 🌏 workspace_init           # ✅ can list multiple `Profile`s
    |- 📝 profile_params.yaml  # ❌ cannot read profile params of different underlying type
```

## Capabilities

This kind of command can:

* Read or write workspace parameters.
* Read or write multiple profiles' parameters &ndash; as long as they are of the same type (same `struct`).

This kind of command cannot:

* Read or write flow parameters -- see `SingleProfileSingleFlow` or
  `MultiProfileSingleFlow`.
* Read or write flow state -- see `SingleProfileSingleFlow` or
  `MultiProfileSingleFlow`.
