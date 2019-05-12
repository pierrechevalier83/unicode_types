/// \u{fb00} → \u{fb4f}\
///\
/// ﬀ ﬁ ﬂ ﬃ ﬄ ﬅ ﬆ ﬓ ﬔ ﬕ ﬖ ﬗ יִ ﬞ ײַ ﬠ
/// ﬡ ﬢ ﬣ ﬤ ﬥ ﬦ ﬧ ﬨ ﬩ שׁ שׂ שּׁ שּׂ אַ אָ אּ
/// בּ גּ דּ הּ וּ זּ טּ יּ ךּ כּ לּ מּ נּ סּ ףּ פּ
/// צּ קּ רּ שּ תּ וֹ בֿ כֿ פֿ
pub mod constants {
    /// \u{fb00}: 'ﬀ'
    pub const LATIN_SMALL_LIGATURE_FF: char = 'ﬀ';
    /// \u{fb01}: 'ﬁ'
    pub const LATIN_SMALL_LIGATURE_FI: char = 'ﬁ';
    /// \u{fb02}: 'ﬂ'
    pub const LATIN_SMALL_LIGATURE_FL: char = 'ﬂ';
    /// \u{fb03}: 'ﬃ'
    pub const LATIN_SMALL_LIGATURE_FFI: char = 'ﬃ';
    /// \u{fb04}: 'ﬄ'
    pub const LATIN_SMALL_LIGATURE_FFL: char = 'ﬄ';
    /// \u{fb05}: 'ﬅ'
    pub const LATIN_SMALL_LIGATURE_LONG_S_T: char = 'ﬅ';
    /// \u{fb06}: 'ﬆ'
    pub const LATIN_SMALL_LIGATURE_ST: char = 'ﬆ';
    /// \u{fb13}: 'ﬓ'
    pub const ARMENIAN_SMALL_LIGATURE_MEN_NOW: char = 'ﬓ';
    /// \u{fb14}: 'ﬔ'
    pub const ARMENIAN_SMALL_LIGATURE_MEN_ECH: char = 'ﬔ';
    /// \u{fb15}: 'ﬕ'
    pub const ARMENIAN_SMALL_LIGATURE_MEN_INI: char = 'ﬕ';
    /// \u{fb16}: 'ﬖ'
    pub const ARMENIAN_SMALL_LIGATURE_VEW_NOW: char = 'ﬖ';
    /// \u{fb17}: 'ﬗ'
    pub const ARMENIAN_SMALL_LIGATURE_MEN_XEH: char = 'ﬗ';
    /// \u{fb1d}: 'יִ'
    pub const HEBREW_LETTER_YOD_WITH_HIRIQ: char = 'יִ';
    /// \u{fb1e}: 'ﬞ'
    pub const HEBREW_POINT_JUDEO_DASH_SPANISH_VARIKA: char = 'ﬞ';
    /// \u{fb1f}: 'ײַ'
    pub const HEBREW_LIGATURE_YIDDISH_YOD_YOD_PATAH: char = 'ײַ';
    /// \u{fb20}: 'ﬠ'
    pub const HEBREW_LETTER_ALTERNATIVE_AYIN: char = 'ﬠ';
    /// \u{fb21}: 'ﬡ'
    pub const HEBREW_LETTER_WIDE_ALEF: char = 'ﬡ';
    /// \u{fb22}: 'ﬢ'
    pub const HEBREW_LETTER_WIDE_DALET: char = 'ﬢ';
    /// \u{fb23}: 'ﬣ'
    pub const HEBREW_LETTER_WIDE_HE: char = 'ﬣ';
    /// \u{fb24}: 'ﬤ'
    pub const HEBREW_LETTER_WIDE_KAF: char = 'ﬤ';
    /// \u{fb25}: 'ﬥ'
    pub const HEBREW_LETTER_WIDE_LAMED: char = 'ﬥ';
    /// \u{fb26}: 'ﬦ'
    pub const HEBREW_LETTER_WIDE_FINAL_MEM: char = 'ﬦ';
    /// \u{fb27}: 'ﬧ'
    pub const HEBREW_LETTER_WIDE_RESH: char = 'ﬧ';
    /// \u{fb28}: 'ﬨ'
    pub const HEBREW_LETTER_WIDE_TAV: char = 'ﬨ';
    /// \u{fb29}: '﬩'
    pub const HEBREW_LETTER_ALTERNATIVE_PLUS_SIGN: char = '﬩';
    /// \u{fb2a}: 'שׁ'
    pub const HEBREW_LETTER_SHIN_WITH_SHIN_DOT: char = 'שׁ';
    /// \u{fb2b}: 'שׂ'
    pub const HEBREW_LETTER_SHIN_WITH_SIN_DOT: char = 'שׂ';
    /// \u{fb2c}: 'שּׁ'
    pub const HEBREW_LETTER_SHIN_WITH_DAGESH_AND_SHIN_DOT: char = 'שּׁ';
    /// \u{fb2d}: 'שּׂ'
    pub const HEBREW_LETTER_SHIN_WITH_DAGESH_AND_SIN_DOT: char = 'שּׂ';
    /// \u{fb2e}: 'אַ'
    pub const HEBREW_LETTER_ALEF_WITH_PATAH: char = 'אַ';
    /// \u{fb2f}: 'אָ'
    pub const HEBREW_LETTER_ALEF_WITH_QAMATS: char = 'אָ';
    /// \u{fb30}: 'אּ'
    pub const HEBREW_LETTER_ALEF_WITH_MAPIQ: char = 'אּ';
    /// \u{fb31}: 'בּ'
    pub const HEBREW_LETTER_BET_WITH_DAGESH: char = 'בּ';
    /// \u{fb32}: 'גּ'
    pub const HEBREW_LETTER_GIMEL_WITH_DAGESH: char = 'גּ';
    /// \u{fb33}: 'דּ'
    pub const HEBREW_LETTER_DALET_WITH_DAGESH: char = 'דּ';
    /// \u{fb34}: 'הּ'
    pub const HEBREW_LETTER_HE_WITH_MAPIQ: char = 'הּ';
    /// \u{fb35}: 'וּ'
    pub const HEBREW_LETTER_VAV_WITH_DAGESH: char = 'וּ';
    /// \u{fb36}: 'זּ'
    pub const HEBREW_LETTER_ZAYIN_WITH_DAGESH: char = 'זּ';
    /// \u{fb38}: 'טּ'
    pub const HEBREW_LETTER_TET_WITH_DAGESH: char = 'טּ';
    /// \u{fb39}: 'יּ'
    pub const HEBREW_LETTER_YOD_WITH_DAGESH: char = 'יּ';
    /// \u{fb3a}: 'ךּ'
    pub const HEBREW_LETTER_FINAL_KAF_WITH_DAGESH: char = 'ךּ';
    /// \u{fb3b}: 'כּ'
    pub const HEBREW_LETTER_KAF_WITH_DAGESH: char = 'כּ';
    /// \u{fb3c}: 'לּ'
    pub const HEBREW_LETTER_LAMED_WITH_DAGESH: char = 'לּ';
    /// \u{fb3e}: 'מּ'
    pub const HEBREW_LETTER_MEM_WITH_DAGESH: char = 'מּ';
    /// \u{fb40}: 'נּ'
    pub const HEBREW_LETTER_NUN_WITH_DAGESH: char = 'נּ';
    /// \u{fb41}: 'סּ'
    pub const HEBREW_LETTER_SAMEKH_WITH_DAGESH: char = 'סּ';
    /// \u{fb43}: 'ףּ'
    pub const HEBREW_LETTER_FINAL_PE_WITH_DAGESH: char = 'ףּ';
    /// \u{fb44}: 'פּ'
    pub const HEBREW_LETTER_PE_WITH_DAGESH: char = 'פּ';
    /// \u{fb46}: 'צּ'
    pub const HEBREW_LETTER_TSADI_WITH_DAGESH: char = 'צּ';
    /// \u{fb47}: 'קּ'
    pub const HEBREW_LETTER_QOF_WITH_DAGESH: char = 'קּ';
    /// \u{fb48}: 'רּ'
    pub const HEBREW_LETTER_RESH_WITH_DAGESH: char = 'רּ';
    /// \u{fb49}: 'שּ'
    pub const HEBREW_LETTER_SHIN_WITH_DAGESH: char = 'שּ';
    /// \u{fb4a}: 'תּ'
    pub const HEBREW_LETTER_TAV_WITH_DAGESH: char = 'תּ';
    /// \u{fb4b}: 'וֹ'
    pub const HEBREW_LETTER_VAV_WITH_HOLAM: char = 'וֹ';
    /// \u{fb4c}: 'בֿ'
    pub const HEBREW_LETTER_BET_WITH_RAFE: char = 'בֿ';
    /// \u{fb4d}: 'כֿ'
    pub const HEBREW_LETTER_KAF_WITH_RAFE: char = 'כֿ';
    /// \u{fb4e}: 'פֿ'
    pub const HEBREW_LETTER_PE_WITH_RAFE: char = 'פֿ';
}

