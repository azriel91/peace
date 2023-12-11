# 💮 Stepwise Resumability

```rust ,ignore
 fn execute<T>(
    execution_state: ExecutionState, // Add this
    interrupt_rx: mut Receiver<InterruptSignal>,
) -> Result<T, InterruptSignal> {
    [step_1, step_2, step_3]
        .into_iter()
        .try_fold(params, |(mut last_param, step)| {
            interruptibility_check(&mut interrupt_rx)?;

            let step_state = execution_state.get(step.id());
            step(step_state, last_param)
        })
}
```

```rust ,ignore
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExecutionState { .. }

impl ExecutionState {
    pub fn get<T>(step_id: StepId) -> T { .. }
}
```

## Pros / Cons

* **Heavier API:** Step logic needs to cater for initial state.
* **Usage Safety:** Step can determine if the saved initial state is in-sync with actual state, and warn user.
