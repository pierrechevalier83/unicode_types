/// \u{590} → \u{5ff}\
///\
/// ֑ ֒ ֓ ֔ ֕ ֖ ֗ ֘ ֙ ֚ ֛ ֜ ֝ ֞ ֟ ֠
/// ֡ ֢ ֣ ֤ ֥ ֦ ֧ ֨ ֩ ֪ ֫ ֬ ֭ ֮ ֯ ְ
/// ֱ ֲ ֳ ִ ֵ ֶ ַ ָ ֹ ֺ ֻ ּ ֽ ־ ֿ ׀
/// ׁ ׂ ׃ ׄ ׅ ׆ ׇ א ב ג ד ה ו ז ח ט
/// י ך כ ל ם מ ן נ ס ע ף פ ץ צ ק ר
/// ש ת ׯ װ ױ ײ ׳ ״
pub mod constants {
    /// \u{591}: '֑'
    pub const HEBREW_ACCENT_ETNAHTA: char = '֑';
    /// \u{592}: '֒'
    pub const HEBREW_ACCENT_SEGOL: char = '֒';
    /// \u{593}: '֓'
    pub const HEBREW_ACCENT_SHALSHELET: char = '֓';
    /// \u{594}: '֔'
    pub const HEBREW_ACCENT_ZAQEF_QATAN: char = '֔';
    /// \u{595}: '֕'
    pub const HEBREW_ACCENT_ZAQEF_GADOL: char = '֕';
    /// \u{596}: '֖'
    pub const HEBREW_ACCENT_TIPEHA: char = '֖';
    /// \u{597}: '֗'
    pub const HEBREW_ACCENT_REVIA: char = '֗';
    /// \u{598}: '֘'
    pub const HEBREW_ACCENT_ZARQA: char = '֘';
    /// \u{599}: '֙'
    pub const HEBREW_ACCENT_PASHTA: char = '֙';
    /// \u{59a}: '֚'
    pub const HEBREW_ACCENT_YETIV: char = '֚';
    /// \u{59b}: '֛'
    pub const HEBREW_ACCENT_TEVIR: char = '֛';
    /// \u{59c}: '֜'
    pub const HEBREW_ACCENT_GERESH: char = '֜';
    /// \u{59d}: '֝'
    pub const HEBREW_ACCENT_GERESH_MUQDAM: char = '֝';
    /// \u{59e}: '֞'
    pub const HEBREW_ACCENT_GERSHAYIM: char = '֞';
    /// \u{59f}: '֟'
    pub const HEBREW_ACCENT_QARNEY_PARA: char = '֟';
    /// \u{5a0}: '֠'
    pub const HEBREW_ACCENT_TELISHA_GEDOLA: char = '֠';
    /// \u{5a1}: '֡'
    pub const HEBREW_ACCENT_PAZER: char = '֡';
    /// \u{5a2}: '֢'
    pub const HEBREW_ACCENT_ATNAH_HAFUKH: char = '֢';
    /// \u{5a3}: '֣'
    pub const HEBREW_ACCENT_MUNAH: char = '֣';
    /// \u{5a4}: '֤'
    pub const HEBREW_ACCENT_MAHAPAKH: char = '֤';
    /// \u{5a5}: '֥'
    pub const HEBREW_ACCENT_MERKHA: char = '֥';
    /// \u{5a6}: '֦'
    pub const HEBREW_ACCENT_MERKHA_KEFULA: char = '֦';
    /// \u{5a7}: '֧'
    pub const HEBREW_ACCENT_DARGA: char = '֧';
    /// \u{5a8}: '֨'
    pub const HEBREW_ACCENT_QADMA: char = '֨';
    /// \u{5a9}: '֩'
    pub const HEBREW_ACCENT_TELISHA_QETANA: char = '֩';
    /// \u{5aa}: '֪'
    pub const HEBREW_ACCENT_YERAH_BEN_YOMO: char = '֪';
    /// \u{5ab}: '֫'
    pub const HEBREW_ACCENT_OLE: char = '֫';
    /// \u{5ac}: '֬'
    pub const HEBREW_ACCENT_ILUY: char = '֬';
    /// \u{5ad}: '֭'
    pub const HEBREW_ACCENT_DEHI: char = '֭';
    /// \u{5ae}: '֮'
    pub const HEBREW_ACCENT_ZINOR: char = '֮';
    /// \u{5af}: '֯'
    pub const HEBREW_MARK_MASORA_CIRCLE: char = '֯';
    /// \u{5b0}: 'ְ'
    pub const HEBREW_POINT_SHEVA: char = 'ְ';
    /// \u{5b1}: 'ֱ'
    pub const HEBREW_POINT_HATAF_SEGOL: char = 'ֱ';
    /// \u{5b2}: 'ֲ'
    pub const HEBREW_POINT_HATAF_PATAH: char = 'ֲ';
    /// \u{5b3}: 'ֳ'
    pub const HEBREW_POINT_HATAF_QAMATS: char = 'ֳ';
    /// \u{5b4}: 'ִ'
    pub const HEBREW_POINT_HIRIQ: char = 'ִ';
    /// \u{5b5}: 'ֵ'
    pub const HEBREW_POINT_TSERE: char = 'ֵ';
    /// \u{5b6}: 'ֶ'
    pub const HEBREW_POINT_SEGOL: char = 'ֶ';
    /// \u{5b7}: 'ַ'
    pub const HEBREW_POINT_PATAH: char = 'ַ';
    /// \u{5b8}: 'ָ'
    pub const HEBREW_POINT_QAMATS: char = 'ָ';
    /// \u{5b9}: 'ֹ'
    pub const HEBREW_POINT_HOLAM: char = 'ֹ';
    /// \u{5ba}: 'ֺ'
    pub const HEBREW_POINT_HOLAM_HASER_FOR_VAV: char = 'ֺ';
    /// \u{5bb}: 'ֻ'
    pub const HEBREW_POINT_QUBUTS: char = 'ֻ';
    /// \u{5bc}: 'ּ'
    pub const HEBREW_POINT_DAGESH_OR_MAPIQ: char = 'ּ';
    /// \u{5bd}: 'ֽ'
    pub const HEBREW_POINT_METEG: char = 'ֽ';
    /// \u{5be}: '־'
    pub const HEBREW_PUNCTUATION_MAQAF: char = '־';
    /// \u{5bf}: 'ֿ'
    pub const HEBREW_POINT_RAFE: char = 'ֿ';
    /// \u{5c0}: '׀'
    pub const HEBREW_PUNCTUATION_PASEQ: char = '׀';
    /// \u{5c1}: 'ׁ'
    pub const HEBREW_POINT_SHIN_DOT: char = 'ׁ';
    /// \u{5c2}: 'ׂ'
    pub const HEBREW_POINT_SIN_DOT: char = 'ׂ';
    /// \u{5c3}: '׃'
    pub const HEBREW_PUNCTUATION_SOF_PASUQ: char = '׃';
    /// \u{5c4}: 'ׄ'
    pub const HEBREW_MARK_UPPER_DOT: char = 'ׄ';
    /// \u{5c5}: 'ׅ'
    pub const HEBREW_MARK_LOWER_DOT: char = 'ׅ';
    /// \u{5c6}: '׆'
    pub const HEBREW_PUNCTUATION_NUN_HAFUKHA: char = '׆';
    /// \u{5c7}: 'ׇ'
    pub const HEBREW_POINT_QAMATS_QATAN: char = 'ׇ';
    /// \u{5d0}: 'א'
    pub const HEBREW_LETTER_ALEF: char = 'א';
    /// \u{5d1}: 'ב'
    pub const HEBREW_LETTER_BET: char = 'ב';
    /// \u{5d2}: 'ג'
    pub const HEBREW_LETTER_GIMEL: char = 'ג';
    /// \u{5d3}: 'ד'
    pub const HEBREW_LETTER_DALET: char = 'ד';
    /// \u{5d4}: 'ה'
    pub const HEBREW_LETTER_HE: char = 'ה';
    /// \u{5d5}: 'ו'
    pub const HEBREW_LETTER_VAV: char = 'ו';
    /// \u{5d6}: 'ז'
    pub const HEBREW_LETTER_ZAYIN: char = 'ז';
    /// \u{5d7}: 'ח'
    pub const HEBREW_LETTER_HET: char = 'ח';
    /// \u{5d8}: 'ט'
    pub const HEBREW_LETTER_TET: char = 'ט';
    /// \u{5d9}: 'י'
    pub const HEBREW_LETTER_YOD: char = 'י';
    /// \u{5da}: 'ך'
    pub const HEBREW_LETTER_FINAL_KAF: char = 'ך';
    /// \u{5db}: 'כ'
    pub const HEBREW_LETTER_KAF: char = 'כ';
    /// \u{5dc}: 'ל'
    pub const HEBREW_LETTER_LAMED: char = 'ל';
    /// \u{5dd}: 'ם'
    pub const HEBREW_LETTER_FINAL_MEM: char = 'ם';
    /// \u{5de}: 'מ'
    pub const HEBREW_LETTER_MEM: char = 'מ';
    /// \u{5df}: 'ן'
    pub const HEBREW_LETTER_FINAL_NUN: char = 'ן';
    /// \u{5e0}: 'נ'
    pub const HEBREW_LETTER_NUN: char = 'נ';
    /// \u{5e1}: 'ס'
    pub const HEBREW_LETTER_SAMEKH: char = 'ס';
    /// \u{5e2}: 'ע'
    pub const HEBREW_LETTER_AYIN: char = 'ע';
    /// \u{5e3}: 'ף'
    pub const HEBREW_LETTER_FINAL_PE: char = 'ף';
    /// \u{5e4}: 'פ'
    pub const HEBREW_LETTER_PE: char = 'פ';
    /// \u{5e5}: 'ץ'
    pub const HEBREW_LETTER_FINAL_TSADI: char = 'ץ';
    /// \u{5e6}: 'צ'
    pub const HEBREW_LETTER_TSADI: char = 'צ';
    /// \u{5e7}: 'ק'
    pub const HEBREW_LETTER_QOF: char = 'ק';
    /// \u{5e8}: 'ר'
    pub const HEBREW_LETTER_RESH: char = 'ר';
    /// \u{5e9}: 'ש'
    pub const HEBREW_LETTER_SHIN: char = 'ש';
    /// \u{5ea}: 'ת'
    pub const HEBREW_LETTER_TAV: char = 'ת';
    /// \u{5ef}: 'ׯ'
    pub const HEBREW_YOD_TRIANGLE: char = 'ׯ';
    /// \u{5f0}: 'װ'
    pub const HEBREW_LIGATURE_YIDDISH_DOUBLE_VAV: char = 'װ';
    /// \u{5f1}: 'ױ'
    pub const HEBREW_LIGATURE_YIDDISH_VAV_YOD: char = 'ױ';
    /// \u{5f2}: 'ײ'
    pub const HEBREW_LIGATURE_YIDDISH_DOUBLE_YOD: char = 'ײ';
    /// \u{5f3}: '׳'
    pub const HEBREW_PUNCTUATION_GERESH: char = '׳';
    /// \u{5f4}: '״'
    pub const HEBREW_PUNCTUATION_GERSHAYIM: char = '״';
}