/// \u{fb00} → \u{fb4f}\
///\
/// ﬀ ﬁ ﬂ ﬃ ﬄ ﬅ ﬆ ﬓ ﬔ ﬕ ﬖ ﬗ יִ ﬞ ײַ ﬠ
/// ﬡ ﬢ ﬣ ﬤ ﬥ ﬦ ﬧ ﬨ ﬩ שׁ שׂ שּׁ שּׂ אַ אָ אּ
/// בּ גּ דּ הּ וּ זּ טּ יּ ךּ כּ לּ מּ נּ סּ ףּ פּ
/// צּ קּ רּ שּ תּ וֹ בֿ כֿ פֿ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlphabeticPresentationForms {
    /// \u{fb00}: 'ﬀ'
    LatinSmallLigatureFf,
    /// \u{fb01}: 'ﬁ'
    LatinSmallLigatureFi,
    /// \u{fb02}: 'ﬂ'
    LatinSmallLigatureFl,
    /// \u{fb03}: 'ﬃ'
    LatinSmallLigatureFfi,
    /// \u{fb04}: 'ﬄ'
    LatinSmallLigatureFfl,
    /// \u{fb05}: 'ﬅ'
    LatinSmallLigatureLongST,
    /// \u{fb06}: 'ﬆ'
    LatinSmallLigatureSt,
    /// \u{fb13}: 'ﬓ'
    ArmenianSmallLigatureMenNow,
    /// \u{fb14}: 'ﬔ'
    ArmenianSmallLigatureMenEch,
    /// \u{fb15}: 'ﬕ'
    ArmenianSmallLigatureMenIni,
    /// \u{fb16}: 'ﬖ'
    ArmenianSmallLigatureVewNow,
    /// \u{fb17}: 'ﬗ'
    ArmenianSmallLigatureMenXeh,
    /// \u{fb1d}: 'יִ'
    HebrewLetterYodWithHiriq,
    /// \u{fb1e}: 'ﬞ'
    HebrewPointJudeoDashSpanishVarika,
    /// \u{fb1f}: 'ײַ'
    HebrewLigatureYiddishYodYodPatah,
    /// \u{fb20}: 'ﬠ'
    HebrewLetterAlternativeAyin,
    /// \u{fb21}: 'ﬡ'
    HebrewLetterWideAlef,
    /// \u{fb22}: 'ﬢ'
    HebrewLetterWideDalet,
    /// \u{fb23}: 'ﬣ'
    HebrewLetterWideHe,
    /// \u{fb24}: 'ﬤ'
    HebrewLetterWideKaf,
    /// \u{fb25}: 'ﬥ'
    HebrewLetterWideLamed,
    /// \u{fb26}: 'ﬦ'
    HebrewLetterWideFinalMem,
    /// \u{fb27}: 'ﬧ'
    HebrewLetterWideResh,
    /// \u{fb28}: 'ﬨ'
    HebrewLetterWideTav,
    /// \u{fb29}: '﬩'
    HebrewLetterAlternativePlusSign,
    /// \u{fb2a}: 'שׁ'
    HebrewLetterShinWithShinDot,
    /// \u{fb2b}: 'שׂ'
    HebrewLetterShinWithSinDot,
    /// \u{fb2c}: 'שּׁ'
    HebrewLetterShinWithDageshAndShinDot,
    /// \u{fb2d}: 'שּׂ'
    HebrewLetterShinWithDageshAndSinDot,
    /// \u{fb2e}: 'אַ'
    HebrewLetterAlefWithPatah,
    /// \u{fb2f}: 'אָ'
    HebrewLetterAlefWithQamats,
    /// \u{fb30}: 'אּ'
    HebrewLetterAlefWithMapiq,
    /// \u{fb31}: 'בּ'
    HebrewLetterBetWithDagesh,
    /// \u{fb32}: 'גּ'
    HebrewLetterGimelWithDagesh,
    /// \u{fb33}: 'דּ'
    HebrewLetterDaletWithDagesh,
    /// \u{fb34}: 'הּ'
    HebrewLetterHeWithMapiq,
    /// \u{fb35}: 'וּ'
    HebrewLetterVavWithDagesh,
    /// \u{fb36}: 'זּ'
    HebrewLetterZayinWithDagesh,
    /// \u{fb38}: 'טּ'
    HebrewLetterTetWithDagesh,
    /// \u{fb39}: 'יּ'
    HebrewLetterYodWithDagesh,
    /// \u{fb3a}: 'ךּ'
    HebrewLetterFinalKafWithDagesh,
    /// \u{fb3b}: 'כּ'
    HebrewLetterKafWithDagesh,
    /// \u{fb3c}: 'לּ'
    HebrewLetterLamedWithDagesh,
    /// \u{fb3e}: 'מּ'
    HebrewLetterMemWithDagesh,
    /// \u{fb40}: 'נּ'
    HebrewLetterNunWithDagesh,
    /// \u{fb41}: 'סּ'
    HebrewLetterSamekhWithDagesh,
    /// \u{fb43}: 'ףּ'
    HebrewLetterFinalPeWithDagesh,
    /// \u{fb44}: 'פּ'
    HebrewLetterPeWithDagesh,
    /// \u{fb46}: 'צּ'
    HebrewLetterTsadiWithDagesh,
    /// \u{fb47}: 'קּ'
    HebrewLetterQofWithDagesh,
    /// \u{fb48}: 'רּ'
    HebrewLetterReshWithDagesh,
    /// \u{fb49}: 'שּ'
    HebrewLetterShinWithDagesh,
    /// \u{fb4a}: 'תּ'
    HebrewLetterTavWithDagesh,
    /// \u{fb4b}: 'וֹ'
    HebrewLetterVavWithHolam,
    /// \u{fb4c}: 'בֿ'
    HebrewLetterBetWithRafe,
    /// \u{fb4d}: 'כֿ'
    HebrewLetterKafWithRafe,
    /// \u{fb4e}: 'פֿ'
    HebrewLetterPeWithRafe,
}

