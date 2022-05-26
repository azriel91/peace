use std::fmt::Debug;

use fn_graph::{DataAccessDyn, DataBorrow, Resources, R, W};

use crate::DataInit;

/// Defines the logic to instantiate and retrieve runtime data.
///
/// # Note for API Consumers
///
/// This trait is implemented by using the [`Data` derive].
///
/// [`Data` derive]: peace_data_derive::Data
pub trait Data<'borrow>: DataAccessDyn {
    /// Instantiates each of `Self`'s fields in the provided [`Resources`].
    ///
    /// This should be a sensible default.
    ///
    /// # Parameters
    ///
    /// * `resources`: `Any` map to insert the instance of each field into.
    fn init(resources: &mut Resources);

    /// Borrows each of `Self`'s fields from the provided [`Resources`].
    ///
    /// # Parameters
    ///
    /// * `resources`: `Any` map to borrow the data from.
    fn borrow(resources: &'borrow Resources) -> Self;
}

impl<'borrow, T> Data<'borrow> for R<'borrow, T>
where
    T: DataInit + Debug + Send + Sync + 'static,
{
    fn init(resources: &mut Resources) {
        <T as DataInit>::init(resources)
    }

    fn borrow(resources: &'borrow Resources) -> Self {
        <Self as DataBorrow>::borrow(resources)
    }
}

impl<'borrow, T> Data<'borrow> for W<'borrow, T>
where
    T: DataInit + Debug + Send + Sync + 'static,
{
    fn init(resources: &mut Resources) {
        <T as DataInit>::init(resources)
    }

    fn borrow(resources: &'borrow Resources) -> Self {
        <Self as DataBorrow>::borrow(resources)
    }
}
