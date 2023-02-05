use peace_fmt::{async_trait, Presenter};
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
        let style = &console::Style::new().color256(247); // grey
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

    fn list<'f>(&'f mut self) -> peace_fmt::PresentableList<'output, 'f, Self> {
        peace_fmt::PresentableList::new(self)
    }
}
