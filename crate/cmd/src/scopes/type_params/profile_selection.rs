use std::fmt;

use peace_core::Profile;

/// A `Profile` is not yet selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileNotSelected;

/// A `Profile` is selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileSelected(pub(crate) Profile);

/// The `Profile` will be read from workspace params using the provided key
/// during command context build.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileFromWorkspaceParam<'key, WorkspaceParamsK>(pub(crate) &'key WorkspaceParamsK);

/// Filter function for `MultiProfile` scopes.
pub struct ProfileFilterFn(pub(crate) Box<dyn Fn(&Profile) -> bool>);

impl fmt::Debug for ProfileFilterFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ProfileFilterFn")
            .field(&"Box<dyn Fn(&Profile) -> bool")
            .finish()
    }
}
