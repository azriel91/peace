use peace::{
    cfg::{app_name, profile},
    cmd_ctx::{CmdCtxSpnf, ProfileSelection},
};

use crate::{
    no_op_output::NoOpOutput, peace_cmd_ctx_types::TestCctNoOpOutput, test_support::workspace,
};

#[tokio::test]
async fn coverage_getters() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_spnf")).await?;
    let profile = profile!("test_profile");

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtxSpnf::<TestCctNoOpOutput>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_selection(ProfileSelection::Specified(profile.clone()))
        .await?;

    let _output = cmd_ctx.output();
    let _output_mut = cmd_ctx.output_mut();
    let _fields = cmd_ctx.fields();
    let fields_mut = cmd_ctx.fields_mut();
    let _ = fields_mut.interruptibility_state();
    let _ = fields_mut.workspace();
    let _ = fields_mut.workspace_dir();
    let _ = fields_mut.peace_dir();
    let _ = fields_mut.peace_app_dir();
    let _ = fields_mut.profile();
    let _ = fields_mut.profile_dir();
    let _ = fields_mut.profile_history_dir();
    let _ = fields_mut.workspace_params_type_reg();
    let _ = fields_mut.workspace_params_type_reg_mut();
    let _ = fields_mut.workspace_params();
    let _ = fields_mut.workspace_params_mut();
    let _ = fields_mut.profile_params_type_reg();
    let _ = fields_mut.profile_params_type_reg_mut();
    let _ = fields_mut.profile_params();
    let _ = fields_mut.profile_params_mut();

    Ok(())
}
