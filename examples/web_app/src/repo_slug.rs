use std::{fmt, str::FromStr};

#[cfg(feature = "error_reporting")]
use peace::miette::SourceSpan;
use serde::{Deserialize, Serialize};

use crate::RepoSlugError;

const FORWARD_SLASH: char = '/';

/// A repository slug, e.g. `username/repo_name`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RepoSlug {
    /// Account repo_name the app resides in.
    account: String,
    /// Name of the repository.
    repo_name: String,
}

impl RepoSlug {
    fn account_validate(input: &str, account_segment: &str) -> Result<(), RepoSlugError> {
        if Self::is_valid_segment(account_segment) {
            Ok(())
        } else {
            #[cfg(feature = "error_reporting")]
            let span = {
                let start = 0;
                let length = account_segment.len();
                SourceSpan::from((start, length))
            };
            Err(RepoSlugError::AccountInvalid {
                input: input.to_string(),
                account: account_segment.to_string(),
                #[cfg(feature = "error_reporting")]
                span,
            })
        }
    }

    fn repo_name_validate(
        input: &str,
        repo_name_segment: &str,
        #[cfg(feature = "error_reporting")] segment_byte_offset: usize,
    ) -> Result<(), RepoSlugError> {
        if Self::is_valid_segment(repo_name_segment) {
            Ok(())
        } else {
            #[cfg(feature = "error_reporting")]
            let span = {
                let start = segment_byte_offset;
                let length = repo_name_segment.len();
                SourceSpan::from((start, length))
            };
            Err(RepoSlugError::RepoNameInvalid {
                input: input.to_string(),
                repo_name: repo_name_segment.to_string(),
                #[cfg(feature = "error_reporting")]
                span,
            })
        }
    }

    /// Returns whether the provided `&str` is a valid account or repo_name.
    ///
    /// The ID must begin with an ascii alphabetic character, and subsequent
    /// characters must be either a letter, number, hyphen, or underscore.
    fn is_valid_segment(segment: &str) -> bool {
        let mut chars = segment.chars();
        let first_char = chars.next();
        let first_char_valid = first_char.map(|c| c.is_ascii_alphabetic()).unwrap_or(false);
        let remainder_chars_valid =
            chars.all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '-' || c == '_');

        first_char_valid && remainder_chars_valid
    }
}

impl fmt::Display for RepoSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.account, self.repo_name)
    }
}

impl FromStr for RepoSlug {
    type Err = RepoSlugError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut segments = input.splitn(3, FORWARD_SLASH);
        let account = segments.next();
        let repo_name = segments.next();
        let remainder = segments.next();

        if let (Some(account), Some(repo_name), None) = (account, repo_name, remainder) {
            Self::account_validate(input, account)?;
            #[cfg(not(feature = "error_reporting"))]
            Self::repo_name_validate(input, repo_name)?;
            #[cfg(feature = "error_reporting")]
            Self::repo_name_validate(input, repo_name, account.len() + FORWARD_SLASH.len_utf8())?;

            let account = account.to_string();
            let repo_name = repo_name.to_string();
            Ok(RepoSlug { account, repo_name })
        } else {
            Err(RepoSlugError::SegmentCountInvalid {
                input: input.to_string(),
            })
        }
    }
}
