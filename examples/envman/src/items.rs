//! Umbrella module for items.

pub mod peace_aws_iam_policy;
pub mod peace_aws_iam_role;
pub mod peace_aws_instance_profile;
pub mod peace_aws_s3_bucket;
pub mod peace_aws_s3_object;

// Hack: need to find a better way to do this.
#[cfg(feature = "error_reporting")]
macro_rules! aws_error_desc {
    ($error:expr) => {{
        use aws_sdk_iam::error::ProvideErrorMetadata;

        let (error_code, desc) = match $error {
            aws_sdk_iam::error::SdkError::ServiceError(service_error) => {
                let error_code = service_error.err().code().map(|s| s.to_string());
                let desc = service_error.err().message().map(|s| s.to_string());

                (error_code, desc)
            }
            _ => {
                // most variants do not `impl Error`, but we can
                // access the underlying error through
                // `sdk_error.source()`.

                let mut source = Option::<&dyn std::error::Error>::Some($error);
                while let Some(source_next) = source.and_then(std::error::Error::source) {
                    source = Some(source_next);
                }

                let error_code = None;
                let desc = source.map(|source| format!("{source}"));

                (error_code, desc)
            }
        };

        let mut aws_desc = String::new();
        let mut desc_span_start = 0;
        let mut desc_len = 0;
        if let Some(error_code) = error_code {
            aws_desc.push_str(&format!("{error_code}: "));
            desc_span_start = aws_desc.len();
        }
        if let Some(desc) = desc {
            aws_desc.push_str(&desc);
            desc_len = desc.len();
        }
        let aws_desc_span = peace::miette::SourceSpan::from((desc_span_start, desc_len));

        (aws_desc, aws_desc_span)
    }};
}

#[cfg(feature = "error_reporting")]
pub(crate) use aws_error_desc;
