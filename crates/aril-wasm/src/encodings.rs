use aril_core::Encoding;

/// All supported encodings in `aril-core`.
pub const SUPPORTED_ENCODINGS: &[Encoding] = &[
    Encoding::Anu,
    Encoding::Indica,
    Encoding::ShreelipiAvid,
    Encoding::Anjal,
    Encoding::Bamini,
    Encoding::Boomi,
    Encoding::Dinakaran,
    Encoding::Dinamani,
    Encoding::Dinathanthy,
    Encoding::Kavipriya,
    Encoding::Murasoli,
    Encoding::Mylai,
    Encoding::Nakkeeran,
    Encoding::Roman,
    Encoding::Tab,
    Encoding::Tam,
    Encoding::Tscii,
    Encoding::Pallavar,
    Encoding::Indoweb,
    Encoding::Koeln,
    Encoding::Libi,
    Encoding::OldVikatan,
    Encoding::Webulagam,
    Encoding::Diacritic,
    Encoding::Shreelipi,
    Encoding::Softview,
    Encoding::Tace,
    Encoding::Vanavil,
    Encoding::Unicode,
];

/// Returns the display string for an [`Encoding`].
#[must_use]
pub const fn encoding_to_str(encoding: Encoding) -> &'static str {
    match encoding {
        Encoding::Auto => "Auto",
        Encoding::Unicode => "Unicode",
        Encoding::Anu => "Anu",
        Encoding::Indica => "Indica",
        Encoding::ShreelipiAvid => "ShreelipiAvid",
        Encoding::Anjal => "Anjal",
        Encoding::Bamini => "Bamini",
        Encoding::Boomi => "Boomi",
        Encoding::Dinakaran => "Dinakaran",
        Encoding::Dinamani => "Dinamani",
        Encoding::Dinathanthy => "Dinathanthy",
        Encoding::Kavipriya => "Kavipriya",
        Encoding::Murasoli => "Murasoli",
        Encoding::Mylai => "Mylai",
        Encoding::Nakkeeran => "Nakkeeran",
        Encoding::Roman => "Roman",
        Encoding::Tab => "Tab",
        Encoding::Tam => "Tam",
        Encoding::Tscii => "Tscii",
        Encoding::Pallavar => "Pallavar",
        Encoding::Indoweb => "Indoweb",
        Encoding::Koeln => "Koeln",
        Encoding::Libi => "Libi",
        Encoding::OldVikatan => "OldVikatan",
        Encoding::Webulagam => "Webulagam",
        Encoding::Diacritic => "Diacritic",
        Encoding::Shreelipi => "Shreelipi",
        Encoding::Softview => "Softview",
        Encoding::Tace => "Tace",
        Encoding::Vanavil => "Vanavil",
    }
}

/// Matches embedded PDF `/BaseFont` or `/FontName` values to an [`Encoding`].
///
/// Strips PDF subset prefixes (e.g. `"BCDFEE+Bamini"`) and checks for known
/// font names and publishing families.
#[must_use]
pub fn detect_encoding_from_font(font_name: &str) -> Option<Encoding> {
    // PDF subset fonts prepend a 6-character tag followed by '+' (e.g., "ABCDEF+Bamini")
    let clean = font_name.split('+').next_back().unwrap_or(font_name);

    if let Some(enc) = Encoding::parse_name(clean) {
        return Some(enc);
    }

    let lower = clean.to_lowercase();
    if lower.contains("bamini") || lower.contains("baamini") {
        Some(Encoding::Bamini)
    } else if lower.contains("tsc") || lower.contains("tscii") {
        Some(Encoding::Tscii)
    } else if lower.contains("vanavil") || lower.contains("avvaiyar") {
        Some(Encoding::Vanavil)
    } else if lower.contains("shree") {
        if lower.contains("avid") {
            Some(Encoding::ShreelipiAvid)
        } else {
            Some(Encoding::Shreelipi)
        }
    } else if lower.contains("kavipriya") {
        Some(Encoding::Kavipriya)
    } else if lower.contains("boomi") {
        Some(Encoding::Boomi)
    } else if lower.contains("softview") {
        Some(Encoding::Softview)
    } else if lower.contains("dinakaran") {
        Some(Encoding::Dinakaran)
    } else if lower.contains("dinamani") {
        Some(Encoding::Dinamani)
    } else if lower.contains("dinathanthy") || lower.contains("thanthi") {
        Some(Encoding::Dinathanthy)
    } else if lower.contains("murasoli") {
        Some(Encoding::Murasoli)
    } else if lower.contains("nakkeeran") {
        Some(Encoding::Nakkeeran)
    } else if lower.contains("vikatan") {
        Some(Encoding::OldVikatan)
    } else if lower.contains("webulagam") {
        Some(Encoding::Webulagam)
    } else if lower.contains("tab") {
        Some(Encoding::Tab)
    } else if lower.contains("tam") {
        Some(Encoding::Tam)
    } else if lower.contains("tace") {
        Some(Encoding::Tace)
    } else if lower.contains("anjal") || lower.contains("inaimathi") {
        Some(Encoding::Anjal)
    } else if lower.contains("mylai") {
        Some(Encoding::Mylai)
    } else if lower.contains("anu") {
        Some(Encoding::Anu)
    } else if lower.contains("indica") {
        Some(Encoding::Indica)
    } else if lower.contains("libi") {
        Some(Encoding::Libi)
    } else if lower.contains("pallavar") {
        Some(Encoding::Pallavar)
    } else if lower.contains("indoweb") {
        Some(Encoding::Indoweb)
    } else if lower.contains("diacritic") {
        Some(Encoding::Diacritic)
    } else if lower.contains("koeln") || lower.contains("cologne") {
        Some(Encoding::Koeln)
    } else if lower.contains("roman") {
        Some(Encoding::Roman)
    } else {
        None
    }
}

/// Parses user-supplied encoding input, using `Encoding::parse_name` with fallback to font detection heuristics.
pub fn parse_user_encoding(input: &str) -> Result<Encoding, String> {
    Encoding::parse_name(input)
        .or_else(|| detect_encoding_from_font(input))
        .ok_or_else(|| format!("Unknown or unsupported encoding: '{input}'"))
}
