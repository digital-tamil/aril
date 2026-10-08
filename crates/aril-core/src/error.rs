use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to build Aho-Corasick string matching automaton: {0}")]
    AutomatonBuild(#[from] aho_corasick::BuildError),

    #[error("Could not auto-detect the input encoding. Please specify the encoding explicitly.")]
    DetectionFailed,

    #[error("Unknown or unsupported encoding: '{0}'")]
    UnsupportedEncoding(String),
}
