/* tslint:disable */
/* eslint-disable */
/**
* @param {string} url
* @param {string} name
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_init(url: string, name: string): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_fetch(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_status(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_goal(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_diff(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_ensure_dry(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_ensure(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_clean_dry(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
* @param {WorkspaceAndOutput} workspace_and_output
* @returns {Promise<WorkspaceAndOutput>}
*/
export function wasm_clean(workspace_and_output: WorkspaceAndOutput): Promise<WorkspaceAndOutput>;
/**
*/
export class IntoUnderlyingByteSource {
  free(): void;
/**
* @param {ReadableByteStreamController} controller
*/
  start(controller: ReadableByteStreamController): void;
/**
* @param {ReadableByteStreamController} controller
* @returns {Promise<any>}
*/
  pull(controller: ReadableByteStreamController): Promise<any>;
/**
*/
  cancel(): void;
/**
*/
  readonly autoAllocateChunkSize: number;
/**
*/
  readonly type: string;
}
/**
*/
export class IntoUnderlyingSink {
  free(): void;
/**
* @param {any} chunk
* @returns {Promise<any>}
*/
  write(chunk: any): Promise<any>;
/**
* @returns {Promise<any>}
*/
  close(): Promise<any>;
/**
* @param {any} reason
* @returns {Promise<any>}
*/
  abort(reason: any): Promise<any>;
}
/**
*/
export class IntoUnderlyingSource {
  free(): void;
/**
* @param {ReadableStreamDefaultController} controller
* @returns {Promise<any>}
*/
  pull(controller: ReadableStreamDefaultController): Promise<any>;
/**
*/
  cancel(): void;
}
/**
*/
export class WorkspaceAndOutput {
  free(): void;
/**
*/
  output: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_workspaceandoutput_free: (a: number, b: number) => void;
  readonly __wbg_get_workspaceandoutput_output: (a: number, b: number) => void;
  readonly __wbg_set_workspaceandoutput_output: (a: number, b: number, c: number) => void;
  readonly wasm_init: (a: number, b: number, c: number, d: number) => number;
  readonly wasm_fetch: (a: number) => number;
  readonly wasm_status: (a: number) => number;
  readonly wasm_goal: (a: number) => number;
  readonly wasm_diff: (a: number) => number;
  readonly wasm_ensure_dry: (a: number) => number;
  readonly wasm_ensure: (a: number) => number;
  readonly wasm_clean_dry: (a: number) => number;
  readonly wasm_clean: (a: number) => number;
  readonly __wbg_intounderlyingbytesource_free: (a: number, b: number) => void;
  readonly intounderlyingbytesource_type: (a: number, b: number) => void;
  readonly intounderlyingbytesource_autoAllocateChunkSize: (a: number) => number;
  readonly intounderlyingbytesource_start: (a: number, b: number) => void;
  readonly intounderlyingbytesource_pull: (a: number, b: number) => number;
  readonly intounderlyingbytesource_cancel: (a: number) => void;
  readonly __wbg_intounderlyingsink_free: (a: number, b: number) => void;
  readonly intounderlyingsink_write: (a: number, b: number) => number;
  readonly intounderlyingsink_close: (a: number) => number;
  readonly intounderlyingsink_abort: (a: number, b: number) => number;
  readonly __wbg_intounderlyingsource_free: (a: number, b: number) => void;
  readonly intounderlyingsource_pull: (a: number, b: number) => number;
  readonly intounderlyingsource_cancel: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1f9659fe31371ed4: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h0eeca02dec7b2f2b: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
