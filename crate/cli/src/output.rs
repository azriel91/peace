pub use self::{
    cli_colorize::CliColorize, cli_colorize_opt::CliColorizeOpt,
    cli_colorize_parse_error::CliColorizeOptParseError, cli_md_presenter::CliMdPresenter,
    cli_output::CliOutput, cli_output_builder::CliOutputBuilder,
    cli_output_target::CliOutputTarget,
};

mod cli_colorize;
mod cli_colorize_opt;
mod cli_colorize_parse_error;
mod cli_md_presenter;
mod cli_output;
mod cli_output_builder;
mod cli_output_target;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        pub use self::{
            cli_progress_format::CliProgressFormat,
            cli_progress_format_opt::CliProgressFormatOpt,
            cli_progress_format_opt_parse_error::CliProgressFormatOptParseError,
        };

        mod cli_progress_format;
        mod cli_progress_format_opt;
        mod cli_progress_format_opt_parse_error;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "error_reporting")] {
        pub(crate) use self::report_handler::ReportHandler;

        mod report_handler;
    }
}
