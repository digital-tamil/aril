//! # aril-wasm
//!
//! WebAssembly interface for high-performance legacy Tamil font conversion
//! and PDF extraction across browsers, Node.js, and edge runtimes.

mod encodings;
mod pdf;

use aril_core::Encoding;
use encodings::{SUPPORTED_ENCODINGS, encoding_to_str, parse_user_encoding};
use pdf::{PdfConvertOptions, PdfDocumentResult, PdfMetadata};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Output of `auto_to_unicode`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextConversionResult {
    pub text: String,
    pub encoding: String,
}

/// Automatically hooks panics to `console.error` upon instantiation.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Explicitly hooks panics to `console.error`.
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Converts legacy Tamil text to Unicode.
///
/// If `encoding` is omitted, `null`, or `"auto"`, the layout is auto-detected.
#[wasm_bindgen]
pub fn to_unicode(input: &str, encoding: Option<String>) -> Result<String, JsValue> {
    let enc = match encoding {
        Some(ref name) => parse_user_encoding(name).map_err(|e| JsValue::from_str(&e))?,
        None => Encoding::Auto,
    };

    aril_core::to_unicode(input, enc).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Converts Unicode Tamil text back into a legacy font encoding layout.
#[wasm_bindgen]
pub fn to_legacy(input: &str, encoding: &str) -> Result<String, JsValue> {
    let enc = parse_user_encoding(encoding).map_err(|e| JsValue::from_str(&e))?;
    aril_core::to_legacy(input, enc).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Auto-detects the legacy encoding and converts the text into Unicode.
///
/// Returns an object: `{ text: string, encoding: string }`.
#[wasm_bindgen]
pub fn auto_to_unicode(input: &str) -> Result<JsValue, JsValue> {
    let (converted, encoding) =
        aril_core::auto_to_unicode(input).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let result = TextConversionResult {
        text: converted,
        encoding: encoding_to_str(encoding).to_string(),
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Identifies the legacy encoding layout of the input string without converting it.
#[wasm_bindgen]
pub fn detect_encoding(input: &str) -> Option<String> {
    aril_core::detect_encoding(input).map(|enc| encoding_to_str(enc).to_string())
}

/// Returns an array of all supported encoding names.
#[wasm_bindgen]
pub fn get_supported_encodings() -> Vec<String> {
    SUPPORTED_ENCODINGS
        .iter()
        .map(|&e| encoding_to_str(e).to_string())
        .collect()
}

/// Extracts text from a PDF byte slice and converts it to Unicode Tamil.
#[wasm_bindgen]
pub fn convert_pdf_to_unicode(
    pdf_bytes: &[u8],
    encoding: Option<String>,
) -> Result<String, JsValue> {
    let opts = PdfConvertOptions {
        encoding,
        ..Default::default()
    };
    let result = pdf::process_pdf(pdf_bytes, &opts).map_err(|e| JsValue::from_str(&e))?;
    Ok(result.combined_text)
}

/// Converts a single page (1-indexed) of a PDF to Unicode Tamil.
#[wasm_bindgen]
pub fn convert_pdf_page_to_unicode(
    pdf_bytes: &[u8],
    page_number: u32,
    encoding: Option<String>,
) -> Result<String, JsValue> {
    let opts = PdfConvertOptions {
        encoding,
        start_page: Some(page_number),
        end_page: Some(page_number),
        ..Default::default()
    };
    let result = pdf::process_pdf(pdf_bytes, &opts).map_err(|e| JsValue::from_str(&e))?;

    result
        .pages
        .into_iter()
        .next()
        .map(|p| p.text)
        .ok_or_else(|| JsValue::from_str(&format!("Page {page_number} not found")))
}

/// Converts a PDF document and returns a structured JavaScript object:
///
/// ```typescript
/// interface PdfDocumentResult {
///   totalPages: number;
///   detectedFonts: string[];
///   detectedEncoding?: string;
///   combinedText: string;
///   pages: Array<{
///     pageNumber: number;
///     text: string;
///     encoding: string;
///     rawText?: string;
///     error?: string;
///   }>;
/// }
/// ```
#[wasm_bindgen]
pub fn convert_pdf(pdf_bytes: &[u8], options: Option<JsValue>) -> Result<JsValue, JsValue> {
    let opts: PdfConvertOptions = if let Some(val) = options {
        serde_wasm_bindgen::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?
    } else {
        PdfConvertOptions::default()
    };

    let result: PdfDocumentResult =
        pdf::process_pdf(pdf_bytes, &opts).map_err(|e| JsValue::from_str(&e))?;

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Converts a PDF and serializes the structured result as a JSON string.
#[wasm_bindgen]
pub fn convert_pdf_to_json(pdf_bytes: &[u8], options: Option<JsValue>) -> Result<String, JsValue> {
    let opts: PdfConvertOptions = if let Some(val) = options {
        serde_wasm_bindgen::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?
    } else {
        PdfConvertOptions::default()
    };

    let result = pdf::process_pdf(pdf_bytes, &opts).map_err(|e| JsValue::from_str(&e))?;
    serde_json::to_string(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Extracts raw glyph characters from the PDF without conversion.
#[wasm_bindgen]
pub fn extract_pdf_raw_text(pdf_bytes: &[u8]) -> Result<String, JsValue> {
    pdf::extract_raw_text(pdf_bytes).map_err(|e| JsValue::from_str(&e))
}

/// Inspects the PDF and returns its total page count, fonts, and suggested encoding.
#[wasm_bindgen]
pub fn inspect_pdf(pdf_bytes: &[u8]) -> Result<JsValue, JsValue> {
    let meta: PdfMetadata = pdf::inspect_pdf(pdf_bytes).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&meta).map_err(|e| JsValue::from_str(&e.to_string()))
}
