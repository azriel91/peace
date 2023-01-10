pub use self::{cli_output::CliOutput, cli_output_builder::CliOutputBuilder};

mod cli_output;
mod cli_output_builder;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_colorized")] {
        pub use self::{
            cli_colorize::{CliColorize, CliColorizeUsed},
            cli_colorize_parse_error::CliColorizeParseError,
        };

        mod cli_colorize;
        mod cli_colorize_parse_error;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        pub use self::{
            cli_progress_format::{CliProgressFormat, CliProgressFormatUsed},
            cli_progress_format_parse_error::CliProgressFormatParseError,
        };

        mod cli_progress_format;
        mod cli_progress_format_parse_error;
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "output_colorized", feature = "output_progress"))] {
        pub use self::cli_output_target::CliOutputTarget;

        mod cli_output_target;
    }
}
