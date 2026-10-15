use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

pub(crate) fn is_pdf_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
}

#[cfg(feature = "pdf")]
pub(crate) fn extract_pdf_text<P: AsRef<Path>>(
    path: P,
    pages_spec: Option<&str>,
    page_markers: bool,
) -> Result<String> {
    use lopdf::Document;

    let path_ref = path.as_ref();
    let doc = Document::load(path_ref)
        .with_context(|| format!("Failed to read PDF document: {}", path_ref.display()))?;

    let pages = doc.get_pages();
    let total_pages = u32::try_from(pages.len()).unwrap_or(u32::MAX);

    if total_pages == 0 {
        return Ok(String::new());
    }

    let selected_pages = match pages_spec {
        Some(spec) => parse_page_ranges(spec, total_pages)?,
        None => (1..=total_pages).collect(),
    };

    let mut output = String::new();

    for (idx, page_num) in selected_pages.iter().enumerate() {
        let page_text = doc.extract_text(&[*page_num]).unwrap_or_default();

        if page_markers {
            if idx > 0 {
                output.push('\n');
            }
            let _ = writeln!(output, "--- பக்கம் {page_num} ---");
        } else if idx > 0 && !output.ends_with('\n') {
            output.push('\n');
        }

        output.push_str(&page_text);
        if !output.ends_with('\n') {
            output.push('\n');
        }
    }

    if output.trim().is_empty() {
        eprintln!(
            "\x1b[33m[Warning]\x1b[0m No selectable text extracted from '{}'. \
            If this PDF contains scanned image pages, OCR is required.",
            path_ref.display()
        );
    }

    Ok(output)
}

#[cfg(not(feature = "pdf"))]
pub(crate) fn extract_pdf_text<P: AsRef<Path>>(
    _path: P,
    _pages_spec: Option<&str>,
    _page_markers: bool,
) -> Result<String> {
    bail!("PDF support was disabled at compile time. Please recompile aril with `--features pdf`.");
}

/// Parses page range syntax such as "1-8", "5,8,18", "18-", "-1330", or "1-8,18,64-96,133,400".
pub(crate) fn parse_page_ranges(spec: &str, total_pages: u32) -> Result<Vec<u32>> {
    let mut pages = BTreeSet::new();

    for part in spec.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if let Some((start_s, end_s)) = part.split_once('-') {
            let start = if start_s.trim().is_empty() {
                1
            } else {
                start_s
                    .trim()
                    .parse::<u32>()
                    .with_context(|| format!("Invalid page number in range: '{start_s}'"))?
            };

            let end = if end_s.trim().is_empty() {
                total_pages
            } else {
                end_s
                    .trim()
                    .parse::<u32>()
                    .with_context(|| format!("Invalid page number in range: '{end_s}'"))?
            };

            if start == 0 || end == 0 {
                bail!("Page numbers must be 1-based (got 0 in '{part}')");
            }
            if start > end {
                bail!("Invalid page range: start ({start}) > end ({end}) in '{part}'");
            }
            if start > total_pages {
                bail!("Page {start} exceeds total document pages ({total_pages}) in '{part}'");
            }

            let clamped_end = end.min(total_pages);
            for p in start..=clamped_end {
                pages.insert(p);
            }
        } else {
            let p = part
                .parse::<u32>()
                .with_context(|| format!("Invalid page number: '{part}'"))?;
            if p == 0 {
                bail!("Page numbers must be 1-based (got 0)");
            }
            if p > total_pages {
                bail!("Page {p} exceeds total document pages ({total_pages})");
            }
            pages.insert(p);
        }
    }

    if pages.is_empty() {
        bail!("Page specification '{spec}' resolved to no valid pages");
    }

    Ok(pages.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_page() {
        // Page 18 (பதினெண் சித்தர்) within a 108-page manuscript (108 வர்மப் புள்ளிகள்)
        assert_eq!(parse_page_ranges("18", 108).unwrap(), vec![18]);
    }

    #[test]
    fn test_page_range() {
        // Pages 1-5: The 5 Sangam landscapes (ஐந்திணை: குறிஞ்சி, முல்லை, மருதம், நெய்தல், பாலை)
        assert_eq!(parse_page_ranges("1-5", 18).unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_open_ended_range() {
        // From 8 (எட்டுத்தொகை) to 10 (பத்துப்பாட்டு)
        assert_eq!(parse_page_ranges("8-", 10).unwrap(), vec![8, 9, 10]);

        // Prefix range through page 3: The 3 Sangam divisions (முப்பால்: அறம், பொருள், இன்பம்)
        assert_eq!(parse_page_ranges("-3", 18).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_mixed_ranges_and_dedup() {
        // Overlapping selections resolving cleanly to the 5 Thinai (ஐந்திணை)
        assert_eq!(
            parse_page_ranges("1-3, 5, 2-4", 18).unwrap(),
            vec![1, 2, 3, 4, 5]
        );

        // Deduplication spanning the 8 Ashta Siddhis (அஷ்ட சித்திகள்)
        assert_eq!(
            parse_page_ranges("1-5, 8, 4-7", 18).unwrap(),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn test_compound_sangam_siddhar_selection() {
        // Discrete milestones: Ainthinai (1-5), Ashta Siddhi (8), Pathuppattu (10), 18 Siddhars (18)
        assert_eq!(
            parse_page_ranges("1-5, 8, 10, 18", 18).unwrap(),
            vec![1, 2, 3, 4, 5, 8, 10, 18]
        );
    }

    #[test]
    fn test_invalid_ranges() {
        // 19th index exceeds the 18 Great Siddhars (பதினெண் சித்தர்கள்)
        assert!(parse_page_ranges("19", 18).is_err());

        // 1331 exceeds the 1330 sacred couplets of Thirukkural (திருக்குறள்)
        assert!(parse_page_ranges("1331", 1330).is_err());

        // 0 does not exist in 1-based leaf pagination
        assert!(parse_page_ranges("0", 18).is_err());

        // Inverted: Ettuthogai (8) cannot flow backward to Ainthinai (5)
        assert!(parse_page_ranges("8-5", 18).is_err());
    }
}
