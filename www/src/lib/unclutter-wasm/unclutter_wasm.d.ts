/* tslint:disable */
/* eslint-disable */
export interface Article {
    byline?: string;
    content: string;
    dir?: string;
    excerpt?: string;
    lang?: string;
    length: number;
    publishedTime?: string;
    siteName?: string;
    textContent: string;
    title: string;
}

export interface Options {
    classesToPreserve?: string[];
    keepClasses?: boolean | undefined;
    linkDensityBias?: number | undefined;
    maxElements?: number | undefined;
    minTextLength?: number | undefined;
    nTopCandidates?: number | undefined;
    useJsonLd?: boolean | undefined;
}


export function isReadable(html: string): boolean;

export function parse(html: string, base_url?: string | null, options?: Options | null): Article;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly isReadable: (a: number, b: number) => number;
    readonly parse: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
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
