/* tslint:disable */
/**
* @param {number} size 
* @returns {number} 
*/
export function alloc(size: number): number;
/**
* @param {number} ptr 
* @param {number} cap 
*/
export function dealloc(ptr: number, cap: number): void;
/**
* @param {number} pointer 
* @param {number} max_width 
* @param {number} max_height 
*/
export function fill(pointer: number, max_width: number, max_height: number): void;
/**
* @param {number} a 
* @param {number} b 
* @returns {number} 
*/
export function add(a: number, b: number): number;
/**
* @param {number} alpha 
* @returns {number} 
*/
export function wsin(alpha: number): number;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        