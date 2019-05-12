/// \u{590} → \u{5ff}\
///\
/// ֑ ֒ ֓ ֔ ֕ ֖ ֗ ֘ ֙ ֚ ֛ ֜ ֝ ֞ ֟ ֠\
/// ֡ ֢ ֣ ֤ ֥ ֦ ֧ ֨ ֩ ֪ ֫ ֬ ֭ ֮ ֯ ְ\
/// ֱ ֲ ֳ ִ ֵ ֶ ַ ָ ֹ ֺ ֻ ּ ֽ ־ ֿ ׀\
/// ׁ ׂ ׃ ׄ ׅ ׆ ׇ א ב ג ד ה ו ז ח ט\
/// י ך כ ל ם מ ן נ ס ע ף פ ץ צ ק ר\
/// ש ת ׯ װ ױ ײ ׳ ״\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{591}: '֑'
    pub const ACCENT_ETNAHTA: char = '֑';
    /// \u{592}: '֒'
    pub const ACCENT_SEGOL: char = '֒';
    /// \u{593}: '֓'
    pub const ACCENT_SHALSHELET: char = '֓';
    /// \u{594}: '֔'
    pub const ACCENT_ZAQEF_QATAN: char = '֔';
    /// \u{595}: '֕'
    pub const ACCENT_ZAQEF_GADOL: char = '֕';
    /// \u{596}: '֖'
    pub const ACCENT_TIPEHA: char = '֖';
    /// \u{597}: '֗'
    pub const ACCENT_REVIA: char = '֗';
    /// \u{598}: '֘'
    pub const ACCENT_ZARQA: char = '֘';
    /// \u{599}: '֙'
    pub const ACCENT_PASHTA: char = '֙';
    /// \u{59a}: '֚'
    pub const ACCENT_YETIV: char = '֚';
    /// \u{59b}: '֛'
    pub const ACCENT_TEVIR: char = '֛';
    /// \u{59c}: '֜'
    pub const ACCENT_GERESH: char = '֜';
    /// \u{59d}: '֝'
    pub const ACCENT_GERESH_MUQDAM: char = '֝';
    /// \u{59e}: '֞'
    pub const ACCENT_GERSHAYIM: char = '֞';
    /// \u{59f}: '֟'
    pub const ACCENT_QARNEY_PARA: char = '֟';
    /// \u{5a0}: '֠'
    pub const ACCENT_TELISHA_GEDOLA: char = '֠';
    /// \u{5a1}: '֡'
    pub const ACCENT_PAZER: char = '֡';
    /// \u{5a2}: '֢'
    pub const ACCENT_ATNAH_HAFUKH: char = '֢';
    /// \u{5a3}: '֣'
    pub const ACCENT_MUNAH: char = '֣';
    /// \u{5a4}: '֤'
    pub const ACCENT_MAHAPAKH: char = '֤';
    /// \u{5a5}: '֥'
    pub const ACCENT_MERKHA: char = '֥';
    /// \u{5a6}: '֦'
    pub const ACCENT_MERKHA_KEFULA: char = '֦';
    /// \u{5a7}: '֧'
    pub const ACCENT_DARGA: char = '֧';
    /// \u{5a8}: '֨'
    pub const ACCENT_QADMA: char = '֨';
    /// \u{5a9}: '֩'
    pub const ACCENT_TELISHA_QETANA: char = '֩';
    /// \u{5aa}: '֪'
    pub const ACCENT_YERAH_BEN_YOMO: char = '֪';
    /// \u{5ab}: '֫'
    pub const ACCENT_OLE: char = '֫';
    /// \u{5ac}: '֬'
    pub const ACCENT_ILUY: char = '֬';
    /// \u{5ad}: '֭'
    pub const ACCENT_DEHI: char = '֭';
    /// \u{5ae}: '֮'
    pub const ACCENT_ZINOR: char = '֮';
    /// \u{5af}: '֯'
    pub const MARK_MASORA_CIRCLE: char = '֯';
    /// \u{5b0}: 'ְ'
    pub const POINT_SHEVA: char = 'ְ';
    /// \u{5b1}: 'ֱ'
    pub const POINT_HATAF_SEGOL: char = 'ֱ';
    /// \u{5b2}: 'ֲ'
    pub const POINT_HATAF_PATAH: char = 'ֲ';
    /// \u{5b3}: 'ֳ'
    pub const POINT_HATAF_QAMATS: char = 'ֳ';
    /// \u{5b4}: 'ִ'
    pub const POINT_HIRIQ: char = 'ִ';
    /// \u{5b5}: 'ֵ'
    pub const POINT_TSERE: char = 'ֵ';
    /// \u{5b6}: 'ֶ'
    pub const POINT_SEGOL: char = 'ֶ';
    /// \u{5b7}: 'ַ'
    pub const POINT_PATAH: char = 'ַ';
    /// \u{5b8}: 'ָ'
    pub const POINT_QAMATS: char = 'ָ';
    /// \u{5b9}: 'ֹ'
    pub const POINT_HOLAM: char = 'ֹ';
    /// \u{5ba}: 'ֺ'
    pub const POINT_HOLAM_HASER_FOR_VAV: char = 'ֺ';
    /// \u{5bb}: 'ֻ'
    pub const POINT_QUBUTS: char = 'ֻ';
    /// \u{5bc}: 'ּ'
    pub const POINT_DAGESH_OR_MAPIQ: char = 'ּ';
    /// \u{5bd}: 'ֽ'
    pub const POINT_METEG: char = 'ֽ';
    /// \u{5be}: '־'
    pub const PUNCTUATION_MAQAF: char = '־';
    /// \u{5bf}: 'ֿ'
    pub const POINT_RAFE: char = 'ֿ';
    /// \u{5c0}: '׀'
    pub const PUNCTUATION_PASEQ: char = '׀';
    /// \u{5c1}: 'ׁ'
    pub const POINT_SHIN_DOT: char = 'ׁ';
    /// \u{5c2}: 'ׂ'
    pub const POINT_SIN_DOT: char = 'ׂ';
    /// \u{5c3}: '׃'
    pub const PUNCTUATION_SOF_PASUQ: char = '׃';
    /// \u{5c4}: 'ׄ'
    pub const MARK_UPPER_DOT: char = 'ׄ';
    /// \u{5c5}: 'ׅ'
    pub const MARK_LOWER_DOT: char = 'ׅ';
    /// \u{5c6}: '׆'
    pub const PUNCTUATION_NUN_HAFUKHA: char = '׆';
    /// \u{5c7}: 'ׇ'
    pub const POINT_QAMATS_QATAN: char = 'ׇ';
    /// \u{5d0}: 'א'
    pub const LETTER_ALEF: char = 'א';
    /// \u{5d1}: 'ב'
    pub const LETTER_BET: char = 'ב';
    /// \u{5d2}: 'ג'
    pub const LETTER_GIMEL: char = 'ג';
    /// \u{5d3}: 'ד'
    pub const LETTER_DALET: char = 'ד';
    /// \u{5d4}: 'ה'
    pub const LETTER_HE: char = 'ה';
    /// \u{5d5}: 'ו'
    pub const LETTER_VAV: char = 'ו';
    /// \u{5d6}: 'ז'
    pub const LETTER_ZAYIN: char = 'ז';
    /// \u{5d7}: 'ח'
    pub const LETTER_HET: char = 'ח';
    /// \u{5d8}: 'ט'
    pub const LETTER_TET: char = 'ט';
    /// \u{5d9}: 'י'
    pub const LETTER_YOD: char = 'י';
    /// \u{5da}: 'ך'
    pub const LETTER_FINAL_KAF: char = 'ך';
    /// \u{5db}: 'כ'
    pub const LETTER_KAF: char = 'כ';
    /// \u{5dc}: 'ל'
    pub const LETTER_LAMED: char = 'ל';
    /// \u{5dd}: 'ם'
    pub const LETTER_FINAL_MEM: char = 'ם';
    /// \u{5de}: 'מ'
    pub const LETTER_MEM: char = 'מ';
    /// \u{5df}: 'ן'
    pub const LETTER_FINAL_NUN: char = 'ן';
    /// \u{5e0}: 'נ'
    pub const LETTER_NUN: char = 'נ';
    /// \u{5e1}: 'ס'
    pub const LETTER_SAMEKH: char = 'ס';
    /// \u{5e2}: 'ע'
    pub const LETTER_AYIN: char = 'ע';
    /// \u{5e3}: 'ף'
    pub const LETTER_FINAL_PE: char = 'ף';
    /// \u{5e4}: 'פ'
    pub const LETTER_PE: char = 'פ';
    /// \u{5e5}: 'ץ'
    pub const LETTER_FINAL_TSADI: char = 'ץ';
    /// \u{5e6}: 'צ'
    pub const LETTER_TSADI: char = 'צ';
    /// \u{5e7}: 'ק'
    pub const LETTER_QOF: char = 'ק';
    /// \u{5e8}: 'ר'
    pub const LETTER_RESH: char = 'ר';
    /// \u{5e9}: 'ש'
    pub const LETTER_SHIN: char = 'ש';
    /// \u{5ea}: 'ת'
    pub const LETTER_TAV: char = 'ת';
    /// \u{5ef}: 'ׯ'
    pub const YOD_TRIANGLE: char = 'ׯ';
    /// \u{5f0}: 'װ'
    pub const LIGATURE_YIDDISH_DOUBLE_VAV: char = 'װ';
    /// \u{5f1}: 'ױ'
    pub const LIGATURE_YIDDISH_VAV_YOD: char = 'ױ';
    /// \u{5f2}: 'ײ'
    pub const LIGATURE_YIDDISH_DOUBLE_YOD: char = 'ײ';
    /// \u{5f3}: '׳'
    pub const PUNCTUATION_GERESH: char = '׳';
    /// \u{5f4}: '״'
    pub const PUNCTUATION_GERSHAYIM: char = '״';
}

