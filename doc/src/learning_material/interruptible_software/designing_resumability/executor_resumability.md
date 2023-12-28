# 🕸️ Executor Resumability

```rust ,ignore
 fn execute<T>(
    execution_state: ExecutionState, // Add this
    interrupt_rx: mut Receiver<InterruptSignal>,
//    params: Params,                // Remove this
) -> Result<T, InterruptSignal> {
    let mut steps = [step_1, step_2, step_3];

    let (steps, params) = match &execution_state {
        ExecutionState::CleanSlate { params } => (steps, params),
        ExecutionState::PreviousState { step_ids_not_done, step_values, .. } => {
            steps.retain(|step| step_ids_not_done.contains(step.id()));
            let params = step_values.last().unwrap();

            (steps, params)
        }
    };

    steps
        .into_iter()
        .try_fold(params, |(mut last_param, step)| {
            interruptibility_check(&mut interrupt_rx)?;
            step(last_param)
        })
}
```

```rust ,ignore
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ExecutionState {
    CleanSlate {
        params: Params,
    },
    PreviousState {
        step_ids_done: Vec<StepId>,
        step_ids_not_done: Vec<StepId>,
        step_values: Vec<Value>,
    },
}
```

## Pros / Cons

* **Simpler API:** Step logic does not need to know about interruptibility concepts.
* **Stale Reasoning:** Saved state may be stale, and the executor does not have the means to check for staleness.