impl Into<char> for AlphabeticPresentationForms {
    fn into(self) -> char {
        use constants::*;
        match self {
            AlphabeticPresentationForms::LatinSmallLigatureFf => LATIN_SMALL_LIGATURE_FF,
            AlphabeticPresentationForms::LatinSmallLigatureFi => LATIN_SMALL_LIGATURE_FI,
            AlphabeticPresentationForms::LatinSmallLigatureFl => LATIN_SMALL_LIGATURE_FL,
            AlphabeticPresentationForms::LatinSmallLigatureFfi => LATIN_SMALL_LIGATURE_FFI,
            AlphabeticPresentationForms::LatinSmallLigatureFfl => LATIN_SMALL_LIGATURE_FFL,
            AlphabeticPresentationForms::LatinSmallLigatureLongST => LATIN_SMALL_LIGATURE_LONG_S_T,
            AlphabeticPresentationForms::LatinSmallLigatureSt => LATIN_SMALL_LIGATURE_ST,
            AlphabeticPresentationForms::ArmenianSmallLigatureMenNow => ARMENIAN_SMALL_LIGATURE_MEN_NOW,
            AlphabeticPresentationForms::ArmenianSmallLigatureMenEch => ARMENIAN_SMALL_LIGATURE_MEN_ECH,
            AlphabeticPresentationForms::ArmenianSmallLigatureMenIni => ARMENIAN_SMALL_LIGATURE_MEN_INI,
            AlphabeticPresentationForms::ArmenianSmallLigatureVewNow => ARMENIAN_SMALL_LIGATURE_VEW_NOW,
            AlphabeticPresentationForms::ArmenianSmallLigatureMenXeh => ARMENIAN_SMALL_LIGATURE_MEN_XEH,
            AlphabeticPresentationForms::HebrewLetterYodWithHiriq => HEBREW_LETTER_YOD_WITH_HIRIQ,
            AlphabeticPresentationForms::HebrewPointJudeoDashSpanishVarika => HEBREW_POINT_JUDEO_DASH_SPANISH_VARIKA,
            AlphabeticPresentationForms::HebrewLigatureYiddishYodYodPatah => HEBREW_LIGATURE_YIDDISH_YOD_YOD_PATAH,
            AlphabeticPresentationForms::HebrewLetterAlternativeAyin => HEBREW_LETTER_ALTERNATIVE_AYIN,
            AlphabeticPresentationForms::HebrewLetterWideAlef => HEBREW_LETTER_WIDE_ALEF,
            AlphabeticPresentationForms::HebrewLetterWideDalet => HEBREW_LETTER_WIDE_DALET,
            AlphabeticPresentationForms::HebrewLetterWideHe => HEBREW_LETTER_WIDE_HE,
            AlphabeticPresentationForms::HebrewLetterWideKaf => HEBREW_LETTER_WIDE_KAF,
            AlphabeticPresentationForms::HebrewLetterWideLamed => HEBREW_LETTER_WIDE_LAMED,
            AlphabeticPresentationForms::HebrewLetterWideFinalMem => HEBREW_LETTER_WIDE_FINAL_MEM,
            AlphabeticPresentationForms::HebrewLetterWideResh => HEBREW_LETTER_WIDE_RESH,
            AlphabeticPresentationForms::HebrewLetterWideTav => HEBREW_LETTER_WIDE_TAV,
            AlphabeticPresentationForms::HebrewLetterAlternativePlusSign => HEBREW_LETTER_ALTERNATIVE_PLUS_SIGN,
            AlphabeticPresentationForms::HebrewLetterShinWithShinDot => HEBREW_LETTER_SHIN_WITH_SHIN_DOT,
            AlphabeticPresentationForms::HebrewLetterShinWithSinDot => HEBREW_LETTER_SHIN_WITH_SIN_DOT,
            AlphabeticPresentationForms::HebrewLetterShinWithDageshAndShinDot => HEBREW_LETTER_SHIN_WITH_DAGESH_AND_SHIN_DOT,
            AlphabeticPresentationForms::HebrewLetterShinWithDageshAndSinDot => HEBREW_LETTER_SHIN_WITH_DAGESH_AND_SIN_DOT,
            AlphabeticPresentationForms::HebrewLetterAlefWithPatah => HEBREW_LETTER_ALEF_WITH_PATAH,
            AlphabeticPresentationForms::HebrewLetterAlefWithQamats => HEBREW_LETTER_ALEF_WITH_QAMATS,
            AlphabeticPresentationForms::HebrewLetterAlefWithMapiq => HEBREW_LETTER_ALEF_WITH_MAPIQ,
            AlphabeticPresentationForms::HebrewLetterBetWithDagesh => HEBREW_LETTER_BET_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterGimelWithDagesh => HEBREW_LETTER_GIMEL_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterDaletWithDagesh => HEBREW_LETTER_DALET_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterHeWithMapiq => HEBREW_LETTER_HE_WITH_MAPIQ,
            AlphabeticPresentationForms::HebrewLetterVavWithDagesh => HEBREW_LETTER_VAV_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterZayinWithDagesh => HEBREW_LETTER_ZAYIN_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterTetWithDagesh => HEBREW_LETTER_TET_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterYodWithDagesh => HEBREW_LETTER_YOD_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterFinalKafWithDagesh => HEBREW_LETTER_FINAL_KAF_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterKafWithDagesh => HEBREW_LETTER_KAF_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterLamedWithDagesh => HEBREW_LETTER_LAMED_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterMemWithDagesh => HEBREW_LETTER_MEM_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterNunWithDagesh => HEBREW_LETTER_NUN_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterSamekhWithDagesh => HEBREW_LETTER_SAMEKH_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterFinalPeWithDagesh => HEBREW_LETTER_FINAL_PE_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterPeWithDagesh => HEBREW_LETTER_PE_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterTsadiWithDagesh => HEBREW_LETTER_TSADI_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterQofWithDagesh => HEBREW_LETTER_QOF_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterReshWithDagesh => HEBREW_LETTER_RESH_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterShinWithDagesh => HEBREW_LETTER_SHIN_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterTavWithDagesh => HEBREW_LETTER_TAV_WITH_DAGESH,
            AlphabeticPresentationForms::HebrewLetterVavWithHolam => HEBREW_LETTER_VAV_WITH_HOLAM,
            AlphabeticPresentationForms::HebrewLetterBetWithRafe => HEBREW_LETTER_BET_WITH_RAFE,
            AlphabeticPresentationForms::HebrewLetterKafWithRafe => HEBREW_LETTER_KAF_WITH_RAFE,
            AlphabeticPresentationForms::HebrewLetterPeWithRafe => HEBREW_LETTER_PE_WITH_RAFE,
        }
    }
}

