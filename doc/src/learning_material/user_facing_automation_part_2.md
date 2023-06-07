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


### Concept 15: Current State, Goal State, Difference

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
download goal
download diff

download ensure
download diff

download clean
download diff
```

</details>


### Concept 16: Profiles

**What:** Use the same workspace input for logically separate environments.

**Value:** Ease the creation of cloned environments.


### Concept 17: Flows

**What:** Perform different workflows within an environment.

**Value:** Workflows automatically share parameters with each other.


### Concept 18: Parameter Limits

**What:** Guard automation from executing with unusual values.

**Value:** Don't accidentally incur large costs.


## Efficiency Concepts (Again!)

### Concept 19: Do It Before It's Asked

> If a process cannot take less than 10 minutes,  
> then to do it in 5,  
> you must begin in the past.

**What:** Execute the automation in the background, show where it is when it is asked to be executed.

**Value:** Reduce waiting time.

**Cost:** Background work consumes resources, and may be redundant.


### Concept 20: Reverse Execution

> At the beginning of a process, what's the fastest way to get to step 9?
>
> Go through steps 1 through 9.
>
> What if you're on step 10?
>
> Transition from step 10 to 9.

**What:** The beginning and destination state are not necessarily the start and end of a process.

**Value:** Allows inspection / mutation of state at a particular point, or re-testing of automation steps after that point.


