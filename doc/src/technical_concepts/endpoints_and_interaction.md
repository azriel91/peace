# Endpoints and Interaction

## Background

For a web UI, we want:

1. A way to invoke `*Cmd`s -- invoke and return an ID.
2. Command invocation request is idempotent / does not initiate another invocation when one is in progress.
3. A way to interrupt `CmdExecution`s -- get the `Sender<InterruptSignal>` by execution ID, send.
4. A way to pull progress from the client -- close / reopen tab, send URL to another person.
5. A way to push progress to the client -- for efficiency. web sockets?
6. For both a local and shared server, a way to open a particular env/`Flow`/`CmdExecution` by default.
7. For a shared server, a way to list `CmdExecution`s.
8. For a local server, to automatically use different ports when running executions in different projects.


## Glossary

| Term | Definition                                             |
|:-----|:-------------------------------------------------------|
| Web  | Refers to server side execution, not client side WASM. |
