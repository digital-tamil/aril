use crate::encodings::Encoding;

struct Signature {
    encoding: Encoding,
    markers: &'static [&'static str],
}
static SIGNATURES: &[Signature] = &[
    Signature {
        encoding: Encoding::Unicode,
        markers: &[
            "த்த",
            "க்க",
            "ந்த",
            "ப்ப",
            "ன்று",
            "ண்டு",
            "ல்லை",
            "கொண்டு",
            "செய்த",
            "இந்த",
            "என்று",
            "தமிழ்",
        ],
    },
    Signature {
        encoding: Encoding::Tscii,
        markers: &[
            "¦¸¡", "§¸¡", "¦¸ª", "¦º¡", "§º¡", "¦ºª", "¦¾¡", "§¾¡", "¦¾ª", "¦À¡", "§À¡", "¦Àª",
            "¦Á¡", "§Á¡", "¦Áª", "¦¿¡", "§¿¡", "¸¡", "¾¡", "À¡", "Á¡",
        ],
    },
    Signature {
        encoding: Encoding::Tab,
        markers: &[
            "ªè÷", "«è£", "ªè£", "è£", "ªê£", "«ê£", "ªê÷", "ªî£", "«î£", "ªî÷", "ªð£", "«ð£",
            "ªð÷", "ªñ£", "«ñ£", "ªñ÷", "è¢", "ê¢", "î¢", "ð¢", "ñ¢", "ó¢", "ô¢",
        ],
    },
    Signature {
        encoding: Encoding::Tam,
        markers: &[
            "ªè£", "«è£", "ªè÷", "ªî£", "«î£", "ªð£", "«ð£", "ªñ£", "«ñ£", "‚", "„", "ˆ", "‰", "Š",
            "‹", "˜", "™", "š", "¡",
        ],
    },
    Signature {
        encoding: Encoding::Bamini,
        markers: &[
            "nfs", "Nfh", "nfh", "Nrh", "nrh", "Njh", "njh", "Nkh", "nkh", "Nth", "nth", "Nwh",
            "nwh", "Nlh", "nlh", "f;", "r;", "j;", "k;", "t;", "w;",
        ],
    },
    Signature {
        encoding: Encoding::Vanavil,
        markers: &[
            "nsh", "bsh", "nzh", "nyh", "nwh", "nuh", "nth", "nrh", "nlh", "nkh", "njh", "nHh",
            "ngh", "nfh", "neh", "ndh", "bzs", "bzh", "bys", "byh", "bws", "bwh", "bjs", "bjh",
        ],
    },
    Signature {
        encoding: Encoding::Dinakaran,
        markers: &[
            "b's", "n'h", "b'h", "b\"s", "n\"h", "b\"h", "bHH", "nHh", "bHh", "b##", "n##", "b!!",
            "n!!", "b$s", "n$h", "bgs", "ngh", "bgh", "bjs", "njh", "bjh",
        ],
    },
    Signature {
        encoding: Encoding::Murasoli,
        markers: &[
            "b#s", "n#h", "b#h", "b‡s", "n‡h", "b‡h", "bPs", "nPh", "bPh", "bAs", "nAh", "bAh",
            "bõs", "nõh", "bõh", "b[s", "n[h", "b[h", "bfs", "bfh",
        ],
    },
    Signature {
        encoding: Encoding::Webulagam,
        markers: &[
            "bBh", "nBh", "bBs", "gh", "b#s", "n#h", "b#h", "b‡s", "n‡h", "b‡h", "bPs", "nPh",
            "bPh", "bAs", "nAh", "bõs", "nõh", "bõh", "b[s", "n[h",
        ],
    },
    Signature {
        encoding: Encoding::Mylai,
        markers: &[
            "ekq", "Eka", "eka", "ecq", "Eca", "eca", "etq", "Eta", "eta", "epq", "Epa", "epa",
            "emq", "Ema", "ema", "eqq", "Eqa", "eqa", "kf", "cf", "tf", "pf", "mf",
        ],
    },
    Signature {
        encoding: Encoding::Shreelipi,
        markers: &[
            "öPÍ", "÷Põ", "öPõ", "Põ", "öuõ", "÷uõ", "öÚõ", "÷Úõ", "ö£õ", "÷£õ", "ö©õ", "÷©õ",
            "öµõ", "÷µõ", "ö»õ", "÷»õ", "öÇõ", "÷Çõ", "öÓõ", "÷Óõ", "øP", "øu", "øÚ", "ø£",
        ],
    },
    Signature {
        encoding: Encoding::ShreelipiAvid,
        markers: &[
            "@Põ", "@uõ", "@Úõ", "@£õ", "@©õ", "@µõ", "@»õ", "@Óõ", "@Íõ", "@Œõ", "öŒõ", "öŒÍ",
            "@Œ", "øŒ", "@P", "@u", "@Ú", "@£", "@©",
        ],
    },
    Signature {
        encoding: Encoding::Libi,
        markers: &[
            "öPÍ", "÷Põ", "öPõ", "öuõ", "÷uõ", "öÚõ", "÷Úõ", "ö£õ", "÷£õ", "ö©õ", "÷©õ", "öµõ",
            "÷µõ", "öÍÍ", "öíí", "öéé", "öÁÁ", "öÇÇ", "öÓÓ",
        ],
    },
    Signature {
        encoding: Encoding::Softview,
        markers: &[
            "ú[ô", "ù[ô", "úQô", "úXô", "ú\\ô", "úWô", "úYô", "úNô", "úPô", "úUô", "úRô", "úTô",
            "úLô", "úSô", "ú]ô", "ùQ[", "ùX[", "ù\\[", "ùW[", "ùY[", "ù[[", "ûQ", "ûX", "ûL",
        ],
    },
    Signature {
        encoding: Encoding::Dinamani,
        markers: &[
            "ùL[", "úLô", "ùLô", "Lô", "úNô", "ùNô", "úRô", "ùRô", "úTô", "ùTô", "úUô", "ùUô",
            "úWô", "ùWô", "úXô", "ùXô", "ùZ[", "úZô", "ûL", "ûN", "ûR", "ûT",
        ],
    },
    Signature {
        encoding: Encoding::Nakkeeran,
        markers: &[
            "ùL[", "úLô", "ùLô", "I[", "úPô", "ùP[", "úRô", "ùR[", "úTô", "ùT[", "úWô", "ùW[",
            "úXô", "ùX[", "ûL", "ûN", "ûR", "ûT", "ü",
        ],
    },
    Signature {
        encoding: Encoding::Kavipriya,
        markers: &[
            "ùஜ[", "úஜô", "ùஜô", "ùஜ", "ûள", "ளô", "ஸô", "ஸý", "ûஸ", "ùஹ[", "úஹô", "ùஹ", "ùஷ[",
            "úஷô", "ùஷ", "ûஷ", "ùL[", "úLô", "ùLô",
        ],
    },
    Signature {
        encoding: Encoding::Dinathanthy,
        markers: &[
            "ÙL[", "ÚLÖ", "ÙLÖ", "LÖ", "ÙN[", "ÚNÖ", "ÙNÖ", "NÖ", "ÙR[", "ÚRÖ", "ÙRÖ", "RÖ", "ÙT[",
            "ÚTÖ", "ÙTÖ", "TÖ", "ÙU[", "ÚUÖ", "ÙUÖ", "UÖ", "ÙW[", "ÚWÖ", "ÛL", "ÛN",
        ],
    },
    Signature {
        encoding: Encoding::Boomi,
        markers: &[
            "%L[", "&L\"", "%L\"", "%M[", "&M\"", "%N[", "&N\"", "%O[", "&O\"", "%P[", "&P\"",
            "%Q[", "&Q\"", "%R[", "&R\"", "%T[", "&T\"", "%U[", "&U\"", "+L", "+N", "+R", "+T",
        ],
    },
    Signature {
        encoding: Encoding::OldVikatan,
        markers: &[
            "`L[", "&L\"", "`L\"", "L\"", "`N[", "&N\"", "`N\"", "N\"", "`R[", "&R\"", "`R\"",
            "R\"", "`T[", "&T\"", "`T\"", "T\"", "`U[", "&U\"", "`U\"", "U\"", "'L", "'N", "'R",
            "'T",
        ],
    },
    Signature {
        encoding: Encoding::Indica,
        markers: &[
            "ºeV", "ÿeV", "ºÔV", "ÿÔV", "ÿÔe", "º>V", "ÿ>V", "ÿ>e", "º™V", "ÿ™V", "º™e", "º√V",
            "ÿ√V", "ÿ√e", "º\\V", "ÿ\\V", "ÿ\\e", "ºƒV", "ÿƒV", "ÿƒe", "ÁÔ", "Á>", "Á™",
        ],
    },
    Signature {
        encoding: Encoding::Anu,
        markers: &[
            "¼eV", "ØeV", "¼ïV", "ØïV", "Øïe", "¼>V", "Ø>V", "Ø>e", "¼ªV", "ØªV", "¼ªe", "¼ÃV",
            "ØÃV", "ØÃe", "¼\\V", "Ø\\V", "Ø\\e", "¼ÄV", "ØÄV", "çï", "ç>", "çª",
        ],
    },
    Signature {
        encoding: Encoding::Pallavar,
        markers: &[
            "¼áV", "ØáV", "áV", "Øáe", "Ø\\á", "Øåª", "¼ïV", "ØïV", "Øïe", "¼>V", "Ø>V", "¼ÄV",
            "ØÄV", "¼ÃV", "ØÃV", "çï", "ç>", "çÄ",
        ],
    },
    Signature {
        encoding: Encoding::Indoweb,
        markers: &[
            "âæó", "îæè", "âæè", "æè", "æÐ", "âòè", "îòè", "òè", "òÐ", "âêè", "îêè", "êè", "êÐ",
            "âçè", "îçè", "çè", "çÐ", "âëè", "îëè", "ëè", "ëÐ", "âõè", "îõè",
        ],
    },
    Signature {
        encoding: Encoding::Anjal,
        markers: &[
            "—ûã", "þû‘", "—û‘", "þó‘", "—ó‘", "— ã", "þ ‘", "— ‘", "—šã", "þš‘", "—œã", "þœ‘",
            "—¥ã", "þ¥‘", "—«ã", "þ«‘", "—°ã", "þ°‘", "—µã", "þµ‘", "€û", "€ó", "€œ",
        ],
    },
    Signature {
        encoding: Encoding::Diacritic,
        markers: &[
            "kṣ", "šrī", "ṇau", "ṟau", "ḻau", "ḷau", "ñau", "ṅau", "ṭau", "kṣō", "kṣē", "kṣā",
            "ṟō", "ḻō", "ḷō", "ṇō", "ṭō", "ṟā", "ḻā", "ḷā", "ṇā", "ṭā",
        ],
    },
    Signature {
        encoding: Encoding::Roman,
        markers: &[
            "nthau", "nthai", "nthee", "nthoo", "nthaa", "nthuu", "nthii", "zhau", "zhai", "zhee",
            "zhoo", "zhaa", "zhuu", "zhii", "thau", "thai", "thee", "thoo",
        ],
    },
    Signature {
        encoding: Encoding::Koeln,
        markers: &[
            "n2au", "n2ai", "n2a", "n2i", "n_au", "n_ai", "n_a", "n_i", "üau", "üai", "üa", "üi",
            "n^au", "n^ai", "n^a", "n^i", "¤au", "¤ai", "¤a",
        ],
    },
    Signature {
        encoding: Encoding::Tace,
        markers: &[
            "\u{E210}", "\u{E212}", "\u{E213}", "\u{E214}", "\u{E250}", "\u{E270}", "\u{E290}",
            "\u{E2B0}", "\u{E2D0}", "\u{E2F0}", "\u{E310}", "\u{E330}", "\u{E350}", "", "", "",
            "", "", "", "", "", "", "", "",
        ],
    },
];
const MIN_CONFIDENCE_THRESHOLD: usize = 2;

pub fn detect_encoding(input: &str) -> Option<Encoding> {
    let tamil_unicode_count = input
        .chars()
        .filter(|&c| ('\u{0B80}'..='\u{0BFF}').contains(&c))
        .count();

    if tamil_unicode_count >= 2 {
        return Some(Encoding::Unicode);
    }
    let mut best_encoding = None;
    let mut max_score = 0;

    for sig in SIGNATURES {
        let mut score = 0;
        for &marker in sig.markers {
            if input.contains(marker) {
                score += 1;
            }
        }
        if score > max_score {
            max_score = score;
            best_encoding = Some(sig.encoding);
        }
    }

    if max_score >= MIN_CONFIDENCE_THRESHOLD {
        best_encoding
    } else {
        None
    }
}
