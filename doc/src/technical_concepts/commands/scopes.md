# Scopes

The scope of a command determines:

* If it has access to zero, one, or many `Profile` directories.
* If it has access to zero, one, or many `Flow` directories.
* Whether it can deserialize profile params.
* Whether it can deserialize flow params.
* Whether it can deserialize states from each flow directory.

**Scenario:** `envman` is an example tool that manages server environments.

In the following directory structure:

* `internal_dev_a`, `customer_a_dev`, `customer_a_prod` are all separate **Profile**s.
* `deploy`, `config`, and `benchmark` are separate **Flow**s.
* Each flow tracks its own `flow_params.yaml`, `states_desired.yaml`, and `states_saved.yaml`

```bash
# peace app dir
path/to/repo/.peace/envman
|- 📝 workspace_params.yaml
|
|- 🌏 internal_dev_a
|   |- 📝 profile_params.yaml
|   |
|   |- 🌊 deploy
|   |   |- 📝 flow_params.yaml
|   |   |- 📋 states_desired.yaml
|   |   |- 📋 states_saved.yaml
|   |
|   |- 🌊 config
|   |- 🌊 benchmark
|
|- 🌏 customer_a_dev
|   |- 📝 profile_params.yaml
|   |
|   |- 🌊 deploy - ..
|   |- 🌊 config - ..
|
|- 🌏 customer_a_prod
    |- 📝 profile_params.yaml
    |
    |- 🌊 deploy - ..
    |- 🌊 config - ..
```

See each page for details of each scope:

* **[No Profile No Flow]\:** Commands that only work with workspace parameters.
* **[Single Profile No Flow]\:** Commands that work with a single profile, without any item specs.
* **[Single Profile Single Flow]\:** Commands that work with one profile and one flow.
* **[Multi Profile No Flow]\:** Commands that work with multiple profiles, without any item specs.
* **[Multi Profile Single Flow]\:** Commands that work with multiple profiles, and a single flow.

[No Profile No Flow]: scopes/no_profile_no_flow.md
[Single Profile No Flow]: scopes/single_profile_no_flow.md
[Single Profile Single Flow]: scopes/single_profile_single_flow.md
[Multi Profile No Flow]: scopes/multi_profile_no_flow.md
[Multi Profile Single Flow]: scopes/multi_profile_single_flow.md
