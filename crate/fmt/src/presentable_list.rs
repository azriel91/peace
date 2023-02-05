use futures::stream::{self, TryStreamExt};

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
    pub async fn entries<P, I>(self, entries: I) -> PresentableList<'output, 'presenter, PR>
    where
        P: Presentable + 'presenter,
        I: IntoIterator<Item = &'presenter P>,
    {
        if self.result.is_ok() {
            // Hack: async and `&mut self` don't go well together.
            //
            // This essentially goes:
            //
            // ```rust
            // entries
            //     .into_iter()
            //     .for_each(|entry| entry.present(self.presenter))
            //     .await?;
            // ```
            //
            // But because of compilation requirements, we:
            //
            // * have ownership of `self`
            // * pass it through `try_fold`
            // * extract `self` back out
            // * return `self`
            let (Ok(self_) | Err(self_)) = stream::iter(
                entries
                    .into_iter()
                    .map(Result::<_, PresentableList<'output, 'presenter, PR>>::Ok),
            )
            .try_fold(self, |mut self_, entry| async {
                self_.result = entry.present(self_.presenter).await;

                if self_.result.is_ok() {
                    Ok(self_)
                } else {
                    Err(self_)
                }
            })
            .await;

            self_
        } else {
            self
        }
    }

    /// Finishes presenting and returns any errors encountered.
    pub fn finish(self) -> Result<(), PR::Error> {
        self.result
    }
}
