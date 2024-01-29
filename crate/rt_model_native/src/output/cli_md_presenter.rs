use console::Style;
use futures::stream::{self, TryStreamExt};
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

    async fn list_aligned_with<'f, P0, P1, I, T, F>(
        &mut self,
        list_type: ListType,
        iter: I,
        f: F,
    ) -> Result<(), std::io::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> &'f (P0, P1),
    {
        let iterator = iter.into_iter();
        let number_column_count = Self::number_column_count(&iterator);

        let mut buffer = Vec::<u8>::with_capacity(256);
        let mut cli_output_in_memory = CliOutput::new_with_writer(&mut buffer);
        let mut width_buffer_presenter = CliMdPresenter::new(&mut cli_output_in_memory);

        // Render the first presentable of all items, so we can determine the maximum
        // width.
        let (entries, max_width, _width_buffer_presenter, _f) =
            stream::iter(iterator.map(Result::<_, std::io::Error>::Ok))
                .try_fold(
                    (Vec::new(), None, &mut width_buffer_presenter, f),
                    |(mut entries, mut max_width, width_buffer_presenter, f), entry| async move {
                        let (presentable_0, presentable_1) = f(entry);

                        presentable_0.present(width_buffer_presenter).await?;
                        let rendered_0 = &width_buffer_presenter.output.writer;
                        let rendered_0_lossy = String::from_utf8_lossy(rendered_0);
                        let width = console::measure_text_width(&rendered_0_lossy);

                        width_buffer_presenter.output.writer.clear();

                        if let Some(current_max) = max_width {
                            if current_max < width {
                                max_width = Some(width);
                            }
                        } else {
                            max_width = Some(width);
                        }

                        entries.push(((presentable_0, width), presentable_1));

                        Ok((entries, max_width, width_buffer_presenter, f))
                    },
                )
                .await?;

        let style_white = &console::Style::new().color256(15); // white
        let max_width = max_width.unwrap_or(0);
        let padding_bytes = " ".repeat(max_width);

        match list_type {
            ListType::Numbered => {
                for (index, ((presentable_0, presentable_0_width), presentable_1)) in
                    entries.into_iter().enumerate()
                {
                    self.list_number_write(style_white, index, number_column_count)
                        .await?;

                    self.list_aligned_item_write(
                        max_width,
                        &padding_bytes,
                        presentable_0,
                        presentable_0_width,
                        presentable_1,
                    )
                    .await?;
                }
            }
            ListType::Bulleted => {
                for ((presentable_0, presentable_0_width), presentable_1) in entries.into_iter() {
                    self.list_bullet_write(style_white).await?;

                    self.list_aligned_item_write(
                        max_width,
                        &padding_bytes,
                        presentable_0,
                        presentable_0_width,
                        presentable_1,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    async fn list_aligned_item_write<P0, P1>(
        &mut self,
        max_width: usize,
        padding_bytes: &str,
        presentable_0: &P0,
        presentable_0_width: usize,
        presentable_1: &P1,
    ) -> Result<(), std::io::Error>
    where
        P0: Presentable,
        P1: Presentable,
    {
        presentable_0.present(self).await?;
        let padding = max_width.saturating_sub(presentable_0_width) + 1;
        self.output
            .writer
            .write_all(padding_bytes[0..padding].as_bytes())
            .await?;
        presentable_1.present(self).await?;
        self.output.writer.write_all(b"\n").await?;
        Ok(())
    }

    async fn list_bullet_write(self: &mut Self, style_white: &Style) -> Result<(), std::io::Error> {
        self.colorize_maybe("*", style_white).await?;
        self.output.writer.write_all(b" ").await?;

        Ok(())
    }

    async fn list_number_write(
        self: &mut Self,
        style_white: &Style,
        index: usize,
        number_column_count: usize,
    ) -> Result<(), std::io::Error> {
        let list_number = index + 1;
        self.colorize_list_number(style_white, number_column_count, list_number)
            .await?;

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
        self.list_numbered_with(iter, std::convert::identity).await
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

    async fn list_numbered_aligned<'f, P0, P1, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = &'f (P0, P1)>,
    {
        self.list_numbered_aligned_with(iter, std::convert::identity)
            .await
    }

    async fn list_numbered_aligned_with<'f, P0, P1, I, T, F>(
        &mut self,
        iter: I,
        f: F,
    ) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> &'f (P0, P1),
    {
        self.list_aligned_with(ListType::Numbered, iter, f).await
    }

    async fn list_bulleted<'f, P, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + 'f,
        I: IntoIterator<Item = &'f P>,
    {
        self.list_bulleted_with(iter, std::convert::identity).await
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

    async fn list_bulleted_aligned<'f, P0, P1, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = &'f (P0, P1)>,
    {
        self.list_bulleted_aligned_with(iter, std::convert::identity)
            .await
    }

    async fn list_bulleted_aligned_with<'f, P0, P1, I, T, F>(
        &mut self,
        iter: I,
        f: F,
    ) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> &'f (P0, P1),
    {
        self.list_aligned_with(ListType::Bulleted, iter, f).await
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

#[derive(Clone, Copy, Debug)]
enum ListType {
    Numbered,
    Bulleted,
}
