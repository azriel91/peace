# Output

Output is providing information to a user.

Output can be categorized with number of *input dimensions*:

* **Disclosure:**
    - Push: Information is pushed to users once available.
    - Pull: Information is requested by users, may be real time or historical.
* **Rate:** *(of requests)*
    - High: Information that constantly updates.
    - Low: Information that once created isn't expected to change.
* **Consumer:**
    - Human: Person using the application.
    - Software: ReST API consumer.
    - Both: Readable and parseable text output.
* **Accumulation:**
    - Append: Information continuously increases.
    - Replace: Latest information is relevant.

<details open>
<summary>Example scenarios and solutions for different output combinations</summary>

| #  | Disclosure | Rate | Consumer | Accumulation | Example Scenario                        | Example solution    |
|:---|:-----------|:-----|:---------|:-------------|:----------------------------------------|:--------------------|
| 1  | Push       | High | Human    | Append       | Execution progress in CI / builds       | Text                |
| 2  | Push       | High | Human    | Replace      | Execution in interactive CLI            | Progress bars       |
| 3  | Push       | High | Software | Append       | Execution progress delta web socket API | Serialized          |
| 4  | Push       | High | Software | Replace      | Execution progress web socket API       | Serialized          |
| 5  | Push       | High | Both     | Append       | none                                    |                     |
| 6  | Push       | High | Both     | Replace      | none                                    |                     |
| 7  | Push       | Low  | Human    | Append       | Execution errors in CLI                 | Friendly errors     |
| 8  | Push       | Low  | Human    | Replace      | Execution outcome in CLI                | Markdown output     |
| 9  | Push       | Low  | Software | Append       | Execution errors web socket API         | Serialized          |
| 10 | Push       | Low  | Software | Replace      | Execution outcome web socket API        | Serialized          |
| 11 | Push       | Low  | Both     | Append       | Execution errors in file                | Friendly serialized |
| 12 | Push       | Low  | Both     | Replace      | Execution outcome in file               | Friendly serialized |
| 13 | Pull       | High | Human    | Append       | Historical execution progress logs      | Text                |
| 14 | Pull       | High | Human    | Replace      | none                                    |                     |
| 15 | Pull       | High | Software | Append       | none                                    | Serialized          |
| 16 | Pull       | High | Software | Replace      | Historical execution progress ReST API  | Serialized          |
| 17 | Pull       | High | Both     | Append       | none                                    |                     |
| 18 | Pull       | High | Both     | Replace      | none                                    |                     |
| 19 | Pull       | Low  | Human    | Append       | Execution errors in web page            | Formatted errors    |
| 20 | Pull       | Low  | Human    | Replace      | Execution outcome in web page           | Formatted report    |
| 21 | Pull       | Low  | Software | Append       | Execution errors ReST API               | Serialized          |
| 22 | Pull       | Low  | Software | Replace      | Execution outcome ReST API              | Serialized          |
| 23 | Pull       | Low  | Both     | Append       | Historical execution errors in file     | Friendly serialized |
| 24 | Pull       | Low  | Both     | Replace      | Historical execution outcome in file    | Friendly serialized |

</details>

From the above table, further thoughts include:

* Information that changes at a high rate implies a high rate of requests.
* Information requested at a high rate may need to be small (in size).
* Progress output is frequent, and is most important in real time / as it happens.
* If frequency is high and the output is transferred between hosts, then ideally the size of the output is reduced.
* The point of serialized data without the friendliness requirement is space and time efficiency.
* Friendly serialized data may be a format such as YAML / JSON / TOML.
* Pulled data is either over an API, or viewing historical information, meaning pulled data needs to be serializable.
* Web page output may be large, but mental overload can be avoided by hiding information through interactivity.


## Execution Progress

Execution progress output is most useful as it happens. This can consist of the following information:


### Progress Status

Whether a task has started, is in-progress, stalled, or completed.

* **Waiting:** Task has not yet started.
* **Running:** Task is in progress.
* **Stalled:** Task has not received any progress updates for a given period.
* **Pending Input:** Task is pending user input.
* **Ended:** Task has finished running, and cannot be restarted.

The following variant is possible conceptually, but not applicable to the Peace framework:

* **Stopped:** Task is not running, but can be started.

    This is not applicable because Peace uses runtime borrowing to manage state, and a stopped task has potentially altered data non-atomically, so locking the data is not useful, and unlocking the data may cause undefined behaviour due to reasoning over inconsistent state.

    For rate limiting tasks, the task in its entirety would be held back.


### Progress Measure

* Unit of measurement: Steps, Bytes, Percentage
* Units total: Known / Unknown
* Units completed: Numeric / Unknown
* Units remaining: Numeric / Unknown


### Progress Timings

* Start timestamp
* Elapsed duration
* Estimated completion timestamp
* Completion timestamp


## Outcome

### Outcome Status

* **Success:** Task has completed successfully.
* **Break:** Task has stopped for manual action not managed by the command.
* **Error:** Task has stopped with an error.


### Outcome Messages

* Nothing
* Informatives: What the user needs to do next.
* Warnings


### Outcome Errors

* What happened
* Why it is considered an error
* Source of the information that led to the error
* Suggestions for fixing


## Format

Output format can be optimized for different consumers. For example:

* Interactive command line:

    - Hide detail to reduce mental overload.
    - Show enough to give an appropriate picture.
    - Use colour to highlight / dim detail based on level of importance.
    - Show commands to display additional detail.
    - Use a format that makes sense when copying and pasting into a different application, e.g. markdown.

* Web page:

    - Use interactive elements to allow detail to be revealed when needed.
    - Use consistent colour for different levels of detail.

* Network transfer:

    - Use a serialization format that is small to cater for latency.
    - Use a serialization format that is not difficult to deserialize to reduce CPU utilization.
