/* tslint:disable */
/* eslint-disable */
/**
* @returns {any}
*/
export function get_memory(): any;
/**
* @param {number} len
* @returns {number}
*/
export function allocate_vector(len: number): number;
/**
* @param {number} ptr
* @param {number} len
*/
export function deallocate_vector(ptr: number, len: number): void;
/**
*/
export class PostcodeRangeLookup {
  free(): void;
/**
* @returns {(string)[]}
*/
  get_value_values(): (string)[];
/**
* @returns {PostcodeRangeLookup}
*/
  static from_binary_data(): PostcodeRangeLookup;
/**
* @param {string} postcodes
* @param {string} delimiter
* @param {boolean} check_valid_postcode
* @returns {string}
*/
  get_values_str(postcodes: string, delimiter: string, check_valid_postcode: boolean): string;
/**
* @param {(string)[]} postcodes
* @param {boolean} check_valid_postcode
* @returns {(string)[]}
*/
  get_values_str_vec(postcodes: (string)[], check_valid_postcode: boolean): (string)[];
/**
* @param {Float64Array} postcodes
* @returns {Uint32Array}
*/
  get_values_float_array(postcodes: Float64Array): Uint32Array;
/**
* @param {BigUint64Array} postcodes
* @returns {Uint32Array}
*/
  get_values_vec_array(postcodes: BigUint64Array): Uint32Array;
/**
* @param {BigUint64Array} postcodes
* @param {boolean} check_valid_postcode
* @returns {Uint32Array}
*/
  get_values_vec(postcodes: BigUint64Array, check_valid_postcode: boolean): Uint32Array;
/**
* @param {bigint} int_postcode
* @param {boolean} check_valid_postcode
* @returns {number | undefined}
*/
  get_value_int(int_postcode: bigint, check_valid_postcode: boolean): number | undefined;
/**
* @param {string} postcode
* @param {boolean} check_valid_postcode
* @returns {string | undefined}
*/
  get_value(postcode: string, check_valid_postcode: boolean): string | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_memory: () => number;
  readonly allocate_vector: (a: number) => number;
  readonly deallocate_vector: (a: number, b: number) => void;
  readonly __wbg_postcoderangelookup_free: (a: number) => void;
  readonly postcoderangelookup_get_value_values: (a: number, b: number) => void;
  readonly postcoderangelookup_from_binary_data: () => number;
  readonly postcoderangelookup_get_values_str: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly postcoderangelookup_get_values_str_vec: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly postcoderangelookup_get_values_float_array: (a: number, b: number, c: number, d: number) => void;
  readonly postcoderangelookup_get_values_vec_array: (a: number, b: number, c: number) => void;
  readonly postcoderangelookup_get_values_vec: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly postcoderangelookup_get_value_int: (a: number, b: number, c: number, d: number) => void;
  readonly postcoderangelookup_get_value: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
