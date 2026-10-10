use clap::ValueEnum;
use std::fmt;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[value(rename_all = "kebab-case")]
pub enum CliEncoding {
    Auto,
    Unicode,

    Bamini,
    Boomi,
    Kavipriya,
    Shreelipi,
    ShreelipiAvid,
    Softview,
    Vanavil,
    Anu,
    Indica,
    Libi,
    Pallavar,
    Indoweb,

    Dinakaran,
    Dinamani,
    Dinathanthy,
    Murasoli,
    Nakkeeran,
    OldVikatan,
    Webulagam,

    Tab,
    Tam,
    Tscii,
    Tace,

    Anjal,
    Roman,
    Diacritic,
    Koeln,
    Mylai,
}

impl fmt::Display for CliEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::Unicode => "unicode",
            Self::Bamini => "bamini",
            Self::Boomi => "boomi",
            Self::Kavipriya => "kavipriya",
            Self::Shreelipi => "shreelipi",
            Self::ShreelipiAvid => "shreelipi-avid",
            Self::Softview => "softview",
            Self::Vanavil => "vanavil",
            Self::Anu => "anu",
            Self::Indica => "indica",
            Self::Libi => "libi",
            Self::Pallavar => "pallavar",
            Self::Indoweb => "indoweb",
            Self::Dinakaran => "dinakaran",
            Self::Dinamani => "dinamani",
            Self::Dinathanthy => "dinathanthy",
            Self::Murasoli => "murasoli",
            Self::Nakkeeran => "nakkeeran",
            Self::OldVikatan => "old-vikatan",
            Self::Webulagam => "webulagam",
            Self::Tab => "tab",
            Self::Tam => "tam",
            Self::Tscii => "tscii",
            Self::Tace => "tace",
            Self::Anjal => "anjal",
            Self::Roman => "roman",
            Self::Diacritic => "diacritic",
            Self::Koeln => "koeln",
            Self::Mylai => "mylai",
        };
        write!(f, "{s}")
    }
}
impl From<CliEncoding> for aril_core::Encoding {
    fn from(enc: CliEncoding) -> Self {
        match enc {
            CliEncoding::Auto => Self::Auto,
            CliEncoding::Unicode => Self::Unicode,
            CliEncoding::Bamini => Self::Bamini,
            CliEncoding::Boomi => Self::Boomi,
            CliEncoding::Kavipriya => Self::Kavipriya,
            CliEncoding::Shreelipi => Self::Shreelipi,
            CliEncoding::ShreelipiAvid => Self::ShreelipiAvid,
            CliEncoding::Softview => Self::Softview,
            CliEncoding::Vanavil => Self::Vanavil,
            CliEncoding::Anu => Self::Anu,
            CliEncoding::Indica => Self::Indica,
            CliEncoding::Libi => Self::Libi,
            CliEncoding::Pallavar => Self::Pallavar,
            CliEncoding::Indoweb => Self::Indoweb,
            CliEncoding::Dinakaran => Self::Dinakaran,
            CliEncoding::Dinamani => Self::Dinamani,
            CliEncoding::Dinathanthy => Self::Dinathanthy,
            CliEncoding::Murasoli => Self::Murasoli,
            CliEncoding::Nakkeeran => Self::Nakkeeran,
            CliEncoding::OldVikatan => Self::OldVikatan,
            CliEncoding::Webulagam => Self::Webulagam,
            CliEncoding::Tab => Self::Tab,
            CliEncoding::Tam => Self::Tam,
            CliEncoding::Tscii => Self::Tscii,
            CliEncoding::Tace => Self::Tace,
            CliEncoding::Anjal => Self::Anjal,
            CliEncoding::Roman => Self::Roman,
            CliEncoding::Diacritic => Self::Diacritic,
            CliEncoding::Koeln => Self::Koeln,
            CliEncoding::Mylai => Self::Mylai,
        }
    }
}

pub(crate) fn print_encodings_catalog() {
    println!("\x1b[1;36m=== Supported Encodings in aril (29 Layouts + Unicode) ===\x1b[0m\n");
    println!("\x1b[1mDTP & Publishing Fonts:\x1b[0m");
    println!("  bamini, boomi, kavipriya, shreelipi, shreelipi-avid,");
    println!("  softview, vanavil, anu, indica, libi, pallavar, indoweb\n");
    println!("\x1b[1mNews & Media Layouts:\x1b[0m");
    println!("  dinakaran, dinamani, dinathanthy, murasoli,");
    println!("  nakkeeran, old-vikatan, webulagam\n");
    println!("\x1b[1mGovernment & Standards:\x1b[0m");
    println!("  tab, tam, tscii, tace\n");
    println!("\x1b[1mTransliteration & Academic:\x1b[0m");
    println!("  anjal, roman, diacritic, koeln, mylai\n");
    println!("\x1b[1mModern & Meta:\x1b[0m");
    println!("  unicode, auto (heuristic automatic detection)\n");
}
