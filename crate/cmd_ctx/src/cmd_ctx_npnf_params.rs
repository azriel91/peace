use std::future::IntoFuture;

use futures::{future::LocalBoxFuture, FutureExt};
use interruptible::Interruptibility;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_params::ParamsValue;
use peace_resource_rt::internal::WorkspaceParamsFile;
use peace_rt_model::{params::WorkspaceParamsOpt, Workspace, WorkspaceInitializer};
use type_reg::untagged::TypeReg;
use typed_builder::TypedBuilder;

use crate::{CmdCtxBuilderSupport, CmdCtxNpnf, CmdCtxNpnfFields, CmdCtxTypes};

/// Context for a command that only works with workspace parameters -- no
/// profile / no flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 ..                       # ❌ cannot read or write `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
///
/// This kind of command cannot:
///
/// * Read or write profile parameters -- see [`CmdCtxSpnf`] or [`CmdCtxMpnf`].
/// * Read or write flow parameters or state -- see [`CmdCtxSpsf`] or
///   [`CmdCtxMpsf`].
///
/// [`CmdCtxMpnf`]: crate::CmdCtxMpnf
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
/// [`CmdCtxSpnf`]: crate::CmdCtxSpnf
/// [`CmdCtxSpsf`]: crate::CmdCtxSpsf
#[derive(Debug, TypedBuilder)]
#[builder(build_method(vis="", name=build_partial))]
pub struct CmdCtxNpnfParams<'ctx, CmdCtxTypesT>
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
}

// Use one of the following to obtain the generated type signature:
//
// ```sh
// cargo expand -p peace_cmd_ctx cmd_ctx_npnf_builder
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
    CmdCtxNpnfParamsBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            (WorkspaceParamsOpt<CmdCtxTypesT::WorkspaceParamsKey>,),
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    pub async fn build(self) -> Result<CmdCtxNpnf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError> {
        let CmdCtxNpnfParams {
            output,
            interruptibility,
            workspace,
            workspace_params: workspace_params_provided,
        } = self.build_partial();

        let mut workspace_params_type_reg = TypeReg::new();
        CmdCtxTypesT::workspace_params_register(&mut workspace_params_type_reg);

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

        let dirs_to_create = [
            AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
        ];

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        {
            WorkspaceInitializer::dirs_create(storage, dirs_to_create).await?;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            WorkspaceInitializer::dirs_create(dirs_to_create).await?;

            let workspace_dir = workspace_dirs.workspace_dir();
            std::env::set_current_dir(workspace_dir).map_err(|error| {
                peace_rt_model::Error::Native(peace_rt_model::NativeError::CurrentDirSet {
                    workspace_dir: workspace_dir.clone(),
                    error,
                })
            })?;
        }

        let interruptibility_state = interruptibility.into();

        // Serialize params to `PeaceAppDir`.
        CmdCtxBuilderSupport::workspace_params_serialize(
            &workspace_params,
            storage,
            &workspace_params_file,
        )
        .await?;

        let cmd_ctx_npnf = CmdCtxNpnf {
            output,
            fields: CmdCtxNpnfFields {
                interruptibility_state,
                workspace,
                workspace_params_type_reg,
                workspace_params,
            },
        };

        Ok(cmd_ctx_npnf)
    }
}

#[allow(non_camel_case_types)]
impl<
        'ctx,
        CmdCtxTypesT,
        __interruptibility: ::typed_builder::Optional<Interruptibility<'static>> + 'ctx,
    > IntoFuture
    for CmdCtxNpnfParamsBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            (WorkspaceParamsOpt<CmdCtxTypesT::WorkspaceParamsKey>,),
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Future that returns the `CmdCtxNpnf`.
    ///
    /// This is boxed since [TAIT] is not yet available ([rust#63063]).
    ///
    /// [TAIT]: https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html
    /// [rust#63063]: https://github.com/rust-lang/rust/issues/63063
    type IntoFuture =
        LocalBoxFuture<'ctx, Result<CmdCtxNpnf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError>>;
    type Output = <Self::IntoFuture as std::future::Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        self.build().boxed_local()
    }
}
