# Workspace

When a command is run, Peace stores information about that execution.

It stores this in a `.peace` directory within a workspace directory. The automation tool developer needs to tell Peace how to determine the workspace directory.

```rust ,ignore
let cmd_ctx = CmdCtxSpsf::builder
    ::<EnvManError, _>(output, workspace)
    // ..
    .await?;
```


```rust ,ignore
pub enum WorkspaceSpec {
    /// Use the exe working directory as the workspace directory.
    ///
    /// The working directory is the directory that the user ran the program in.
    WorkingDir,
    /// Use a specified path.
    Path(PathBuf),
    /// Traverse up from the working directory until the given file is found.
    ///
    /// The workspace directory is the parent directory that contains a file or
    /// directory with the provided name.
    FirstDirWithFile(OsString),
}
```
