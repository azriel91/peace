# Workspace Active Profile Switch

> This is like `git switch $branch`.

This changes the stored profile in `workspace_params`, used in conjunction with the above.

Suitable scopes for this command are `NoProfileNoFlow` or `SingleProfileNoFlow`.


## Command Creation

To create this command:

1. When building the command context:

    - Include a `Profile` as a workspace param.
    - Also set the current profile using `with_profile`.

When the command context is built, the default profile is saved to `workspace_params.yaml`.
