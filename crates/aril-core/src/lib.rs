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
    fn test_bamini_to_unicode() {
        let output = to_unicode("jkpo;", Encoding::Bamini).unwrap();
        assert_eq!(output, "தமிழ்");
    }

    #[test]
    fn test_bamini_roundtrip() {
        let original_bamini = "jkpo;";
        let unicode = to_unicode(original_bamini, Encoding::Bamini).unwrap();
        assert_eq!(unicode, "தமிழ்");

        let back_to_bamini = to_legacy(&unicode, Encoding::Bamini).unwrap();
        assert_eq!(back_to_bamini, original_bamini);
    }

    #[test]
    fn test_auto_detection() {
        let bamini_sample = "jkpo; nfs Nfh";
        let (converted, encoding) = auto_to_unicode(bamini_sample).unwrap();
        assert_eq!(encoding, Encoding::Bamini);
        assert!(converted.contains("தமிழ்"));
    }
}
