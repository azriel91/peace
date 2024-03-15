cfg_if::cfg_if! {
    if #[cfg(feature = "cli")] {
        // CLI app

        use envman::model::EnvManError;

        mod main_cli;

        #[cfg(not(feature = "error_reporting"))]
        pub fn main() -> Result<(), EnvManError> {
            main_cli::run()
        }

        #[cfg(feature = "error_reporting")]
        pub fn main() -> peace::miette::Result<(), peace::miette::Report> {
            // Important to return `peace::miette::Report` instead of calling
            // `IntoDiagnostic::intoDiagnostic` on the `Error`, as that does not present the
            // diagnostic contextual information to the user.
            //
            // See <https://docs.rs/miette/latest/miette/trait.IntoDiagnostic.html#warning>.

            // The explicit mapping for `PeaceRtError` appears to be necessary to display
            // the diagnostic information. i.e. `miette` does not automatically delegate to
            // the #[diagnostic_source].
            //
            // This is fixed by <https://github.com/zkat/miette/pull/170>.

            main_cli::run().map_err(|envman_error| match envman_error {
                EnvManError::PeaceItemFileDownload(err) => peace::miette::Report::from(err),
                EnvManError::PeaceRtError(err) => peace::miette::Report::from(err),
                other => peace::miette::Report::from(other),
            })
        }
    } else if #[cfg(feature = "ssr")] {
        // web server
        use envman::model::EnvManError;

        #[cfg(not(feature = "error_reporting"))]
        pub fn main() -> Result<(), EnvManError> {
            use envman::flows::EnvDeployFlow;
            use peace::webi::output::WebiOutput;

            let runtime = tokio::runtime::Builder::new_multi_thread()
                .thread_name("main")
                .thread_stack_size(3 * 1024 * 1024)
                .enable_io()
                .enable_time()
                .build()
                .map_err(EnvManError::TokioRuntimeInit)?;

            runtime.block_on(async move {
                let flow = EnvDeployFlow::flow().await?;
                let flow_spec_info = flow.flow_spec_info();

                let webi_output = WebiOutput::new(None, flow_spec_info);

                webi_output.start().await.map_err(EnvManError::from)
            })
        }

        #[cfg(feature = "error_reporting")]
        pub fn main() -> peace::miette::Result<(), peace::miette::Report> {
            // Important to return `peace::miette::Report` instead of calling
            // `IntoDiagnostic::intoDiagnostic` on the `Error`, as that does not present the
            // diagnostic contextual information to the user.
            //
            // See <https://docs.rs/miette/latest/miette/trait.IntoDiagnostic.html#warning>.

            // The explicit mapping for `PeaceRtError` appears to be necessary to display
            // the diagnostic information. i.e. `miette` does not automatically delegate to
            // the #[diagnostic_source].
            //
            // This is fixed by <https://github.com/zkat/miette/pull/170>.

            let runtime = tokio::runtime::Builder::new_multi_thread()
                .thread_name("main")
                .thread_stack_size(3 * 1024 * 1024)
                .enable_io()
                .enable_time()
                .build()
                .map_err(EnvManError::TokioRuntimeInit)?;

            let webi_output = WebiOutput::new(None);

            runtime
                .block_on(async move {
                    webi_output.start().await.map_err(EnvManError::from)
                })
                .map_err(peace::miette::Report::from)
        }
    } else if #[cfg(feature = "csr")] {
        // web client logic
        fn main() {}
    } else {
        compile_error!(
            r#"Please enable one of the following features:

* "cli"
* "ssr"
* "csr"
"#);
    }
}
