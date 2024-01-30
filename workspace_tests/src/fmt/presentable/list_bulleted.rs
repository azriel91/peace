use peace::{
    fmt::{presentable::ListBulleted, Presentable},
    rt_model::output::{CliColorizeOpt, CliMdPresenter},
};

use crate::fmt::presentable::cli_output;

#[tokio::test]
async fn present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    let list_bulleted =
        ListBulleted::new((1..=11).map(|n| format!("Item {n}")).collect::<Vec<_>>());
    list_bulleted.present(&mut presenter).await?;

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;15m*\u{1b}[0m Item 1
\u{1b}[38;5;15m*\u{1b}[0m Item 2
\u{1b}[38;5;15m*\u{1b}[0m Item 3
\u{1b}[38;5;15m*\u{1b}[0m Item 4
\u{1b}[38;5;15m*\u{1b}[0m Item 5
\u{1b}[38;5;15m*\u{1b}[0m Item 6
\u{1b}[38;5;15m*\u{1b}[0m Item 7
\u{1b}[38;5;15m*\u{1b}[0m Item 8
\u{1b}[38;5;15m*\u{1b}[0m Item 9
\u{1b}[38;5;15m*\u{1b}[0m Item 10
\u{1b}[38;5;15m*\u{1b}[0m Item 11
",
        output
    );
    assert_eq!(
        r#"* Item 1
* Item 2
* Item 3
* Item 4
* Item 5
* Item 6
* Item 7
* Item 8
* Item 9
* Item 10
* Item 11
"#,
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[test]
fn debug() {
    assert_eq!(
        r#"ListBulleted(["abc"])"#,
        format!("{:?}", ListBulleted::new(vec!["abc"]))
    )
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "- abc\n",
        serde_yaml::to_string(&ListBulleted::new(vec!["abc"]))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        ListBulleted::new(vec![(String::from("abc"))]),
        serde_yaml::from_str("- abc")?,
    );
    Ok(())
}
