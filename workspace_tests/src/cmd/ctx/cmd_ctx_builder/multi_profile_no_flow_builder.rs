use std::collections::BTreeMap;

use peace::{
    cfg::app_name,
    cmd_ctx::CmdCtxMpnf,
    profile_model::{profile, Profile, ProfileInvalidFmt},
    resource_rt::{
        paths::{ProfileDir, ProfileHistoryDir},
        type_reg::untagged::TypeReg,
    },
    rt_model::{Error as PeaceRtError, NativeError},
};

use crate::{
    no_op_output::NoOpOutput, peace_cmd_ctx_types::TestCctNoOpOutput, test_support::workspace_with,
    PeaceTestError,
};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        (profile_dirs, profile_history_dirs)
    };

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        (profile_dirs, profile_history_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(123))
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        (profile_dirs, profile_history_dirs)
    };

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(123))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let profile_other_dir = ProfileDir::from((peace_app_dir, &profile_other));
        let profile_other_history_dir = ProfileHistoryDir::from(&profile_other_dir);

        profile_dirs.insert(profile.clone(), profile_dir);
        profile_dirs.insert(profile_other.clone(), profile_other_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);
        profile_history_dirs.insert(profile_other.clone(), profile_other_history_dir);

        (profile_dirs, profile_history_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_to_profile_params = fields.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone(), profile_other], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_filter() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_filter_fn(|profile| **profile == "test_profile")
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        profile_dirs.insert(profile.clone(), profile_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);

        (profile_dirs, profile_history_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone()], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_filter(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param::<u32>(&profile, String::from("profile_param_0"), Some(123))
        .with_workspace_param(String::from("profile"), Some(profile.clone()))
        .with_profile_param::<u64>(&profile, String::from("profile_param_1"), None)
        .with_workspace_param(
            String::from("ws_param_1"),
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_filter_fn(|profile| **profile == "test_profile")
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let (profile_dirs, profile_history_dirs) = {
        let mut profile_dirs = BTreeMap::new();
        let mut profile_history_dirs = BTreeMap::new();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        profile_dirs.insert(profile.clone(), profile_dir);

        profile_history_dirs.insert(profile.clone(), profile_history_dir);

        (profile_dirs, profile_history_dirs)
    };

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_to_profile_params = fields.profile_to_profile_params();
    let profile_params = profile_to_profile_params
        .get(&profile)
        .expect("Expected profile params to exist.");
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&[profile.clone()], fields.profiles());
    assert_eq!(&profile_dirs, fields.profile_dirs());
    assert_eq!(&profile_history_dirs, fields.profile_history_dirs());
    assert_eq!(Some(&profile), workspace_params.get("profile"));
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get("ws_param_1")
    );
    assert_eq!(Some(&1u32), profile_params.get("profile_param_0"));
    assert_eq!(Some(&2u64), profile_params.get("profile_param_1"));
    Ok(())
}

#[tokio::test]
async fn list_profile_dirs_invalid_profile_name() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    // sneaky usage of `new_unchecked`.
    let profile_other = Profile::new_unchecked("test_profile_spécïál");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx_result = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .build()
        .await;

    let profile_other_dir = ProfileDir::from((workspace.dirs().peace_app_dir(), &profile_other));
    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &cmd_ctx_result,
                    Err(PeaceTestError::PeaceRt(PeaceRtError::Native(
                        NativeError::ProfileDirInvalidName {
                            dir_name,
                            path,
                            error,
                        }
                    )))

                    if dir_name == "test_profile_spécïál"
                    && path == &*profile_other_dir
                    && error == &ProfileInvalidFmt::new("test_profile_spécïál".into())
                ),
                "expected `cmd_ctx_result` to be \n\
                `Err(PeaceTestError::PeaceRt(PeaceRtError::Native(NativeError::ProfileDirInvalidName {{ .. }})))`,\n\
                but was {cmd_ctx_result:?}",
            );
        }
    })();

    Ok(())
}

#[tokio::test]
async fn getters() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let mut cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .await?;

    assert!(matches!(cmd_ctx.output(), NoOpOutput));
    assert!(matches!(cmd_ctx.output_mut(), NoOpOutput));

    let fields = cmd_ctx.fields();
    assert_eq!(workspace.dirs().workspace_dir(), fields.workspace_dir());
    assert_eq!(workspace.dirs().peace_dir(), fields.peace_dir());
    assert_eq!(workspace.dirs().peace_app_dir(), fields.peace_app_dir());
    assert!(matches!(fields.workspace_params_type_reg(), TypeReg { .. }));
    assert!(matches!(fields.profile_params_type_reg(), TypeReg { .. }));

    Ok(())
}

#[tokio::test]
async fn debug() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let profile = profile!("test_profile");
    let profile_other = profile!("test_profile_other");
    let workspace = workspace_with(
        &tempdir,
        app_name!("test_multi_profile_no_flow"),
        &[profile.clone(), profile_other.clone()],
        None,
    )
    .await?;

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .await?;

    assert!(format!("{cmd_ctx:?}").contains("CmdCtxMpsf {"));

    let fields = cmd_ctx.fields();
    assert!(format!("{fields:?}").contains("CmdCtxMpnfFields {"));

    Ok(())
}
