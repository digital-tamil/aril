use aril_core::Encoding;
use lopdf::{Document, ObjectId};
use serde::{Deserialize, Serialize};

use crate::encodings::{detect_encoding_from_font, encoding_to_str, parse_user_encoding};

/// Single converted page result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfPageResult {
    pub page_number: u32,
    pub text: String,
    pub encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Full document conversion result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfDocumentResult {
    pub total_pages: u32,
    pub detected_fonts: Vec<String>,
    pub detected_encoding: Option<String>,
    pub combined_text: String,
    pub pages: Vec<PdfPageResult>,
}

/// Document metadata inspection output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfMetadata {
    pub total_pages: u32,
    pub fonts: Vec<String>,
    pub suggested_encoding: Option<String>,
}

/// Configuration options passed from JavaScript/TypeScript.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PdfConvertOptions {
    pub encoding: Option<String>,
    pub include_raw: Option<bool>,
    pub start_page: Option<u32>,
    pub end_page: Option<u32>,
}

/// Scans all objects in the PDF for `/BaseFont` and `/FontName` entries.
#[must_use]
pub fn extract_all_fonts(doc: &Document) -> Vec<String> {
    let mut fonts = Vec::new();
    for obj in doc.objects.values() {
        if let Ok(dict) = obj.as_dict() {
            let font_entry = dict.get(b"BaseFont").or_else(|_| dict.get(b"FontName"));
            if let Ok(font_obj) = font_entry
                && let Ok(name_bytes) = font_obj.as_name() {
                    let full = String::from_utf8_lossy(name_bytes);
                    let font_name = full.split('+').next_back().unwrap_or(&full).to_string();
                    if !fonts.contains(&font_name) {
                        fonts.push(font_name);
                    }
                }
        }
    }
    fonts
}

/// Retrieves fonts associated with a specific page.
#[must_use]
pub fn extract_page_fonts(doc: &Document, page_id: ObjectId) -> Vec<String> {
    let mut fonts = Vec::new();
    if let Ok(font_map) = doc.get_page_fonts(page_id) {
        for (_, dict) in font_map {
            let font_entry = dict.get(b"BaseFont").or_else(|_| dict.get(b"FontName"));
            if let Ok(font_obj) = font_entry
                && let Ok(name_bytes) = font_obj.as_name() {
                    let full = String::from_utf8_lossy(name_bytes);
                    let font_name = full.split('+').next_back().unwrap_or(&full).to_string();
                    if !fonts.contains(&font_name) {
                        fonts.push(font_name);
                    }
                }
        }
    }
    fonts
}

