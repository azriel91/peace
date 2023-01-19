use peace::{
    cfg::{app_name, item_spec_id, profile, AppName, FlowId, ItemSpecId, Profile, State},
    resources::states::{StateDiffs, StatesCleaned, StatesCurrent, StatesDesired, StatesEnsured},
    rt::cmds::{
        sub::{StatesCurrentDiscoverCmd, StatesDesiredDiscoverCmd},
        CleanCmd, DiffCmd, EnsureCmd, StatesDiscoverCmd,
    },
    rt_model::{CmdContext, InMemoryTextOutput, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};
use peace_item_specs::sh_cmd::{
    ShCmd, ShCmdError, ShCmdExecutionRecord, ShCmdItemSpec, ShCmdParams, ShCmdState, ShCmdStateDiff,
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
            let state_diff_sh_cmd = ShCmd::new("Powershell.exe").arg("-Command").arg(concat!(
                "& { ",
                include_str!("sh_cmd_item_spec/windows/test_file_creation_state_diff.ps1"),
                " }"
            ));
            let ensure_check_sh_cmd = ShCmd::new("Powershell.exe").arg("-Command").arg(concat!(
                "& { ",
                include_str!("sh_cmd_item_spec/windows/test_file_creation_ensure_check.ps1"),
                " }"
            ));
            let ensure_exec_sh_cmd = ShCmd::new("Powershell.exe").arg("-Command").arg(concat!(
                "& { ",
                include_str!("sh_cmd_item_spec/windows/test_file_creation_ensure_exec.ps1"),
                " }"
            ));
            let clean_check_sh_cmd = ShCmd::new("Powershell.exe").arg("-Command").arg(concat!(
                "& { ",
                include_str!("sh_cmd_item_spec/windows/test_file_creation_clean_check.ps1"),
                " }"
            ));
            let clean_exec_sh_cmd = ShCmd::new("Powershell.exe").arg("-Command").arg(concat!(
                "& { ",
                include_str!("sh_cmd_item_spec/windows/test_file_creation_clean_exec.ps1"),
                " }"
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
        app_name!(),
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
        app_name!(),
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
        .get::<State<TestFileCreationShCmdStateLogical, ShCmdExecutionRecord>, _>(
            &TestFileCreationShCmdItemSpec::ID,
        )
        .unwrap();
    if let ShCmdState::Some {
        stdout,
        stderr,
        marker: _,
    } = &state_desired.logical
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

#[tokio::test]
async fn state_diff_returns_shell_command_state_diff() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
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

    // Discover states current and desired
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Diff them
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext { resources, .. } = DiffCmd::exec(cmd_context).await?;

    let state_diffs = resources.borrow::<StateDiffs>();
    let state_diff = state_diffs
        .get::<ShCmdStateDiff, _>(&TestFileCreationShCmdItemSpec::ID)
        .unwrap();
    assert_eq!("creation_required", state_diff.stdout());
    assert_eq!("`test_file` will be created", state_diff.stderr());

    Ok(())
}

#[tokio::test]
async fn ensure_when_creation_required_executes_ensure_exec_shell_command()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
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

    // Discover states current and desired
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Create the file
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext { resources, .. } = EnsureCmd::exec(cmd_context).await?;

    let states_ensured = resources.borrow::<StatesEnsured>();
    let state_ensured = states_ensured
        .get::<TestFileCreationShCmdState, _>(&TestFileCreationShCmdItemSpec::ID)
        .unwrap();
    if let ShCmdState::Some {
        stdout,
        stderr,
        marker: _,
    } = &state_ensured.logical
    {
        assert_eq!("exists", stdout);
        assert_eq!("`test_file` exists", stderr);
    } else {
        panic!("Expected `state_ensured` to be `ShCmdState::Some` after `EnsureCmd` execution.");
    }

    Ok(())
}

#[tokio::test]
async fn ensure_when_exists_sync_does_not_reexecute_ensure_exec_shell_command()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
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

    // Discover states current and desired
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Create the file
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    EnsureCmd::exec(cmd_context).await?;

    // Diff state after creation
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext { resources, .. } = DiffCmd::exec(cmd_context).await?;

    let state_diffs = resources.borrow::<StateDiffs>();
    let state_diff = state_diffs
        .get::<ShCmdStateDiff, _>(&TestFileCreationShCmdItemSpec::ID)
        .unwrap();
    assert_eq!("exists_sync", state_diff.stdout());
    assert_eq!("nothing to do", state_diff.stderr());

    // Run again, for idempotence checck
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext { resources, .. } = EnsureCmd::exec(cmd_context).await?;

    let states_ensured = resources.borrow::<StatesEnsured>();
    let state_ensured = states_ensured
        .get::<TestFileCreationShCmdState, _>(&TestFileCreationShCmdItemSpec::ID)
        .unwrap();
    if let ShCmdState::Some {
        stdout,
        stderr,
        marker: _,
    } = &state_ensured.logical
    {
        assert_eq!("exists", stdout);
        assert_eq!("`test_file` exists", stderr);
    } else {
        panic!("Expected `state_ensured` to be `ShCmdState::Some` after `EnsureCmd` execution.");
    }

    Ok(())
}

#[tokio::test]
async fn clean_when_exists_sync_executes_shell_command() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::new(
        app_name!(),
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

    // Discover states current and desired
    StatesDiscoverCmd::exec(cmd_context).await?;

    // Create the file
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    EnsureCmd::exec(cmd_context).await?;

    assert!(tempdir.path().join("test_file").exists());

    // Clean the file
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    CleanCmd::exec(cmd_context).await?;

    assert!(!tempdir.path().join("test_file").exists());

    // Run again, for idempotence checck
    let cmd_context = CmdContext::builder(&workspace, &graph, &mut output).await?;
    let CmdContext { resources, .. } = CleanCmd::exec(cmd_context).await?;

    let states_cleaned = resources.borrow::<StatesCleaned>();
    let state_cleaned = states_cleaned
        .get::<TestFileCreationShCmdState, _>(&TestFileCreationShCmdItemSpec::ID)
        .unwrap();
    if let ShCmdState::Some {
        stdout,
        stderr,
        marker: _,
    } = &state_cleaned.logical
    {
        assert_eq!("not_exists", stdout);
        assert_eq!("`test_file` does not exist", stderr);
    } else {
        panic!("Expected `state_cleaned` to be `ShCmdState::Some` after `CleanCmd` execution.");
    }

    Ok(())
}
