/* tslint:disable */
/* eslint-disable */
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function md5(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha2_224(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha2_256(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha2_384(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha2_512(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha1(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha3_224(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha3_256(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha3_384(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sha3_512(blob: Blob, cb: Function): Promise<string>;
/**
* @param {Blob} blob
* @param {Function} cb
* @returns {Promise<string>}
*/
export function sm3(blob: Blob, cb: Function): Promise<string>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly md5: (a: number, b: number) => number;
  readonly sha2_224: (a: number, b: number) => number;
  readonly sha2_256: (a: number, b: number) => number;
  readonly sha2_384: (a: number, b: number) => number;
  readonly sha2_512: (a: number, b: number) => number;
  readonly sha1: (a: number, b: number) => number;
  readonly sha3_224: (a: number, b: number) => number;
  readonly sha3_256: (a: number, b: number) => number;
  readonly sha3_384: (a: number, b: number) => number;
  readonly sha3_512: (a: number, b: number) => number;
  readonly sm3: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbac97d8a46b64b05: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h0ad39b4c1c0bacfb: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
