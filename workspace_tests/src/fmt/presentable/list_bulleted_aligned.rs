use peace::{
    fmt::{
        presentable::{Bold, CodeInline, ListBulletedAligned},
        Presentable,
    },
    rt_model::output::{CliColorizeOpt, CliMdPresenter},
};

use crate::fmt::presentable::cli_output;

#[tokio::test]
async fn present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    let list_bulleted_aligned = ListBulletedAligned::new(
        (1..=11)
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
    );
    list_bulleted_aligned.present(&mut presenter).await?;

    let output = String::from_utf8(buffer)?;
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

#[test]
fn debug() {
    assert_eq!(
        r#"ListBulletedAligned([("abc", "def")])"#,
        format!("{:?}", ListBulletedAligned::new(vec![("abc", "def")]))
    )
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        r#"- - abc
  - def
"#,
        serde_yaml::to_string(&ListBulletedAligned::new(vec![("abc", "def")]))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        ListBulletedAligned::new(vec![(String::from("abc"), String::from("def"))]),
        serde_yaml::from_str("- [abc, def]")?,
    );
    Ok(())
}

#[test]
fn from() {
    assert_eq!(
        ListBulletedAligned::new(vec![("abc", "def")]),
        ListBulletedAligned::from(vec![("abc", "def")])
    );
}
