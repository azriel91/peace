# Workspace Initialization

> This is like `git init`.

When initializing a project workspace, the suitable scope is different depending on the amount of work done:

## Command Creation

* If it only stores parameters provided by the user, shared across profiles, and does not use any item spec, use `NoProfileNoFlow`.

    To create this command:

    1. Build the command context with the provided parameters.

    When the command context is built:

    - Workspace parameters are written to `workspace_params.yaml`.

* If it stores parameters for a *default* profile, and does not use any item spec, use `SingleProfileNoFlow`.

    To create this command:

    1. Build the command context with the provided parameters.

    When the command context is built:

    - Workspace parameters are written to `workspace_params.yaml`.
    - Profile parameters are written to `profile_params.yaml`.

* If it does any repeatable work, such as download files or clone a repository, use `SingleProfileSingleFlow`.

    To create this command:

    1. Build the command context with the provided parameters.
    2. Call `StateDiscoverCmd::exec` to discover the current and desired states.
    3. Call `EnsureCmd::exec` to execute the flow.

    When the command context is built:

    - Workspace parameters are written to `workspace_params.yaml`.
    - Profile parameters are written to `profile_params.yaml`.
    - Flow parameters are written to `flow_params.yaml`.
