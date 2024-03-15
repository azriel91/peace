# Cmd Invocation

Need to cater for:

1. **CLI usage:** Invocation returns the result.
2. **Web usage:** Invocation returns an ID.
3. **Any usage:** For a given profile, two `CmdExecution`s cannot run at the same time, even for different flows.
4. **Web usage:** For a given profile, re-invocation returns existing in-progress `CmdExecution` ID. This can be deferred if two browser tabs for the same workspace + profile combination both disable the deploy button when a `CmdExecution` is  initiated.


## Option A1: `exec` delegates to `request_exec`

For the CLI usage, to reduce code duplication `*Cmd`s can provide function that return the `Result<CmdOutcome, ..>`, where internally it calls the method that returns an execution ID, but immediately waits for that execution's completion.

```rust
pub async fn exec<'ctx>(
    cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, CmdCtxTypesT>>,
) -> Result<CmdOutcome<_, CmdCtxTypesT::AppError>, CmdCtxTypesT::AppError>
where
    CmdCtxTypesT: 'ctx,
{
    let execution_id = Self::request_exec(cmd_ctx);

    executions.get(execution_id).await
}
```


## Option A2: `request_exec` delegates to `exec`

```rust ,ignore
pub async fn request_exec<'ctx>(
    cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, CmdCtxTypesT>>,
) -> ExecutionId
where
    CmdCtxTypesT: 'ctx,
{
    if let Some(execution_id) = executions.get((workspace, profile)) {
        return execution_id;
    };

    let execution_id = server.generate_execution_id(workspace, profile).await;
    let cmd_execution = Self::exec(cmd_ctx);
    // or send(..) the execution request to a queue, and the queue receiver calls the `exec`.

    executions.put(execution_id, cmd_execution).await

    execution_id
}
```

## Web Interface

Web Server:

* Needs to hold a collection of all executions.
* Needs to hold mapping from Execution ID to `CmdExecution`, and/or parts of the `CmdExecution`.

    Storing parts separately can with access and extensibility:

    - Sometimes we don't want to borrow the full `CmdExecution`, only part of it.
    - Adding new things gets stored in a different server context state, so components that are not concerned with the new state don't need to access it.
        <!--  -->

    Need to make sure all context is added in the same place, otherwise it is difficult to track "what makes up a `CmdExecution`".


### 1. Web Server `CmdExecutions` Tracking

* `CmdExecutions` is the collection of in-progress executions, not just their serializable info.

    Possibly a `LinkedHashMap<ExecutionId, Box<dyn CmdExecutionRt>>`, where `CmdExecutionRt` is a trait over the concrete `CmdExecution`s which are type parameterized.

* `CmdExecutionsInfo` is a serializable collection of both in-progress and historical execution infos.

    Possibly a `LinkedHashMap<ExecutionId, CmdExecutionInfo>`.

```rust ,ignore
// Web Server set up needs to track everything
// or, link to a database that tracks everything
let cmd_executions = CmdExecutions::default();
let router = Router::new()
    // ..
    .leptos_routes_with_context(
        &leptos_options,
        routes,
        move || {
            // ..
            leptos::provide_context(Arc::clone(cmd_executions));
        },
        move || view! {  <Home /> },
    )
    // ..
    ;
```

### 2. Web Component `CmdExecutionInfos` Access For Display

`CmdExecutionInfos` is the serializable type used to represent `CmdExecutions` for display:

```rust ,ignore
/// Returns the list of `CmdExecutions` that have run / are in-progress on the server.
#[leptos::server(endpoint = "/cmd_execution_infos")]
pub async fn cmd_execution_infos(
) -> Result<CmdExecutionInfos, ServerFnError<NoCustomError>> {
    let cmd_execution_infos = leptos::use_context::<CmdExecutionInfos>()
        .ok_or_else(|| {
            ServerFnError::<NoCustomError>::ServerError(
                "`CmdExecutionInfos` was not set.".to_string()
            )
        })?;

    Ok(cmd_execution_infos)
}

#[component]
pub fn CmdExecutionsList() -> impl IntoView {
    let cmd_execution_infos_resource = leptos::create_resource(
        || (),
        move |()| async move { cmd_execution_infos().await.unwrap() },
    );
    let cmd_execution_infos = move || {
        cmd_execution_infos_resource
            .get()
            .expect("Expected `cmd_execution_infos` to always be generated successfully.")
    };

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <For each=cmd_execution_infos /* display the info */ />
        </Transition>
    }
}
```


### 3. Web Component `CmdExecutions` Access For `*Cmd` Invocation

