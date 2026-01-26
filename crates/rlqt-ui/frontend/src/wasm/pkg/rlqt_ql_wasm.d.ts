/* tslint:disable */
/* eslint-disable */

export function complete_field(prefix: string): any;

export function complete_label(prefix: string): any;

export function complete_preset(prefix: string): any;

export function complete_severity(prefix: string): any;

export function complete_subsystem(prefix: string): any;

export function get_autocomplete_data(): any;

export function init(): void;

export function suggest_field(input: string): any;

export function suggest_label(input: string): any;

export function suggest_preset(input: string): any;

export function suggest_severity(input: string): any;

export function suggest_subsystem(input: string): any;

export function validate_query(input: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly complete_field: (a: number, b: number) => number;
    readonly complete_label: (a: number, b: number) => number;
    readonly complete_preset: (a: number, b: number) => number;
    readonly complete_severity: (a: number, b: number) => number;
    readonly complete_subsystem: (a: number, b: number) => number;
    readonly get_autocomplete_data: () => number;
    readonly suggest_field: (a: number, b: number) => number;
    readonly suggest_label: (a: number, b: number) => number;
    readonly suggest_preset: (a: number, b: number) => number;
    readonly suggest_severity: (a: number, b: number) => number;
    readonly suggest_subsystem: (a: number, b: number) => number;
    readonly validate_query: (a: number, b: number) => number;
    readonly init: () => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
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
