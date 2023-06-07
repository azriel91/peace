# State Diff

> This is like `git status` / `git diff`.
>
> `git status` is a diff between the latest commit, and the working directory, summarized as one line per file.
>
> `git diff` is that same diff, but shown with more detail.

This kind of command shows the difference between two `State`s for each managed item.

Typically this is the difference between the current (or saved) state, and its goal state. It can also be the difference between the current states between two profiles.

Suitable scopes for this command are:

* `SingleProfileSingleFlow`: For diffing state within one profile.
* `MultiProfileSingleFlow`: For diffing state across multiple profiles.


## Command Creation

To create this command for a single profile:

1. When building the command context:

    - Provide the profile.
    - Provide the flow ID.

2. Determine the "from" and "to" states:

    The "from" state is usually one of:

    - Discovering the current state.
    - Reading `states_current.yaml`.

    The "to" state is usually one of:

    - Discovering the goal state.
    - Reading `states_goal.yaml`.
    - The *clean* state.

3. Call the state `DiffCmd`.

To create this command for multiple profiles:

1. When building the command context:

    - Filter the profiles if necessary.
    - Provide the flow ID.

2. Determine the "from" and "to" states:

    The "from" state is usually one of:

    - Discovering the current state.
    - Reading `states_current.yaml`.

    The "to" state is usually one of:

    - Discovering the goal state.
    - Reading `states_goal.yaml`.
    - The *clean* state.

3. Call the state `DiffCmd` for each pair of states.
