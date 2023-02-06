use peace_fmt::{async_trait, Presentable, Presenter};
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[cfg(feature = "output_colorized")]
use crate::output::CliColorize;
use crate::output::CliOutput;

/// Command line markdown presenter.
///
/// Formats `Presentable` data as markdown on the CLI.
#[derive(Debug)]
pub struct CliMdPresenter<'output, W> {
    output: &'output mut CliOutput<W>,
}

impl<'output, W> CliMdPresenter<'output, W>
where
    W: AsyncWrite + std::marker::Unpin,
{
    /// Returns a new `CliMdPresenter`.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write to.
    pub fn new(output: &'output mut CliOutput<W>) -> Self {
        Self { output }
    }

    #[cfg(feature = "output_colorized")]
    async fn colorize_maybe(
        &mut self,
        s: &str,
        style: &console::Style,
    ) -> Result<(), std::io::Error> {
        let colorize = self.output.colorize;
        let writer = &mut self.output.writer;

        if colorize == CliColorize::Colored {
            let s_colorized = style.apply_to(s);
            writer
                .write_all(format!("{s_colorized}").as_bytes())
                .await?;
        } else {
            writer.write_all(s.as_bytes()).await?;
        }

        Ok(())
    }

    fn number_column_count<I>(iterator: &I) -> usize
    where
        I: Iterator,
    {
        let (min, max_maybe) = iterator.size_hint();
        let n = max_maybe.unwrap_or(min);
        n.checked_ilog10()
            .and_then(|log10| usize::try_from(log10).ok())
            .unwrap_or(0)
            + 1
    }
}

#[async_trait(?Send)]
impl<'output, W> Presenter<'output> for CliMdPresenter<'output, W>
where
    W: AsyncWrite + std::marker::Unpin,
{
    type Error = std::io::Error;

    #[cfg(feature = "output_colorized")]
    async fn id(&mut self, id: &str) -> Result<(), Self::Error> {
        let style = console::Style::new().color256(75); // blue
        self.colorize_maybe(id, &style).await?;

        Ok(())
    }

    #[cfg(not(feature = "output_colorized"))]
    async fn id(&mut self, id: &str) -> Result<(), Self::Error> {
        self.output.writer.write_all(id.as_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "output_colorized")]
    async fn name(&mut self, name: &str) -> Result<(), Self::Error> {
        let style = console::Style::new().bold();
        self.colorize_maybe(name, &style).await?;

        Ok(())
    }

    #[cfg(not(feature = "output_colorized"))]
    async fn name(&mut self, name: &str) -> Result<(), Self::Error> {
        self.output.writer.write_all(name.as_bytes()).await?;
        Ok(())
    }

    async fn text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.output.writer.write_all(text.as_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "output_colorized")]
    async fn tag(&mut self, tag: &str) -> Result<(), Self::Error> {
        let style = &console::Style::new().color256(219).bold(); // purple
        self.colorize_maybe(format!("⦗{tag}⦘").as_str(), style)
            .await?;

        Ok(())
    }

    #[cfg(not(feature = "output_colorized"))]
    async fn tag(&mut self, tag: &str) -> Result<(), Self::Error> {
        self.output.writer.write_all(b"\xE2\xA6\x97").await?;
        self.output.writer.write_all(tag.as_bytes()).await?;
        self.output.writer.write_all(b"\xE2\xA6\x98").await?;

        Ok(())
    }

    #[cfg(feature = "output_colorized")]
    async fn code_inline(&mut self, code: &str) -> Result<(), Self::Error> {
        let style = &console::Style::new().color256(105); // pale blue
        self.colorize_maybe(format!("`{code}`").as_str(), style)
            .await?;
        Ok(())
    }

    #[cfg(not(feature = "output_colorized"))]
    async fn code_inline(&mut self, text: &str) -> Result<(), Self::Error> {
        self.output.writer.write_all(b"`").await?;
        self.output.writer.write_all(text.as_bytes()).await?;
        self.output.writer.write_all(b"`").await?;
        Ok(())
    }

    async fn list_numbered<'f, P, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + 'f,
        I: IntoIterator<Item = &'f P>,
    {
        let iterator = iter.into_iter();
        let number_column_count = Self::number_column_count(&iterator);
        for (index, entry) in iterator.enumerate() {
            let n = index + 1;
            self.output
                .writer
                .write_all(format!("{n:>number_column_count$}. ").as_bytes())
                .await?;
            entry.present(self).await?;
            self.output.writer.write_all(b"\n").await?;
        }

        Ok(())
    }

    async fn list_numbered_with<'f, P, I, T, F>(&mut self, iter: I, f: F) -> Result<(), Self::Error>
    where
        P: Presentable,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> P,
    {
        let iterator = iter.into_iter();
        let number_column_count = Self::number_column_count(&iterator);
        for (index, entry) in iterator.enumerate() {
            let n = index + 1;
            self.output
                .writer
                .write_all(format!("{n:>number_column_count$}. ").as_bytes())
                .await?;
            let presentable = f(entry);
            presentable.present(self).await?;
            self.output.writer.write_all(b"\n").await?;
        }

        Ok(())
    }

    async fn list_bulleted<'f, P, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + 'f,
        I: IntoIterator<Item = &'f P>,
    {
        for entry in iter.into_iter() {
            self.output.writer.write_all(b"* ").await?;
            entry.present(self).await?;
            self.output.writer.write_all(b"\n").await?;
        }

        Ok(())
    }

    async fn list_bulleted_with<'f, P, I, T, F>(&mut self, iter: I, f: F) -> Result<(), Self::Error>
    where
        P: Presentable,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> P,
    {
        for entry in iter.into_iter() {
            self.output.writer.write_all(b"* ").await?;
            let presentable = f(entry);
            presentable.present(self).await?;
            self.output.writer.write_all(b"\n").await?;
        }

        Ok(())
    }
}
