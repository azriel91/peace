use peace::rt_model::output::OutputWrite;

#[cfg(not(feature = "error_reporting"))]
#[tokio::test]
async fn write_err() -> Result<(), Box<dyn std::error::Error>> {
    use peace::{item_model::item_id, rt_model::InMemoryTextOutput};

    let mut output = InMemoryTextOutput::new();
    let item_id = item_id!("test_item");
    let error = peace::rt_model::Error::ParamsSpecNotFound { item_id };

    output.write_err(&error).await?;

    assert_eq!(
        "A `Params::Spec` was not present for item: `test_item`\n",
        output.into_inner().as_str()
    );

    Ok(())
}

#[cfg(feature = "error_reporting")]
#[tokio::test]
async fn write_err() -> Result<(), Box<dyn std::error::Error>> {
    use peace::{item_model::item_id, rt_model::InMemoryTextOutput};

    let mut output = InMemoryTextOutput::new();
    let item_id = item_id!("test_item");
    let error = peace::rt_model::Error::ParamsSpecNotFound { item_id };

    output.write_err(&error).await?;

    assert_eq!(
        r#"peace_rt_model::params_spec_not_found

  x A `Params::Spec` was not present for item: `test_item`
  help: If you are an end user, please ask for help from the providers of your automation tool.
        
        If you are developing a tool with the Peace framework,
        please open an issue in the Peace repository:
        
        https://github.com/azriel91/peace/

"#,
        output.into_inner().as_str()
    );

    Ok(())
}
