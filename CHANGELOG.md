# Changelog

## unreleased

* `FileDownload` item spec now supports base64 storage for WASM target.
* Implement `TarXItemSpec` for native target. ([#62])

[#62]: https://github.com/azriel91/peace/pull/62


## 0.0.5 (2022-12-18)

* `ShCmdStateDiffFnSpec` correctly runs `state_diff_sh_cmd` for state diff. ([#57])
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
