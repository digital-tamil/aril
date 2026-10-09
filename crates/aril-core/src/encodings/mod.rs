pub mod anjal;
pub mod anu;
pub mod bamini;
pub mod boomi;
pub mod diacritic;
pub mod dinakaran;
pub mod dinamani;
pub mod dinathanthy;
pub mod indica;
pub mod indoweb;
pub mod kavipriya;
pub mod koeln;
pub mod libi;
pub mod murasoli;
pub mod mylai;
pub mod nakkeeran;
pub mod oldvikatan;
pub mod pallavar;
pub mod roman;
pub mod shreelipi;
pub mod shreelipiavid;
pub mod softview;
pub mod tab;
pub mod tace;
pub mod tam;
pub mod tscii;
pub mod vanavil;
pub mod webulagam;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Encoding {
    Auto = 0,
    Unicode = 1,
    Anu,
    Indica,
    ShreelipiAvid,
    Anjal,
    Bamini,
    Boomi,
    Dinakaran,
    Dinamani,
    Dinathanthy,
    Kavipriya,
    Murasoli,
    Mylai,
    Nakkeeran,
    Roman,
    Tab,
    Tam,
    Tscii,
    Pallavar,
    Indoweb,
    Koeln,
    Libi,
    OldVikatan,
    Webulagam,
    Diacritic,
    Shreelipi,
    Softview,
    Tace,
    Vanavil,
}

impl Encoding {
    pub fn mapping_table(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Auto | Self::Unicode => &[],
            Self::Anu => anu::ANU_MAP,
            Self::Indica => indica::INDICA_MAP,
            Self::ShreelipiAvid => shreelipiavid::SHREELIPIAVID_MAP,
            Self::Anjal => anjal::ANJAL_MAP,
            Self::Bamini => bamini::BAMINI_MAP,
            Self::Boomi => boomi::BOOMI_MAP,
            Self::Dinakaran => dinakaran::DINAKARAN_MAP,
            Self::Dinamani => dinamani::DINAMANI_MAP,
            Self::Dinathanthy => dinathanthy::DINATHANTHY_MAP,
            Self::Kavipriya => kavipriya::KAVIPRIYA_MAP,
            Self::Murasoli => murasoli::MURASOLI_MAP,
            Self::Mylai => mylai::MYLAI_MAP,
            Self::Nakkeeran => nakkeeran::NAKKEERAN_MAP,
            Self::Roman => roman::ROMAN_MAP,
            Self::Tab => tab::TAB_MAP,
            Self::Tam => tam::TAM_MAP,
            Self::Tscii => tscii::TSCII_MAP,
            Self::Pallavar => pallavar::PALLAVAR_MAP,
            Self::Indoweb => indoweb::INDOWEB_MAP,
            Self::Koeln => koeln::KOELN_MAP,
            Self::Libi => libi::LIBI_MAP,
            Self::OldVikatan => oldvikatan::OLDVIKATAN_MAP,
            Self::Webulagam => webulagam::WEBULAGAM_MAP,
            Self::Diacritic => diacritic::DIACRITIC_MAP,
            Self::Shreelipi => shreelipi::SHREELIPI_MAP,
            Self::Softview => softview::SOFTVIEW_MAP,
            Self::Tace => tace::TACE_MAP,
            Self::Vanavil => vanavil::VANAVIL_MAP,
        }
    }

    pub fn parse_name(name: &str) -> Option<Self> {
        match name.to_lowercase().replace('-', "").as_str() {
            "auto" => Some(Self::Auto),
            "unicode" | "utf8" => Some(Self::Unicode),
            "anu" => Some(Self::Anu),
            "indica" => Some(Self::Indica),
            "shreelipiavid" | "avid" => Some(Self::ShreelipiAvid),
            "anjal" => Some(Self::Anjal),
            "bamini" => Some(Self::Bamini),
            "boomi" => Some(Self::Boomi),
            "dinakaran" => Some(Self::Dinakaran),
            "dinamani" => Some(Self::Dinamani),
            "dinathanthy" => Some(Self::Dinathanthy),
            "kavipriya" => Some(Self::Kavipriya),
            "murasoli" => Some(Self::Murasoli),
            "mylai" => Some(Self::Mylai),
            "nakkeeran" => Some(Self::Nakkeeran),
            "roman" => Some(Self::Roman),
            "tab" => Some(Self::Tab),
            "tam" => Some(Self::Tam),
            "tscii" => Some(Self::Tscii),
            "pallavar" => Some(Self::Pallavar),
            "indoweb" => Some(Self::Indoweb),
            "koeln" => Some(Self::Koeln),
            "libi" => Some(Self::Libi),
            "oldvikatan" => Some(Self::OldVikatan),
            "webulagam" => Some(Self::Webulagam),
            "diacritic" => Some(Self::Diacritic),
            "shreelipi" => Some(Self::Shreelipi),
            "softview" => Some(Self::Softview),
            "tace" => Some(Self::Tace),
            "vanavil" => Some(Self::Vanavil),
            _ => None,
        }
    }
}