impl std::convert::TryFrom<char> for AlphabeticPresentationForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LATIN_SMALL_LIGATURE_FF => Ok(AlphabeticPresentationForms::LatinSmallLigatureFf),
            LATIN_SMALL_LIGATURE_FI => Ok(AlphabeticPresentationForms::LatinSmallLigatureFi),
            LATIN_SMALL_LIGATURE_FL => Ok(AlphabeticPresentationForms::LatinSmallLigatureFl),
            LATIN_SMALL_LIGATURE_FFI => Ok(AlphabeticPresentationForms::LatinSmallLigatureFfi),
            LATIN_SMALL_LIGATURE_FFL => Ok(AlphabeticPresentationForms::LatinSmallLigatureFfl),
            LATIN_SMALL_LIGATURE_LONG_S_T => Ok(AlphabeticPresentationForms::LatinSmallLigatureLongST),
            LATIN_SMALL_LIGATURE_ST => Ok(AlphabeticPresentationForms::LatinSmallLigatureSt),
            ARMENIAN_SMALL_LIGATURE_MEN_NOW => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenNow),
            ARMENIAN_SMALL_LIGATURE_MEN_ECH => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenEch),
            ARMENIAN_SMALL_LIGATURE_MEN_INI => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenIni),
            ARMENIAN_SMALL_LIGATURE_VEW_NOW => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureVewNow),
            ARMENIAN_SMALL_LIGATURE_MEN_XEH => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenXeh),
            HEBREW_LETTER_YOD_WITH_HIRIQ => Ok(AlphabeticPresentationForms::HebrewLetterYodWithHiriq),
            HEBREW_POINT_JUDEO_DASH_SPANISH_VARIKA => Ok(AlphabeticPresentationForms::HebrewPointJudeoDashSpanishVarika),
            HEBREW_LIGATURE_YIDDISH_YOD_YOD_PATAH => Ok(AlphabeticPresentationForms::HebrewLigatureYiddishYodYodPatah),
            HEBREW_LETTER_ALTERNATIVE_AYIN => Ok(AlphabeticPresentationForms::HebrewLetterAlternativeAyin),
            HEBREW_LETTER_WIDE_ALEF => Ok(AlphabeticPresentationForms::HebrewLetterWideAlef),
            HEBREW_LETTER_WIDE_DALET => Ok(AlphabeticPresentationForms::HebrewLetterWideDalet),
            HEBREW_LETTER_WIDE_HE => Ok(AlphabeticPresentationForms::HebrewLetterWideHe),
            HEBREW_LETTER_WIDE_KAF => Ok(AlphabeticPresentationForms::HebrewLetterWideKaf),
            HEBREW_LETTER_WIDE_LAMED => Ok(AlphabeticPresentationForms::HebrewLetterWideLamed),
            HEBREW_LETTER_WIDE_FINAL_MEM => Ok(AlphabeticPresentationForms::HebrewLetterWideFinalMem),
            HEBREW_LETTER_WIDE_RESH => Ok(AlphabeticPresentationForms::HebrewLetterWideResh),
            HEBREW_LETTER_WIDE_TAV => Ok(AlphabeticPresentationForms::HebrewLetterWideTav),
            HEBREW_LETTER_ALTERNATIVE_PLUS_SIGN => Ok(AlphabeticPresentationForms::HebrewLetterAlternativePlusSign),
            HEBREW_LETTER_SHIN_WITH_SHIN_DOT => Ok(AlphabeticPresentationForms::HebrewLetterShinWithShinDot),
            HEBREW_LETTER_SHIN_WITH_SIN_DOT => Ok(AlphabeticPresentationForms::HebrewLetterShinWithSinDot),
            HEBREW_LETTER_SHIN_WITH_DAGESH_AND_SHIN_DOT => Ok(AlphabeticPresentationForms::HebrewLetterShinWithDageshAndShinDot),
            HEBREW_LETTER_SHIN_WITH_DAGESH_AND_SIN_DOT => Ok(AlphabeticPresentationForms::HebrewLetterShinWithDageshAndSinDot),
            HEBREW_LETTER_ALEF_WITH_PATAH => Ok(AlphabeticPresentationForms::HebrewLetterAlefWithPatah),
            HEBREW_LETTER_ALEF_WITH_QAMATS => Ok(AlphabeticPresentationForms::HebrewLetterAlefWithQamats),
            HEBREW_LETTER_ALEF_WITH_MAPIQ => Ok(AlphabeticPresentationForms::HebrewLetterAlefWithMapiq),
            HEBREW_LETTER_BET_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterBetWithDagesh),
            HEBREW_LETTER_GIMEL_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterGimelWithDagesh),
            HEBREW_LETTER_DALET_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterDaletWithDagesh),
            HEBREW_LETTER_HE_WITH_MAPIQ => Ok(AlphabeticPresentationForms::HebrewLetterHeWithMapiq),
            HEBREW_LETTER_VAV_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterVavWithDagesh),
            HEBREW_LETTER_ZAYIN_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterZayinWithDagesh),
            HEBREW_LETTER_TET_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterTetWithDagesh),
            HEBREW_LETTER_YOD_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterYodWithDagesh),
            HEBREW_LETTER_FINAL_KAF_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterFinalKafWithDagesh),
            HEBREW_LETTER_KAF_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterKafWithDagesh),
            HEBREW_LETTER_LAMED_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterLamedWithDagesh),
            HEBREW_LETTER_MEM_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterMemWithDagesh),
            HEBREW_LETTER_NUN_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterNunWithDagesh),
            HEBREW_LETTER_SAMEKH_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterSamekhWithDagesh),
            HEBREW_LETTER_FINAL_PE_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterFinalPeWithDagesh),
            HEBREW_LETTER_PE_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterPeWithDagesh),
            HEBREW_LETTER_TSADI_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterTsadiWithDagesh),
            HEBREW_LETTER_QOF_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterQofWithDagesh),
            HEBREW_LETTER_RESH_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterReshWithDagesh),
            HEBREW_LETTER_SHIN_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterShinWithDagesh),
            HEBREW_LETTER_TAV_WITH_DAGESH => Ok(AlphabeticPresentationForms::HebrewLetterTavWithDagesh),
            HEBREW_LETTER_VAV_WITH_HOLAM => Ok(AlphabeticPresentationForms::HebrewLetterVavWithHolam),
            HEBREW_LETTER_BET_WITH_RAFE => Ok(AlphabeticPresentationForms::HebrewLetterBetWithRafe),
            HEBREW_LETTER_KAF_WITH_RAFE => Ok(AlphabeticPresentationForms::HebrewLetterKafWithRafe),
            HEBREW_LETTER_PE_WITH_RAFE => Ok(AlphabeticPresentationForms::HebrewLetterPeWithRafe),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AlphabeticPresentationForms {
    fn into(self) -> u32 {
        let c: char = self.into();
        let hex = c
            .escape_unicode()
            .to_string()
            .replace("\\u{", "")
            .replace("}", "");
        u32::from_str_radix(&hex, 16).unwrap()
    }
}

