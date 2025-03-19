use std::{collections::BTreeMap, fmt::Debug};

use interruptible::Interruptibility;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_profile_model::Profile;
use peace_resource_rt::internal::WorkspaceParamsFile;
use peace_rt_model::{
    params::{ProfileParams, WorkspaceParams},
    Workspace, WorkspaceInitializer,
};
use type_reg::untagged::TypeReg;
use typed_builder::TypedBuilder;

use crate::{
    CmdCtxBuilderSupport, CmdCtxBuilderSupportMulti, CmdCtxMpnf, CmdCtxMpnfFields, CmdCtxTypes,
    ProfileFilterFn,
};

/// A command that works with multiple profiles, not scoped to a flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
/// |   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
/// |   |
/// |   |- ..                      # ❌ cannot read or write `Flow` information
/// |
/// |- 🌏 customer_a_dev           # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |
/// |- 🌏 customer_a_prod          # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |
/// |- 🌏 workspace_init           # ✅ can list multiple `Profile`s
///     |- 📝 profile_params.yaml  # ❌ cannot read profile params of different underlying type
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write multiple profiles' parameters &ndash; as long as they are of
///   the same type (same `struct`).
///
/// This kind of command cannot:
///
/// * Read or write flow parameters -- see [`CmdCtxMpsf`].
/// * Read or write flow state -- see [`CmdCtxMpsf`].
///
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
#[derive(Debug, TypedBuilder)]
#[builder(build_method(vis="", name=build_partial))]
pub struct CmdCtxMpnfParams<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,
    /// The interrupt channel receiver if this `CmdExecution` is interruptible.
    #[builder(default = Interruptibility::NonInterruptible)]
    pub interruptibility: Interruptibility<'static>,
    /// Workspace that the `peace` tool runs in.
    pub workspace: OwnedOrRef<'ctx, Workspace>,
    /// Function to filter the profiles that are accessible by this command.
    #[builder(default = None)]
    pub profile_filter_fn: Option<ProfileFilterFn>,
    /// Workspace params.
    pub workspace_params: WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey>,
    /// Profile params for the profile.
    pub profile_to_profile_params: BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>,
}

// Use one of the following to obtain the generated type signature:
//
// ```sh
// cargo expand -p peace_cmd_ctx cmd_ctx_mpnf_builder
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
        __profile_filter_fn: ::typed_builder::Optional<Option<ProfileFilterFn>>,
    >
    CmdCtxMpnfParamsBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            __profile_filter_fn,
            (WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey>,),
            (BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>,),
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    pub async fn build(self) -> Result<CmdCtxMpnf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError> {
        let CmdCtxMpnfParams {
            output,
            interruptibility,
            workspace,
            profile_filter_fn,
            mut workspace_params,
            profile_to_profile_params: profile_to_profile_params_provided,
        } = self.build_partial();

        let workspace_params_type_reg = TypeReg::new();
        let profile_params_type_reg = TypeReg::new();

        let workspace_dirs = workspace.dirs();
        let storage = workspace.storage();

        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());
        CmdCtxBuilderSupport::workspace_params_merge(
            storage,
            &workspace_params_type_reg,
            &mut workspace_params,
            &workspace_params_file,
        )
        .await?;

        let profiles = CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profiles_from_peace_app_dir(
            workspace_dirs.peace_app_dir(),
            profile_filter_fn.as_ref(),
        )
        .await?;

        let (profile_dirs, profile_history_dirs) =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profile_and_history_dirs_read(
                &profiles,
                workspace_dirs,
            );

        let mut dirs_to_create = vec![
            AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
        ];

        profile_dirs
            .values()
            .map(AsRef::<std::path::Path>::as_ref)
            .chain(
                profile_history_dirs
                    .values()
                    .map(AsRef::<std::path::Path>::as_ref),
            )
            .for_each(|dir| dirs_to_create.push(dir));

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

        // profile_params_deserialize
        let profile_to_profile_params =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profile_params_deserialize(
                &profile_dirs,
                profile_to_profile_params_provided,
                storage,
                &profile_params_type_reg,
            )
            .await?;

        let interruptibility_state = interruptibility.into();

        // Serialize params to `PeaceAppDir`.
        CmdCtxBuilderSupport::workspace_params_serialize(
            &workspace_params,
            storage,
            &workspace_params_file,
        )
        .await?;

        // profile_params_serialize
        CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profile_params_serialize(
            &profile_to_profile_params,
            &profile_dirs,
            storage,
        )
        .await?;

        let cmd_ctx_mpnf = CmdCtxMpnf {
            output,
            fields: CmdCtxMpnfFields {
                interruptibility_state,
                workspace,
                profiles,
                profile_dirs,
                profile_history_dirs,
                workspace_params_type_reg,
                workspace_params,
                profile_params_type_reg,
                profile_to_profile_params,
            },
        };

        Ok(cmd_ctx_mpnf)
    }
}
