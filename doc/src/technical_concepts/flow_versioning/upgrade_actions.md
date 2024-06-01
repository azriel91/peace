# Upgrade Actions

There are 4 levels of upgrade actions that can be applied to items.

* Reserialization with existing stored values.
* State discovery: current / goal.
* Item ensure.
* Replacement (clean and ensure) required.

The following table shows what is required for ensuring an environment to be in sync with the change. If multiple changes are involved, then the highest level of upgrade action needs to be applied to the environment for params and state, and the environment to be in sync, i.e. `max(upgrade_actions)`.


> Notes:
>
> * "Data" refers to a step's `Params`, `State`, or `StateDiff`.
> * Apply is associated with params, state, and diff.
>
>     `apply_clean` needs to know `state_clean` based on the parameters used to compute `state_goal` at the time of the previous `apply_goal`.
>
> * "Predecessor *action*" means what upgrade action is needed for *this* item, given a predecessor has had *action* applied to it. In other words, the change for a step can imply an upgrade action for successors.

| Change                                    | Upgrade action                   |
|:------------------------------------------|:---------------------------------|
| Param value                               | `apply_goal`                     |
| Data type field rename                    | `reserialization`                |
| Data type field addition                  | `state_discovery`                |
| Data type field removal                   | `state_discovery`                |
| Data type field modification              | `apply_goal`                     |
| Flow item addition                        | `apply_goal`                     |
| Flow item removal                         | `apply_clean`                    |
| Flow item replacement                     | `apply_clean`, `apply_goal`      |
| Predecessor replacement, `Edge::Link`     | `apply_goal`                     |
| Predecessor replacement, `Edge::Contains` | `apply_clean`, `apply_goal`      |
| Predecessor addition / modification       | `apply_goal` (in case of update) |
| Predecessor removal                       | `apply_clean` (successor first)  |
