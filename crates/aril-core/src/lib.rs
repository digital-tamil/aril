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

/// Thread-safe lazy cache holding pre-compiled Converter instances
static CONVERTER_CACHE: [OnceLock<Converter>; MAX_ENCODINGS] = {
    const INIT: OnceLock<Converter> = OnceLock::new();
    [INIT; MAX_ENCODINGS]
};

/// Retrieves or compiles a static reference to the Converter
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

/// Converts legacy Tamil text to Unicode
pub fn to_unicode(input: &str, encoding: Encoding) -> Result<String, Error> {
    let resolved = match encoding {
        Encoding::Auto => detect_encoding(input).ok_or(Error::DetectionFailed)?,

        enc => enc,
    };

    let converter = get_converter(resolved)?;
    Ok(converter.to_unicode(input))
}

/// Converts Unicode Tamil text back to a legacy font layout
pub fn to_legacy(input: &str, encoding: Encoding) -> Result<String, Error> {
    if encoding == Encoding::Auto {
        return Err(Error::UnsupportedEncoding(
            "Encoding::Auto cannot be used for to_legacy conversion".into(),
        ));
    }

    let converter = get_converter(encoding)?;
    Ok(converter.to_legacy(input))
}

/// Automatically detects font encoding and converts to Unicode
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
