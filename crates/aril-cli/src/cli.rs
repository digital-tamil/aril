use clap::{ArgGroup, Parser, Subcommand, ValueHint, builder::styling};
use clap_complete::Shell;
use std::path::PathBuf;

use crate::encoding::CliEncoding;

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Cyan.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Yellow.on_default())
    .placeholder(styling::AnsiColor::Magenta.on_default());

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(
    name = "aril",
    author,
    version,
    about = "Ultra-fast Tamil font converter & auto-detector",
    long_about = "High-throughput, deterministic CLI converter for 29+ legacy Tamil font encodings to and from Unicode.",
    styles = STYLES,
    group(
        ArgGroup::new("source")
            .args(["files", "text"])
    )
)]
pub(crate) struct Cli {
    /// Source encoding (defaults to auto-detection)
    #[arg(short = 'f', long = "from", value_enum, default_value = "auto")]
    pub from: CliEncoding,

    /// Target destination encoding
    #[arg(short = 't', long = "to", value_enum, default_value = "unicode")]
    pub to: CliEncoding,

    /// Convert inline string directly
    #[arg(short = 's', long = "text")]
    pub text: Option<String>,

    /// Files or directories to convert (use '-' or omit for stdin)
    #[arg(value_name = "FILES")]
    pub files: Vec<PathBuf>,

    /// Destination output file path (single file conversion only)
    #[arg(short = 'o', long = "output", conflicts_with = "in_place", value_hint =  ValueHint::FilePath)]
    pub output: Option<PathBuf>,

    /// Overwrite input files in-place
    #[arg(short = 'i', long = "in-place")]
    pub in_place: bool,

    /// Create .bak backup before modifying files in-place
    #[arg(long = "backup", requires = "in_place")]
    pub backup: bool,

    /// Preview actions without writing any changes to disk
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Display inline colored diff showing legacy vs converted characters
    #[arg(short = 'd', long = "diff")]
    pub diff: bool,

    /// Recursively search through directories
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,

    /// Comma-separated file extensions to process recursively
    #[arg(long = "ext", default_value = "txt,html,htm,srt,vtt,md")]
    pub extensions: String,

    /// Show conversion statistics, throughput, and execution time
    #[arg(long = "stats")]
    pub stats: bool,

    /// Output results as structured JSON (useful for pipelines and CI)
    #[arg(long = "json")]
    pub json: bool,

    /// Print verbose internal diagnostic messages
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Detect the font encoding of files or stdin without converting
    Detect {
        /// Files to inspect (omitted or '-' for stdin)
        #[arg(value_name = "FILES")]
        files: Vec<PathBuf>,

        /// Output results in JSON format
        #[arg(long = "json")]
        json: bool,
    },
    /// Launch interactive live-conversion REPL
    Repl,
    /// List all 29+ supported Tamil font encodings
    List,
    /// Generate shell autocompletion script
    Completions {
        /// Target shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}
