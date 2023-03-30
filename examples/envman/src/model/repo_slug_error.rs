#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

/// Error parsing a [`RepoSlug`].
///
/// [`RepoSlug`]: crate::RepoSlug
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum RepoSlugError {
    /// Account ID provided is invalid.
    #[error("Account ID provided is invalid: `{}`.", account)]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(envman::account_invalid),
            help(
                "Account IDs must begin with an ascii letter, \
                and only contain letters, numbers, hyphens, and underscores."
            )
        )
    )]
    AccountInvalid {
        /// Repository slug input string.
        #[cfg_attr(feature = "error_reporting", source_code)]
        input: String,
        /// The account segment of the string.
        account: String,
        /// Span of the account segment in the slug.
        #[cfg(feature = "error_reporting")]
        #[label]
        span: SourceSpan,
    },

    /// Repository name provided is invalid.
    #[error("Repository name provided is invalid: `{}`.", repo_name)]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(envman::repo_name_invalid),
            help(
                "Repository names must begin with an ascii letter, \
                and only contain letters, numbers, hyphens, and underscores."
            )
        )
    )]
    RepoNameInvalid {
        /// Repository slug input string.
        #[cfg_attr(feature = "error_reporting", source_code)]
        input: String,
        /// The repository name segment of the string.
        repo_name: String,
        /// Span of the repository name segment in the slug.
        #[cfg(feature = "error_reporting")]
        #[label]
        span: SourceSpan,
    },

    /// Repository slug has invalid number of segments.
    #[error("Repository slug has invalid number of segments: `{}`.", input)]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(envman::segment_count_invalid),
            help("Repository slug must contain exactly one slash.")
        )
    )]
    SegmentCountInvalid {
        /// Repository slug input string.
        input: String,
    },
}
