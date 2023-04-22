# Changelog

## unreleased

* Add `ItemSpec::Params` associated type. ([#116], [#117])
* Rename `OpCtx` to `FnCtx`. ([#116], [#117])
* Update `ItemSpec` functions to take in `Self::Params`. ([#116], [#117])

[#116]: https://github.com/azriel91/peace/issues/116
[#117]: https://github.com/azriel91/peace/pull/117


## 0.0.9 (2023-04-13)

* Rename `app_cycle` example to `envman`. ([#35], [#107])
* Develop `envman` example to have sensible errors and state display messages. ([#35], [#107])
* Return both command outcome and errors in `ApplyCmd`. ([#107])
* `StatesDiscoverCmd` discovers states concurrently. ([#107])
* Serialize `StatesDesired` as part of `ApplyCmd`. ([#107])
* Consolidate `ItemSpec` functions into single trait. ([#96], [#109])
* Remove `StatesCurrentDiscoverCmd` and `StatesDesiredDiscoverCmd`. ([#110], [#111])
* Update `DiffCmd` to take in states to diff. ([#101], [#112])
* Add `DiffCmd::current_and_desired` to diff current and desired states of a profile. ([#113], [#114])
* Add `DiffCmd::diff_profiles_current` to diff current states of two profiles. ([#113], [#114])

[#107]: https://github.com/azriel91/peace/pull/107
[#96]: https://github.com/azriel91/peace/issues/96
[#109]: https://github.com/azriel91/peace/pull/109
[#110]: https://github.com/azriel91/peace/issues/110
[#111]: https://github.com/azriel91/peace/pull/111
[#101]: https://github.com/azriel91/peace/issues/101
[#112]: https://github.com/azriel91/peace/pull/112
[#113]: https://github.com/azriel91/peace/issues/113
[#114]: https://github.com/azriel91/peace/pull/114


## 0.0.8 (2023-03-25)

* Move `R, W, ROpt, WOpt, RMaybe, WMaybe` to `peace_data::accessors`. ([#94], [#95])
* Automatically insert `Current<ItemSpec::State>` after state current and ensure exec executions. ([#94], [#95])
* Automatically insert `Desired<ItemSpec::State>` after state desired discover execution. ([#94], [#95])
* Consolidate `EnsureOpSpec` and `CleanOpSpec` into `ApplyOpSpec`. ([#67], [#99])
* Add icons to CLI progress bars. ([#102], [#103])
* Add elapsed / ETA time to CLI progress bars. ([#102], [#103])
* Display messages in CLI progress bars. ([#102], [#103])
* Display progress bars during state discovery. ([#100], [#104])
* Clear progress bars on command end. ([#100], [#104])
* Include entry for current and discovered states, and diff in `envman` example. ([#91], [#105])
* Sort progress bars based on insertion order. ([#91], [#105])
* Use `▰` and `▱` parallelogram characters for progress bars. ([#91], [#105])
* Spinner progress is now rendered. ([#91], [#105])

[#94]: https://github.com/azriel91/peace/issues/94
[#95]: https://github.com/azriel91/peace/pull/95
[#67]: https://github.com/azriel91/peace/issues/67
[#99]: https://github.com/azriel91/peace/pull/99
[#102]: https://github.com/azriel91/peace/issues/102
[#103]: https://github.com/azriel91/peace/pull/103
[#100]: https://github.com/azriel91/peace/issues/100
[#104]: https://github.com/azriel91/peace/pull/104
[#91]: https://github.com/azriel91/peace/issues/91
[#105]: https://github.com/azriel91/peace/pull/105


## 0.0.7 (2023-03-06)

* Add [`cargo-deny`] and [`cargo-about`] CI checks. ([#76])
* Add [`peace::fmt::{Presentable, Presenter}`] traits. ([#77], [#79])
* Add [`peace::rt_model::CliMdPresenter`] which is the default for presenting text in `CliOutput`. ([#77], [#79])
* Hold `WorkspaceParams`, `ProfileParams`, and `FlowParams` type registries in `CmdContext`. ([#35], [#80])
* Add new `CmdCtx` that contains workspace / profile / flow scoped information. ([#81], [#82])
* Update examples to use `CmdCtx`. ([#83], [#85])
* Remove the old `CmdContext`. ([#83], [#85])

[`cargo-deny`]: https://github.com/EmbarkStudios/cargo-deny
[`cargo-about`]: https://github.com/EmbarkStudios/cargo-about
[#76]: https://github.com/azriel91/peace/pull/76
[#77]: https://github.com/azriel91/peace/issues/77
[#79]: https://github.com/azriel91/peace/pull/79
[#80]: https://github.com/azriel91/peace/pull/80
[#81]: https://github.com/azriel91/peace/issues/81
[#82]: https://github.com/azriel91/peace/pull/82
[#83]: https://github.com/azriel91/peace/issues/83
[#85]: https://github.com/azriel91/peace/pull/85


## 0.0.6 (2023-01-21)

* `FileDownload` item spec now supports base64 storage for WASM target. ([#62])
* Implement `TarXItemSpec` for native target. ([#62])
* Support multiple workspace, profile, and flow parameters. ([#45], [#63])
* Support progress bars in `CliOutput`. ([#42], [#66])
* Consolidate `StateLogical` and `StatePhysical` into `ItemSpec::State`. ([#69], [#70])
* Use [ETag] to determine if a file needs to be re-downloaded. ([#68], [#71])
* Add `PeaceAppDir` layer so different Peace tools don't conflict with each other. ([#35], [#72])
* Move `profile` and `flow_id` parameters to `CmdContextBuilder`. ([#35], [#73])
* Support reading `Profile` from workspace params. ([#35], [#73])

[ETag]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag
[#62]: https://github.com/azriel91/peace/pull/62
[#45]: https://github.com/azriel91/peace/issues/45
[#63]: https://github.com/azriel91/peace/pull/63
[#42]: https://github.com/azriel91/peace/issues/42
[#66]: https://github.com/azriel91/peace/pull/66
[#69]: https://github.com/azriel91/peace/issues/69
[#70]: https://github.com/azriel91/peace/pull/70
[#68]: https://github.com/azriel91/peace/issues/68
[#71]: https://github.com/azriel91/peace/pull/71
[#35]: https://github.com/azriel91/peace/issues/35
[#72]: https://github.com/azriel91/peace/pull/72
[#73]: https://github.com/azriel91/peace/pull/73


## 0.0.5 (2022-12-18)

* `ShCmdStateDiffFn` correctly runs `state_diff_sh_cmd` for state diff. ([#57])
* `ShCmdParams` now uses the `Id` type parameter so that different `ShCmdItemSpec`s can be used correctly. ([#57])
* `ShCmdItemSpec` takes in optional `ShCmdParams<Id>` and inserts it into `resources`. ([#57])
* `CmdContextBuilder` sets the current directory to the workspace directory. ([#57])
* `StatesDesired` is now stored as `State<Logical, Placeholder>`. ([#52], [#58])
* Re-read discovered `States` are now named `StatesSaved`. ([#52], [#60])
* `StatesCurrent` is only present when the discovered in the current execution. ([#52], [#60])
* `States*Deserialize` errors are consolidated into a single variant. ([#52], [#60])
* `States*Serialize` errors are consolidated into a single variant. ([#52], [#60])

[#57]: https://github.com/azriel91/peace/pull/57
[#52]: https://github.com/azriel91/peace/issues/52
[#58]: https://github.com/azriel91/peace/pull/58
[#60]: https://github.com/azriel91/peace/pull/60


## 0.0.4 (2022-11-29)

* Require `Display` to be implemented for `StateLogical` and `StatePhysical`. ([#28], [#37])
* Output states and diff as text on single line. ([#28], [#37])
* Support CLI output with colour with the `"output_colorized"` feature. ([#28], [#38])
* Support CLI output as YAML. ([#28], [#39])
* Support CLI output as JSON with the `"output_json"` feature. ([#28], [#39])
* Error compatibility with `miette` with the `"error_reporting"` feature. ([#28], [#40])
* Item specs are published as part of the `peace_item_specs` crate. ([#44])
* `file_download` item spec is type parameterized. ([#44])
* Add `ShCmdItemSpec`, which allows item spec logic to be defined by shell commands. ([#53], [#54])

[#28]: https://github.com/azriel91/peace/issues/28
[#37]: https://github.com/azriel91/peace/pull/37
[#38]: https://github.com/azriel91/peace/pull/38
[#39]: https://github.com/azriel91/peace/pull/39
[#40]: https://github.com/azriel91/peace/pull/40
[#44]: https://github.com/azriel91/peace/pull/44
[#53]: https://github.com/azriel91/peace/issues/53
[#54]: https://github.com/azriel91/peace/pull/54


## 0.0.3 (2022-09-30)

* Peace book &ndash; https://peace.mk/. ([#22], [#23])
* WASM Support. ([#20], [#21])
* Workspace, profile, and flow directories. ([#15], [#24], [#26])
* Workspace, profile, and flow initialization parameters. ([#26], [#29], [#30])
* `StatesDiscoverCmd` to discover both current and desired states. ([#24], [#26], [#27])
* `CleanCmd` to clean up an item. ([#33], [#34])

[#15]: https://github.com/azriel91/peace/issues/15
[#20]: https://github.com/azriel91/peace/issues/20
[#21]: https://github.com/azriel91/peace/pull/21
[#22]: https://github.com/azriel91/peace/issues/22
[#23]: https://github.com/azriel91/peace/pull/23
[#24]: https://github.com/azriel91/peace/issues/24
[#26]: https://github.com/azriel91/peace/pull/26
[#27]: https://github.com/azriel91/peace/pull/27
[#29]: https://github.com/azriel91/peace/issues/29
[#30]: https://github.com/azriel91/peace/pull/30
[#33]: https://github.com/azriel91/peace/issues/33
[#34]: https://github.com/azriel91/peace/pull/34


## 0.0.2 (2022-08-03)

* `StatesCurrentDiscoverCmd` to discover the current state.
* `StatesDesiredDiscoverCmd` to discover the desired state.
* `DiffCmd` to compute the difference between the current and desired states.
* `EnsureCmd` to transform the current state into the desired state. ([#17], [#18])

[#17]: https://github.com/azriel91/peace/issues/17
[#18]: https://github.com/azriel91/peace/pull/18
