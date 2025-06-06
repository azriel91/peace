use std::future::IntoFuture;

use futures::{future::LocalBoxFuture, FutureExt};
use interruptible::Interruptibility;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_params::ParamsValue;
use peace_resource_rt::{
    internal::{ProfileParamsFile, WorkspaceParamsFile},
    paths::{ProfileDir, ProfileHistoryDir},
};
use peace_rt_model::{
    params::{ProfileParamsOpt, WorkspaceParamsOpt},
    Workspace, WorkspaceInitializer,
};
use type_reg::untagged::TypeReg;
use typed_builder::TypedBuilder;

use crate::{CmdCtxBuilderSupport, CmdCtxSpnf, CmdCtxSpnfFields, CmdCtxTypes, ProfileSelection};

/// A command that works with a single profile, not scoped to a flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can read `Profile`
/// |   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
/// |   |
/// |   |- 🌊 ..                   # ❌ cannot read or write Flow information
/// |
/// |- 🌏 ..                       # ❌ cannot read or write other `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write a single profile's parameters. For multiple profiles, see
///   [`CmdCtxMpnf`].
///
/// This kind of command cannot:
///
/// * Read or write flow parameters or state -- see [`CmdCtxSpsf`] or
///   [`CmdCtxMpsf`].
///
/// [`CmdCtxMpnf`]: crate::CmdCtxMpnf
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
/// [`CmdCtxSpsf`]: crate::CmdCtxSpsf
#[derive(Debug, TypedBuilder)]
#[builder(build_method(vis="", name=build_partial))]
pub struct CmdCtxSpnfParams<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    #[builder(setter(prefix = "with_"))]
    pub output: OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,
    /// The interrupt channel receiver if this `CmdExecution` is interruptible.
    #[builder(setter(prefix = "with_"), default = Interruptibility::NonInterruptible)]
    pub interruptibility: Interruptibility<'static>,
    /// Workspace that the `peace` tool runs in.
    #[builder(setter(prefix = "with_"))]
    pub workspace: OwnedOrRef<'ctx, Workspace>,
    /// The profile this command operates on.
    #[builder(setter(prefix = "with_"))]
    pub profile_selection: ProfileSelection<'ctx, CmdCtxTypesT::WorkspaceParamsKey>,
    /// Workspace params.
    //
    // NOTE: When updating this mutator, also update it for all the other `CmdCtx*Params` types.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = WorkspaceParamsOpt::default()),
        mutators(
            /// Sets the value at the given workspace params key.
            ///
            /// # Parameters
            ///
            /// * `key`: The key to store the given value against.
            /// * `value`: The value to store at the given key. This is an
            ///   `Option` so that you may remove a value if desired.
            ///
            /// # Type Parameters
            ///
            /// * `V`: The serializable type stored at the given key.
            pub fn with_workspace_param<V>(
                &mut self,
                key: CmdCtxTypesT::WorkspaceParamsKey,
                value: Option<V>,
            )
            where
                V: ParamsValue,
            {
                let _ = self.workspace_params.insert(key, value);
            }
        )
    )]
    #[builder(setter(prefix = "with_"))]
    pub workspace_params: WorkspaceParamsOpt<<CmdCtxTypesT as CmdCtxTypes>::WorkspaceParamsKey>,
    /// Profile params for the profile.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = ProfileParamsOpt::default()),
        mutators(
            /// Sets the value at the given profile params key.
            ///
            /// # Parameters
            ///
            /// * `key`: The key to store the given value against.
            /// * `value`: The value to store at the given key. This is an
            ///   `Option` so that you may remove a value if desired.
            ///
            /// # Type Parameters
            ///
            /// * `V`: The serializable type stored at the given key.
            pub fn with_profile_param<V>(
                &mut self,
                key: CmdCtxTypesT::ProfileParamsKey,
                value: Option<V>,
            )
            where
                V: ParamsValue,
            {
                let _ = self.profile_params.insert(key, value);
            }
        )
    )]
    pub profile_params: ProfileParamsOpt<<CmdCtxTypesT as CmdCtxTypes>::ProfileParamsKey>,
}

