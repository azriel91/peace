use peace::{
    cfg::{item_spec_id, profile, FlowId, ItemSpecId, Profile, State},
    resources::states::{StatesCurrent, StatesDesired},
    rt::cmds::sub::{StatesCurrentDiscoverCmd, StatesDesiredDiscoverCmd},
    rt_model::{CmdContext, InMemoryTextOutput, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};
use peace_item_specs::sh_cmd::{
    ShCmd, ShCmdError, ShCmdExecutionRecord, ShCmdItemSpec, ShCmdParams, ShCmdState,
};

/// Creates a file.
#[derive(Clone, Copy, Debug)]
pub struct TestFileCreationShCmdItemSpec;

pub type TestFileCreationShCmdStateLogical = ShCmdState<TestFileCreationShCmdItemSpec>;
pub type TestFileCreationShCmdState =
    State<TestFileCreationShCmdStateLogical, ShCmdExecutionRecord>;

impl TestFileCreationShCmdItemSpec {
    /// ID
    pub const ID: ItemSpecId = item_spec_id!("test_file_creation");

    /// Returns a new `TestFileCreationShCmdItemSpec`.
    pub fn new() -> ShCmdItemSpec<Self> {
        #[cfg(unix)]
        let sh_cmd_params = {
            let state_current_sh_cmd = ShCmd::new("bash").arg("-c").arg(include_str!(
                "sh_cmd_item_spec/unix/test_file_creation_state_current.sh"
            ));

            let state_desired_sh_cmd = ShCmd::new("bash").arg("-c").arg(include_str!(
                "sh_cmd_item_spec/unix/test_file_creation_state_desired.sh"
            ));
            let state_diff_sh_cmd = ShCmd::new("bash").arg("-c").arg(include_str!(
                "sh_cmd_item_spec/unix/test_file_creation_state_diff.sh"
            ));
            let ensure_check_sh_cmd = ShCmd::new("bash").arg("-c").arg(include_str!(
                "sh_cmd_item_spec/unix/test_file_creation_ensure_check.sh"
            ));
            let ensure_exec_sh_cmd = ShCmd::new("bash").arg("-c").arg(include_str!(
                "sh_cmd_item_spec/unix/test_file_creation_ensure_exec.sh"
            ));
            let clean_check_sh_cmd = ShCmd::new("bash").arg("-c").arg(include_str!(
                "sh_cmd_item_spec/unix/test_file_creation_clean_check.sh"
            ));
            let clean_exec_sh_cmd = ShCmd::new("bash").arg("-c").arg(include_str!(
                "sh_cmd_item_spec/unix/test_file_creation_clean_exec.sh"
            ));
            ShCmdParams::<TestFileCreationShCmdItemSpec>::new(
                state_current_sh_cmd,
                state_desired_sh_cmd,
                state_diff_sh_cmd,
                ensure_check_sh_cmd,
                ensure_exec_sh_cmd,
                clean_check_sh_cmd,
                clean_exec_sh_cmd,
            )
        };

        #[cfg(windows)]
        let sh_cmd_params = {
            let state_current_sh_cmd =
                ShCmd::new("Powershell.exe")
                    .arg("-Command")
                    .arg(include_str!(
                        "sh_cmd_item_spec/windows/test_file_creation_state_current.ps1"
                    ));

            let state_desired_sh_cmd =
                ShCmd::new("Powershell.exe")
                    .arg("-Command")
                    .arg(include_str!(
                        "sh_cmd_item_spec/windows/test_file_creation_state_desired.ps1"
                    ));
            let state_diff_sh_cmd = ShCmd::new("Powershell.exe")
                .arg("-Command")
                .arg(include_str!(
                    "sh_cmd_item_spec/windows/test_file_creation_state_diff.ps1"
                ));
            let ensure_check_sh_cmd =
                ShCmd::new("Powershell.exe")
                    .arg("-Command")
                    .arg(include_str!(
                        "sh_cmd_item_spec/windows/test_file_creation_ensure_check.ps1"
                    ));
            let ensure_exec_sh_cmd =
                ShCmd::new("Powershell.exe")
                    .arg("-Command")
                    .arg(include_str!(
                        "sh_cmd_item_spec/windows/test_file_creation_ensure_exec.ps1"
                    ));
            let clean_check_sh_cmd =
                ShCmd::new("Powershell.exe")
                    .arg("-Command")
                    .arg(include_str!(
                        "sh_cmd_item_spec/windows/test_file_creation_clean_check.ps1"
                    ));
            let clean_exec_sh_cmd = ShCmd::new("Powershell.exe")
                .arg("-Command")
                .arg(include_str!(
                    "sh_cmd_item_spec/windows/test_file_creation_clean_exec.ps1"
                ));
            ShCmdParams::<TestFileCreationShCmdItemSpec>::new(
                state_current_sh_cmd,
                state_desired_sh_cmd,
                state_diff_sh_cmd,
                ensure_check_sh_cmd,
                ensure_exec_sh_cmd,
                clean_check_sh_cmd,
                clean_exec_sh_cmd,
            )
        };

        ShCmdItemSpec::new(Self::ID, Some(sh_cmd_params))
    }
}

#[tokio::test]
async fn state_current_returns_shell_command_current_state()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<ShCmdError>::new();
        graph_builder.add_fn(TestFileCreationShCmdItemSpec::new().into());
        graph_builder.build()
    };
    let mut output = InMemoryTextOutput::new();
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;

    let CmdContext { resources, .. } = StatesCurrentDiscoverCmd::exec(cmd_context).await?;
    let states_current = resources.borrow::<StatesCurrent>();
    let state_current = states_current
        .get::<TestFileCreationShCmdState, _>(&TestFileCreationShCmdItemSpec::ID)
        .unwrap();
    if let ShCmdState::Some {
        stdout,
        stderr,
        marker: _,
    } = &state_current.logical
    {
        assert_eq!("not_exists", stdout);
        assert_eq!("`test_file` does not exist", stderr);
    } else {
        panic!(
            "Expected `state_current` to be `ShCmdState::Some` after `StatesCurrent` discovery."
        );
    }

    Ok(())
}

#[tokio::test]
async fn state_desired_returns_shell_command_desired_state()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        WorkspaceSpec::Path(tempdir.path().to_path_buf()),
        profile!("test_profile"),
        FlowId::new(crate::fn_name_short!())?,
    )?;
    let graph = {
        let mut graph_builder = ItemSpecGraphBuilder::<ShCmdError>::new();
        graph_builder.add_fn(TestFileCreationShCmdItemSpec::new().into());
        graph_builder.build()
    };
    let mut output = InMemoryTextOutput::new();
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;

    let CmdContext { resources, .. } = StatesDesiredDiscoverCmd::exec(cmd_context).await?;
    let states_desired = resources.borrow::<StatesDesired>();
    let state_desired = states_desired
        .get::<TestFileCreationShCmdStateLogical, _>(&TestFileCreationShCmdItemSpec::ID)
        .unwrap();
    if let ShCmdState::Some {
        stdout,
        stderr,
        marker: _,
    } = &state_desired
    {
        assert_eq!("exists", stdout);
        assert_eq!("`test_file` exists", stderr);
    } else {
        panic!(
            "Expected `state_desired` to be `ShCmdState::Some` after `StatesDesired` discovery."
        );
    }

    Ok(())
}