/// \u{590} → \u{5ff}\
///\
/// ֑ ֒ ֓ ֔ ֕ ֖ ֗ ֘ ֙ ֚ ֛ ֜ ֝ ֞ ֟ ֠
/// ֡ ֢ ֣ ֤ ֥ ֦ ֧ ֨ ֩ ֪ ֫ ֬ ֭ ֮ ֯ ְ
/// ֱ ֲ ֳ ִ ֵ ֶ ַ ָ ֹ ֺ ֻ ּ ֽ ־ ֿ ׀
/// ׁ ׂ ׃ ׄ ׅ ׆ ׇ א ב ג ד ה ו ז ח ט
/// י ך כ ל ם מ ן נ ס ע ף פ ץ צ ק ר
/// ש ת ׯ װ ױ ײ ׳ ״
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hebrew {
    /// \u{591}: '֑'
    HebrewAccentEtnahta,
    /// \u{592}: '֒'
    HebrewAccentSegol,
    /// \u{593}: '֓'
    HebrewAccentShalshelet,
    /// \u{594}: '֔'
    HebrewAccentZaqefQatan,
    /// \u{595}: '֕'
    HebrewAccentZaqefGadol,
    /// \u{596}: '֖'
    HebrewAccentTipeha,
    /// \u{597}: '֗'
    HebrewAccentRevia,
    /// \u{598}: '֘'
    HebrewAccentZarqa,
    /// \u{599}: '֙'
    HebrewAccentPashta,
    /// \u{59a}: '֚'
    HebrewAccentYetiv,
    /// \u{59b}: '֛'
    HebrewAccentTevir,
    /// \u{59c}: '֜'
    HebrewAccentGeresh,
    /// \u{59d}: '֝'
    HebrewAccentGereshMuqdam,
    /// \u{59e}: '֞'
    HebrewAccentGershayim,
    /// \u{59f}: '֟'
    HebrewAccentQarneyPara,
    /// \u{5a0}: '֠'
    HebrewAccentTelishaGedola,
    /// \u{5a1}: '֡'
    HebrewAccentPazer,
    /// \u{5a2}: '֢'
    HebrewAccentAtnahHafukh,
    /// \u{5a3}: '֣'
    HebrewAccentMunah,
    /// \u{5a4}: '֤'
    HebrewAccentMahapakh,
    /// \u{5a5}: '֥'
    HebrewAccentMerkha,
    /// \u{5a6}: '֦'
    HebrewAccentMerkhaKefula,
    /// \u{5a7}: '֧'
    HebrewAccentDarga,
    /// \u{5a8}: '֨'
    HebrewAccentQadma,
    /// \u{5a9}: '֩'
    HebrewAccentTelishaQetana,
    /// \u{5aa}: '֪'
    HebrewAccentYerahBenYomo,
    /// \u{5ab}: '֫'
    HebrewAccentOle,
    /// \u{5ac}: '֬'
    HebrewAccentIluy,
    /// \u{5ad}: '֭'
    HebrewAccentDehi,
    /// \u{5ae}: '֮'
    HebrewAccentZinor,
    /// \u{5af}: '֯'
    HebrewMarkMasoraCircle,
    /// \u{5b0}: 'ְ'
    HebrewPointSheva,
    /// \u{5b1}: 'ֱ'
    HebrewPointHatafSegol,
    /// \u{5b2}: 'ֲ'
    HebrewPointHatafPatah,
    /// \u{5b3}: 'ֳ'
    HebrewPointHatafQamats,
    /// \u{5b4}: 'ִ'
    HebrewPointHiriq,
    /// \u{5b5}: 'ֵ'
    HebrewPointTsere,
    /// \u{5b6}: 'ֶ'
    HebrewPointSegol,
    /// \u{5b7}: 'ַ'
    HebrewPointPatah,
    /// \u{5b8}: 'ָ'
    HebrewPointQamats,
    /// \u{5b9}: 'ֹ'
    HebrewPointHolam,
    /// \u{5ba}: 'ֺ'
    HebrewPointHolamHaserForVav,
    /// \u{5bb}: 'ֻ'
    HebrewPointQubuts,
    /// \u{5bc}: 'ּ'
    HebrewPointDageshOrMapiq,
    /// \u{5bd}: 'ֽ'
    HebrewPointMeteg,
    /// \u{5be}: '־'
    HebrewPunctuationMaqaf,
    /// \u{5bf}: 'ֿ'
    HebrewPointRafe,
    /// \u{5c0}: '׀'
    HebrewPunctuationPaseq,
    /// \u{5c1}: 'ׁ'
    HebrewPointShinDot,
    /// \u{5c2}: 'ׂ'
    HebrewPointSinDot,
    /// \u{5c3}: '׃'
    HebrewPunctuationSofPasuq,
    /// \u{5c4}: 'ׄ'
    HebrewMarkUpperDot,
    /// \u{5c5}: 'ׅ'
    HebrewMarkLowerDot,
    /// \u{5c6}: '׆'
    HebrewPunctuationNunHafukha,
    /// \u{5c7}: 'ׇ'
    HebrewPointQamatsQatan,
    /// \u{5d0}: 'א'
    HebrewLetterAlef,
    /// \u{5d1}: 'ב'
    HebrewLetterBet,
    /// \u{5d2}: 'ג'
    HebrewLetterGimel,
    /// \u{5d3}: 'ד'
    HebrewLetterDalet,
    /// \u{5d4}: 'ה'
    HebrewLetterHe,
    /// \u{5d5}: 'ו'
    HebrewLetterVav,
    /// \u{5d6}: 'ז'
    HebrewLetterZayin,
    /// \u{5d7}: 'ח'
    HebrewLetterHet,
    /// \u{5d8}: 'ט'
    HebrewLetterTet,
    /// \u{5d9}: 'י'
    HebrewLetterYod,
    /// \u{5da}: 'ך'
    HebrewLetterFinalKaf,
    /// \u{5db}: 'כ'
    HebrewLetterKaf,
    /// \u{5dc}: 'ל'
    HebrewLetterLamed,
    /// \u{5dd}: 'ם'
    HebrewLetterFinalMem,
    /// \u{5de}: 'מ'
    HebrewLetterMem,
    /// \u{5df}: 'ן'
    HebrewLetterFinalNun,
    /// \u{5e0}: 'נ'
    HebrewLetterNun,
    /// \u{5e1}: 'ס'
    HebrewLetterSamekh,
    /// \u{5e2}: 'ע'
    HebrewLetterAyin,
    /// \u{5e3}: 'ף'
    HebrewLetterFinalPe,
    /// \u{5e4}: 'פ'
    HebrewLetterPe,
    /// \u{5e5}: 'ץ'
    HebrewLetterFinalTsadi,
    /// \u{5e6}: 'צ'
    HebrewLetterTsadi,
    /// \u{5e7}: 'ק'
    HebrewLetterQof,
    /// \u{5e8}: 'ר'
    HebrewLetterResh,
    /// \u{5e9}: 'ש'
    HebrewLetterShin,
    /// \u{5ea}: 'ת'
    HebrewLetterTav,
    /// \u{5ef}: 'ׯ'
    HebrewYodTriangle,
    /// \u{5f0}: 'װ'
    HebrewLigatureYiddishDoubleVav,
    /// \u{5f1}: 'ױ'
    HebrewLigatureYiddishVavYod,
    /// \u{5f2}: 'ײ'
    HebrewLigatureYiddishDoubleYod,
    /// \u{5f3}: '׳'
    HebrewPunctuationGeresh,
    /// \u{5f4}: '״'
    HebrewPunctuationGershayim,
}