/// An enum to represent all characters in the Hebrew block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hebrew {
    /// \u{591}: '֑'
    AccentEtnahta,
    /// \u{592}: '֒'
    AccentSegol,
    /// \u{593}: '֓'
    AccentShalshelet,
    /// \u{594}: '֔'
    AccentZaqefQatan,
    /// \u{595}: '֕'
    AccentZaqefGadol,
    /// \u{596}: '֖'
    AccentTipeha,
    /// \u{597}: '֗'
    AccentRevia,
    /// \u{598}: '֘'
    AccentZarqa,
    /// \u{599}: '֙'
    AccentPashta,
    /// \u{59a}: '֚'
    AccentYetiv,
    /// \u{59b}: '֛'
    AccentTevir,
    /// \u{59c}: '֜'
    AccentGeresh,
    /// \u{59d}: '֝'
    AccentGereshMuqdam,
    /// \u{59e}: '֞'
    AccentGershayim,
    /// \u{59f}: '֟'
    AccentQarneyPara,
    /// \u{5a0}: '֠'
    AccentTelishaGedola,
    /// \u{5a1}: '֡'
    AccentPazer,
    /// \u{5a2}: '֢'
    AccentAtnahHafukh,
    /// \u{5a3}: '֣'
    AccentMunah,
    /// \u{5a4}: '֤'
    AccentMahapakh,
    /// \u{5a5}: '֥'
    AccentMerkha,
    /// \u{5a6}: '֦'
    AccentMerkhaKefula,
    /// \u{5a7}: '֧'
    AccentDarga,
    /// \u{5a8}: '֨'
    AccentQadma,
    /// \u{5a9}: '֩'
    AccentTelishaQetana,
    /// \u{5aa}: '֪'
    AccentYerahBenYomo,
    /// \u{5ab}: '֫'
    AccentOle,
    /// \u{5ac}: '֬'
    AccentIluy,
    /// \u{5ad}: '֭'
    AccentDehi,
    /// \u{5ae}: '֮'
    AccentZinor,
    /// \u{5af}: '֯'
    MarkMasoraCircle,
    /// \u{5b0}: 'ְ'
    PointSheva,
    /// \u{5b1}: 'ֱ'
    PointHatafSegol,
    /// \u{5b2}: 'ֲ'
    PointHatafPatah,
    /// \u{5b3}: 'ֳ'
    PointHatafQamats,
    /// \u{5b4}: 'ִ'
    PointHiriq,
    /// \u{5b5}: 'ֵ'
    PointTsere,
    /// \u{5b6}: 'ֶ'
    PointSegol,
    /// \u{5b7}: 'ַ'
    PointPatah,
    /// \u{5b8}: 'ָ'
    PointQamats,
    /// \u{5b9}: 'ֹ'
    PointHolam,
    /// \u{5ba}: 'ֺ'
    PointHolamHaserForVav,
    /// \u{5bb}: 'ֻ'
    PointQubuts,
    /// \u{5bc}: 'ּ'
    PointDageshOrMapiq,
    /// \u{5bd}: 'ֽ'
    PointMeteg,
    /// \u{5be}: '־'
    PunctuationMaqaf,
    /// \u{5bf}: 'ֿ'
    PointRafe,
    /// \u{5c0}: '׀'
    PunctuationPaseq,
    /// \u{5c1}: 'ׁ'
    PointShinDot,
    /// \u{5c2}: 'ׂ'
    PointSinDot,
    /// \u{5c3}: '׃'
    PunctuationSofPasuq,
    /// \u{5c4}: 'ׄ'
    MarkUpperDot,
    /// \u{5c5}: 'ׅ'
    MarkLowerDot,
    /// \u{5c6}: '׆'
    PunctuationNunHafukha,
    /// \u{5c7}: 'ׇ'
    PointQamatsQatan,
    /// \u{5d0}: 'א'
    LetterAlef,
    /// \u{5d1}: 'ב'
    LetterBet,
    /// \u{5d2}: 'ג'
    LetterGimel,
    /// \u{5d3}: 'ד'
    LetterDalet,
    /// \u{5d4}: 'ה'
    LetterHe,
    /// \u{5d5}: 'ו'
    LetterVav,
    /// \u{5d6}: 'ז'
    LetterZayin,
    /// \u{5d7}: 'ח'
    LetterHet,
    /// \u{5d8}: 'ט'
    LetterTet,
    /// \u{5d9}: 'י'
    LetterYod,
    /// \u{5da}: 'ך'
    LetterFinalKaf,
    /// \u{5db}: 'כ'
    LetterKaf,
    /// \u{5dc}: 'ל'
    LetterLamed,
    /// \u{5dd}: 'ם'
    LetterFinalMem,
    /// \u{5de}: 'מ'
    LetterMem,
    /// \u{5df}: 'ן'
    LetterFinalNun,
    /// \u{5e0}: 'נ'
    LetterNun,
    /// \u{5e1}: 'ס'
    LetterSamekh,
    /// \u{5e2}: 'ע'
    LetterAyin,
    /// \u{5e3}: 'ף'
    LetterFinalPe,
    /// \u{5e4}: 'פ'
    LetterPe,
    /// \u{5e5}: 'ץ'
    LetterFinalTsadi,
    /// \u{5e6}: 'צ'
    LetterTsadi,
    /// \u{5e7}: 'ק'
    LetterQof,
    /// \u{5e8}: 'ר'
    LetterResh,
    /// \u{5e9}: 'ש'
    LetterShin,
    /// \u{5ea}: 'ת'
    LetterTav,
    /// \u{5ef}: 'ׯ'
    YodTriangle,
    /// \u{5f0}: 'װ'
    LigatureYiddishDoubleVav,
    /// \u{5f1}: 'ױ'
    LigatureYiddishVavYod,
    /// \u{5f2}: 'ײ'
    LigatureYiddishDoubleYod,
    /// \u{5f3}: '׳'
    PunctuationGeresh,
    /// \u{5f4}: '״'
    PunctuationGershayim,
}

