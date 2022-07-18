use std::fmt::Debug;

use fn_graph::{DataAccess, DataAccessDyn, DataBorrow, Resources, R, W};

/// Defines the logic to instantiate and retrieve runtime data.
///
/// # Note for API Consumers
///
/// This trait is implemented by using the [`Data` derive].
///
/// [`Data` derive]: peace_data_derive::Data
pub trait Data<'borrow>: DataAccess + DataAccessDyn + Send {
    /// Borrows each of `Self`'s fields from the provided [`Resources`].
    ///
    /// # Parameters
    ///
    /// * `resources`: `Any` map to borrow the data from.
    fn borrow(resources: &'borrow Resources) -> Self;
}

impl<'borrow, T> Data<'borrow> for R<'borrow, T>
where
    T: Debug + Send + Sync + 'static,
{
    fn borrow(resources: &'borrow Resources) -> Self {
        <Self as DataBorrow>::borrow(resources)
    }
}

impl<'borrow, T> Data<'borrow> for W<'borrow, T>
where
    T: Debug + Send + Sync + 'static,
{
    fn borrow(resources: &'borrow Resources) -> Self {
        <Self as DataBorrow>::borrow(resources)
    }
}
