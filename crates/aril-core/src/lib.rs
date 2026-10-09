//! # aril-core
//!
//! `aril-core` is an ultra-fast, bidirectional converter between legacy Tamil
//! font encodings and standard Unicode.
//!
//! ## Overview
//!
//! In early Tamil digital computing, typists, presses, and media houses relied on
//! proprietary 8-bit glyph layouts mapped over ASCII and extended ASCII codes.
//! `aril-core` provides high-throughput, deterministic conversion between these legacy
//! formats and modern Unicode.
//!
//! ### Engine Architecture
//!
//! - **Single-Pass Automaton:** Unlike legacy converters that cascade hundreds of sequential
//!   regex passes, `aril-core` compiles bidirectional font tables into [Aho–Corasick](https://docs.rs/aho-corasick)
//!   state machines, ensuring linear-time `O(N)` throughput on corpus-scale text.
//! - **Deterministic Disambiguation:** Resolves complex multi-glyph ligatures (such as prefix/suffix
//!   split vowels) using leftmost-longest match semantics with zero backtracking.
//!
//! ## Supported Encodings
//!
//! `aril-core` supports bidirectional conversion for **29 encodings**, plus auto-detection:
//!
//! | Category | Supported Encodings |
//! | :--- | :--- |
//! | **DTP & Publishing Fonts** | [`Bamini`][Encoding::Bamini], [`Boomi`][Encoding::Boomi], [`Kavipriya`][Encoding::Kavipriya], [`Shreelipi`][Encoding::Shreelipi], [`ShreelipiAvid`][Encoding::ShreelipiAvid], [`Softview`][Encoding::Softview], [`Vanavil`][Encoding::Vanavil], [`Anu`][Encoding::Anu], [`Indica`][Encoding::Indica], [`Libi`][Encoding::Libi], [`Pallavar`][Encoding::Pallavar], [`Indoweb`][Encoding::Indoweb] |
//! | **News & Media Layouts** | [`Dinakaran`][Encoding::Dinakaran], [`Dinamani`][Encoding::Dinamani], [`Dinathanthy`][Encoding::Dinathanthy], [`Murasoli`][Encoding::Murasoli], [`Nakkeeran`][Encoding::Nakkeeran], [`OldVikatan`][Encoding::OldVikatan], [`Webulagam`][Encoding::Webulagam] |
//! | **Government & Standards** | [`Tab`][Encoding::Tab], [`Tam`][Encoding::Tam], [`Tscii`][Encoding::Tscii], [`Tace`][Encoding::Tace] |
//! | **Transliteration & Academic** | [`Anjal`][Encoding::Anjal], [`Roman`][Encoding::Roman], [`Diacritic`][Encoding::Diacritic], [`Koeln`][Encoding::Koeln], [`Mylai`][Encoding::Mylai] |
//! | **Modern & Meta** | [`Unicode`][Encoding::Unicode], [`Auto`][Encoding::Auto] (heuristic detection) |
//!
//! ## Feature Flags
//!
//! - `parallel`: Enables multi-threaded batch conversions across multi-core processors
//!   using [Rayon](https://docs.rs/rayon).
//!
//! ## Quick Start
//!
//! ### Automatic Detection & Conversion
//!
//! ```rust
//! use aril_core::{auto_to_unicode, Encoding};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let legacy_text = "ePykzp kplw;W xUtd;"; // Bamini for "நீலமணி மிடற்று ஒருவன்"
//!     let (unicode_text, detected) = auto_to_unicode(legacy_text)?;
//!
//!     assert_eq!(detected, Encoding::Bamini);
//!     assert_eq!(unicode_text, "நீலமணி மிடற்று ஒருவன்");
//!     Ok(())
//! }
//! ```
//!
//! ### Explicit Bidirectional Conversion
//!
//! ```rust
//! use aril_core::{to_legacy, to_unicode, Encoding};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let input = "khNahd;"; // Bamini
//!     
//!     // Legacy -> Unicode
//!     let unicode = to_unicode(input, Encoding::Bamini)?;
//!     assert_eq!(unicode, "மாயோன்");
//!
//!     // Unicode -> Legacy (Round-trip)
//!     let legacy_roundtrip = to_legacy(&unicode, Encoding::Bamini)?;
//!     assert_eq!(legacy_roundtrip, input);
//!     Ok(())
//! }
//! ```

#![allow(clippy::invisible_characters)]
mod detector;
mod encodings;
mod engine;
mod error;

