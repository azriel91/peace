use crate::{Presentable, Presenter};

/// A list of presentable items.
///
/// This isn't constructed directly, but through [`Presenter::list`].
///
/// [`Presenter::list`]: crate::Presenter::list
pub struct PresentableList<'output, 'presenter> {
    presenter: &'presenter mut dyn Presenter<'output>,
    result: crate::Result,
}

impl<'output, 'presenter> PresentableList<'output, 'presenter>
where
    'output: 'presenter,
{
    /// Adds an entry to the presented list.
    pub fn entry<P>(&mut self, entry: &P) -> &mut Self
    where
        P: Presentable + 'output,
    {
        self.result = self.result.and_then(|()| entry.present(self.presenter));
        self
    }

    /// Adds multiple entries to the presented list.
    pub fn entries<P, I>(&mut self, entries: I) -> &mut Self
    where
        P: Presentable + 'output,
        I: IntoIterator<Item = &'output P>,
    {
        entries.into_iter().for_each(|entry| {
            self.entry(entry);
        });
        self
    }

    /// Finishes presenting and returns any errors encountered.
    pub fn finish(self) -> crate::Result {
        self.result
    }
}
