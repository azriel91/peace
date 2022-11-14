# User Facing Automation - Part 2

## Workflow Concepts

### Concept 14: Storing Parameters

**What:** Take in parameters, recall them automatically.

**Value:** Don't require users to pass in parameters repeatedly.

<details>
<summary>Example</summary>

```bash
download init https://ifconfig.me ip.json
```

```bash
download ensure
download clean
```

</details>


### Concept 15: Current State, Desired State, Difference

**What:** Show users what is, and what will be, before doing anything.

**Value:** Give users clarity on what would happen, before doing it.

<details>
<summary>Example</summary>

```bash
download clean
download init https://ifconfig.me ip.json
```

```bash
download status
download desired
download diff

download ensure
download diff

download clean
download diff
```

</details>


### Concept 16: Profiles

**What:** Use the same project files for logically separate environments.

**Value:** Ease the creation of cloned environments.


### Concept 17: Flows

**What:** Perform different workflows within an environment.

**Value:** Workflows automatically share parameters with each other.


### Concept 18: Parameter Limits

**What:** Guard automation from executing with unusual values.

**Value:** Don't accidentally take down a system.
