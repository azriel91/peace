use crate::{Presentable, Presenter};

/// A list of presentable items.
///
/// This isn't constructed directly, but through [`Presenter::list`].
///
/// [`Presenter::list`]: crate::Presenter::list
pub struct PresentableList<'output, 'presenter, PR>
where
    PR: Presenter<'output>,
{
    presenter: &'presenter mut PR,
    result: Result<(), PR::Error>,
}

impl<'output, 'presenter, PR> PresentableList<'output, 'presenter, PR>
where
    PR: Presenter<'output>,
    'output: 'presenter,
{
    pub fn new(presenter: &'presenter mut PR) -> Self {
        Self {
            presenter,
            result: Ok(()),
        }
    }

    /// Adds an entry to the presented list.
    pub async fn entry<P>(mut self, entry: &P) -> PresentableList<'output, 'presenter, PR>
    where
        P: Presentable + 'presenter,
    {
        if self.result.is_ok() {
            self.result = entry.present(self.presenter).await;
        }
        self
    }

    /// Adds multiple entries to the presented list.
    pub async fn entries<P, I>(mut self, entries: I) -> PresentableList<'output, 'presenter, PR>
    where
        P: Presentable + 'presenter,
        I: IntoIterator<Item = &'presenter P>,
    {
        if self.result.is_ok() {
            // Much simpler code than the functional alternative.
            //
            // Passing `mut self` through `try_for_each` doesn't work, and through
            // `try_fold` needs a lot of code wrangling.
            for entry in entries.into_iter() {
                self.result = entry.present(self.presenter).await;

                if self.result.is_err() {
                    break;
                }
            }
        }
        self
    }

    /// Finishes presenting and returns any errors encountered.
    pub fn finish(self) -> Result<(), PR::Error> {
        self.result
    }
}
