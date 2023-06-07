# Single Profile Single Flow

This scope is for a command that works with one profile and one flow.

```bash
path/to/repo/.peace/envman
|- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
|
|- 🌏 internal_dev_a
|   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
|   |
|   |- 🌊 deploy                   # ✅ can read `FlowId`
|   |   |- 📝 flow_params.yaml     # ✅ can read or write `FlowParams`
|   |   |- 📋 states_goal.yaml  # ✅ can read or write `StatesGoal`
|   |   |- 📋 states_current.yaml    # ✅ can read or write `StatesCurrentStored`
|   |
|   |- 🌊 ..                   # ❌ cannot read or write other `Flow` information
|
|- 🌏 ..                       # ❌ cannot read or write other `Profile` information
```

## Capabilities

This kind of command can:

* Read or write workspace parameters.
* Read or write a single profile's parameters. For multiple profiles, see
  `MultiProfileNoFlow`.

This kind of command cannot:

* Read or write flow parameters -- see `MultiProfileNoFlow`.
* Read or write flow state -- see `SingleProfileSingleFlow` or
  `MultiProfileSingleFlow`.
