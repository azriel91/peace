# State Apply

> This is like `git commit` / `git push` / `git clean`, depending on your perspective.

This kind of command applies the desired state over the current state.

This generally requires what is saved in `states_saved.yaml` to match the newly discovered current state.

The only suitable scope for this command is `SingleProfileSingleFlow`.


## Command Creation

To create this command:

1. When building the command context:

    - Provide the profile.
    - Provide the flow ID.

2. Call the `EnsureCmd`.

    This will call `ApplyFns::exec` for each item, beginning from the first item, until the last item.
