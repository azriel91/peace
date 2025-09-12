use peace::{
    cfg::{app_name, profile},
    cmd_ctx::{CmdCtxSpnf, CmdCtxTypes, ProfileSelection},
    resource_rt::paths::{ProfileDir, ProfileHistoryDir},
};

use crate::{no_op_output::NoOpOutput, test_support::workspace, PeaceTestError};

use super::{ProfileParamsKey, WorkspaceParamsKey};

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_workspace_param(WorkspaceParamsKey::Profile, Some(profile.clone()))
        .with_workspace_param(
            WorkspaceParamsKey::StringParam,
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(
        Some(&profile),
        workspace_params.get(&WorkspaceParamsKey::Profile)
    );
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get(&WorkspaceParamsKey::StringParam)
    );
    Ok(())
}

#[tokio::test]
async fn build_with_profile_params() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param(ProfileParamsKey::U32Param, Some(1u32))
        .with_profile_param(ProfileParamsKey::U64Param, Some(2u64))
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(Some(&1u32), profile_params.get(&ProfileParamsKey::U32Param));
    assert_eq!(Some(&2u64), profile_params.get(&ProfileParamsKey::U64Param));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params() -> Result<(), Box<dyn std::error::Error>>
{
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_profile_param(ProfileParamsKey::U32Param, Some(1u32))
        .with_workspace_param(WorkspaceParamsKey::Profile, Some(profile.clone()))
        .with_profile_param(ProfileParamsKey::U64Param, Some(2u64))
        .with_workspace_param(
            WorkspaceParamsKey::StringParam,
            Some("ws_param_1_value".to_string()),
        )
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(
        Some(&profile),
        workspace_params.get(&WorkspaceParamsKey::Profile)
    );
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get(&WorkspaceParamsKey::StringParam)
    );
    assert_eq!(Some(&1u32), profile_params.get(&ProfileParamsKey::U32Param));
    assert_eq!(Some(&2u64), profile_params.get(&ProfileParamsKey::U64Param));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_from_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_workspace_param(WorkspaceParamsKey::Profile, Some(profile.clone()))
        .with_workspace_param(
            WorkspaceParamsKey::StringParam,
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            WorkspaceParamsKey::Profile.into(),
        ))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(
        Some(&profile),
        workspace_params.get(&WorkspaceParamsKey::Profile)
    );
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get(&WorkspaceParamsKey::StringParam)
    );
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_with_profile_from_params(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let output = NoOpOutput;
    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output(output.into())
        .with_workspace((&workspace).into())
        .with_profile_param(ProfileParamsKey::U32Param, Some(1u32))
        .with_workspace_param(WorkspaceParamsKey::Profile, Some(profile.clone()))
        .with_profile_param(ProfileParamsKey::U64Param, Some(2u64))
        .with_workspace_param(
            WorkspaceParamsKey::StringParam,
            Some("ws_param_1_value".to_string()),
        )
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            WorkspaceParamsKey::Profile.into(),
        ))
        .build()
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(
        Some(&profile),
        workspace_params.get(&WorkspaceParamsKey::Profile)
    );
    assert_eq!(
        Some(&"ws_param_1_value".to_string()),
        workspace_params.get(&WorkspaceParamsKey::StringParam)
    );
    assert_eq!(Some(&1u32), profile_params.get(&ProfileParamsKey::U32Param));
    assert_eq!(Some(&2u64), profile_params.get(&ProfileParamsKey::U64Param));
    Ok(())
}

#[tokio::test]
async fn build_with_workspace_params_with_profile_params_none_provided(
) -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf_params")).await?;
    let profile = profile!("test_profile");

    let mut output = NoOpOutput;
    let cmd_ctx_save = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .with_workspace_param(WorkspaceParamsKey::Profile, Some(profile.clone()))
        .with_workspace_param(
            WorkspaceParamsKey::StringParam,
            Some(String::from("ws_param_1_value")),
        )
        .with_workspace_param(WorkspaceParamsKey::U8Param, None::<u8>)
        .with_profile_param(ProfileParamsKey::U32Param, Some(1u32))
        .with_profile_param(ProfileParamsKey::U64Param, Some(2u64))
        .await?;
    drop(cmd_ctx_save);

    let cmd_ctx = CmdCtxSpnf::<TestCctCmdCtxSpnf>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(
            WorkspaceParamsKey::Profile.into(),
        ))
        .await?;

    let peace_app_dir = workspace.dirs().peace_app_dir();
    let profile_dir = ProfileDir::from((peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let fields = cmd_ctx.fields();
    let workspace_params = fields.workspace_params();
    let profile_params = fields.profile_params();
    assert!(std::ptr::eq(&workspace, fields.workspace()));
    assert_eq!(peace_app_dir, fields.workspace().dirs().peace_app_dir());
    assert_eq!(&profile, fields.profile());
    assert_eq!(&profile_dir, fields.profile_dir());
    assert_eq!(&profile_history_dir, fields.profile_history_dir());
    assert_eq!(
        Some(&profile),
        workspace_params.get(&WorkspaceParamsKey::Profile)
    );
    assert_eq!(
        Some(&String::from("ws_param_1_value")),
        workspace_params.get(&WorkspaceParamsKey::StringParam)
    );
    assert_eq!(
        None::<u8>,
        workspace_params.get(&WorkspaceParamsKey::U8Param).copied()
    );
    assert_eq!(Some(&1u32), profile_params.get(&ProfileParamsKey::U32Param));
    assert_eq!(Some(&2u64), profile_params.get(&ProfileParamsKey::U64Param));
    Ok(())
}

#[derive(Debug)]
pub struct TestCctCmdCtxSpnf;

impl CmdCtxTypes for TestCctCmdCtxSpnf {
    type AppError = PeaceTestError;
    type FlowParamsKey = ();
    type MappingFns = ();
    type Output = NoOpOutput;
    type ProfileParamsKey = ProfileParamsKey;
    type WorkspaceParamsKey = WorkspaceParamsKey;
}