pub use detector::detect_encoding;
pub use encodings::Encoding;
pub use engine::Converter;
pub use error::Error;
use std::sync::OnceLock;

/// Number of supported font slots in the cache
const MAX_ENCODINGS: usize = 32;

#[allow(clippy::declare_interior_mutable_const)]
/// Thread-safe lazy cache holding pre-compiled Converter instances
static CONVERTER_CACHE: [OnceLock<Converter>; MAX_ENCODINGS] = {
    const INIT: OnceLock<Converter> = OnceLock::new();
    [INIT; MAX_ENCODINGS]
};

/// Retrieves or compiles a static reference to the [`Converter`] for a given encoding.
///
/// If the requested converter has not yet been initialized, it will be built from its
/// static mapping table and stored in `CONVERTER_CACHE`. Subsequent calls are `O(1)`
/// and completely lock-free.
///
/// # Arguments
///
/// * `encoding` - The targeted [`Encoding`].
///
/// # Errors
///
/// Returns [`Error::UnsupportedEncoding`] if:
/// - The discriminant of `encoding` is greater than or equal to `MAX_ENCODINGS`.
/// - The static mapping table fails compilation during initialization.
///
/// # Panics
///
/// Panics if the internal static replacement table contains syntax errors or invalid
/// state machine patterns that prevent the [`Converter`] from compiling.
///
/// # Examples
///
/// ```rust
/// use aril_core::{get_converter, Encoding};
///
/// let converter = get_converter(Encoding::Bamini).expect("Bamini converter must compile");
/// assert_eq!(converter.to_unicode("rptd;"), "சிவன்");
/// ```
pub fn get_converter(encoding: Encoding) -> Result<&'static Converter, Error> {
    let idx = encoding as usize;
    if idx >= MAX_ENCODINGS {
        return Err(Error::UnsupportedEncoding(format!("{:?}", encoding)));
    }

    let converter = CONVERTER_CACHE[idx].get_or_init(|| {
        Converter::new(encoding).expect("Static mapping table must compile without error")
    });

    Ok(converter)
}

/// Converts a legacy Tamil text string into standardized Unicode.
///
/// When [`Encoding::Auto`] is supplied, the source encoding is automatically detected
/// before running the conversion.
///
/// # Arguments
///
/// * `input` - The slice of text to transform.
/// * `encoding` - The source [`Encoding`] scheme (or [`Encoding::Auto`]).
///
/// # Errors
///
/// Returns an error if:
/// - `encoding` is [`Encoding::Auto`] and the detector fails to identify the format ([`Error::DetectionFailed`]).
/// - The resolved encoding index exceeds `MAX_ENCODINGS` ([`Error::UnsupportedEncoding`]).
///
/// # Examples
///
/// ```rust
/// use aril_core::{to_unicode, Encoding};
///
/// // Converting Bamini encoded text
/// let unicode = to_unicode("NrNahd;", Encoding::Bamini).unwrap();
/// assert_eq!(unicode, "சேயோன்");
///
/// // Converting TSCII encoded text
/// let unicode_tscii = to_unicode("º¢Åý", Encoding::Tscii).unwrap();
/// assert_eq!(unicode_tscii, "சிவன்");
/// ```
pub fn to_unicode(input: &str, encoding: Encoding) -> Result<String, Error> {
    let resolved = match encoding {
        Encoding::Auto => detect_encoding(input).ok_or(Error::DetectionFailed)?,

        enc => enc,
    };

    let converter = get_converter(resolved)?;
    Ok(converter.to_unicode(input))
}

/// Converts a Unicode Tamil text string back to a targeted legacy font layout.
///
/// This reverses the Unicode representation into legacy byte sequences, facilitating
/// compatibility with legacy print drivers, DTP tools, or typewriter font systems.
///
/// # Arguments
///
/// * `input` - The Unicode string to be converted.
/// * `encoding` - The target legacy [`Encoding`] (must not be [`Encoding::Auto`]).
///
/// # Errors
///
/// Returns an error if:
/// - `encoding` is [`Encoding::Auto`], because reverse layout generation requires an explicit destination format.
/// - The requested encoding is not supported ([`Error::UnsupportedEncoding`]).
///
/// # Examples
///
/// ```rust
/// use aril_core::{to_legacy, Encoding, Error};
///
/// // Valid conversion
/// let legacy = to_legacy("சிவன்", Encoding::Bamini).unwrap();
/// assert_eq!(legacy, "rptd;");
///
/// // Attempting auto-detection for legacy conversion returns an error
/// let result = to_legacy("சிவன்", Encoding::Auto);
/// assert!(matches!(result, Err(Error::UnsupportedEncoding(_))));
/// ```
pub fn to_legacy(input: &str, encoding: Encoding) -> Result<String, Error> {
    if encoding == Encoding::Auto {
        return Err(Error::UnsupportedEncoding(
            "Encoding::Auto cannot be used for to_legacy conversion".into(),
        ));
    }

    let converter = get_converter(encoding)?;
    Ok(converter.to_legacy(input))
}

