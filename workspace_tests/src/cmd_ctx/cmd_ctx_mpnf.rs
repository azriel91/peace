use peace::{cfg::app_name, cmd_ctx::CmdCtxMpnf};

use crate::{
    no_op_output::NoOpOutput, peace_cmd_ctx_types::TestCctNoOpOutput, test_support::workspace,
};

#[tokio::test]
async fn coverage_getters() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(&tempdir, app_name!("test_cmd_ctx_mpnf")).await?;

    let mut output = NoOpOutput;
    let mut cmd_ctx = CmdCtxMpnf::<TestCctNoOpOutput>::builder()
        .with_output((&mut output).into())
        .with_workspace((&workspace).into())
        .with_profile_filter_fn(|profile_candidate| profile_candidate.as_str() == "test_profile")
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
    let _ = fields_mut.profiles();
    let _ = fields_mut.profile_dirs();
    let _ = fields_mut.profile_history_dirs();
    let _ = fields_mut.workspace_params_type_reg();
    let _ = fields_mut.workspace_params_type_reg_mut();
    let _ = fields_mut.workspace_params();
    let _ = fields_mut.workspace_params_mut();
    let _ = fields_mut.profile_params_type_reg();
    let _ = fields_mut.profile_params_type_reg_mut();
    let _ = fields_mut.profile_to_profile_params();
    let _ = fields_mut.profile_to_profile_params_mut();

    Ok(())
}
