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


### Concept 17: Flows


### Concept 18: Parameter Limits