/// Automatically detects the font encoding of a legacy text string and converts it to Unicode.
///
/// This is a convenience helper equivalent to invoking [`detect_encoding`] followed by
/// [`to_unicode`].
///
/// # Arguments
///
/// * `input` - The legacy string to inspect and convert.
///
/// # Returns
///
/// Returns a tuple containing:
/// 1. The converted Unicode [`String`].
/// 2. The detected [`Encoding`].
///
/// # Errors
///
/// Returns [`Error::DetectionFailed`] if the string does not exhibit identifiable characteristics
/// of any known legacy Tamil encoding layout.
///
/// # Examples
///
/// ```rust
/// use aril_core::{auto_to_unicode, Encoding};
///
/// let (result, encoding) = auto_to_unicode("ePykzp kplw;W xUtd;").unwrap();
/// assert_eq!(encoding, Encoding::Bamini);
/// assert_eq!(result, "நீலமணி மிடற்று ஒருவன்");
/// ```
pub fn auto_to_unicode(input: &str) -> Result<(String, Encoding), Error> {
    let detected = detect_encoding(input).ok_or(Error::DetectionFailed)?;
    let converter = get_converter(detected)?;
    Ok((converter.to_unicode(input), detected))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murugan_and_seyon_bamini() {
        let bamini_murugan = "KUfd;";
        let converted_murugan = to_unicode(bamini_murugan, Encoding::Bamini).unwrap();
        assert_eq!(converted_murugan, "முருகன்");

        let bamini_seyon = "NrNahd;";
        let converted_seyon = to_unicode(bamini_seyon, Encoding::Bamini).unwrap();
        assert_eq!(converted_seyon, "சேயோன்");

        let roundtrip_murugan = to_legacy(&converted_murugan, Encoding::Bamini).unwrap();
        assert_eq!(roundtrip_murugan, bamini_murugan);

        let roundtrip_seyon = to_legacy(&converted_seyon, Encoding::Bamini).unwrap();
        assert_eq!(roundtrip_seyon, bamini_seyon);
    }

    #[test]
    fn test_murugan_tscii() {
        let tscii_murugan = "ÓÕ¸ý";
        let converted = to_unicode(tscii_murugan, Encoding::Tscii).unwrap();
        assert_eq!(converted, "முருகன்");

        let back_to_tscii = to_legacy(&converted, Encoding::Tscii).unwrap();
        assert_eq!(back_to_tscii, tscii_murugan);
    }

    #[test]
    fn test_sivan_and_aalamar_selvan_bamini() {
        let bamini_sivan = "rptd;";
        let converted_sivan = to_unicode(bamini_sivan, Encoding::Bamini).unwrap();
        assert_eq!(converted_sivan, "சிவன்");

        let bamini_aalamar_selvan = "Myku; nry;td;";
        let converted_aalamar = to_unicode(bamini_aalamar_selvan, Encoding::Bamini).unwrap();
        assert_eq!(converted_aalamar, "ஆலமர் செல்வன்");

        assert_eq!(
            to_legacy(&converted_aalamar, Encoding::Bamini).unwrap(),
            bamini_aalamar_selvan
        );
    }

    #[test]
    fn test_purananuru_91_neelamani_midatru_oruvan() {
        let bamini_verse = "ePykzp kplw;W xUtd;";
        let converted = to_unicode(bamini_verse, Encoding::Bamini).unwrap();
        assert_eq!(converted, "நீலமணி மிடற்று ஒருவன்");

        let roundtrip = to_legacy(&converted, Encoding::Bamini).unwrap();
        assert_eq!(roundtrip, bamini_verse);
    }

    #[test]
    fn test_sivan_tscii() {
        let tscii_sivan = "º¢Åý";
        let converted = to_unicode(tscii_sivan, Encoding::Tscii).unwrap();
        assert_eq!(converted, "சிவன்");
    }

    #[test]
    fn test_nediyon_and_mayon_bamini() {
        let bamini_nediyon = "nelpNahd;";
        let converted_nediyon = to_unicode(bamini_nediyon, Encoding::Bamini).unwrap();
        assert_eq!(converted_nediyon, "நெடியோன்");

        let bamini_mayon = "khNahd;";
        let converted_mayon = to_unicode(bamini_mayon, Encoding::Bamini).unwrap();
        assert_eq!(converted_mayon, "மாயோன்");

        assert_eq!(
            to_legacy(&converted_nediyon, Encoding::Bamini).unwrap(),
            bamini_nediyon
        );
    }

    #[test]
    fn test_nediyon_tscii() {
        let tscii_nediyon = "¦¿Ê§Â¡ý";
        let converted = to_unicode(tscii_nediyon, Encoding::Tscii).unwrap();
        assert_eq!(converted, "நெடியோன்");

        let tscii_mayon = "Á¡§Â¡ý";
        let converted_mayon = to_unicode(tscii_mayon, Encoding::Tscii).unwrap();
        assert_eq!(converted_mayon, "மாயோன்");
    }

    #[test]
    fn test_tolkappiyam_akattinaiyiyal_sutra_5() {
        let bamini_text = "khNahd; Nka fhLiw cyfKk; NrNahd; Nka iktiu cyfKk;";
        let expected_unicode = "மாயோன் மேய காடுறை உலகமும் சேயோன் மேய மைவரை உலகமும்";

        let converted = to_unicode(bamini_text, Encoding::Bamini).unwrap();
        assert_eq!(converted, expected_unicode);

        let roundtrip = to_legacy(&converted, Encoding::Bamini).unwrap();
        assert_eq!(roundtrip, bamini_text);
    }

    #[test]
    fn test_paripatal_hymn_to_nediyon() {
        let bamini_paripatal = "jPapDs; njwy; eP G+tpDs; ehw;wk; eP";
        let expected = "தீயினுள் தெறல் நீ பூவினுள் நாற்றம் நீ";

        let converted = to_unicode(bamini_paripatal, Encoding::Bamini).unwrap();
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_purananuru_192_universal_verse_tscii() {
        let tscii_puram = "Â¡Ðõ °§Ã Â¡ÅÕõ §¸Ç¢÷";
        let converted = to_unicode(tscii_puram, Encoding::Tscii).unwrap();
        assert_eq!(converted, "யாதும் ஊரே யாவரும் கேளிர்");
    }

    #[test]
    fn test_tirumurukarruppatai_opening_native_unicode_passthrough() {
        let sangam_verse = "உலகம் உவப்ப வலனேர்பு திரிதரு பலர்புகழ் ஞாயிறு கடற்கண் டாஅங்கு";

        let (result, detected) = auto_to_unicode(sangam_verse).unwrap();
        assert_eq!(detected, Encoding::Unicode);
        assert_eq!(result, sangam_verse);
    }

    #[test]
    fn test_kuruntokai_40_sem_pulam_peyal_neer_passthrough() {
        let kuruntokai_verse = "செம்புலப் பெயல் நீர் போல அன்புடை நெஞ்சம் தாம் கலந்தனவே";

        let converted = to_unicode(kuruntokai_verse, Encoding::Unicode).unwrap();
        assert_eq!(converted, kuruntokai_verse);
    }

    #[test]
    #[cfg(feature = "parallel")]
    fn test_parallel_batch_sangam_corpus() {
        let converter = get_converter(Encoding::Bamini).unwrap();

        let corpus = vec![
            "KUfd;".to_string(),
            "rptd;".to_string(),
            "nelpNahd;".to_string(),
            "NrNahd;".to_string(),
            "khNahd;".to_string(),
            "khNahd; Nka fhLiw cyfKk; NrNahd; Nka iktiu cyfKk;".to_string(),
        ];

        let results = converter.convert_batch_parallel(&corpus);

        assert_eq!(results[0], "முருகன்");
        assert_eq!(results[1], "சிவன்");
        assert_eq!(results[2], "நெடியோன்");
        assert_eq!(results[3], "சேயோன்");
        assert_eq!(results[4], "மாயோன்");
        assert_eq!(results[5], "மாயோன் மேய காடுறை உலகமும் சேயோன் மேய மைவரை உலகமும்");
    }
}
