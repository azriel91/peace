# Resumable Implementation

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=439c1e23bf900b46fa3aa876e5606a5f)

```rust
# use std::any::Any;
# use std::fmt::Debug;
# use std::iter::IntoIterator;
# use std::marker::PhantomData;
#
fn main() {
    let _execution_state = ExecutionState::clean_slate(true);
    let execution_state = ExecutionState::PreviousState {
        step_ids_done: vec![1, 2],
        step_ids_not_done: vec![3],
        step_values: vec![Box::new(234u32), Box::new(3.5f64)],
    };

    match execute::<String>(execution_state, Receiver::new(3)) {
        Ok(value) => println!("main: {}", value),
        Err(InterruptSignal) => println!("main: interrupted!"),
    }
}

fn execute<T>(
    execution_state: ExecutionState,
    mut interrupt_rx: Receiver<InterruptSignal>,
) -> Result<T, InterruptSignal>
where
    T: 'static,
{
    let steps = vec![step(1, step_1), step(2, step_2), step(3, step_3)];
    let (steps_to_execute, params) = filter_what_to_do(execution_state, steps);

    steps_to_execute
        .into_iter()
        .try_fold(params, |last_param, step| {
            interruptibility_check(&mut interrupt_rx)?;
            Ok(step.exec(last_param))
        })
        .map(|boxed_any| *boxed_any.downcast().unwrap())
}

fn filter_what_to_do(
    execution_state: ExecutionState,
    mut steps: Vec<Box<dyn StepErased>>,
) -> (Vec<Box<dyn StepErased>>, Box<dyn Any>) {
    match execution_state {
        ExecutionState::CleanSlate { params } => (steps, params),
        ExecutionState::PreviousState {
            step_ids_not_done,
            mut step_values,
            ..
        } => {
            steps.retain(|step| step_ids_not_done.contains(&step.id()));
            let params = step_values.pop().unwrap();

            (steps, params)
        }
    }
}

fn step_1(takes_bool: bool) -> u32 {
    let number = 123;
    println!("step_1 ({takes_bool}: bool) -> {number}");
    number
}

fn step_2(takes_u32: u32) -> f64 {
    let float = 1.5;
    println!("step_2 ({takes_u32}: u32) -> {float}");
    float
}

fn step_3(takes_f64: f64) -> String {
    let string = String::from("magic");
    println!("step_3 ({takes_f64}: f64) -> {string}");
    string
}

# fn step<F, I, O>(id: u32, f: F) -> Box<dyn StepErased>
# where
#     F: Fn(I) -> O + 'static,
#     I: 'static,
#     O: 'static,
# {
#     Box::new(StepWrapper::new(id, f))
# }
#
# trait Step {
#     type Input: Any;
#     type Output: Any;
#     fn id(&self) -> u32;
#     fn exec(&self, input: Self::Input) -> Self::Output;
# }
#
# impl<F, I, O> Step for StepWrapper<F, I, O>
# where
#     F: Fn(I) -> O,
#     I: 'static,
#     O: 'static,
# {
#     type Input = I;
#     type Output = O;
#
#     fn id(&self) -> u32 {
#         self.0
#     }
#
#     fn exec(&self, input: Self::Input) -> Self::Output {
#         (self.1)(input)
#     }
# }
#
# trait StepErased {
#     fn id(&self) -> u32;
#     fn exec(&self, input: Box<dyn Any>) -> Box<dyn Any>;
# }
#
# struct StepWrapper<F, I, O>(u32, F, PhantomData<(I, O)>);
# impl<F, I, O> StepWrapper<F, I, O> {
#     fn new(id: u32, f: F) -> Self {
#         Self(id, f, PhantomData)
#     }
# }
#
# impl<F, I, O> StepErased for StepWrapper<F, I, O>
# where
#     StepWrapper<F, I, O>: Step<Input = I, Output = O>,
#     I: 'static,
#     O: 'static,
# {
#     fn id(&self) -> u32 {
#         self.0
#     }
#
#     fn exec(&self, input: Box<dyn Any>) -> Box<dyn Any> {
#         let input = *input.downcast::<I>().unwrap();
#         let output = Step::exec(self, input);
#         Box::new(output)
#     }
# }
#
# struct Receiver<T>(u32, PhantomData<T>);
# impl<T> Receiver<T> {
#     pub fn new(n: u32) -> Self {
#         Self(n, PhantomData)
#     }
# }
# #[derive(Debug)]
# struct InterruptSignal;
# fn interruptibility_check(rx: &mut Receiver<InterruptSignal>) -> Result<(), InterruptSignal> {
#     if (rx.0) == 0 {
#         Err(InterruptSignal)
#     } else {
#         rx.0 -= 1;
#         Result::Ok(())
#     }
# }
#
# #[allow(dead_code)]
#[derive(Debug)]
enum ExecutionState {
    CleanSlate {
        params: Box<dyn Any>,
    },
    PreviousState {
        step_ids_done: Vec<u32>,
        step_ids_not_done: Vec<u32>,
        step_values: Vec<Box<dyn Any>>,
    },
}
#
# #[allow(dead_code)]
# impl ExecutionState {
#     fn clean_slate<I>(i: I) -> Self
#     where
#         I: 'static,
#     {
#         ExecutionState::CleanSlate {
#             params: Box::new(i),
#         }
#     }
# }
```

<!--
1. Here we can start with a clean slate, or with some previous state.
2. If we start with a clean slate, then we will run all the steps.
3. If we start with previous state, then we only run the steps that haven't been executed.
-->
