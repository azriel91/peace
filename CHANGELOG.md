# Changelog

## unreleased

* Require `Display` to be implemented for `StateLogical` and `StatePhysical`. ([#28], [#37])
* Output states and diff as text on single line. ([#28], [#37])
* Support CLI output with colour with the `"output_colorized"` feature. ([#28], [#38])
* Support CLI output as YAML. ([#28], [#39])
* Support CLI output as JSON with the `"output_json"` feature. ([#28], [#39])

[#28]: https://github.com/azriel91/peace/issues/28
[#37]: https://github.com/azriel91/peace/pull/37
[#38]: https://github.com/azriel91/peace/pull/38
[#39]: https://github.com/azriel91/peace/pull/39

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
