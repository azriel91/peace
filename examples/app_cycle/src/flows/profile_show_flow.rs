use peace::{
    cfg::{app_name, AppName, Profile},
    rt::cmds::sub::StatesSavedReadCmd,
    rt_model::{
        output::OutputWrite, ItemSpecGraph, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
    },
};

use crate::{
    cmds::CmdCtxBuilder,
    model::{AppCycleError, EnvType},
};

/// Flow to show the current profile.
#[derive(Debug)]
pub struct ProfileShowFlow;

impl ProfileShowFlow {
    /// Shows the currently active profile.
    ///
    /// The active profile is stored in workspace params.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let graph = Self::graph()?;

        let cmd_context = CmdCtxBuilder::new(&workspace, &graph, output)
            .with_profile_from_last_used()
            .await?;

        let cmd_context = StatesSavedReadCmd::exec(cmd_context).await?;
        let resources = &cmd_context.resources;
        let profile = &*resources.borrow::<Profile>();
        let env_type = &*resources.borrow::<EnvType>();

        eprintln!("Profile: {profile}");
        eprintln!("Type: {env_type}");

        Ok(())
    }

    fn graph() -> Result<ItemSpecGraph<AppCycleError>, AppCycleError> {
        let graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

        // No item specs, as we are just storing profile init params.

        Ok(graph_builder.build())
    }
}
