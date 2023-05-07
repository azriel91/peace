# Empathetic Code Design

> *How clearly can we communicate through code?*

```rust ,ignore
let mut cmd_context = CmdContext::builder()
    .with_flow(flow!(Step1, Step2, Step3))
    .with_item_spec_params([
        Step1Params::spec()
            .with_a(A(1))
            .build(),
        Step2Params::spec()
            .with_b(B(2))
            .build(),
        Step3Params::spec()
            .with_c_from_map(|a: &A, b: &B| C(a + b))
            .build(),
    ])
    .with_output(&mut cli_output)
    .build();

StatusCmd::exec(&mut cmd_context).await?;
DeployCmd::exec(&mut cmd_context).await?;
```