impl Into<char> for Hebrew {
    fn into(self) -> char {
        use constants::*;
        match self {
            Hebrew::AccentEtnahta => ACCENT_ETNAHTA,
            Hebrew::AccentSegol => ACCENT_SEGOL,
            Hebrew::AccentShalshelet => ACCENT_SHALSHELET,
            Hebrew::AccentZaqefQatan => ACCENT_ZAQEF_QATAN,
            Hebrew::AccentZaqefGadol => ACCENT_ZAQEF_GADOL,
            Hebrew::AccentTipeha => ACCENT_TIPEHA,
            Hebrew::AccentRevia => ACCENT_REVIA,
            Hebrew::AccentZarqa => ACCENT_ZARQA,
            Hebrew::AccentPashta => ACCENT_PASHTA,
            Hebrew::AccentYetiv => ACCENT_YETIV,
            Hebrew::AccentTevir => ACCENT_TEVIR,
            Hebrew::AccentGeresh => ACCENT_GERESH,
            Hebrew::AccentGereshMuqdam => ACCENT_GERESH_MUQDAM,
            Hebrew::AccentGershayim => ACCENT_GERSHAYIM,
            Hebrew::AccentQarneyPara => ACCENT_QARNEY_PARA,
            Hebrew::AccentTelishaGedola => ACCENT_TELISHA_GEDOLA,
            Hebrew::AccentPazer => ACCENT_PAZER,
            Hebrew::AccentAtnahHafukh => ACCENT_ATNAH_HAFUKH,
            Hebrew::AccentMunah => ACCENT_MUNAH,
            Hebrew::AccentMahapakh => ACCENT_MAHAPAKH,
            Hebrew::AccentMerkha => ACCENT_MERKHA,
            Hebrew::AccentMerkhaKefula => ACCENT_MERKHA_KEFULA,
            Hebrew::AccentDarga => ACCENT_DARGA,
            Hebrew::AccentQadma => ACCENT_QADMA,
            Hebrew::AccentTelishaQetana => ACCENT_TELISHA_QETANA,
            Hebrew::AccentYerahBenYomo => ACCENT_YERAH_BEN_YOMO,
            Hebrew::AccentOle => ACCENT_OLE,
            Hebrew::AccentIluy => ACCENT_ILUY,
            Hebrew::AccentDehi => ACCENT_DEHI,
            Hebrew::AccentZinor => ACCENT_ZINOR,
            Hebrew::MarkMasoraCircle => MARK_MASORA_CIRCLE,
            Hebrew::PointSheva => POINT_SHEVA,
            Hebrew::PointHatafSegol => POINT_HATAF_SEGOL,
            Hebrew::PointHatafPatah => POINT_HATAF_PATAH,
            Hebrew::PointHatafQamats => POINT_HATAF_QAMATS,
            Hebrew::PointHiriq => POINT_HIRIQ,
            Hebrew::PointTsere => POINT_TSERE,
            Hebrew::PointSegol => POINT_SEGOL,
            Hebrew::PointPatah => POINT_PATAH,
            Hebrew::PointQamats => POINT_QAMATS,
            Hebrew::PointHolam => POINT_HOLAM,
            Hebrew::PointHolamHaserForVav => POINT_HOLAM_HASER_FOR_VAV,
            Hebrew::PointQubuts => POINT_QUBUTS,
            Hebrew::PointDageshOrMapiq => POINT_DAGESH_OR_MAPIQ,
            Hebrew::PointMeteg => POINT_METEG,
            Hebrew::PunctuationMaqaf => PUNCTUATION_MAQAF,
            Hebrew::PointRafe => POINT_RAFE,
            Hebrew::PunctuationPaseq => PUNCTUATION_PASEQ,
            Hebrew::PointShinDot => POINT_SHIN_DOT,
            Hebrew::PointSinDot => POINT_SIN_DOT,
            Hebrew::PunctuationSofPasuq => PUNCTUATION_SOF_PASUQ,
            Hebrew::MarkUpperDot => MARK_UPPER_DOT,
            Hebrew::MarkLowerDot => MARK_LOWER_DOT,
            Hebrew::PunctuationNunHafukha => PUNCTUATION_NUN_HAFUKHA,
            Hebrew::PointQamatsQatan => POINT_QAMATS_QATAN,
            Hebrew::LetterAlef => LETTER_ALEF,
            Hebrew::LetterBet => LETTER_BET,
            Hebrew::LetterGimel => LETTER_GIMEL,
            Hebrew::LetterDalet => LETTER_DALET,
            Hebrew::LetterHe => LETTER_HE,
            Hebrew::LetterVav => LETTER_VAV,
            Hebrew::LetterZayin => LETTER_ZAYIN,
            Hebrew::LetterHet => LETTER_HET,
            Hebrew::LetterTet => LETTER_TET,
            Hebrew::LetterYod => LETTER_YOD,
            Hebrew::LetterFinalKaf => LETTER_FINAL_KAF,
            Hebrew::LetterKaf => LETTER_KAF,
            Hebrew::LetterLamed => LETTER_LAMED,
            Hebrew::LetterFinalMem => LETTER_FINAL_MEM,
            Hebrew::LetterMem => LETTER_MEM,
            Hebrew::LetterFinalNun => LETTER_FINAL_NUN,
            Hebrew::LetterNun => LETTER_NUN,
            Hebrew::LetterSamekh => LETTER_SAMEKH,
            Hebrew::LetterAyin => LETTER_AYIN,
            Hebrew::LetterFinalPe => LETTER_FINAL_PE,
            Hebrew::LetterPe => LETTER_PE,
            Hebrew::LetterFinalTsadi => LETTER_FINAL_TSADI,
            Hebrew::LetterTsadi => LETTER_TSADI,
            Hebrew::LetterQof => LETTER_QOF,
            Hebrew::LetterResh => LETTER_RESH,
            Hebrew::LetterShin => LETTER_SHIN,
            Hebrew::LetterTav => LETTER_TAV,
            Hebrew::YodTriangle => YOD_TRIANGLE,
            Hebrew::LigatureYiddishDoubleVav => LIGATURE_YIDDISH_DOUBLE_VAV,
            Hebrew::LigatureYiddishVavYod => LIGATURE_YIDDISH_VAV_YOD,
            Hebrew::LigatureYiddishDoubleYod => LIGATURE_YIDDISH_DOUBLE_YOD,
            Hebrew::PunctuationGeresh => PUNCTUATION_GERESH,
            Hebrew::PunctuationGershayim => PUNCTUATION_GERSHAYIM,
        }
    }
}

