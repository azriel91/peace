use peace_flow_model::FlowId;
use peace_profile_model::Profile;

#[derive(Debug, thiserror::Error)]
#[error("Failed to deserialize params specs for `{profile}/{flow_id}`.")]
#[cfg_attr(
    feature = "error_reporting",
    derive(miette::Diagnostic),
    diagnostic(
        code(peace_rt_model::params_specs_deserialize),
        help(
            "Make sure that all commands using the `{flow_id}` flow, also use the same item graph.\n\
            This is because all Items are used to deserialize state.\n\
            \n\
            If the item graph is different, it may make sense to use a different flow ID."
        )
    )
)]
pub struct ParamsSpecsDeserializeError {
    /// Profile of the flow.
    pub profile: Profile,
    /// Flow ID whose params specs are being deserialized.
    pub flow_id: FlowId,
    /// Source text to be deserialized.
    #[cfg(feature = "error_reporting")]
    #[source_code]
    pub params_specs_file_source: miette::NamedSource<String>,
    /// Offset within the source text that the error occurred.
    #[cfg(feature = "error_reporting")]
    #[label("{}", error_message)]
    pub error_span: Option<miette::SourceOffset>,
    /// Message explaining the error.
    #[cfg(feature = "error_reporting")]
    pub error_message: String,
    /// Offset within the source text surrounding the error.
    #[cfg(feature = "error_reporting")]
    #[label]
    pub context_span: Option<miette::SourceOffset>,
    /// Underlying error.
    #[source]
    pub error: serde_yaml::Error,
}

#[cfg(feature = "error_reporting")]
impl<'b> std::borrow::Borrow<dyn miette::Diagnostic + 'b> for Box<ParamsSpecsDeserializeError> {
    fn borrow<'s>(&'s self) -> &'s (dyn miette::Diagnostic + 'b) {
        self.as_ref()
    }
}

#[cfg(feature = "error_reporting")]
impl miette::Diagnostic for Box<ParamsSpecsDeserializeError> {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.as_ref().code()
    }

    fn severity(&self) -> Option<miette::Severity> {
        self.as_ref().severity()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.as_ref().help()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.as_ref().url()
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        self.as_ref().source_code()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.as_ref().labels()
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn miette::Diagnostic> + 'a>> {
        self.as_ref().related()
    }

    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        self.as_ref().diagnostic_source()
    }
}
