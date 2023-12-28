# Interruptible Implementation

## No Interruptibility

```rust ,ignore
fn execute(
    params: Params,
) -> Outcome {
    let output_1 = step_1(params);
    let output_2 = step_2(output_1);
    let outcome = step_3(output_2);

    return outcome;
}
```

## Basic

```rust ,ignore
fn execute(
    interrupt_rx: mut Receiver<InterruptSignal>,
    params: Params,
) -> ControlFlow<InterruptSignal, Outcome> {
    let () = interruptibility_check(&mut interrupt_rx)?;
    let output_1 = step_1(params);

    let () = interruptibility_check(&mut interrupt_rx)?;
    let output_2 = step_2(output_1);

    let () = interruptibility_check(&mut interrupt_rx)?;
    let outcome = step_3(output_2);

    ControlFlow::Continue(outcome)
}

fn interruptibility_check(receiver: &mut Receiver<InterruptSignal>)
-> ControlFlow<InterruptSignal, ()> {
    if let Ok(interrupt_signal) = interrupt_rx.try_recv() {
        ControlFlow::Continue(())
    } else {
        ControlFlow::Break(interrupt_signal)
    }
}
```

## "Better"

```rust ,ignore
fn execute<T>(
    interrupt_rx: mut Receiver<InterruptSignal>,
    params: Params,
) -> Result<T, InterruptSignal> {
    [step_1, step_2, step_3]
        .into_iter()
        .try_fold(params, |(mut last_param, step)| {
            interruptibility_check(&mut interrupt_rx)?;
            step(last_param)
        })
}
```

## Real Code

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7746f25a2b7b00dad4b523f44570740b)

```rust
# use std::any::Any;
# use std::iter::IntoIterator;
# use std::marker::PhantomData;
#
fn main() {
    match execute::<String, _>(Receiver::new(3), true) {
        Ok(value) => println!("main: {}", value),
        Err(InterruptSignal) => println!("main: interrupted!"),
    }
}

fn execute<T, I>(
    mut interrupt_rx: Receiver<InterruptSignal>,
    params: I,
) -> Result<T, InterruptSignal>
where
    T: 'static,
    I: 'static,
{
    [step(step_1), step(step_2), step(step_3)]
        .into_iter()
        .try_fold(Box::new(params) as Box<dyn Any>, |last_param, step| {
            interruptibility_check(&mut interrupt_rx)?;
            Ok(step.exec(last_param))
        })
        .map(|boxed_any| *boxed_any.downcast().unwrap())
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
#
# fn step<F, I, O>(f: F) -> Box<dyn StepErased>
# where
#     F: Fn(I) -> O + 'static,
#     I: 'static,
#     O: 'static,
# {
#     Box::new(StepWrapper::new(f))
# }
#
# trait Step {
#     type Input: Any;
#     type Output: Any;
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
#     fn exec(&self, input: Self::Input) -> Self::Output {
#         (self.0)(input)
#     }
# }
#
# trait StepErased {
#     fn exec(&self, input: Box<dyn Any>) -> Box<dyn Any>;
# }
#
# struct StepWrapper<F, I, O>(F, PhantomData<(I, O)>);
# impl<F, I, O> StepWrapper<F, I, O> {
#     fn new(f: F) -> Self {
#         Self(f, PhantomData)
#     }
# }
#
# impl<F, I, O> StepErased for StepWrapper<F, I, O>
# where
#     StepWrapper<F, I, O>: Step<Input = I, Output = O>,
#     I: 'static,
#     O: 'static,
# {
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
```

<!--
1. To implement interruptibility, we start with the non-interruptible logic.
2. And we saw earlier we could sprinkle interruptibility checks before each step.
3. But we've lost clarity.
4. Can we do "better"?
5. Yes we can.
6. We can implement interruptibility, without sacrificing maintainability.
-->
