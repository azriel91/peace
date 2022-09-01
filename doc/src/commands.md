# Commands

Commands suitable to expose to users.

1. `InitCmd`: Calls `setup` for each `ItemSpec`.
2. `StateDiscoverCmd`: Retrieves current and desired states.

    *🚧 not yet implemented, states are retrieved on each command invocation*

3. `StateCurrentCmd`: Displays current state.
4. `StateDesiredCmd`: Displays desired state.
5. `DiffCmd`: Displays state difference.
6. `EnsureCmd`: Transforms the current state into the desired state.

    *🚧 There is a plan to allow a subset of `ItemSpec`s to be executed.*

7. `EnsureDryCmd`: Dry-run transformation of the current state into the desired state.
8. `CleanCmd`: Cleans up the items so that they do not exist.

    *🚧 not yet implemented*

9. `CleanDryCmd`: Dry-run clean up of the items.

    *🚧 not yet implemented*
