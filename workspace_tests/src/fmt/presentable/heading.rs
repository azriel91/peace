use futures::{stream, StreamExt, TryStreamExt};
use peace::{
    cli::output::{CliColorizeOpt, CliMdPresenter},
    fmt::{
        presentable::{CodeInline, Heading, HeadingLevel},
        Presentable,
    },
};

use crate::fmt::cli_output;

#[tokio::test]
async fn present() -> Result<(), Box<dyn std::error::Error>> {
    stream::iter([
        (
            HeadingLevel::Level1,
            "\u{1b}[38;5;243m\u{1b}[1m#\u{1b}[0m \u{1b}[38;5;75m\u{1b}[1m`code`\u{1b}[0m\n\n",
            "# `code`\n\n",
        ),
        (
            HeadingLevel::Level2,
            "\u{1b}[38;5;243m\u{1b}[1m##\u{1b}[0m \u{1b}[38;5;75m\u{1b}[1m`code`\u{1b}[0m\n\n",
            "## `code`\n\n",
        ),
        (
            HeadingLevel::Level3,
            "\u{1b}[38;5;243m\u{1b}[1m###\u{1b}[0m \u{1b}[38;5;75m\u{1b}[1m`code`\u{1b}[0m\n\n",
            "### `code`\n\n",
        ),
        (
            HeadingLevel::Level4,
            "\u{1b}[38;5;243m\u{1b}[1m####\u{1b}[0m \u{1b}[38;5;75m\u{1b}[1m`code`\u{1b}[0m\n\n",
            "#### `code`\n\n",
        ),
        (
            HeadingLevel::Level5,
            "\u{1b}[38;5;243m\u{1b}[1m#####\u{1b}[0m \u{1b}[38;5;75m\u{1b}[1m`code`\u{1b}[0m\n\n",
            "##### `code`\n\n",
        ),
        (
            HeadingLevel::Level6,
            "\u{1b}[38;5;243m\u{1b}[1m######\u{1b}[0m \u{1b}[38;5;75m\u{1b}[1m`code`\u{1b}[0m\n\n",
            "###### `code`\n\n",
        ),
    ])
    .map(Result::<_, Box<dyn std::error::Error>>::Ok)
    .try_for_each(|(heading_level, expected_colorized, expected)| async move {
        let mut cli_output = cli_output(CliColorizeOpt::Always);
        let mut presenter = CliMdPresenter::new(&mut cli_output);

        let heading = Heading::new(heading_level, CodeInline::new("code".into()));
        heading.present(&mut presenter).await?;

        let output = String::from_utf8(cli_output.writer().clone())?;
        assert_eq!(expected_colorized, output);
        assert_eq!(expected, console::strip_ansi_codes(&output));
        Ok(())
    })
    .await
}

#[test]
fn debug() {
    assert_eq!(
        "Heading { \
            level: Level1, \
            presentable: \"abc\" \
        }",
        format!("{:?}", Heading::new(HeadingLevel::Level1, "abc"))
    )
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "\
        level: Level1\n\
        presentable: abc\n\
        ",
        serde_yaml::to_string(&Heading::new(HeadingLevel::Level1, "abc"))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        Heading::new(HeadingLevel::Level1, "abc"),
        serde_yaml::from_str(
            "\
            level: Level1\n\
            presentable: abc\n\
            "
        )?,
    );
    Ok(())
}
