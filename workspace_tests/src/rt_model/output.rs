mod cli_colorize_opt;
mod cli_colorize_opt_parse_error;
mod cli_md_presenter;
mod cli_output;
mod cli_output_builder;
mod cli_output_target;
mod output_format;
mod output_format_parse_error;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        mod cli_progress_format_opt;
        mod cli_progress_format_opt_parse_error;
    }
}