// Use one of the following to obtain the generated type signature:
//
// ```sh
// cargo expand -p peace_cmd_ctx cmd_ctx_spnf_builder
// ```
//
// Sublime text command:
//
// **LSP-rust-analyzer: Expand Macro Recursively** while the caret is on the
// `TypedBuilder` derive.
#[allow(non_camel_case_types)]
impl<
        'ctx,
        CmdCtxTypesT,
        __interruptibility: ::typed_builder::Optional<Interruptibility<'static>>,
    >
    CmdCtxSpnfParamsBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            (ProfileSelection<'ctx, CmdCtxTypesT::WorkspaceParamsKey>,),
            (WorkspaceParamsOpt<CmdCtxTypesT::WorkspaceParamsKey>,),
            (ProfileParamsOpt<CmdCtxTypesT::ProfileParamsKey>,),
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    pub async fn build(self) -> Result<CmdCtxSpnf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError> {
        let CmdCtxSpnfParams {
            output,
            interruptibility,
            workspace,
            profile_selection,
            workspace_params: workspace_params_provided,
            profile_params: profile_params_provided,
        } = self.build_partial();

        let mut workspace_params_type_reg = TypeReg::new();
        CmdCtxTypesT::workspace_params_register(&mut workspace_params_type_reg);
        let mut profile_params_type_reg = TypeReg::new();
        CmdCtxTypesT::profile_params_register(&mut profile_params_type_reg);

        let workspace_dirs = workspace.dirs();
        let storage = workspace.storage();

        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());
        let workspace_params = CmdCtxBuilderSupport::workspace_params_merge(
            storage,
            &workspace_params_type_reg,
            workspace_params_provided,
            &workspace_params_file,
        )
        .await?;

        let profile = CmdCtxBuilderSupport::profile_from_profile_selection(
            profile_selection,
            &workspace_params,
            storage,
            &workspace_params_file,
        )
        .await?;

        let profile_ref = &profile;
        let profile_dir = ProfileDir::from((workspace_dirs.peace_app_dir(), profile_ref));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let dirs_to_create = [
            AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
            AsRef::<std::path::Path>::as_ref(&profile_dir),
            AsRef::<std::path::Path>::as_ref(&profile_history_dir),
        ];

        let storage = workspace.storage();

        // profile_params_deserialize
        let profile_params_file = ProfileParamsFile::from(&profile_dir);
        let profile_params = CmdCtxBuilderSupport::profile_params_merge(
            storage,
            &profile_params_type_reg,
            profile_params_provided,
            &profile_params_file,
        )
        .await?;

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        {
            WorkspaceInitializer::dirs_create(storage, dirs_to_create).await?;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            WorkspaceInitializer::dirs_create(dirs_to_create).await?;

            let workspace_dir = workspace_dirs.workspace_dir();
            std::env::set_current_dir(workspace_dir).map_err(
                #[cfg_attr(coverage_nightly, coverage(off))]
                |error| {
                    peace_rt_model::Error::Native(peace_rt_model::NativeError::CurrentDirSet {
                        workspace_dir: workspace_dir.clone(),
                        error,
                    })
                },
            )?;
        }

        let interruptibility_state = interruptibility.into();

        // Serialize params to `PeaceAppDir`.
        CmdCtxBuilderSupport::workspace_params_serialize(
            &workspace_params,
            storage,
            &workspace_params_file,
        )
        .await?;

        CmdCtxBuilderSupport::profile_params_serialize(
            &profile_params,
            storage,
            &profile_params_file,
        )
        .await?;

        // Track items in memory.
        let mut resources = peace_resource_rt::Resources::new();

        CmdCtxBuilderSupport::workspace_params_insert(workspace_params.clone(), &mut resources);
        resources.insert(workspace_params_file);

        CmdCtxBuilderSupport::profile_params_insert(profile_params.clone(), &mut resources);
        resources.insert(profile_params_file);

        // Insert resources
        {
            let (app_name, workspace_dirs, storage) = (*workspace).clone().into_inner();
            let (workspace_dir, peace_dir, peace_app_dir) = workspace_dirs.into_inner();

            resources.insert(app_name);
            resources.insert(storage);
            resources.insert(workspace_dir);
            resources.insert(peace_dir);
            resources.insert(peace_app_dir);
            resources.insert(profile_dir.clone());
            resources.insert(profile_history_dir.clone());
            resources.insert(profile.clone());
        }

        let cmd_ctx_spnf = CmdCtxSpnf {
            output,
            fields: CmdCtxSpnfFields {
                interruptibility_state,
                workspace,
                profile,
                profile_dir,
                profile_history_dir,
                workspace_params_type_reg,
                workspace_params,
                profile_params_type_reg,
                profile_params,
            },
        };

        Ok(cmd_ctx_spnf)
    }
}

#[allow(non_camel_case_types)]
impl<
        'ctx,
        CmdCtxTypesT,
        __interruptibility: ::typed_builder::Optional<Interruptibility<'static>> + 'ctx,
    > IntoFuture
    for CmdCtxSpnfParamsBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            (ProfileSelection<'ctx, CmdCtxTypesT::WorkspaceParamsKey>,),
            (WorkspaceParamsOpt<CmdCtxTypesT::WorkspaceParamsKey>,),
            (ProfileParamsOpt<CmdCtxTypesT::ProfileParamsKey>,),
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Future that returns the `CmdCtxSpnf`.
    ///
    /// This is boxed since [TAIT] is not yet available ([rust#63063]).
    ///
    /// [TAIT]: https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html
    /// [rust#63063]: https://github.com/rust-lang/rust/issues/63063
    type IntoFuture =
        LocalBoxFuture<'ctx, Result<CmdCtxSpnf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError>>;
    type Output = <Self::IntoFuture as std::future::Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        self.build().boxed_local()
    }
}