Given a `workspace`, `profile`, `flow_id`, a `*Cmd` and `*Cmd` parameters, a user should be able to send a `CmdExecutionRequest`. Some of these parameters should be able to be defaulted, e.g. for a local automation server which is run from the workspace directory.


---

Should [`create_action`](https://book.leptos.dev/async/13_actions.html) or [`create_server_action`](https://docs.rs/leptos/latest/leptos/fn.create_server_action.html) be used?

<details open>

Answer From `@Lazer` ([discord](https://discord.com/channels/1031524867910148188/1031524868883218474/1215847615183327352))

You can provide params to server actions via hidden inputs if necessary.

```rust ,ignore
#[server(endpoint = "check_code")]
pub async fn check_code(s: Uuid, c: Code) -> Result<UserMetadata, ServerFnError> {
    todo!();
}

let check_action = create_server_action::<CheckCode>();
<ActionForm action=check_action class="mx-auto px-6 py-4 rounded-xl bg-white max-w-[400]">
    <input type="hidden" id="s" name="s" value=s />
    <div class="mb-4">
        <label for="c" class="block text-md text-gray-700">
            Verification Code
        </label>
        <input
            class="various tailwind"
            id="c" name="c" prop:value=c required type="number" placeholder="6 digit code"
            on:input=move |ev| c.set(event_target_value(&ev))/>
    </div>
    <ErrorDisplay res=check_action />
    <div class="mb-6">
        <p class="text-sm my-1 text-grey-600" hidden=move || email().is_none()>
            Sent code to {move || email()}
        </p>
        <a class="text-sm my-1 text-grey-600 hover:underline" href="loginhelp">
            "Didn't get an email?"
        </a>
    </div>
    <button type="submit" disabled=check_action.pending()
        class="various tailwind">
        SUBMIT
    </button>
</ActionForm>
```

</details>


#### Example

```rust ,ignore
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EnsureCmdArgs {
    workspace: Workspace,
    profile: Profile,
    flow_id: FlowId,
}

pub struct CmdExecutionQueues(HashMap<(Workspace, Profile), Sender<CmdExecutionRequest>>);

#[leptos::server(endpoint = "/ensure_cmd")]
pub async fn ensure_cmd(
    ensure_cmd_args: EnsureCmdArgs,
) -> Result<ExecutionId, ServerFnError<NoCustomError>> {
    let cmd_execution_queues = leptos::use_context::<CmdExecutionQueues>()
        .ok_or_else(|| {
            ServerFnError::<NoCustomError>::ServerError(
                "`Sender<CmdExecutionRequest>` was not set.".to_string()
            )
        })?;
    let cmd_execution_infos = leptos::use_context::<CmdExecutionInfos>()
        .ok_or_else(|| {
            ServerFnError::<NoCustomError>::ServerError(
                "`Sender<CmdExecutionInfos>` was not set.".to_string()
            )
        })?;

    let execution_id = cmd_execution_queues.get(&(workspace, profile))
        .map(|cmd_execution_req_tx| {
            let execution_id = ExecutionId::new_rand();
            let cmd_execution_req = CmdExecutionReq {
                execution_id,
                ensure_cmd_args,
            };

            let cmd_execution_info = CmdExecutionInfo::new(execution_id, ensure_cmd_args);
            cmd_execution_infos.insert(execution_id, cmd_execution_info);

            cmd_execution_req_tx.send(cmd_execution_req).await;

            execution_id
        })
        .ok_or_else(|| {
            ServerFnError::<NoCustomError>::ServerError(
                format!("No `CmdExecutionQueue` for {workspace} {profile}.")
            )
        });

    Ok(execution_id)
}

#[component]
pub fn EnsureButton() -> impl IntoView {
    let ensure_cmd = leptos::create_action(
        |workspace: Workspace, profile: Profile, flow_id: FlowId| {
            let execution_id = execution_id.clone();
            async move { ensure_cmd(EnsureCmdParams { workspace, profile, flow_id }).await }
        },
    );
    let submitted = ensure_cmd.input(); // RwSignal<Option<String>>
    let pending = ensure_cmd.pending(); // ReadSignal<bool>
    let todo_id = ensure_cmd.value(); // RwSignal<Option<Uuid>>

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default(); // don't reload the page.
                ensure_cmd.dispatch();
            }
        >
            // Execution ID
            <button type="submit">"Deploy"</button>
        </form>
        // use our loading state
        <p>{move || pending().then("Loading...")}</p>
    }
}
```
