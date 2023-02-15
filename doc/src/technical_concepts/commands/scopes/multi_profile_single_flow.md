# Multi Profile Single Flow

This scope is for a command that works with multiple profiles, and a single flow.

```bash
path/to/repo/.peace/envman
|- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
|
|- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
|   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
|   |
|   |- 🌊 deploy                   # ✅ can read `FlowId`
|   |   |- 📝 flow_params.yaml     # ✅ can read or write `FlowParams`
|   |   |- 📋 states_desired.yaml  # ✅ can read or write `StatesDesired`
|   |   |- 📋 states_saved.yaml    # ✅ can read or write `StatesSaved`
|   |
|   |- 🌊 ..                       # ❌ cannot read or write other `Flow` information
|
|- 🌏 customer_a_dev           # ✅
|   |- 📝 profile_params.yaml  # ✅
|   |
|   |- 🌊 deploy                   # ✅
|       |- 📝 flow_params.yaml     # ✅
|       |- 📋 states_desired.yaml  # ✅
|       |- 📋 states_saved.yaml    # ✅
|
|- 🌏 customer_a_prod          # ✅
|   |- 📝 profile_params.yaml  # ✅
|   |
|   |- 🌊 deploy                   # ✅
|       |- 📝 flow_params.yaml     # ✅
|       |- 📋 states_desired.yaml  # ✅
|       |- 📋 states_saved.yaml    # ✅
|
|
|- 🌏 workspace_init           # ✅ can list multiple `Profile`s
    |- 📝 profile_params.yaml  # ❌ cannot read profile params of different underlying type
|   |- 🌊 workspace_init       # ❌ cannot read unrelated flows
```

## Capabilities

This kind of command can:

* Read or write workspace parameters.
* Read or write multiple profiles' parameters &ndash; as long as they are of the same type (same `struct`).
* Read or write flow parameters for the same flow.
* Read or write flow state for the same flow.

This kind of command cannot:

* Read or write flow parameters for different flows.
* Read or write flow state for different flows.