impl std::convert::TryFrom<u32> for AlphabeticPresentationForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AlphabeticPresentationForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AlphabeticPresentationForms {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        AlphabeticPresentationForms::LatinSmallLigatureFf
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            AlphabeticPresentationForms::LatinSmallLigatureFf => "latin small ligature ff",
            AlphabeticPresentationForms::LatinSmallLigatureFi => "latin small ligature fi",
            AlphabeticPresentationForms::LatinSmallLigatureFl => "latin small ligature fl",
            AlphabeticPresentationForms::LatinSmallLigatureFfi => "latin small ligature ffi",
            AlphabeticPresentationForms::LatinSmallLigatureFfl => "latin small ligature ffl",
            AlphabeticPresentationForms::LatinSmallLigatureLongST => "latin small ligature long s t",
            AlphabeticPresentationForms::LatinSmallLigatureSt => "latin small ligature st",
            AlphabeticPresentationForms::ArmenianSmallLigatureMenNow => "armenian small ligature men now",
            AlphabeticPresentationForms::ArmenianSmallLigatureMenEch => "armenian small ligature men ech",
            AlphabeticPresentationForms::ArmenianSmallLigatureMenIni => "armenian small ligature men ini",
            AlphabeticPresentationForms::ArmenianSmallLigatureVewNow => "armenian small ligature vew now",
            AlphabeticPresentationForms::ArmenianSmallLigatureMenXeh => "armenian small ligature men xeh",
            AlphabeticPresentationForms::HebrewLetterYodWithHiriq => "hebrew letter yod with hiriq",
            AlphabeticPresentationForms::HebrewPointJudeoDashSpanishVarika => "hebrew point judeo-spanish varika",
            AlphabeticPresentationForms::HebrewLigatureYiddishYodYodPatah => "hebrew ligature yiddish yod yod patah",
            AlphabeticPresentationForms::HebrewLetterAlternativeAyin => "hebrew letter alternative ayin",
            AlphabeticPresentationForms::HebrewLetterWideAlef => "hebrew letter wide alef",
            AlphabeticPresentationForms::HebrewLetterWideDalet => "hebrew letter wide dalet",
            AlphabeticPresentationForms::HebrewLetterWideHe => "hebrew letter wide he",
            AlphabeticPresentationForms::HebrewLetterWideKaf => "hebrew letter wide kaf",
            AlphabeticPresentationForms::HebrewLetterWideLamed => "hebrew letter wide lamed",
            AlphabeticPresentationForms::HebrewLetterWideFinalMem => "hebrew letter wide final mem",
            AlphabeticPresentationForms::HebrewLetterWideResh => "hebrew letter wide resh",
            AlphabeticPresentationForms::HebrewLetterWideTav => "hebrew letter wide tav",
            AlphabeticPresentationForms::HebrewLetterAlternativePlusSign => "hebrew letter alternative plus sign",
            AlphabeticPresentationForms::HebrewLetterShinWithShinDot => "hebrew letter shin with shin dot",
            AlphabeticPresentationForms::HebrewLetterShinWithSinDot => "hebrew letter shin with sin dot",
            AlphabeticPresentationForms::HebrewLetterShinWithDageshAndShinDot => "hebrew letter shin with dagesh and shin dot",
            AlphabeticPresentationForms::HebrewLetterShinWithDageshAndSinDot => "hebrew letter shin with dagesh and sin dot",
            AlphabeticPresentationForms::HebrewLetterAlefWithPatah => "hebrew letter alef with patah",
            AlphabeticPresentationForms::HebrewLetterAlefWithQamats => "hebrew letter alef with qamats",
            AlphabeticPresentationForms::HebrewLetterAlefWithMapiq => "hebrew letter alef with mapiq",
            AlphabeticPresentationForms::HebrewLetterBetWithDagesh => "hebrew letter bet with dagesh",
            AlphabeticPresentationForms::HebrewLetterGimelWithDagesh => "hebrew letter gimel with dagesh",
            AlphabeticPresentationForms::HebrewLetterDaletWithDagesh => "hebrew letter dalet with dagesh",
            AlphabeticPresentationForms::HebrewLetterHeWithMapiq => "hebrew letter he with mapiq",
            AlphabeticPresentationForms::HebrewLetterVavWithDagesh => "hebrew letter vav with dagesh",
            AlphabeticPresentationForms::HebrewLetterZayinWithDagesh => "hebrew letter zayin with dagesh",
            AlphabeticPresentationForms::HebrewLetterTetWithDagesh => "hebrew letter tet with dagesh",
            AlphabeticPresentationForms::HebrewLetterYodWithDagesh => "hebrew letter yod with dagesh",
            AlphabeticPresentationForms::HebrewLetterFinalKafWithDagesh => "hebrew letter final kaf with dagesh",
            AlphabeticPresentationForms::HebrewLetterKafWithDagesh => "hebrew letter kaf with dagesh",
            AlphabeticPresentationForms::HebrewLetterLamedWithDagesh => "hebrew letter lamed with dagesh",
            AlphabeticPresentationForms::HebrewLetterMemWithDagesh => "hebrew letter mem with dagesh",
            AlphabeticPresentationForms::HebrewLetterNunWithDagesh => "hebrew letter nun with dagesh",
            AlphabeticPresentationForms::HebrewLetterSamekhWithDagesh => "hebrew letter samekh with dagesh",
            AlphabeticPresentationForms::HebrewLetterFinalPeWithDagesh => "hebrew letter final pe with dagesh",
            AlphabeticPresentationForms::HebrewLetterPeWithDagesh => "hebrew letter pe with dagesh",
            AlphabeticPresentationForms::HebrewLetterTsadiWithDagesh => "hebrew letter tsadi with dagesh",
            AlphabeticPresentationForms::HebrewLetterQofWithDagesh => "hebrew letter qof with dagesh",
            AlphabeticPresentationForms::HebrewLetterReshWithDagesh => "hebrew letter resh with dagesh",
            AlphabeticPresentationForms::HebrewLetterShinWithDagesh => "hebrew letter shin with dagesh",
            AlphabeticPresentationForms::HebrewLetterTavWithDagesh => "hebrew letter tav with dagesh",
            AlphabeticPresentationForms::HebrewLetterVavWithHolam => "hebrew letter vav with holam",
            AlphabeticPresentationForms::HebrewLetterBetWithRafe => "hebrew letter bet with rafe",
            AlphabeticPresentationForms::HebrewLetterKafWithRafe => "hebrew letter kaf with rafe",
            AlphabeticPresentationForms::HebrewLetterPeWithRafe => "hebrew letter pe with rafe",
        }
    }
}
