use syn::{parse_quote, Path};

use crate::cmd::{FlowCount, ProfileCount};

/// Scope to generate the `CmdCtxBuilder` impl for.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// A command that works with multiple profiles, not scoped to a flow.
    MultiProfileNoFlow,
    /// A command that works with multiple profiles, and a single flow.
    MultiProfileSingleFlow,
    /// A command that only works with workspace parameters.
    NoProfileNoFlow,
    /// A command that works with a single profile, not scoped to a flow.
    SingleProfileNoFlow,
    /// A command that works with one profile and one flow.
    SingleProfileSingleFlow,
}

impl Scope {
    /// Returns the scope's type for the built command.
    pub fn type_path(self) -> Path {
        match self {
            Scope::MultiProfileNoFlow => parse_quote!(crate::scopes::MultiProfileNoFlow),
            Scope::MultiProfileSingleFlow => {
                parse_quote!(crate::scopes::MultiProfileSingleFlow)
            }
            Scope::NoProfileNoFlow => parse_quote!(crate::scopes::NoProfileNoFlow),
            Scope::SingleProfileNoFlow => parse_quote!(crate::scopes::SingleProfileNoFlow),
            Scope::SingleProfileSingleFlow => {
                parse_quote!(crate::scopes::SingleProfileSingleFlow)
            }
        }
    }

    /// Returns this scope as a snake case `&str`.
    pub fn as_str(self) -> &'static str {
        match self {
            Scope::MultiProfileNoFlow => "multi_profile_no_flow",
            Scope::MultiProfileSingleFlow => "multi_profile_single_flow",
            Scope::NoProfileNoFlow => "no_profile_no_flow",
            Scope::SingleProfileNoFlow => "single_profile_no_flow",
            Scope::SingleProfileSingleFlow => "single_profile_single_flow",
        }
    }

    /// Returns the number of profiles accessed by this scope.
    pub fn profile_count(self) -> ProfileCount {
        match self {
            Scope::MultiProfileNoFlow => ProfileCount::Multiple,
            Scope::MultiProfileSingleFlow => ProfileCount::Multiple,
            Scope::NoProfileNoFlow => ProfileCount::None,
            Scope::SingleProfileNoFlow => ProfileCount::One,
            Scope::SingleProfileSingleFlow => ProfileCount::One,
        }
    }

    /// Returns the number of flows accessed by this scope.
    pub fn flow_count(self) -> FlowCount {
        match self {
            Scope::MultiProfileNoFlow => FlowCount::None,
            Scope::MultiProfileSingleFlow => FlowCount::One,
            Scope::NoProfileNoFlow => FlowCount::None,
            Scope::SingleProfileNoFlow => FlowCount::None,
            Scope::SingleProfileSingleFlow => FlowCount::One,
        }
    }

    /// Returns whether this scope supports accessing profile params.
    pub fn profile_params_supported(self) -> bool {
        match self.profile_count() {
            ProfileCount::None => false,
            ProfileCount::One | ProfileCount::Multiple => true,
        }
    }

    /// Returns whether this scope supports accessing flow params.
    pub fn flow_params_supported(self) -> bool {
        match self.flow_count() {
            FlowCount::None => false,
            FlowCount::One => true,
        }
    }
}
