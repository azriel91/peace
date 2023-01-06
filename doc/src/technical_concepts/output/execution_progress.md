# Execution Progress

Execution progress output is most useful as it happens. Execution progress output should fulfill the following concerns:

* easy for a human to understand
* easy to be sent from an item spec implementation
* pushed to output in real time
* pulled from client in real time


## Information

Information that is likely useful to the consumer, and therefore should be captured, and sent if bandwidth is not an issue.


### Progress Status

Whether a task has started, is in-progress, stalled, or completed.

* **Waiting:** Task has not yet started.
* **Running:** Task is in progress.
* **Stalled:** Task has not received any progress updates for a given period.

    Implementors are responsible for sending progress updates, but if there are no progress updates, or an identical "it's running" progress update is continuously received, then Peace may determine that the task may have stalled, and user attention is required.

    Peace may also provide a hook for implementors to output a suggestion to the user.

* **Pending Input:** Task is pending user input.
* **Ended:** Task has finished running, and cannot be restarted.

The following variant is possible conceptually, but not applicable to the Peace framework:

* **Stopped:** Task is not running, but can be started.

    This is not applicable because Peace uses runtime borrowing to manage state, and a stopped task has potentially altered data non-atomically, so locking the data is not useful, and unlocking the data may cause undefined behaviour due to reasoning over inconsistent state.

    For rate limiting tasks, the task in its entirety would be held back.


### Progress Measurement

* Unit of measurement: Steps, Bytes, Percentage, None
* Units total: Known / Unknown
* Units current: Numeric / Unknown
* Units remaining: Numeric / Unknown. This is `total - current`.
* Unit of progress tick: as it happens.


### Progress Timings

* Time started
* Elapsed duration
* Remaining duration estimate
* Time of completion estimate
* Time of completion


## Information Production / Consumption

As much as possible, Peace should collect progress information without burdening the implementor.

| Info                        | Produced / Captured by |
|:----------------------------|:-----------------------|
| Progress status             | Framework              |
| Unit of measurement         | Implementor            |
| Units total                 | Implementor            |
| Units initially completed   | Implementor            |
| Units remaining             | Framework              |
| Unit of progress tick       | Implementor            |
| Time started                | Framework              |
| Elapsed duration            | Framework              |
| Remaining duration estimate | Framework              |
| Time of completion estimate | Framework              |
| Time of completion          | Framework              |

Peace should support the following usages out-of-the-box<sup>1</sup>:

* Interactive CLI
* Non-interactive CLI
* CI logging
* WASM
* REST endpoint (pull)
* REST endpoint (push)


<sup>1</sup> even if it is not yet implemented, it should be designed in a way that allows for it

## API

Implementors define the following in `EnsureOpSpec::check`:

* Unit of measurement: Steps, Bytes, Percentage, None
* Units total: Known / Unknown
* Units initially completed: Numeric / Unknown

Units remaining is tracked by Peace as the unit of progress is ticked (updated).

There is a tradeoff between:

* Implementors manually ticking progress

    - More accurate information
    - Code becomes messy -- progress ticks scattered throughout code.
    - [`zzz`] has a really good way of doing this without making things messy:

        ```rust
        (0..1000).into_iter().progress()
        //                    ^^^^^^^^^^
        // This is all that's needed to render the progress bar
        // assuming `size_hint()` is implemented.
        ```

        Peace could provide something like:

        ```rust
        impl EnsureOpSpec for MyEnsureOpSpec {
            async fn exec(data: Data<'_>) -> Result<(), E> {
                [
                    Self::fn_1,
                    Self::fn_2,
                    Self::fn_3,
                    Self::fn_4,
                ].progress(data);
            }
        }
        ```

        But that constrains the signature of each sub function to be the same. A type parameterized function chain &ndash; where the return value of one function is the parameter to the next &ndash; may be messy, especially for error messages.

    - For spinners, manually ticking the progress bar can be cumbersome, but automatic ticking will give a false impression of progress happening. Somewhat mitigated by stall detection.

* Codegen to add progress ticks

    This could be done with a proc macro that counts the number of expressions, with some clever handling of conditionals, and inserts `tick()` expressions.

For now Peace will require implementors to manual tick the progress, and in the future, explore the following option:

* Trait to automatically tick progress for iterators.
* Proc macro to automatically insert ticks.


[`zzz`]: https://github.com/athre0z/zzz
