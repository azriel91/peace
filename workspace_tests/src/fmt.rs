use peace::{
    cli::output::{CliColorizeOpt, CliOutput, CliOutputBuilder},
    cli_model::OutputFormat,
};

mod either;
mod presentable;

/// Returns a new `CliOutput` with `OutputFormat::Text`.
fn cli_output(colorize: CliColorizeOpt) -> CliOutput<Vec<u8>> {
    CliOutputBuilder::new_with_writer(Vec::new())
        .with_outcome_format(OutputFormat::Text)
        .with_colorize(colorize)
        .build()
}
