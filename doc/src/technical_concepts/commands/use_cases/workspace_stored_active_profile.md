# Workspace Stored Active Profile

> This is like `git switch -c $branch`.

This stores the profile to use in `workspace_params`, so the use does not have to provide the profile on every command invocation.

Suitable scopes for this command are `NoProfileNoFlow` or `SingleProfileNoFlow`.


## Command Creation

To create this command:

1. When building the command context:

    - Include a `Profile` as a workspace param.
    - If setting profile parameters, use the `SingleProfileNoFlow` scope, and set the current profile using `with_profile`.

2. When building the command context, load the profile from workspace params using: `with_profile_from_workspace_params`.

It is best practice to set the active profile during workspace initialization, so that it is never `None` when subsequent commands are invoked.
