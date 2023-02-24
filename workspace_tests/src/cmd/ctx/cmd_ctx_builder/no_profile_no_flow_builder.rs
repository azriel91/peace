use peace::{
    cfg::{app_name, AppName},
    cmd::ctx::CmdCtx,
};

use super::workspace;

#[tokio::test]
async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = workspace(tempdir, app_name!("test_no_profile_no_flow"))?;

    let cmd_ctx = CmdCtx::builder_no_profile_no_flow(&workspace)
        .build()
        .await?;

    assert!(std::ptr::eq(&workspace, cmd_ctx.workspace()));
    Ok(())
}
