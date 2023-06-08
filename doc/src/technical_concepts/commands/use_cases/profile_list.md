# Profile List

> This is like `git branch`.

When listing profiles, the most suitable scope is `MultiProfileNoFlow`.

There are a number of variations of profile listings:

* **Simple:** Simple list of profiles within the `PeaceAppDir`.
* **Filtered:** List profiles within the `PeaceAppDir` for some criteria, e.g. excluding a `workspace_init` profile.

    This is especially relevant when profile parameters are loaded, as the `ProfileParams` must be the same across the listed profiles to be able to be loaded.

* **Augmented:** List profiles and some profile parameters.

    This is valuable when the user needs to see the profile parameter, such as whether the profile is managing a development environment or a production environment.


## Command Creation

To create this command:

1. Build the command context.

    - Make sure to register the workspace / profile params types, even if the value for all of them is `None`.
    - Provide the filter function if necessary.

2. Present the list of profiles using the goal output.
