/* tslint:disable */
/* eslint-disable */

/**
 * Auto-detects the legacy encoding and converts the text into Unicode.
 *
 * Returns an object: `{ text: string, encoding: string }`.
 */
export function auto_to_unicode(input: string): any;

/**
 * Converts a PDF document and returns a structured JavaScript object:
 *
 * ```typescript
 * interface PdfDocumentResult {
 *   totalPages: number;
 *   detectedFonts: string[];
 *   detectedEncoding?: string;
 *   combinedText: string;
 *   pages: Array<{
 *     pageNumber: number;
 *     text: string;
 *     encoding: string;
 *     rawText?: string;
 *     error?: string;
 *   }>;
 * }
 * ```
 */
export function convert_pdf(pdf_bytes: Uint8Array, options?: any | null): any;

/**
 * Converts a single page (1-indexed) of a PDF to Unicode Tamil.
 */
export function convert_pdf_page_to_unicode(pdf_bytes: Uint8Array, page_number: number, encoding?: string | null): string;

/**
 * Converts a PDF and serializes the structured result as a JSON string.
 */
export function convert_pdf_to_json(pdf_bytes: Uint8Array, options?: any | null): string;

/**
 * Extracts text from a PDF byte slice and converts it to Unicode Tamil.
 */
export function convert_pdf_to_unicode(pdf_bytes: Uint8Array, encoding?: string | null): string;

/**
 * Identifies the legacy encoding layout of the input string without converting it.
 */
export function detect_encoding(input: string): string | undefined;

/**
 * Extracts raw glyph characters from the PDF without conversion.
 */
export function extract_pdf_raw_text(pdf_bytes: Uint8Array): string;

/**
 * Returns an array of all supported encoding names.
 */
export function get_supported_encodings(): string[];

/**
 * Explicitly hooks panics to `console.error`.
 */
export function init_panic_hook(): void;

/**
 * Inspects the PDF and returns its total page count, fonts, and suggested encoding.
 */
export function inspect_pdf(pdf_bytes: Uint8Array): any;

/**
 * Automatically hooks panics to `console.error` upon instantiation.
 */
export function start(): void;

/**
 * Converts Unicode Tamil text back into a legacy font encoding layout.
 */
export function to_legacy(input: string, encoding: string): string;

/**
 * Converts legacy Tamil text to Unicode.
 *
 * If `encoding` is omitted, `null`, or `"auto"`, the layout is auto-detected.
 */
export function to_unicode(input: string, encoding?: string | null): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly auto_to_unicode: (a: number, b: number, c: number) => void;
    readonly convert_pdf: (a: number, b: number, c: number, d: number) => void;
    readonly convert_pdf_page_to_unicode: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly convert_pdf_to_json: (a: number, b: number, c: number, d: number) => void;
    readonly convert_pdf_to_unicode: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly detect_encoding: (a: number, b: number, c: number) => void;
    readonly extract_pdf_raw_text: (a: number, b: number, c: number) => void;
    readonly get_supported_encodings: (a: number) => void;
    readonly init_panic_hook: () => void;
    readonly inspect_pdf: (a: number, b: number, c: number) => void;
    readonly start: () => void;
    readonly to_legacy: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly to_unicode: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export4: (a: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
