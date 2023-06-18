use peace_fmt::{async_trait, presentable::HeadingLevel, Presentable, Presenter};
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::output::{CliColorize, CliOutput};

/// Command line markdown presenter.
///
/// Formats `Presentable` data as markdown on the CLI.
#[derive(Debug)]
pub struct CliMdPresenter<'output, W> {
    /// The CLI output to write to.
    output: &'output mut CliOutput<W>,
    /// Whether to render text in ANSI bold.
    cli_bold: CliBold,
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
        Self {
            output,
            cli_bold: CliBold::default(),
        }
    }

    async fn colorize_maybe(
        &mut self,
        s: &str,
        style: &console::Style,
    ) -> Result<(), std::io::Error> {
        let colorize = self.output.colorize;
        let writer = &mut self.output.writer;

        if colorize == CliColorize::Colored {
            let s_colorized = style.apply_to(s);
            if self.cli_bold.is_bold() {
                let s_colorized_bolded = s_colorized.bold();
                writer
                    .write_all(format!("{s_colorized_bolded}").as_bytes())
                    .await?;
            } else {
                writer
                    .write_all(format!("{s_colorized}").as_bytes())
                    .await?;
            }
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

    /// Pedantic: Don't highlight surrounding spaces with white.
    async fn colorize_list_number(
        &mut self,
        style: &console::Style,
        number_column_count: usize,
        list_number: usize,
    ) -> Result<(), std::io::Error> {
        let colorize = self.output.colorize;
        let writer = &mut self.output.writer;

        let list_number_digits = list_number
            .checked_ilog10()
            .and_then(|log10| usize::try_from(log10).ok())
            .unwrap_or(0)
            + 1;
        let leading_space_count: usize = number_column_count.saturating_sub(list_number_digits);

        if colorize == CliColorize::Colored {
            let list_number_colorized = style.apply_to(format!("{list_number}."));
            let leading_spaces = " ".repeat(leading_space_count);
            writer
                .write_all(format!("{leading_spaces}{list_number_colorized} ").as_bytes())
                .await?;
        } else {
            let list_number_padded = format!("{list_number:>number_column_count$}. ");
            writer.write_all(list_number_padded.as_bytes()).await?;
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl<'output, W> Presenter<'output> for CliMdPresenter<'output, W>
where
    W: AsyncWrite + std::marker::Unpin,
{
    type Error = std::io::Error;

    async fn heading<P>(
        &mut self,
        heading_level: HeadingLevel,
        presentable: &P,
    ) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized,
    {
        let hash_style = &console::Style::new().bold().color256(243); // grey;
        let leading_hashes = match heading_level {
            HeadingLevel::Level1 => "#",
            HeadingLevel::Level2 => "##",
            HeadingLevel::Level3 => "###",
            HeadingLevel::Level4 => "####",
            HeadingLevel::Level5 => "#####",
            HeadingLevel::Level6 => "######",
        };

        self.cli_bold.increment();
        self.colorize_maybe(leading_hashes, hash_style).await?;
        self.output.writer.write_all(b" ").await?;
        presentable.present(self).await?;

        self.cli_bold.decrement();

        self.output.writer.write_all(b"\n\n").await?;

        Ok(())
    }

    async fn id(&mut self, id: &str) -> Result<(), Self::Error> {
        let style = console::Style::new().color256(75); // blue
        self.colorize_maybe(id, &style).await?;

        Ok(())
    }

    async fn name(&mut self, name: &str) -> Result<(), Self::Error> {
        let style = console::Style::new().bold();
        self.colorize_maybe(format!("**{name}**").as_str(), &style)
            .await?;

        Ok(())
    }

    async fn text(&mut self, text: &str) -> Result<(), Self::Error> {
        let style = console::Style::new();
        self.colorize_maybe(text, &style).await?;

        Ok(())
    }

    async fn bold<P>(&mut self, presentable: &P) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized,
    {
        self.cli_bold.increment();
        self.output.writer.write_all(b"**").await?;
        presentable.present(self).await?;
        self.output.writer.write_all(b"**").await?;
        self.cli_bold.decrement();

        Ok(())
    }

    async fn tag(&mut self, tag: &str) -> Result<(), Self::Error> {
        let style = &console::Style::new().color256(219).bold(); // purple
        self.colorize_maybe(format!("⦗{tag}⦘").as_str(), style)
            .await?;

        Ok(())
    }

    async fn code_inline(&mut self, code: &str) -> Result<(), Self::Error> {
        let style = &console::Style::new().color256(75); // pale blue
        self.colorize_maybe(format!("`{code}`").as_str(), style)
            .await?;
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
            let list_number = index + 1;

            let style = &console::Style::new().color256(15); // white
            self.colorize_list_number(style, number_column_count, list_number)
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
            let list_number = index + 1;

            let style = &console::Style::new().color256(15); // white
            self.colorize_list_number(style, number_column_count, list_number)
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
            let style = &console::Style::new().color256(15); // white
            self.colorize_maybe("*", style).await?;
            self.output.writer.write_all(b" ").await?;

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
            let style = &console::Style::new().color256(15); // white
            self.colorize_maybe("*", style).await?;
            self.output.writer.write_all(b" ").await?;

            let presentable = f(entry);
            presentable.present(self).await?;
            self.output.writer.write_all(b"\n").await?;
        }

        Ok(())
    }
}

/// Whether to render text in ANSI bold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CliBold(u32);

impl CliBold {
    /// Returns whether the CLI text should be rendered as bold.
    fn is_bold(self) -> bool {
        self.0 > 0
    }

    fn increment(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
}