/// Processes the PDF buffer and converts extracted text to Unicode Tamil.
pub fn process_pdf(
    pdf_bytes: &[u8],
    options: &PdfConvertOptions,
) -> Result<PdfDocumentResult, String> {
    let doc = Document::load_mem(pdf_bytes).map_err(|e| format!("Invalid PDF document: {e}"))?;

    let detected_fonts = extract_all_fonts(&doc);
    let doc_suggested_encoding = detected_fonts
        .iter()
        .find_map(|f| detect_encoding_from_font(f));

    let forced_encoding = match &options.encoding {
        Some(name) => match parse_user_encoding(name)? {
            Encoding::Auto => None,
            enc => Some(enc),
        },
        None => None,
    };

    let page_map = doc.get_pages();
    let total_pages = page_map.len() as u32;

    if total_pages == 0 {
        return Ok(PdfDocumentResult {
            total_pages: 0,
            detected_fonts,
            detected_encoding: None,
            combined_text: String::new(),
            pages: Vec::new(),
        });
    }

    let start_page = options.start_page.unwrap_or(1).clamp(1, total_pages);
    let end_page = options
        .end_page
        .unwrap_or(total_pages)
        .clamp(start_page, total_pages);
    let include_raw = options.include_raw.unwrap_or(false);

    let mut pages = Vec::with_capacity((end_page - start_page + 1) as usize);
    let mut combined_parts = Vec::with_capacity(pages.capacity());
    let mut resolved_encodings = Vec::new();

    for page_num in start_page..=end_page {
        let page_id = page_map.get(&page_num).copied();

        match doc.extract_text(&[page_num]) {
            Ok(raw_text) => {
                if raw_text.trim().is_empty() {
                    pages.push(PdfPageResult {
                        page_number: page_num,
                        text: String::new(),
                        encoding: "Empty".to_string(),
                        raw_text: if include_raw { Some(raw_text) } else { None },
                        error: None,
                    });
                    continue;
                }

                // Check page-level font references if available
                let page_font_enc = page_id
                    .map(|pid| extract_page_fonts(&doc, pid))
                    .and_then(|f_list| f_list.iter().find_map(|f| detect_encoding_from_font(f)));

                // Resolution priority:
                // - User manual override
                // - Statistical/heuristic detection on raw page text
                // - Page-level font dictionary metadata
                // - Document-level font dictionary metadata
                // - auto_to_unicode fallback
                let (converted, enc) = if let Some(enc) = forced_encoding {
                    let conv =
                        aril_core::to_unicode(&raw_text, enc).unwrap_or_else(|_| raw_text.clone());
                    (conv, enc)
                } else if let Some(detected) = aril_core::detect_encoding(&raw_text) {
                    let conv = aril_core::to_unicode(&raw_text, detected)
                        .unwrap_or_else(|_| raw_text.clone());
                    (conv, detected)
                } else if let Some(font_enc) = page_font_enc.or(doc_suggested_encoding) {
                    let conv = aril_core::to_unicode(&raw_text, font_enc)
                        .unwrap_or_else(|_| raw_text.clone());
                    (conv, font_enc)
                } else {
                    match aril_core::auto_to_unicode(&raw_text) {
                        Ok((conv, detected)) => (conv, detected),
                        Err(_) => (raw_text.clone(), Encoding::Unicode),
                    }
                };

                resolved_encodings.push(enc);
                combined_parts.push(converted.clone());

                pages.push(PdfPageResult {
                    page_number: page_num,
                    text: converted,
                    encoding: encoding_to_str(enc).to_string(),
                    raw_text: if include_raw { Some(raw_text) } else { None },
                    error: None,
                });
            }
            Err(e) => {
                pages.push(PdfPageResult {
                    page_number: page_num,
                    text: String::new(),
                    encoding: "Error".to_string(),
                    raw_text: None,
                    error: Some(format!("Failed to extract page {page_num}: {e}")),
                });
            }
        }
    }

    let overall_encoding = if let Some(enc) = forced_encoding {
        Some(encoding_to_str(enc).to_string())
    } else if let Some(first) = resolved_encodings.first() {
        Some(encoding_to_str(*first).to_string())
    } else {
        doc_suggested_encoding.map(|e| encoding_to_str(e).to_string())
    };

    let combined_text = combined_parts.join("\n\n");

    Ok(PdfDocumentResult {
        total_pages,
        detected_fonts,
        detected_encoding: overall_encoding,
        combined_text,
        pages,
    })
}

/// Extracts raw character streams from the PDF without conversion.
pub fn extract_raw_text(pdf_bytes: &[u8]) -> Result<String, String> {
    let doc = Document::load_mem(pdf_bytes).map_err(|e| format!("Invalid PDF document: {e}"))?;
    let page_numbers: Vec<u32> = doc.get_pages().keys().copied().collect();
    doc.extract_text(&page_numbers)
        .map_err(|e| format!("Failed to extract text: {e}"))
}

/// Inspects PDF metadata (pages, fonts, suggested encoding).
pub fn inspect_pdf(pdf_bytes: &[u8]) -> Result<PdfMetadata, String> {
    let doc = Document::load_mem(pdf_bytes).map_err(|e| format!("Invalid PDF document: {e}"))?;
    let fonts = extract_all_fonts(&doc);
    let suggested_encoding = fonts
        .iter()
        .find_map(|f| detect_encoding_from_font(f))
        .map(|e| encoding_to_str(e).to_string());
    let total_pages = doc.get_pages().len() as u32;

    Ok(PdfMetadata {
        total_pages,
        fonts,
        suggested_encoding,
    })
}
