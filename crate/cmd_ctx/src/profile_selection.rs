use own::OwnedOrRef;
use peace_profile_model::Profile;

/// How the `CmdCtx` knows which `Profile` to use.
///
/// This is applicable to single flow `CmdCtx`s: [`CmdCtxSpsf`] and
/// [`CmdCtxMpsf`]
///
/// [`CmdCtxSpsf`]: crate::CmdCtxSpsf
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileSelection<'f, WorkspaceParamsK> {
    /// A `Profile` is selected.
    ProfileSelected(Profile),

    /// The `Profile` will be read from workspace params using the provided key
    /// during command context build.
    ProfileFromWorkspaceParam(OwnedOrRef<'f, WorkspaceParamsK>),
}
