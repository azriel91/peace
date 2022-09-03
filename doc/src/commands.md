# Commands

Commands suitable to expose to users.

1. `InitCmd`: Calls `setup` for each `ItemSpec`.
2. `StateDiscoverCmd`: Retrieves current and desired states.

    *🚧 not yet implemented, states are retrieved on each command invocation*

3. `StatesCurrentDiscoverCmd`: Discovers current states.
4. `StatesDesiredDiscoverCmd`: Discovers desired states.
5. `StateDiscoverCmd`: Discovers both current and desired states.
6. `DiffCmd`: Displays state difference.
7. `EnsureCmd`: Transforms the current state into the desired state.

    *🚧 There is a plan to allow a subset of `ItemSpec`s to be executed.*

8. `EnsureDryCmd`: Dry-run transformation of the current state into the desired state.
9. `CleanCmd`: Cleans up the items so that they do not exist.

    *🚧 not yet implemented*

10. `CleanDryCmd`: Dry-run clean up of the items.

    *🚧 not yet implemented*