impl Into<char> for Hebrew {
    fn into(self) -> char {
        use constants::*;
        match self {
            Hebrew::HebrewAccentEtnahta => HEBREW_ACCENT_ETNAHTA,
            Hebrew::HebrewAccentSegol => HEBREW_ACCENT_SEGOL,
            Hebrew::HebrewAccentShalshelet => HEBREW_ACCENT_SHALSHELET,
            Hebrew::HebrewAccentZaqefQatan => HEBREW_ACCENT_ZAQEF_QATAN,
            Hebrew::HebrewAccentZaqefGadol => HEBREW_ACCENT_ZAQEF_GADOL,
            Hebrew::HebrewAccentTipeha => HEBREW_ACCENT_TIPEHA,
            Hebrew::HebrewAccentRevia => HEBREW_ACCENT_REVIA,
            Hebrew::HebrewAccentZarqa => HEBREW_ACCENT_ZARQA,
            Hebrew::HebrewAccentPashta => HEBREW_ACCENT_PASHTA,
            Hebrew::HebrewAccentYetiv => HEBREW_ACCENT_YETIV,
            Hebrew::HebrewAccentTevir => HEBREW_ACCENT_TEVIR,
            Hebrew::HebrewAccentGeresh => HEBREW_ACCENT_GERESH,
            Hebrew::HebrewAccentGereshMuqdam => HEBREW_ACCENT_GERESH_MUQDAM,
            Hebrew::HebrewAccentGershayim => HEBREW_ACCENT_GERSHAYIM,
            Hebrew::HebrewAccentQarneyPara => HEBREW_ACCENT_QARNEY_PARA,
            Hebrew::HebrewAccentTelishaGedola => HEBREW_ACCENT_TELISHA_GEDOLA,
            Hebrew::HebrewAccentPazer => HEBREW_ACCENT_PAZER,
            Hebrew::HebrewAccentAtnahHafukh => HEBREW_ACCENT_ATNAH_HAFUKH,
            Hebrew::HebrewAccentMunah => HEBREW_ACCENT_MUNAH,
            Hebrew::HebrewAccentMahapakh => HEBREW_ACCENT_MAHAPAKH,
            Hebrew::HebrewAccentMerkha => HEBREW_ACCENT_MERKHA,
            Hebrew::HebrewAccentMerkhaKefula => HEBREW_ACCENT_MERKHA_KEFULA,
            Hebrew::HebrewAccentDarga => HEBREW_ACCENT_DARGA,
            Hebrew::HebrewAccentQadma => HEBREW_ACCENT_QADMA,
            Hebrew::HebrewAccentTelishaQetana => HEBREW_ACCENT_TELISHA_QETANA,
            Hebrew::HebrewAccentYerahBenYomo => HEBREW_ACCENT_YERAH_BEN_YOMO,
            Hebrew::HebrewAccentOle => HEBREW_ACCENT_OLE,
            Hebrew::HebrewAccentIluy => HEBREW_ACCENT_ILUY,
            Hebrew::HebrewAccentDehi => HEBREW_ACCENT_DEHI,
            Hebrew::HebrewAccentZinor => HEBREW_ACCENT_ZINOR,
            Hebrew::HebrewMarkMasoraCircle => HEBREW_MARK_MASORA_CIRCLE,
            Hebrew::HebrewPointSheva => HEBREW_POINT_SHEVA,
            Hebrew::HebrewPointHatafSegol => HEBREW_POINT_HATAF_SEGOL,
            Hebrew::HebrewPointHatafPatah => HEBREW_POINT_HATAF_PATAH,
            Hebrew::HebrewPointHatafQamats => HEBREW_POINT_HATAF_QAMATS,
            Hebrew::HebrewPointHiriq => HEBREW_POINT_HIRIQ,
            Hebrew::HebrewPointTsere => HEBREW_POINT_TSERE,
            Hebrew::HebrewPointSegol => HEBREW_POINT_SEGOL,
            Hebrew::HebrewPointPatah => HEBREW_POINT_PATAH,
            Hebrew::HebrewPointQamats => HEBREW_POINT_QAMATS,
            Hebrew::HebrewPointHolam => HEBREW_POINT_HOLAM,
            Hebrew::HebrewPointHolamHaserForVav => HEBREW_POINT_HOLAM_HASER_FOR_VAV,
            Hebrew::HebrewPointQubuts => HEBREW_POINT_QUBUTS,
            Hebrew::HebrewPointDageshOrMapiq => HEBREW_POINT_DAGESH_OR_MAPIQ,
            Hebrew::HebrewPointMeteg => HEBREW_POINT_METEG,
            Hebrew::HebrewPunctuationMaqaf => HEBREW_PUNCTUATION_MAQAF,
            Hebrew::HebrewPointRafe => HEBREW_POINT_RAFE,
            Hebrew::HebrewPunctuationPaseq => HEBREW_PUNCTUATION_PASEQ,
            Hebrew::HebrewPointShinDot => HEBREW_POINT_SHIN_DOT,
            Hebrew::HebrewPointSinDot => HEBREW_POINT_SIN_DOT,
            Hebrew::HebrewPunctuationSofPasuq => HEBREW_PUNCTUATION_SOF_PASUQ,
            Hebrew::HebrewMarkUpperDot => HEBREW_MARK_UPPER_DOT,
            Hebrew::HebrewMarkLowerDot => HEBREW_MARK_LOWER_DOT,
            Hebrew::HebrewPunctuationNunHafukha => HEBREW_PUNCTUATION_NUN_HAFUKHA,
            Hebrew::HebrewPointQamatsQatan => HEBREW_POINT_QAMATS_QATAN,
            Hebrew::HebrewLetterAlef => HEBREW_LETTER_ALEF,
            Hebrew::HebrewLetterBet => HEBREW_LETTER_BET,
            Hebrew::HebrewLetterGimel => HEBREW_LETTER_GIMEL,
            Hebrew::HebrewLetterDalet => HEBREW_LETTER_DALET,
            Hebrew::HebrewLetterHe => HEBREW_LETTER_HE,
            Hebrew::HebrewLetterVav => HEBREW_LETTER_VAV,
            Hebrew::HebrewLetterZayin => HEBREW_LETTER_ZAYIN,
            Hebrew::HebrewLetterHet => HEBREW_LETTER_HET,
            Hebrew::HebrewLetterTet => HEBREW_LETTER_TET,
            Hebrew::HebrewLetterYod => HEBREW_LETTER_YOD,
            Hebrew::HebrewLetterFinalKaf => HEBREW_LETTER_FINAL_KAF,
            Hebrew::HebrewLetterKaf => HEBREW_LETTER_KAF,
            Hebrew::HebrewLetterLamed => HEBREW_LETTER_LAMED,
            Hebrew::HebrewLetterFinalMem => HEBREW_LETTER_FINAL_MEM,
            Hebrew::HebrewLetterMem => HEBREW_LETTER_MEM,
            Hebrew::HebrewLetterFinalNun => HEBREW_LETTER_FINAL_NUN,
            Hebrew::HebrewLetterNun => HEBREW_LETTER_NUN,
            Hebrew::HebrewLetterSamekh => HEBREW_LETTER_SAMEKH,
            Hebrew::HebrewLetterAyin => HEBREW_LETTER_AYIN,
            Hebrew::HebrewLetterFinalPe => HEBREW_LETTER_FINAL_PE,
            Hebrew::HebrewLetterPe => HEBREW_LETTER_PE,
            Hebrew::HebrewLetterFinalTsadi => HEBREW_LETTER_FINAL_TSADI,
            Hebrew::HebrewLetterTsadi => HEBREW_LETTER_TSADI,
            Hebrew::HebrewLetterQof => HEBREW_LETTER_QOF,
            Hebrew::HebrewLetterResh => HEBREW_LETTER_RESH,
            Hebrew::HebrewLetterShin => HEBREW_LETTER_SHIN,
            Hebrew::HebrewLetterTav => HEBREW_LETTER_TAV,
            Hebrew::HebrewYodTriangle => HEBREW_YOD_TRIANGLE,
            Hebrew::HebrewLigatureYiddishDoubleVav => HEBREW_LIGATURE_YIDDISH_DOUBLE_VAV,
            Hebrew::HebrewLigatureYiddishVavYod => HEBREW_LIGATURE_YIDDISH_VAV_YOD,
            Hebrew::HebrewLigatureYiddishDoubleYod => HEBREW_LIGATURE_YIDDISH_DOUBLE_YOD,
            Hebrew::HebrewPunctuationGeresh => HEBREW_PUNCTUATION_GERESH,
            Hebrew::HebrewPunctuationGershayim => HEBREW_PUNCTUATION_GERSHAYIM,
        }
    }
}

