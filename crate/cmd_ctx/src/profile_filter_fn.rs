use std::fmt;

use peace_profile_model::Profile;

/// Filter function for `MultiProfile` scopes.
pub struct ProfileFilterFn(pub(crate) Box<dyn Fn(&Profile) -> bool>);

impl ProfileFilterFn {
    /// Returns whether the profile passes this filter.
    pub fn call(&self, profile: &Profile) -> bool {
        (self.0)(profile)
    }
}

impl fmt::Debug for ProfileFilterFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ProfileFilterFn")
            .field(&"Box<dyn Fn(&Profile) -> bool")
            .finish()
    }
}
