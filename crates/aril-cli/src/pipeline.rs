use anyhow::{Context, Result, bail};
use aril_core::{Encoding, auto_to_unicode, detect_encoding, to_legacy, to_unicode};

use crate::encoding::CliEncoding;

#[derive(Debug)]
pub struct ConversionResult {
    pub output: String,
    pub source_detected: Option<Encoding>,

}

#[derive(Debug)]
pub struct Pipeline;

impl Pipeline {
    pub(crate) fn execute(
        input: &str,
        from: CliEncoding,
        to: CliEncoding,
    ) -> Result<ConversionResult> {
        let from_core: Encoding = from.into();
        let to_core: Encoding = to.into();

        if to_core == Encoding::Auto {
            bail!("Target encoding (--to) cannot be 'auto'. Please specify an explicit layout.");
        }

        let mut detected_source = None;

        let unicode = match from_core {
            Encoding::Auto => {
                let (res, detected) = auto_to_unicode(input)
                    .map_err(|e| anyhow::anyhow!("Auto-detection failed: {e}"))?;
                detected_source = Some(detected);
                res
            }
            Encoding::Unicode => input.to_string(),
            legacy_enc => to_unicode(input, legacy_enc)
                .with_context(|| format!("Failed converting from {legacy_enc:?} to Unicode"))?,
        };

        let final_output = if to_core == Encoding::Unicode {
            unicode
        } else {
            to_legacy(&unicode, to_core)
                .with_context(|| format!("Failed converting to target {to_core:?}"))?
        };

        Ok(ConversionResult {
            output: final_output,
            source_detected: detected_source,
        })
    }

    pub(crate) fn detect(input: &str) -> Option<Encoding> {
        detect_encoding(input)
    }
}