impl std::convert::TryFrom<char> for Hebrew {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            ACCENT_ETNAHTA => Ok(Hebrew::AccentEtnahta),
            ACCENT_SEGOL => Ok(Hebrew::AccentSegol),
            ACCENT_SHALSHELET => Ok(Hebrew::AccentShalshelet),
            ACCENT_ZAQEF_QATAN => Ok(Hebrew::AccentZaqefQatan),
            ACCENT_ZAQEF_GADOL => Ok(Hebrew::AccentZaqefGadol),
            ACCENT_TIPEHA => Ok(Hebrew::AccentTipeha),
            ACCENT_REVIA => Ok(Hebrew::AccentRevia),
            ACCENT_ZARQA => Ok(Hebrew::AccentZarqa),
            ACCENT_PASHTA => Ok(Hebrew::AccentPashta),
            ACCENT_YETIV => Ok(Hebrew::AccentYetiv),
            ACCENT_TEVIR => Ok(Hebrew::AccentTevir),
            ACCENT_GERESH => Ok(Hebrew::AccentGeresh),
            ACCENT_GERESH_MUQDAM => Ok(Hebrew::AccentGereshMuqdam),
            ACCENT_GERSHAYIM => Ok(Hebrew::AccentGershayim),
            ACCENT_QARNEY_PARA => Ok(Hebrew::AccentQarneyPara),
            ACCENT_TELISHA_GEDOLA => Ok(Hebrew::AccentTelishaGedola),
            ACCENT_PAZER => Ok(Hebrew::AccentPazer),
            ACCENT_ATNAH_HAFUKH => Ok(Hebrew::AccentAtnahHafukh),
            ACCENT_MUNAH => Ok(Hebrew::AccentMunah),
            ACCENT_MAHAPAKH => Ok(Hebrew::AccentMahapakh),
            ACCENT_MERKHA => Ok(Hebrew::AccentMerkha),
            ACCENT_MERKHA_KEFULA => Ok(Hebrew::AccentMerkhaKefula),
            ACCENT_DARGA => Ok(Hebrew::AccentDarga),
            ACCENT_QADMA => Ok(Hebrew::AccentQadma),
            ACCENT_TELISHA_QETANA => Ok(Hebrew::AccentTelishaQetana),
            ACCENT_YERAH_BEN_YOMO => Ok(Hebrew::AccentYerahBenYomo),
            ACCENT_OLE => Ok(Hebrew::AccentOle),
            ACCENT_ILUY => Ok(Hebrew::AccentIluy),
            ACCENT_DEHI => Ok(Hebrew::AccentDehi),
            ACCENT_ZINOR => Ok(Hebrew::AccentZinor),
            MARK_MASORA_CIRCLE => Ok(Hebrew::MarkMasoraCircle),
            POINT_SHEVA => Ok(Hebrew::PointSheva),
            POINT_HATAF_SEGOL => Ok(Hebrew::PointHatafSegol),
            POINT_HATAF_PATAH => Ok(Hebrew::PointHatafPatah),
            POINT_HATAF_QAMATS => Ok(Hebrew::PointHatafQamats),
            POINT_HIRIQ => Ok(Hebrew::PointHiriq),
            POINT_TSERE => Ok(Hebrew::PointTsere),
            POINT_SEGOL => Ok(Hebrew::PointSegol),
            POINT_PATAH => Ok(Hebrew::PointPatah),
            POINT_QAMATS => Ok(Hebrew::PointQamats),
            POINT_HOLAM => Ok(Hebrew::PointHolam),
            POINT_HOLAM_HASER_FOR_VAV => Ok(Hebrew::PointHolamHaserForVav),
            POINT_QUBUTS => Ok(Hebrew::PointQubuts),
            POINT_DAGESH_OR_MAPIQ => Ok(Hebrew::PointDageshOrMapiq),
            POINT_METEG => Ok(Hebrew::PointMeteg),
            PUNCTUATION_MAQAF => Ok(Hebrew::PunctuationMaqaf),
            POINT_RAFE => Ok(Hebrew::PointRafe),
            PUNCTUATION_PASEQ => Ok(Hebrew::PunctuationPaseq),
            POINT_SHIN_DOT => Ok(Hebrew::PointShinDot),
            POINT_SIN_DOT => Ok(Hebrew::PointSinDot),
            PUNCTUATION_SOF_PASUQ => Ok(Hebrew::PunctuationSofPasuq),
            MARK_UPPER_DOT => Ok(Hebrew::MarkUpperDot),
            MARK_LOWER_DOT => Ok(Hebrew::MarkLowerDot),
            PUNCTUATION_NUN_HAFUKHA => Ok(Hebrew::PunctuationNunHafukha),
            POINT_QAMATS_QATAN => Ok(Hebrew::PointQamatsQatan),
            LETTER_ALEF => Ok(Hebrew::LetterAlef),
            LETTER_BET => Ok(Hebrew::LetterBet),
            LETTER_GIMEL => Ok(Hebrew::LetterGimel),
            LETTER_DALET => Ok(Hebrew::LetterDalet),
            LETTER_HE => Ok(Hebrew::LetterHe),
            LETTER_VAV => Ok(Hebrew::LetterVav),
            LETTER_ZAYIN => Ok(Hebrew::LetterZayin),
            LETTER_HET => Ok(Hebrew::LetterHet),
            LETTER_TET => Ok(Hebrew::LetterTet),
            LETTER_YOD => Ok(Hebrew::LetterYod),
            LETTER_FINAL_KAF => Ok(Hebrew::LetterFinalKaf),
            LETTER_KAF => Ok(Hebrew::LetterKaf),
            LETTER_LAMED => Ok(Hebrew::LetterLamed),
            LETTER_FINAL_MEM => Ok(Hebrew::LetterFinalMem),
            LETTER_MEM => Ok(Hebrew::LetterMem),
            LETTER_FINAL_NUN => Ok(Hebrew::LetterFinalNun),
            LETTER_NUN => Ok(Hebrew::LetterNun),
            LETTER_SAMEKH => Ok(Hebrew::LetterSamekh),
            LETTER_AYIN => Ok(Hebrew::LetterAyin),
            LETTER_FINAL_PE => Ok(Hebrew::LetterFinalPe),
            LETTER_PE => Ok(Hebrew::LetterPe),
            LETTER_FINAL_TSADI => Ok(Hebrew::LetterFinalTsadi),
            LETTER_TSADI => Ok(Hebrew::LetterTsadi),
            LETTER_QOF => Ok(Hebrew::LetterQof),
            LETTER_RESH => Ok(Hebrew::LetterResh),
            LETTER_SHIN => Ok(Hebrew::LetterShin),
            LETTER_TAV => Ok(Hebrew::LetterTav),
            YOD_TRIANGLE => Ok(Hebrew::YodTriangle),
            LIGATURE_YIDDISH_DOUBLE_VAV => Ok(Hebrew::LigatureYiddishDoubleVav),
            LIGATURE_YIDDISH_VAV_YOD => Ok(Hebrew::LigatureYiddishVavYod),
            LIGATURE_YIDDISH_DOUBLE_YOD => Ok(Hebrew::LigatureYiddishDoubleYod),
            PUNCTUATION_GERESH => Ok(Hebrew::PunctuationGeresh),
            PUNCTUATION_GERSHAYIM => Ok(Hebrew::PunctuationGershayim),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Hebrew::AccentEtnahta
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Hebrew::AccentEtnahta => "hebrew accent etnahta",
            Hebrew::AccentSegol => "hebrew accent segol",
            Hebrew::AccentShalshelet => "hebrew accent shalshelet",
            Hebrew::AccentZaqefQatan => "hebrew accent zaqef qatan",
            Hebrew::AccentZaqefGadol => "hebrew accent zaqef gadol",
            Hebrew::AccentTipeha => "hebrew accent tipeha",
            Hebrew::AccentRevia => "hebrew accent revia",
            Hebrew::AccentZarqa => "hebrew accent zarqa",
            Hebrew::AccentPashta => "hebrew accent pashta",
            Hebrew::AccentYetiv => "hebrew accent yetiv",
            Hebrew::AccentTevir => "hebrew accent tevir",
            Hebrew::AccentGeresh => "hebrew accent geresh",
            Hebrew::AccentGereshMuqdam => "hebrew accent geresh muqdam",
            Hebrew::AccentGershayim => "hebrew accent gershayim",
            Hebrew::AccentQarneyPara => "hebrew accent qarney para",
            Hebrew::AccentTelishaGedola => "hebrew accent telisha gedola",
            Hebrew::AccentPazer => "hebrew accent pazer",
            Hebrew::AccentAtnahHafukh => "hebrew accent atnah hafukh",
            Hebrew::AccentMunah => "hebrew accent munah",
            Hebrew::AccentMahapakh => "hebrew accent mahapakh",
            Hebrew::AccentMerkha => "hebrew accent merkha",
            Hebrew::AccentMerkhaKefula => "hebrew accent merkha kefula",
            Hebrew::AccentDarga => "hebrew accent darga",
            Hebrew::AccentQadma => "hebrew accent qadma",
            Hebrew::AccentTelishaQetana => "hebrew accent telisha qetana",
            Hebrew::AccentYerahBenYomo => "hebrew accent yerah ben yomo",
            Hebrew::AccentOle => "hebrew accent ole",
            Hebrew::AccentIluy => "hebrew accent iluy",
            Hebrew::AccentDehi => "hebrew accent dehi",
            Hebrew::AccentZinor => "hebrew accent zinor",
            Hebrew::MarkMasoraCircle => "hebrew mark masora circle",
            Hebrew::PointSheva => "hebrew point sheva",
            Hebrew::PointHatafSegol => "hebrew point hataf segol",
            Hebrew::PointHatafPatah => "hebrew point hataf patah",
            Hebrew::PointHatafQamats => "hebrew point hataf qamats",
            Hebrew::PointHiriq => "hebrew point hiriq",
            Hebrew::PointTsere => "hebrew point tsere",
            Hebrew::PointSegol => "hebrew point segol",
            Hebrew::PointPatah => "hebrew point patah",
            Hebrew::PointQamats => "hebrew point qamats",
            Hebrew::PointHolam => "hebrew point holam",
            Hebrew::PointHolamHaserForVav => "hebrew point holam haser for vav",
            Hebrew::PointQubuts => "hebrew point qubuts",
            Hebrew::PointDageshOrMapiq => "hebrew point dagesh or mapiq",
            Hebrew::PointMeteg => "hebrew point meteg",
            Hebrew::PunctuationMaqaf => "hebrew punctuation maqaf",
            Hebrew::PointRafe => "hebrew point rafe",
            Hebrew::PunctuationPaseq => "hebrew punctuation paseq",
            Hebrew::PointShinDot => "hebrew point shin dot",
            Hebrew::PointSinDot => "hebrew point sin dot",
            Hebrew::PunctuationSofPasuq => "hebrew punctuation sof pasuq",
            Hebrew::MarkUpperDot => "hebrew mark upper dot",
            Hebrew::MarkLowerDot => "hebrew mark lower dot",
            Hebrew::PunctuationNunHafukha => "hebrew punctuation nun hafukha",
            Hebrew::PointQamatsQatan => "hebrew point qamats qatan",
            Hebrew::LetterAlef => "hebrew letter alef",
            Hebrew::LetterBet => "hebrew letter bet",
            Hebrew::LetterGimel => "hebrew letter gimel",
            Hebrew::LetterDalet => "hebrew letter dalet",
            Hebrew::LetterHe => "hebrew letter he",
            Hebrew::LetterVav => "hebrew letter vav",
            Hebrew::LetterZayin => "hebrew letter zayin",
            Hebrew::LetterHet => "hebrew letter het",
            Hebrew::LetterTet => "hebrew letter tet",
            Hebrew::LetterYod => "hebrew letter yod",
            Hebrew::LetterFinalKaf => "hebrew letter final kaf",
            Hebrew::LetterKaf => "hebrew letter kaf",
            Hebrew::LetterLamed => "hebrew letter lamed",
            Hebrew::LetterFinalMem => "hebrew letter final mem",
            Hebrew::LetterMem => "hebrew letter mem",
            Hebrew::LetterFinalNun => "hebrew letter final nun",
            Hebrew::LetterNun => "hebrew letter nun",
            Hebrew::LetterSamekh => "hebrew letter samekh",
            Hebrew::LetterAyin => "hebrew letter ayin",
            Hebrew::LetterFinalPe => "hebrew letter final pe",
            Hebrew::LetterPe => "hebrew letter pe",
            Hebrew::LetterFinalTsadi => "hebrew letter final tsadi",
            Hebrew::LetterTsadi => "hebrew letter tsadi",
            Hebrew::LetterQof => "hebrew letter qof",
            Hebrew::LetterResh => "hebrew letter resh",
            Hebrew::LetterShin => "hebrew letter shin",
            Hebrew::LetterTav => "hebrew letter tav",
            Hebrew::YodTriangle => "hebrew yod triangle",
            Hebrew::LigatureYiddishDoubleVav => "hebrew ligature yiddish double vav",
            Hebrew::LigatureYiddishVavYod => "hebrew ligature yiddish vav yod",
            Hebrew::LigatureYiddishDoubleYod => "hebrew ligature yiddish double yod",
            Hebrew::PunctuationGeresh => "hebrew punctuation geresh",
            Hebrew::PunctuationGershayim => "hebrew punctuation gershayim",
        }
    }
}
