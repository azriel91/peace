# 📃 Reporting

## Non-Interruptible Software

```rust ,ignore
fn execute(params: Params) -> Value {
    // ..
}
```

## Interruptible Software

```rust ,ignore
fn execute(params: Params) -> Outcome {
    // ..
}
```

### Outcome

<details open>
<summary>Basic</summary>

```rust ,ignore
enum Outcome {
    /// Execution completed, here is the return value.
    Complete(Value),
    /// Execution was interrupted.
    Interrupted,
}
```

</details>

<details open>
<summary>Step Values</summary>

```rust ,ignore
enum Outcome {
    /// Execution completed, here is the return value.
    Complete(Value),
    /// Execution was interrupted.
    ///
    /// Here's the information we collected so far.
    Interrupted {
        step_1_value: Option<Step1Value>,
        step_2_value: Option<Step2Value>,
        step_3_value: Option<Step3Value>,
    },
}
```

</details>

<details open>
<summary>Step Values and Execution Info</summary>

```rust ,ignore
enum Outcome {
    /// Execution completed, here is the return value.
    Complete(Value),
    /// Execution was interrupted.
    ///
    /// Here's the information we collected so far.
    Interrupted {
        step_1_value: Option<Step1Value>,
        step_2_value: Option<Step2Value>,
        step_3_value: Option<Step3Value>,
        steps_processed: Vec<StepId>,
        steps_not_processed: Vec<StepId>,
    },
}
```

</details>

<!--
1. Reporting is about what we tell the caller of our code, alongside what they asked.
2. Normally when we write code, we return the value that the code is meant to compute.
3. When we write interruptible software, we're not only returning a value, but also reporting on what happened.
4. In this basic `Outcome`, we return the value if the execution is complete, or we tell the caller that we were interrupted, so there is no value to return.
5. We can be more informative, by collecting the values produced by each step, and if we are interrupted, we return those values within our response.
6. Finally, we can also keep track of the steps that we have executed, and steps that we have not executed, and include that in our response.
7. All of this is in preparation for, if we want to resume execution, we have enough information to restart, where we last stopped.
-->
