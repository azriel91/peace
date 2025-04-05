use futures::stream::{self, StreamExt, TryStreamExt};
use peace::{
    cli::output::{CliMdPresenter, CliOutput, CliOutputBuilder},
    cli_model::OutputFormat,
    fmt::{
        presentable::{Bold, CodeInline, HeadingLevel},
        Presenter,
    },
};

use peace::cli::output::CliColorizeOpt;

#[tokio::test]
async fn presents_heading_with_hashes_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    stream::iter([
        (HeadingLevel::Level1, "# `code`\n\n"),
        (HeadingLevel::Level2, "## `code`\n\n"),
        (HeadingLevel::Level3, "### `code`\n\n"),
        (HeadingLevel::Level4, "#### `code`\n\n"),
        (HeadingLevel::Level5, "##### `code`\n\n"),
        (HeadingLevel::Level6, "###### `code`\n\n"),
    ])
    .map(Result::<_, Box<dyn std::error::Error>>::Ok)
    .try_for_each(|(heading_level, expected)| async move {
        let mut cli_output = cli_output(CliColorizeOpt::Never);
        let mut presenter = CliMdPresenter::new(&mut cli_output);

        presenter
            .heading(heading_level, &CodeInline::new("code".into()))
            .await?;

        let output = String::from_utf8(cli_output.writer().clone())?;
        assert_eq!(expected, output);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn presents_heading_with_hashes_color_enabled() -> Result<(), Box<dyn std::error::Error>> {
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

        presenter
            .heading(heading_level, &CodeInline::new("code".into()))
            .await?;

        let output = String::from_utf8(cli_output.writer().clone())?;
        assert_eq!(expected_colorized, output);
        assert_eq!(expected, console::strip_ansi_codes(&output));
        Ok(())
    })
    .await
}

