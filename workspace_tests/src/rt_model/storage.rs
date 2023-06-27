use peace::{
    resources::type_reg::untagged::{TypeMapOpt, TypeReg},
    rt_model::{params::WorkspaceParams, Error, Storage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct TestStruct {
    a: u32,
}

#[test]
fn clone() {
    let _ = Clone::clone(&Storage);
    let _ = Clone::clone(&TestStruct { a: 1 });
}

#[test]
fn debug() {
    assert_eq!("Storage", format!("{Storage:?}"));
    assert_eq!("TestStruct { a: 1 }", format!("{:?}", TestStruct { a: 1 }));
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!("a: 1\n", serde_yaml::to_string(&TestStruct { a: 1 })?);
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(TestStruct { a: 1 }, serde_yaml::from_str("a: 1\n")?);
    Ok(())
}

#[tokio::test]
async fn serialized_read_returns_t_when_path_exists() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let file_path = tempdir.path().join("t.yaml");
    tokio::fs::write(&file_path, br#"a: 1"#).await?;

    let test_struct = Storage
        .serialized_read::<TestStruct, _>(
            crate::fn_name_short!().to_string(),
            &file_path,
            #[cfg_attr(coverage_nightly, no_coverage)]
            |_error| panic!("Expected `test_struct` to be deserialized."),
        )
        .await?;

    assert_eq!(TestStruct { a: 1 }, test_struct);

    Ok(())
}

#[tokio::test]
async fn serialized_read_returns_error_when_path_not_exists()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let file_path = tempdir.path().join("t.yaml");

    let error = Storage
        .serialized_read::<TestStruct, _>(
            crate::fn_name_short!().to_string(),
            &file_path,
            #[cfg_attr(coverage_nightly, no_coverage)]
            |_error| panic!("Expected `Error::ItemNotExists` to be returned."),
        )
        .await
        .unwrap_err();

    assert!(matches!(error, Error::ItemNotExists { path } if path == file_path ));

    Ok(())
}

#[tokio::test]
async fn serialized_read_opt_returns_t_when_path_exists() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let file_path = tempdir.path().join("t.yaml");
    tokio::fs::write(&file_path, br#"a: 1"#).await?;

    let test_struct = Storage
        .serialized_read_opt::<TestStruct, _>(
            crate::fn_name_short!().to_string(),
            &file_path,
            #[cfg_attr(coverage_nightly, no_coverage)]
            |_error| panic!("Expected `test_struct` to be deserialized."),
        )
        .await?;

    assert_eq!(Some(TestStruct { a: 1 }), test_struct);

    Ok(())
}

#[tokio::test]
async fn serialized_read_opt_returns_none_when_path_not_exists()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let file_path = tempdir.path().join("t.yaml");

    let test_struct = Storage
        .serialized_read_opt::<TestStruct, _>(
            crate::fn_name_short!().to_string(),
            &file_path,
            #[cfg_attr(coverage_nightly, no_coverage)]
            |_error| panic!("Expected `None` to be returned."),
        )
        .await?;

    assert!(test_struct.is_none());

    Ok(())
}

#[tokio::test]
async fn serialized_typemap_read_opt_returns_typemap_when_path_exists()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let file_path = tempdir.path().join("t.yaml");
    tokio::fs::write(&file_path, br#"0: { a: 1 }"#).await?;
    let mut type_reg = TypeReg::new();
    type_reg.register::<TestStruct>(0);

    let workspace_params: WorkspaceParams<u32> = Storage
        .serialized_typemap_read_opt(
            crate::fn_name_short!().to_string(),
            &type_reg,
            &file_path,
            #[cfg_attr(coverage_nightly, no_coverage)]
            |_error| panic!("Expected `workspace_params` to be deserialized."),
        )
        .await?
        .map(TypeMapOpt::into_type_map)
        .map(WorkspaceParams::<u32>::from)
        .unwrap();

    assert_eq!(Some(TestStruct { a: 1 }).as_ref(), workspace_params.get(&0));

    Ok(())
}

#[tokio::test]
async fn serialized_typemap_read_opt_returns_none_when_path_not_exists()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let file_path = tempdir.path().join("t.yaml");
    let mut type_reg = TypeReg::new();
    type_reg.register::<TestStruct>(0);

    let workspace_params: Option<WorkspaceParams<u32>> = Storage
        .serialized_typemap_read_opt(
            crate::fn_name_short!().to_string(),
            &type_reg,
            &file_path,
            #[cfg_attr(coverage_nightly, no_coverage)]
            |_error| panic!("Expected `None` to be returned."),
        )
        .await?
        .map(TypeMapOpt::into_type_map)
        .map(WorkspaceParams::<u32>::from);

    assert!(workspace_params.is_none());

    Ok(())
}

#[tokio::test]
async fn serialized_write_serializes_t() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let file_path = tempdir.path().join("t.yaml");

    let test_struct = TestStruct { a: 1 };
    Storage
        .serialized_write(
            crate::fn_name_short!().to_string(),
            &file_path,
            &test_struct,
            #[cfg_attr(coverage_nightly, no_coverage)]
            |_error| panic!("Expected `test_struct` to be serialized."),
        )
        .await?;

    let serialized = tokio::fs::read_to_string(&file_path).await?;

    assert_eq!("a: 1\n", serialized);

    Ok(())
}
