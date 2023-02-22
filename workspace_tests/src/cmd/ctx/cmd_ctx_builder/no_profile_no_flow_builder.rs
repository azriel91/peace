use peace::{
    cfg::{app_name, AppName},
    cmd::{ctx::CmdCtxBuilder, scopes::NoProfileNoFlow},
};

use super::workspace;

#[test]
fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_no_profile_no_flow"))?;

    let cmd_ctx_builder = CmdCtxBuilder::no_profile_no_flow(&workspace);
    let cmd_ctx = cmd_ctx_builder.build();

    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    assert_eq!(&NoProfileNoFlow, cmd_ctx.scope());
    Ok(())
}