impl std::convert::TryFrom<char> for Hebrew {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HEBREW_ACCENT_ETNAHTA => Ok(Hebrew::HebrewAccentEtnahta),
            HEBREW_ACCENT_SEGOL => Ok(Hebrew::HebrewAccentSegol),
            HEBREW_ACCENT_SHALSHELET => Ok(Hebrew::HebrewAccentShalshelet),
            HEBREW_ACCENT_ZAQEF_QATAN => Ok(Hebrew::HebrewAccentZaqefQatan),
            HEBREW_ACCENT_ZAQEF_GADOL => Ok(Hebrew::HebrewAccentZaqefGadol),
            HEBREW_ACCENT_TIPEHA => Ok(Hebrew::HebrewAccentTipeha),
            HEBREW_ACCENT_REVIA => Ok(Hebrew::HebrewAccentRevia),
            HEBREW_ACCENT_ZARQA => Ok(Hebrew::HebrewAccentZarqa),
            HEBREW_ACCENT_PASHTA => Ok(Hebrew::HebrewAccentPashta),
            HEBREW_ACCENT_YETIV => Ok(Hebrew::HebrewAccentYetiv),
            HEBREW_ACCENT_TEVIR => Ok(Hebrew::HebrewAccentTevir),
            HEBREW_ACCENT_GERESH => Ok(Hebrew::HebrewAccentGeresh),
            HEBREW_ACCENT_GERESH_MUQDAM => Ok(Hebrew::HebrewAccentGereshMuqdam),
            HEBREW_ACCENT_GERSHAYIM => Ok(Hebrew::HebrewAccentGershayim),
            HEBREW_ACCENT_QARNEY_PARA => Ok(Hebrew::HebrewAccentQarneyPara),
            HEBREW_ACCENT_TELISHA_GEDOLA => Ok(Hebrew::HebrewAccentTelishaGedola),
            HEBREW_ACCENT_PAZER => Ok(Hebrew::HebrewAccentPazer),
            HEBREW_ACCENT_ATNAH_HAFUKH => Ok(Hebrew::HebrewAccentAtnahHafukh),
            HEBREW_ACCENT_MUNAH => Ok(Hebrew::HebrewAccentMunah),
            HEBREW_ACCENT_MAHAPAKH => Ok(Hebrew::HebrewAccentMahapakh),
            HEBREW_ACCENT_MERKHA => Ok(Hebrew::HebrewAccentMerkha),
            HEBREW_ACCENT_MERKHA_KEFULA => Ok(Hebrew::HebrewAccentMerkhaKefula),
            HEBREW_ACCENT_DARGA => Ok(Hebrew::HebrewAccentDarga),
            HEBREW_ACCENT_QADMA => Ok(Hebrew::HebrewAccentQadma),
            HEBREW_ACCENT_TELISHA_QETANA => Ok(Hebrew::HebrewAccentTelishaQetana),
            HEBREW_ACCENT_YERAH_BEN_YOMO => Ok(Hebrew::HebrewAccentYerahBenYomo),
            HEBREW_ACCENT_OLE => Ok(Hebrew::HebrewAccentOle),
            HEBREW_ACCENT_ILUY => Ok(Hebrew::HebrewAccentIluy),
            HEBREW_ACCENT_DEHI => Ok(Hebrew::HebrewAccentDehi),
            HEBREW_ACCENT_ZINOR => Ok(Hebrew::HebrewAccentZinor),
            HEBREW_MARK_MASORA_CIRCLE => Ok(Hebrew::HebrewMarkMasoraCircle),
            HEBREW_POINT_SHEVA => Ok(Hebrew::HebrewPointSheva),
            HEBREW_POINT_HATAF_SEGOL => Ok(Hebrew::HebrewPointHatafSegol),
            HEBREW_POINT_HATAF_PATAH => Ok(Hebrew::HebrewPointHatafPatah),
            HEBREW_POINT_HATAF_QAMATS => Ok(Hebrew::HebrewPointHatafQamats),
            HEBREW_POINT_HIRIQ => Ok(Hebrew::HebrewPointHiriq),
            HEBREW_POINT_TSERE => Ok(Hebrew::HebrewPointTsere),
            HEBREW_POINT_SEGOL => Ok(Hebrew::HebrewPointSegol),
            HEBREW_POINT_PATAH => Ok(Hebrew::HebrewPointPatah),
            HEBREW_POINT_QAMATS => Ok(Hebrew::HebrewPointQamats),
            HEBREW_POINT_HOLAM => Ok(Hebrew::HebrewPointHolam),
            HEBREW_POINT_HOLAM_HASER_FOR_VAV => Ok(Hebrew::HebrewPointHolamHaserForVav),
            HEBREW_POINT_QUBUTS => Ok(Hebrew::HebrewPointQubuts),
            HEBREW_POINT_DAGESH_OR_MAPIQ => Ok(Hebrew::HebrewPointDageshOrMapiq),
            HEBREW_POINT_METEG => Ok(Hebrew::HebrewPointMeteg),
            HEBREW_PUNCTUATION_MAQAF => Ok(Hebrew::HebrewPunctuationMaqaf),
            HEBREW_POINT_RAFE => Ok(Hebrew::HebrewPointRafe),
            HEBREW_PUNCTUATION_PASEQ => Ok(Hebrew::HebrewPunctuationPaseq),
            HEBREW_POINT_SHIN_DOT => Ok(Hebrew::HebrewPointShinDot),
            HEBREW_POINT_SIN_DOT => Ok(Hebrew::HebrewPointSinDot),
            HEBREW_PUNCTUATION_SOF_PASUQ => Ok(Hebrew::HebrewPunctuationSofPasuq),
            HEBREW_MARK_UPPER_DOT => Ok(Hebrew::HebrewMarkUpperDot),
            HEBREW_MARK_LOWER_DOT => Ok(Hebrew::HebrewMarkLowerDot),
            HEBREW_PUNCTUATION_NUN_HAFUKHA => Ok(Hebrew::HebrewPunctuationNunHafukha),
            HEBREW_POINT_QAMATS_QATAN => Ok(Hebrew::HebrewPointQamatsQatan),
            HEBREW_LETTER_ALEF => Ok(Hebrew::HebrewLetterAlef),
            HEBREW_LETTER_BET => Ok(Hebrew::HebrewLetterBet),
            HEBREW_LETTER_GIMEL => Ok(Hebrew::HebrewLetterGimel),
            HEBREW_LETTER_DALET => Ok(Hebrew::HebrewLetterDalet),
            HEBREW_LETTER_HE => Ok(Hebrew::HebrewLetterHe),
            HEBREW_LETTER_VAV => Ok(Hebrew::HebrewLetterVav),
            HEBREW_LETTER_ZAYIN => Ok(Hebrew::HebrewLetterZayin),
            HEBREW_LETTER_HET => Ok(Hebrew::HebrewLetterHet),
            HEBREW_LETTER_TET => Ok(Hebrew::HebrewLetterTet),
            HEBREW_LETTER_YOD => Ok(Hebrew::HebrewLetterYod),
            HEBREW_LETTER_FINAL_KAF => Ok(Hebrew::HebrewLetterFinalKaf),
            HEBREW_LETTER_KAF => Ok(Hebrew::HebrewLetterKaf),
            HEBREW_LETTER_LAMED => Ok(Hebrew::HebrewLetterLamed),
            HEBREW_LETTER_FINAL_MEM => Ok(Hebrew::HebrewLetterFinalMem),
            HEBREW_LETTER_MEM => Ok(Hebrew::HebrewLetterMem),
            HEBREW_LETTER_FINAL_NUN => Ok(Hebrew::HebrewLetterFinalNun),
            HEBREW_LETTER_NUN => Ok(Hebrew::HebrewLetterNun),
            HEBREW_LETTER_SAMEKH => Ok(Hebrew::HebrewLetterSamekh),
            HEBREW_LETTER_AYIN => Ok(Hebrew::HebrewLetterAyin),
            HEBREW_LETTER_FINAL_PE => Ok(Hebrew::HebrewLetterFinalPe),
            HEBREW_LETTER_PE => Ok(Hebrew::HebrewLetterPe),
            HEBREW_LETTER_FINAL_TSADI => Ok(Hebrew::HebrewLetterFinalTsadi),
            HEBREW_LETTER_TSADI => Ok(Hebrew::HebrewLetterTsadi),
            HEBREW_LETTER_QOF => Ok(Hebrew::HebrewLetterQof),
            HEBREW_LETTER_RESH => Ok(Hebrew::HebrewLetterResh),
            HEBREW_LETTER_SHIN => Ok(Hebrew::HebrewLetterShin),
            HEBREW_LETTER_TAV => Ok(Hebrew::HebrewLetterTav),
            HEBREW_YOD_TRIANGLE => Ok(Hebrew::HebrewYodTriangle),
            HEBREW_LIGATURE_YIDDISH_DOUBLE_VAV => Ok(Hebrew::HebrewLigatureYiddishDoubleVav),
            HEBREW_LIGATURE_YIDDISH_VAV_YOD => Ok(Hebrew::HebrewLigatureYiddishVavYod),
            HEBREW_LIGATURE_YIDDISH_DOUBLE_YOD => Ok(Hebrew::HebrewLigatureYiddishDoubleYod),
            HEBREW_PUNCTUATION_GERESH => Ok(Hebrew::HebrewPunctuationGeresh),
            HEBREW_PUNCTUATION_GERSHAYIM => Ok(Hebrew::HebrewPunctuationGershayim),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Hebrew {
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

impl std::convert::TryFrom<u32> for Hebrew {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Hebrew {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Hebrew {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Hebrew::HebrewAccentEtnahta
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Hebrew::HebrewAccentEtnahta => "hebrew accent etnahta",
            Hebrew::HebrewAccentSegol => "hebrew accent segol",
            Hebrew::HebrewAccentShalshelet => "hebrew accent shalshelet",
            Hebrew::HebrewAccentZaqefQatan => "hebrew accent zaqef qatan",
            Hebrew::HebrewAccentZaqefGadol => "hebrew accent zaqef gadol",
            Hebrew::HebrewAccentTipeha => "hebrew accent tipeha",
            Hebrew::HebrewAccentRevia => "hebrew accent revia",
            Hebrew::HebrewAccentZarqa => "hebrew accent zarqa",
            Hebrew::HebrewAccentPashta => "hebrew accent pashta",
            Hebrew::HebrewAccentYetiv => "hebrew accent yetiv",
            Hebrew::HebrewAccentTevir => "hebrew accent tevir",
            Hebrew::HebrewAccentGeresh => "hebrew accent geresh",
            Hebrew::HebrewAccentGereshMuqdam => "hebrew accent geresh muqdam",
            Hebrew::HebrewAccentGershayim => "hebrew accent gershayim",
            Hebrew::HebrewAccentQarneyPara => "hebrew accent qarney para",
            Hebrew::HebrewAccentTelishaGedola => "hebrew accent telisha gedola",
            Hebrew::HebrewAccentPazer => "hebrew accent pazer",
            Hebrew::HebrewAccentAtnahHafukh => "hebrew accent atnah hafukh",
            Hebrew::HebrewAccentMunah => "hebrew accent munah",
            Hebrew::HebrewAccentMahapakh => "hebrew accent mahapakh",
            Hebrew::HebrewAccentMerkha => "hebrew accent merkha",
            Hebrew::HebrewAccentMerkhaKefula => "hebrew accent merkha kefula",
            Hebrew::HebrewAccentDarga => "hebrew accent darga",
            Hebrew::HebrewAccentQadma => "hebrew accent qadma",
            Hebrew::HebrewAccentTelishaQetana => "hebrew accent telisha qetana",
            Hebrew::HebrewAccentYerahBenYomo => "hebrew accent yerah ben yomo",
            Hebrew::HebrewAccentOle => "hebrew accent ole",
            Hebrew::HebrewAccentIluy => "hebrew accent iluy",
            Hebrew::HebrewAccentDehi => "hebrew accent dehi",
            Hebrew::HebrewAccentZinor => "hebrew accent zinor",
            Hebrew::HebrewMarkMasoraCircle => "hebrew mark masora circle",
            Hebrew::HebrewPointSheva => "hebrew point sheva",
            Hebrew::HebrewPointHatafSegol => "hebrew point hataf segol",
            Hebrew::HebrewPointHatafPatah => "hebrew point hataf patah",
            Hebrew::HebrewPointHatafQamats => "hebrew point hataf qamats",
            Hebrew::HebrewPointHiriq => "hebrew point hiriq",
            Hebrew::HebrewPointTsere => "hebrew point tsere",
            Hebrew::HebrewPointSegol => "hebrew point segol",
            Hebrew::HebrewPointPatah => "hebrew point patah",
            Hebrew::HebrewPointQamats => "hebrew point qamats",
            Hebrew::HebrewPointHolam => "hebrew point holam",
            Hebrew::HebrewPointHolamHaserForVav => "hebrew point holam haser for vav",
            Hebrew::HebrewPointQubuts => "hebrew point qubuts",
            Hebrew::HebrewPointDageshOrMapiq => "hebrew point dagesh or mapiq",
            Hebrew::HebrewPointMeteg => "hebrew point meteg",
            Hebrew::HebrewPunctuationMaqaf => "hebrew punctuation maqaf",
            Hebrew::HebrewPointRafe => "hebrew point rafe",
            Hebrew::HebrewPunctuationPaseq => "hebrew punctuation paseq",
            Hebrew::HebrewPointShinDot => "hebrew point shin dot",
            Hebrew::HebrewPointSinDot => "hebrew point sin dot",
            Hebrew::HebrewPunctuationSofPasuq => "hebrew punctuation sof pasuq",
            Hebrew::HebrewMarkUpperDot => "hebrew mark upper dot",
            Hebrew::HebrewMarkLowerDot => "hebrew mark lower dot",
            Hebrew::HebrewPunctuationNunHafukha => "hebrew punctuation nun hafukha",
            Hebrew::HebrewPointQamatsQatan => "hebrew point qamats qatan",
            Hebrew::HebrewLetterAlef => "hebrew letter alef",
            Hebrew::HebrewLetterBet => "hebrew letter bet",
            Hebrew::HebrewLetterGimel => "hebrew letter gimel",
            Hebrew::HebrewLetterDalet => "hebrew letter dalet",
            Hebrew::HebrewLetterHe => "hebrew letter he",
            Hebrew::HebrewLetterVav => "hebrew letter vav",
            Hebrew::HebrewLetterZayin => "hebrew letter zayin",
            Hebrew::HebrewLetterHet => "hebrew letter het",
            Hebrew::HebrewLetterTet => "hebrew letter tet",
            Hebrew::HebrewLetterYod => "hebrew letter yod",
            Hebrew::HebrewLetterFinalKaf => "hebrew letter final kaf",
            Hebrew::HebrewLetterKaf => "hebrew letter kaf",
            Hebrew::HebrewLetterLamed => "hebrew letter lamed",
            Hebrew::HebrewLetterFinalMem => "hebrew letter final mem",
            Hebrew::HebrewLetterMem => "hebrew letter mem",
            Hebrew::HebrewLetterFinalNun => "hebrew letter final nun",
            Hebrew::HebrewLetterNun => "hebrew letter nun",
            Hebrew::HebrewLetterSamekh => "hebrew letter samekh",
            Hebrew::HebrewLetterAyin => "hebrew letter ayin",
            Hebrew::HebrewLetterFinalPe => "hebrew letter final pe",
            Hebrew::HebrewLetterPe => "hebrew letter pe",
            Hebrew::HebrewLetterFinalTsadi => "hebrew letter final tsadi",
            Hebrew::HebrewLetterTsadi => "hebrew letter tsadi",
            Hebrew::HebrewLetterQof => "hebrew letter qof",
            Hebrew::HebrewLetterResh => "hebrew letter resh",
            Hebrew::HebrewLetterShin => "hebrew letter shin",
            Hebrew::HebrewLetterTav => "hebrew letter tav",
            Hebrew::HebrewYodTriangle => "hebrew yod triangle",
            Hebrew::HebrewLigatureYiddishDoubleVav => "hebrew ligature yiddish double vav",
            Hebrew::HebrewLigatureYiddishVavYod => "hebrew ligature yiddish vav yod",
            Hebrew::HebrewLigatureYiddishDoubleYod => "hebrew ligature yiddish double yod",
            Hebrew::HebrewPunctuationGeresh => "hebrew punctuation geresh",
            Hebrew::HebrewPunctuationGershayim => "hebrew punctuation gershayim",
        }
    }
}