#[tokio::test]
async fn presents_bold_with_double_asterisk_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.bold(&String::from("bold")).await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("**bold**", output);
    assert_eq!("**bold**", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_bold_with_double_asterisk_color_enabled() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.bold(&String::from("bold")).await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("**\u{1b}[1mbold\u{1b}[0m**", output);
    assert_eq!("**bold**", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_bold_wrapping_code_inline_with_double_asterisk_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.bold(&CodeInline::new("code".into())).await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("**`code`**", output);
    assert_eq!("**`code`**", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_bold_wrapping_code_inline_with_double_asterisk_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.bold(&CodeInline::new("code".into())).await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("**\u{1b}[38;5;75m\u{1b}[1m`code`\u{1b}[0m**", output);
    assert_eq!("**`code`**", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_id_as_plain_text_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.id("an_id").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("an_id", output);
    Ok(())
}

#[tokio::test]
async fn presents_id_as_blue_text_color_enabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.id("an_id").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("\u{1b}[38;5;75man_id\u{1b}[0m", output);
    assert_eq!("an_id", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_name_with_double_asterisk_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.name("A Name").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("**A Name**", output);
    Ok(())
}

#[tokio::test]
async fn presents_name_with_double_asterisk_bold_text_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.name("A Name").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("\u{1b}[1m**A Name**\u{1b}[0m", output);
    assert_eq!("**A Name**", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_text_as_plain_text_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.text("hello").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("hello", output);
    Ok(())
}

#[tokio::test]
async fn presents_text_as_plain_text_color_enabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.text("hello").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("hello", output);
    Ok(())
}

#[tokio::test]
async fn presents_tag_with_black_tortoise_shell_plain_text_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.tag("tag").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("⦗tag⦘", output);
    Ok(())
}

#[tokio::test]
async fn presents_tag_with_black_tortoise_shell_purple_text_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.tag("tag").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("\u{1b}[38;5;219m\u{1b}[1m⦗tag⦘\u{1b}[0m", output);
    assert_eq!("⦗tag⦘", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_code_inline_with_backticks_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.code_inline("code_inline").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("`code_inline`", output);
    Ok(())
}

#[tokio::test]
async fn presents_code_inline_with_backticks_blue_text_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter.code_inline("code_inline").await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!("\u{1b}[38;5;75m`code_inline`\u{1b}[0m", output);
    assert_eq!("`code_inline`", console::strip_ansi_codes(&output));
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered(&[String::from("Item 1"), String::from("Item 2")])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            1. Item 1\n\
            2. Item 2\n\
        ",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_white_text_color_enabled() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered(&[String::from("Item 1"), String::from("Item 2")])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
        \u{1b}[38;5;15m1.\u{1b}[0m Item 1\n\
        \u{1b}[38;5;15m2.\u{1b}[0m Item 2\n\
    ",
        output
    );
    assert_eq!(
        "\
            1. Item 1\n\
            2. Item 2\n\
        ",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_padding_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered(
            &(1..=11)
                .map(|n| format!("Item {n}"))
                .collect::<Vec<String>>(),
        )
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        r#" 1. Item 1
 2. Item 2
 3. Item 3
 4. Item 4
 5. Item 5
 6. Item 6
 7. Item 7
 8. Item 8
 9. Item 9
10. Item 10
11. Item 11
"#,
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_padding_color_enabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered(
            &(1..=11)
                .map(|n| format!("Item {n}"))
                .collect::<Vec<String>>(),
        )
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        " \u{1b}[38;5;15m1.\u{1b}[0m Item 1
 \u{1b}[38;5;15m2.\u{1b}[0m Item 2
 \u{1b}[38;5;15m3.\u{1b}[0m Item 3
 \u{1b}[38;5;15m4.\u{1b}[0m Item 4
 \u{1b}[38;5;15m5.\u{1b}[0m Item 5
 \u{1b}[38;5;15m6.\u{1b}[0m Item 6
 \u{1b}[38;5;15m7.\u{1b}[0m Item 7
 \u{1b}[38;5;15m8.\u{1b}[0m Item 8
 \u{1b}[38;5;15m9.\u{1b}[0m Item 9
\u{1b}[38;5;15m10.\u{1b}[0m Item 10
\u{1b}[38;5;15m11.\u{1b}[0m Item 11
",
        output
    );
    assert_eq!(
        r#" 1. Item 1
 2. Item 2
 3. Item 3
 4. Item 4
 5. Item 5
 6. Item 6
 7. Item 7
 8. Item 8
 9. Item 9
10. Item 10
11. Item 11
"#,
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_with_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_with(&[true, false], |first| {
            if *first {
                String::from("Item 1")
            } else {
                String::from("Item 2")
            }
        })
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            1. Item 1\n\
            2. Item 2\n\
        ",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_with_white_text_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_with(&[true, false], |first| {
            if *first {
                String::from("Item 1")
            } else {
                String::from("Item 2")
            }
        })
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
        \u{1b}[38;5;15m1.\u{1b}[0m Item 1\n\
        \u{1b}[38;5;15m2.\u{1b}[0m Item 2\n\
    ",
        output
    );
    assert_eq!(
        "\
            1. Item 1\n\
            2. Item 2\n\
        ",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_with_padding_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_with(1..=11, |n| format!("Item {n}"))
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        r#" 1. Item 1
 2. Item 2
 3. Item 3
 4. Item 4
 5. Item 5
 6. Item 6
 7. Item 7
 8. Item 8
 9. Item 9
10. Item 10
11. Item 11
"#,
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_with_padding_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_with(1..=11, |n| format!("Item {n}"))
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        " \u{1b}[38;5;15m1.\u{1b}[0m Item 1
 \u{1b}[38;5;15m2.\u{1b}[0m Item 2
 \u{1b}[38;5;15m3.\u{1b}[0m Item 3
 \u{1b}[38;5;15m4.\u{1b}[0m Item 4
 \u{1b}[38;5;15m5.\u{1b}[0m Item 5
 \u{1b}[38;5;15m6.\u{1b}[0m Item 6
 \u{1b}[38;5;15m7.\u{1b}[0m Item 7
 \u{1b}[38;5;15m8.\u{1b}[0m Item 8
 \u{1b}[38;5;15m9.\u{1b}[0m Item 9
\u{1b}[38;5;15m10.\u{1b}[0m Item 10
\u{1b}[38;5;15m11.\u{1b}[0m Item 11
",
        output
    );
    assert_eq!(
        r#" 1. Item 1
 2. Item 2
 3. Item 3
 4. Item 4
 5. Item 5
 6. Item 6
 7. Item 7
 8. Item 8
 9. Item 9
10. Item 10
11. Item 11
"#,
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_aligned_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_aligned(&[
            (
                Bold::new(String::from("Short")),
                (
                    String::from("description with "),
                    CodeInline::new("code".into()),
                ),
            ),
            (
                Bold::new(String::from("Long Name")),
                (
                    String::from("another description "),
                    CodeInline::new("code".into()),
                ),
            ),
        ])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            1. **Short**    : description with `code`\n\
            2. **Long Name**: another description `code`\n\
        ",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_aligned_white_text_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_aligned(&[
            (
                Bold::new(String::from("Short")),
                (
                    String::from("description with "),
                    CodeInline::new("code".into()),
                ),
            ),
            (
                Bold::new(String::from("Long Name")),
                (
                    String::from("another description "),
                    CodeInline::new("code".into()),
                ),
            ),
        ])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            \u{1b}[38;5;15m1.\u{1b}[0m **\u{1b}[1mShort\u{1b}[0m**    : description with \u{1b}[38;5;75m`code`\u{1b}[0m\n\
            \u{1b}[38;5;15m2.\u{1b}[0m **\u{1b}[1mLong Name\u{1b}[0m**: another description \u{1b}[38;5;75m`code`\u{1b}[0m\n\
        ",
        output
    );
    assert_eq!(
        "\
            1. **Short**    : description with `code`\n\
            2. **Long Name**: another description `code`\n\
        ",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_aligned_padding_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_aligned(
            &(1..=11)
                .map(|n| {
                    (
                        Bold::new(format!("Item {n}")),
                        (
                            String::from("description with "),
                            CodeInline::new("code".into()),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        " 1. **Item 1** : description with `code`
 2. **Item 2** : description with `code`
 3. **Item 3** : description with `code`
 4. **Item 4** : description with `code`
 5. **Item 5** : description with `code`
 6. **Item 6** : description with `code`
 7. **Item 7** : description with `code`
 8. **Item 8** : description with `code`
 9. **Item 9** : description with `code`
10. **Item 10**: description with `code`
11. **Item 11**: description with `code`
",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_numbered_aligned_padding_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_numbered_aligned(
            &(1..=11)
                .map(|n| {
                    (
                        Bold::new(format!("Item {n}")),
                        (
                            String::from("description with "),
                            CodeInline::new("code".into()),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        " \u{1b}[38;5;15m1.\u{1b}[0m **\u{1b}[1mItem 1\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m2.\u{1b}[0m **\u{1b}[1mItem 2\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m3.\u{1b}[0m **\u{1b}[1mItem 3\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m4.\u{1b}[0m **\u{1b}[1mItem 4\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m5.\u{1b}[0m **\u{1b}[1mItem 5\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m6.\u{1b}[0m **\u{1b}[1mItem 6\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m7.\u{1b}[0m **\u{1b}[1mItem 7\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m8.\u{1b}[0m **\u{1b}[1mItem 8\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m9.\u{1b}[0m **\u{1b}[1mItem 9\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m10.\u{1b}[0m **\u{1b}[1mItem 10\u{1b}[0m**: description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m11.\u{1b}[0m **\u{1b}[1mItem 11\u{1b}[0m**: description with \u{1b}[38;5;75m`code`\u{1b}[0m
",
        output
    );
    assert_eq!(
        " 1. **Item 1** : description with `code`
 2. **Item 2** : description with `code`
 3. **Item 3** : description with `code`
 4. **Item 4** : description with `code`
 5. **Item 5** : description with `code`
 6. **Item 6** : description with `code`
 7. **Item 7** : description with `code`
 8. **Item 8** : description with `code`
 9. **Item 9** : description with `code`
10. **Item 10**: description with `code`
11. **Item 11**: description with `code`
",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted(&[String::from("Item 1"), String::from("Item 2")])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            * Item 1\n\
            * Item 2\n\
        ",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_white_text_color_enabled() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted(&[String::from("Item 1"), String::from("Item 2")])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            \u{1b}[38;5;15m*\u{1b}[0m Item 1\n\
            \u{1b}[38;5;15m*\u{1b}[0m Item 2\n\
        ",
        output
    );
    assert_eq!(
        "\
            * Item 1\n\
            * Item 2\n\
        ",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_with_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted_with(&[true, false], |first| {
            if *first {
                String::from("Item 1")
            } else {
                String::from("Item 2")
            }
        })
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            * Item 1\n\
            * Item 2\n\
        ",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_with_white_text_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted_with(&[true, false], |first| {
            if *first {
                String::from("Item 1")
            } else {
                String::from("Item 2")
            }
        })
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            \u{1b}[38;5;15m*\u{1b}[0m Item 1\n\
            \u{1b}[38;5;15m*\u{1b}[0m Item 2\n\
        ",
        output
    );
    assert_eq!(
        "\
            * Item 1\n\
            * Item 2\n\
        ",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_aligned_color_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted_aligned(&[
            (
                Bold::new(String::from("Short")),
                (
                    String::from("description with "),
                    CodeInline::new("code".into()),
                ),
            ),
            (
                Bold::new(String::from("Long Name")),
                (
                    String::from("another description "),
                    CodeInline::new("code".into()),
                ),
            ),
        ])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            * **Short**    : description with `code`\n\
            * **Long Name**: another description `code`\n\
        ",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_aligned_white_text_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted_aligned(&[
            (
                Bold::new(String::from("Short")),
                (
                    String::from("description with "),
                    CodeInline::new("code".into()),
                ),
            ),
            (
                Bold::new(String::from("Long Name")),
                (
                    String::from("another description "),
                    CodeInline::new("code".into()),
                ),
            ),
        ])
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\
            \u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mShort\u{1b}[0m**    : description with \u{1b}[38;5;75m`code`\u{1b}[0m\n\
            \u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mLong Name\u{1b}[0m**: another description \u{1b}[38;5;75m`code`\u{1b}[0m\n\
        ",
        output
    );
    assert_eq!(
        "\
            * **Short**    : description with `code`\n\
            * **Long Name**: another description `code`\n\
        ",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_aligned_padding_color_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted_aligned(
            &(1..=11)
                .map(|n| {
                    (
                        Bold::new(format!("Item {n}")),
                        (
                            String::from("description with "),
                            CodeInline::new("code".into()),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "* **Item 1** : description with `code`
* **Item 2** : description with `code`
* **Item 3** : description with `code`
* **Item 4** : description with `code`
* **Item 5** : description with `code`
* **Item 6** : description with `code`
* **Item 7** : description with `code`
* **Item 8** : description with `code`
* **Item 9** : description with `code`
* **Item 10**: description with `code`
* **Item 11**: description with `code`
",
        output
    );
    Ok(())
}

#[tokio::test]
async fn presents_list_bulleted_aligned_padding_color_enabled(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_output = cli_output(CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    presenter
        .list_bulleted_aligned(
            &(1..=11)
                .map(|n| {
                    (
                        Bold::new(format!("Item {n}")),
                        (
                            String::from("description with "),
                            CodeInline::new("code".into()),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .await?;

    let output = String::from_utf8(cli_output.writer().clone())?;
    assert_eq!(
        "\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 1\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 2\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 3\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 4\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 5\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 6\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 7\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 8\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 9\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 10\u{1b}[0m**: description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m*\u{1b}[0m **\u{1b}[1mItem 11\u{1b}[0m**: description with \u{1b}[38;5;75m`code`\u{1b}[0m
",
        output
    );
    assert_eq!(
        "* **Item 1** : description with `code`
* **Item 2** : description with `code`
* **Item 3** : description with `code`
* **Item 4** : description with `code`
* **Item 5** : description with `code`
* **Item 6** : description with `code`
* **Item 7** : description with `code`
* **Item 8** : description with `code`
* **Item 9** : description with `code`
* **Item 10**: description with `code`
* **Item 11**: description with `code`
",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

fn cli_output(colorize: CliColorizeOpt) -> CliOutput<Vec<u8>> {
    CliOutputBuilder::new_with_writer(Vec::new())
        .with_outcome_format(OutputFormat::Text)
        .with_colorize(colorize)
        .build()
}
