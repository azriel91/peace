# State Discovery and Display

> This is like `git fetch`.

This kind of command is intended for state discovery that is expensive &ndash; takes more than 500 milliseconds.

Suitable scopes for this command are:

* `SingleProfileSingleFlow`: For discovering state for one profile.
* `MultiProfileSingleFlow`: For discovering state for multiple profiles.


## Command Creation

To create this command:

1. When building the command context:

    - Provide the profile.
    - Provide the flow ID.

2. Call the `StatesDiscoverCmd` depending on the intended use:

    These will store the discovered states under the corresponding `$profile/$flow_id` directory as `states_saved.yaml` or `states_goal.yaml`.

3. If the discovered states are to be displayed, call the relevant state display command(s):

    - `StatesSavedDisplayCmd`: For saved states to be displayed.
    - `StatesGoalDisplayCmd`: For goal states to be displayed.
