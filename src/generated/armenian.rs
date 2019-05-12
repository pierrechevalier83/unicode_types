/// \u{530} → \u{58f}\
///\
/// Ա Բ Գ Դ Ե Զ Է Ը Թ Ժ Ի Լ Խ Ծ Կ Հ\
/// Ձ Ղ Ճ Մ Յ Ն Շ Ո Չ Պ Ջ Ռ Ս Վ Տ Ր\
/// Ց Ւ Փ Ք Օ Ֆ ՙ ՚ ՛ ՜ ՝ ՞ ՟ ՠ ա բ\
/// գ դ ե զ է ը թ ժ ի լ խ ծ կ հ ձ ղ\
/// ճ մ յ ն շ ո չ պ ջ ռ ս վ տ ր ց ւ\
/// փ ք օ ֆ և ֈ ։ ֊ ֍ ֎\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{531}: 'Ա'
    pub const CAPITAL_LETTER_AYB: char = 'Ա';
    /// \u{532}: 'Բ'
    pub const CAPITAL_LETTER_BEN: char = 'Բ';
    /// \u{533}: 'Գ'
    pub const CAPITAL_LETTER_GIM: char = 'Գ';
    /// \u{534}: 'Դ'
    pub const CAPITAL_LETTER_DA: char = 'Դ';
    /// \u{535}: 'Ե'
    pub const CAPITAL_LETTER_ECH: char = 'Ե';
    /// \u{536}: 'Զ'
    pub const CAPITAL_LETTER_ZA: char = 'Զ';
    /// \u{537}: 'Է'
    pub const CAPITAL_LETTER_EH: char = 'Է';
    /// \u{538}: 'Ը'
    pub const CAPITAL_LETTER_ET: char = 'Ը';
    /// \u{539}: 'Թ'
    pub const CAPITAL_LETTER_TO: char = 'Թ';
    /// \u{53a}: 'Ժ'
    pub const CAPITAL_LETTER_ZHE: char = 'Ժ';
    /// \u{53b}: 'Ի'
    pub const CAPITAL_LETTER_INI: char = 'Ի';
    /// \u{53c}: 'Լ'
    pub const CAPITAL_LETTER_LIWN: char = 'Լ';
    /// \u{53d}: 'Խ'
    pub const CAPITAL_LETTER_XEH: char = 'Խ';
    /// \u{53e}: 'Ծ'
    pub const CAPITAL_LETTER_CA: char = 'Ծ';
    /// \u{53f}: 'Կ'
    pub const CAPITAL_LETTER_KEN: char = 'Կ';
    /// \u{540}: 'Հ'
    pub const CAPITAL_LETTER_HO: char = 'Հ';
    /// \u{541}: 'Ձ'
    pub const CAPITAL_LETTER_JA: char = 'Ձ';
    /// \u{542}: 'Ղ'
    pub const CAPITAL_LETTER_GHAD: char = 'Ղ';
    /// \u{543}: 'Ճ'
    pub const CAPITAL_LETTER_CHEH: char = 'Ճ';
    /// \u{544}: 'Մ'
    pub const CAPITAL_LETTER_MEN: char = 'Մ';
    /// \u{545}: 'Յ'
    pub const CAPITAL_LETTER_YI: char = 'Յ';
    /// \u{546}: 'Ն'
    pub const CAPITAL_LETTER_NOW: char = 'Ն';
    /// \u{547}: 'Շ'
    pub const CAPITAL_LETTER_SHA: char = 'Շ';
    /// \u{548}: 'Ո'
    pub const CAPITAL_LETTER_VO: char = 'Ո';
    /// \u{549}: 'Չ'
    pub const CAPITAL_LETTER_CHA: char = 'Չ';
    /// \u{54a}: 'Պ'
    pub const CAPITAL_LETTER_PEH: char = 'Պ';
    /// \u{54b}: 'Ջ'
    pub const CAPITAL_LETTER_JHEH: char = 'Ջ';
    /// \u{54c}: 'Ռ'
    pub const CAPITAL_LETTER_RA: char = 'Ռ';
    /// \u{54d}: 'Ս'
    pub const CAPITAL_LETTER_SEH: char = 'Ս';
    /// \u{54e}: 'Վ'
    pub const CAPITAL_LETTER_VEW: char = 'Վ';
    /// \u{54f}: 'Տ'
    pub const CAPITAL_LETTER_TIWN: char = 'Տ';
    /// \u{550}: 'Ր'
    pub const CAPITAL_LETTER_REH: char = 'Ր';
    /// \u{551}: 'Ց'
    pub const CAPITAL_LETTER_CO: char = 'Ց';
    /// \u{552}: 'Ւ'
    pub const CAPITAL_LETTER_YIWN: char = 'Ւ';
    /// \u{553}: 'Փ'
    pub const CAPITAL_LETTER_PIWR: char = 'Փ';
    /// \u{554}: 'Ք'
    pub const CAPITAL_LETTER_KEH: char = 'Ք';
    /// \u{555}: 'Օ'
    pub const CAPITAL_LETTER_OH: char = 'Օ';
    /// \u{556}: 'Ֆ'
    pub const CAPITAL_LETTER_FEH: char = 'Ֆ';
    /// \u{559}: 'ՙ'
    pub const MODIFIER_LETTER_LEFT_HALF_RING: char = 'ՙ';
    /// \u{55a}: '՚'
    pub const APOSTROPHE: char = '՚';
    /// \u{55b}: '՛'
    pub const EMPHASIS_MARK: char = '՛';
    /// \u{55c}: '՜'
    pub const EXCLAMATION_MARK: char = '՜';
    /// \u{55d}: '՝'
    pub const COMMA: char = '՝';
    /// \u{55e}: '՞'
    pub const QUESTION_MARK: char = '՞';
    /// \u{55f}: '՟'
    pub const ABBREVIATION_MARK: char = '՟';
    /// \u{560}: 'ՠ'
    pub const SMALL_LETTER_TURNED_AYB: char = 'ՠ';
    /// \u{561}: 'ա'
    pub const SMALL_LETTER_AYB: char = 'ա';
    /// \u{562}: 'բ'
    pub const SMALL_LETTER_BEN: char = 'բ';
    /// \u{563}: 'գ'
    pub const SMALL_LETTER_GIM: char = 'գ';
    /// \u{564}: 'դ'
    pub const SMALL_LETTER_DA: char = 'դ';
    /// \u{565}: 'ե'
    pub const SMALL_LETTER_ECH: char = 'ե';
    /// \u{566}: 'զ'
    pub const SMALL_LETTER_ZA: char = 'զ';
    /// \u{567}: 'է'
    pub const SMALL_LETTER_EH: char = 'է';
    /// \u{568}: 'ը'
    pub const SMALL_LETTER_ET: char = 'ը';
    /// \u{569}: 'թ'
    pub const SMALL_LETTER_TO: char = 'թ';
    /// \u{56a}: 'ժ'
    pub const SMALL_LETTER_ZHE: char = 'ժ';
    /// \u{56b}: 'ի'
    pub const SMALL_LETTER_INI: char = 'ի';
    /// \u{56c}: 'լ'
    pub const SMALL_LETTER_LIWN: char = 'լ';
    /// \u{56d}: 'խ'
    pub const SMALL_LETTER_XEH: char = 'խ';
    /// \u{56e}: 'ծ'
    pub const SMALL_LETTER_CA: char = 'ծ';
    /// \u{56f}: 'կ'
    pub const SMALL_LETTER_KEN: char = 'կ';
    /// \u{570}: 'հ'
    pub const SMALL_LETTER_HO: char = 'հ';
    /// \u{571}: 'ձ'
    pub const SMALL_LETTER_JA: char = 'ձ';
    /// \u{572}: 'ղ'
    pub const SMALL_LETTER_GHAD: char = 'ղ';
    /// \u{573}: 'ճ'
    pub const SMALL_LETTER_CHEH: char = 'ճ';
    /// \u{574}: 'մ'
    pub const SMALL_LETTER_MEN: char = 'մ';
    /// \u{575}: 'յ'
    pub const SMALL_LETTER_YI: char = 'յ';
    /// \u{576}: 'ն'
    pub const SMALL_LETTER_NOW: char = 'ն';
    /// \u{577}: 'շ'
    pub const SMALL_LETTER_SHA: char = 'շ';
    /// \u{578}: 'ո'
    pub const SMALL_LETTER_VO: char = 'ո';
    /// \u{579}: 'չ'
    pub const SMALL_LETTER_CHA: char = 'չ';
    /// \u{57a}: 'պ'
    pub const SMALL_LETTER_PEH: char = 'պ';
    /// \u{57b}: 'ջ'
    pub const SMALL_LETTER_JHEH: char = 'ջ';
    /// \u{57c}: 'ռ'
    pub const SMALL_LETTER_RA: char = 'ռ';
    /// \u{57d}: 'ս'
    pub const SMALL_LETTER_SEH: char = 'ս';
    /// \u{57e}: 'վ'
    pub const SMALL_LETTER_VEW: char = 'վ';
    /// \u{57f}: 'տ'
    pub const SMALL_LETTER_TIWN: char = 'տ';
    /// \u{580}: 'ր'
    pub const SMALL_LETTER_REH: char = 'ր';
    /// \u{581}: 'ց'
    pub const SMALL_LETTER_CO: char = 'ց';
    /// \u{582}: 'ւ'
    pub const SMALL_LETTER_YIWN: char = 'ւ';
    /// \u{583}: 'փ'
    pub const SMALL_LETTER_PIWR: char = 'փ';
    /// \u{584}: 'ք'
    pub const SMALL_LETTER_KEH: char = 'ք';
    /// \u{585}: 'օ'
    pub const SMALL_LETTER_OH: char = 'օ';
    /// \u{586}: 'ֆ'
    pub const SMALL_LETTER_FEH: char = 'ֆ';
    /// \u{587}: 'և'
    pub const SMALL_LIGATURE_ECH_YIWN: char = 'և';
    /// \u{588}: 'ֈ'
    pub const SMALL_LETTER_YI_WITH_STROKE: char = 'ֈ';
    /// \u{589}: '։'
    pub const FULL_STOP: char = '։';
    /// \u{58a}: '֊'
    pub const HYPHEN: char = '֊';
    /// \u{58d}: '֍'
    pub const RIGHT_DASH_FACING_ETERNITY_SIGN: char = '֍';
    /// \u{58e}: '֎'
    pub const LEFT_DASH_FACING_ETERNITY_SIGN: char = '֎';
}

/// An enum to represent all characters in the Armenian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Armenian {
    /// \u{531}: 'Ա'
    CapitalLetterAyb,
    /// \u{532}: 'Բ'
    CapitalLetterBen,
    /// \u{533}: 'Գ'
    CapitalLetterGim,
    /// \u{534}: 'Դ'
    CapitalLetterDa,
    /// \u{535}: 'Ե'
    CapitalLetterEch,
    /// \u{536}: 'Զ'
    CapitalLetterZa,
    /// \u{537}: 'Է'
    CapitalLetterEh,
    /// \u{538}: 'Ը'
    CapitalLetterEt,
    /// \u{539}: 'Թ'
    CapitalLetterTo,
    /// \u{53a}: 'Ժ'
    CapitalLetterZhe,
    /// \u{53b}: 'Ի'
    CapitalLetterIni,
    /// \u{53c}: 'Լ'
    CapitalLetterLiwn,
    /// \u{53d}: 'Խ'
    CapitalLetterXeh,
    /// \u{53e}: 'Ծ'
    CapitalLetterCa,
    /// \u{53f}: 'Կ'
    CapitalLetterKen,
    /// \u{540}: 'Հ'
    CapitalLetterHo,
    /// \u{541}: 'Ձ'
    CapitalLetterJa,
    /// \u{542}: 'Ղ'
    CapitalLetterGhad,
    /// \u{543}: 'Ճ'
    CapitalLetterCheh,
    /// \u{544}: 'Մ'
    CapitalLetterMen,
    /// \u{545}: 'Յ'
    CapitalLetterYi,
    /// \u{546}: 'Ն'
    CapitalLetterNow,
    /// \u{547}: 'Շ'
    CapitalLetterSha,
    /// \u{548}: 'Ո'
    CapitalLetterVo,
    /// \u{549}: 'Չ'
    CapitalLetterCha,
    /// \u{54a}: 'Պ'
    CapitalLetterPeh,
    /// \u{54b}: 'Ջ'
    CapitalLetterJheh,
    /// \u{54c}: 'Ռ'
    CapitalLetterRa,
    /// \u{54d}: 'Ս'
    CapitalLetterSeh,
    /// \u{54e}: 'Վ'
    CapitalLetterVew,
    /// \u{54f}: 'Տ'
    CapitalLetterTiwn,
    /// \u{550}: 'Ր'
    CapitalLetterReh,
    /// \u{551}: 'Ց'
    CapitalLetterCo,
    /// \u{552}: 'Ւ'
    CapitalLetterYiwn,
    /// \u{553}: 'Փ'
    CapitalLetterPiwr,
    /// \u{554}: 'Ք'
    CapitalLetterKeh,
    /// \u{555}: 'Օ'
    CapitalLetterOh,
    /// \u{556}: 'Ֆ'
    CapitalLetterFeh,
    /// \u{559}: 'ՙ'
    ModifierLetterLeftHalfRing,
    /// \u{55a}: '՚'
    Apostrophe,
    /// \u{55b}: '՛'
    EmphasisMark,
    /// \u{55c}: '՜'
    ExclamationMark,
    /// \u{55d}: '՝'
    Comma,
    /// \u{55e}: '՞'
    QuestionMark,
    /// \u{55f}: '՟'
    AbbreviationMark,
    /// \u{560}: 'ՠ'
    SmallLetterTurnedAyb,
    /// \u{561}: 'ա'
    SmallLetterAyb,
    /// \u{562}: 'բ'
    SmallLetterBen,
    /// \u{563}: 'գ'
    SmallLetterGim,
    /// \u{564}: 'դ'
    SmallLetterDa,
    /// \u{565}: 'ե'
    SmallLetterEch,
    /// \u{566}: 'զ'
    SmallLetterZa,
    /// \u{567}: 'է'
    SmallLetterEh,
    /// \u{568}: 'ը'
    SmallLetterEt,
    /// \u{569}: 'թ'
    SmallLetterTo,
    /// \u{56a}: 'ժ'
    SmallLetterZhe,
    /// \u{56b}: 'ի'
    SmallLetterIni,
    /// \u{56c}: 'լ'
    SmallLetterLiwn,
    /// \u{56d}: 'խ'
    SmallLetterXeh,
    /// \u{56e}: 'ծ'
    SmallLetterCa,
    /// \u{56f}: 'կ'
    SmallLetterKen,
    /// \u{570}: 'հ'
    SmallLetterHo,
    /// \u{571}: 'ձ'
    SmallLetterJa,
    /// \u{572}: 'ղ'
    SmallLetterGhad,
    /// \u{573}: 'ճ'
    SmallLetterCheh,
    /// \u{574}: 'մ'
    SmallLetterMen,
    /// \u{575}: 'յ'
    SmallLetterYi,
    /// \u{576}: 'ն'
    SmallLetterNow,
    /// \u{577}: 'շ'
    SmallLetterSha,
    /// \u{578}: 'ո'
    SmallLetterVo,
    /// \u{579}: 'չ'
    SmallLetterCha,
    /// \u{57a}: 'պ'
    SmallLetterPeh,
    /// \u{57b}: 'ջ'
    SmallLetterJheh,
    /// \u{57c}: 'ռ'
    SmallLetterRa,
    /// \u{57d}: 'ս'
    SmallLetterSeh,
    /// \u{57e}: 'վ'
    SmallLetterVew,
    /// \u{57f}: 'տ'
    SmallLetterTiwn,
    /// \u{580}: 'ր'
    SmallLetterReh,
    /// \u{581}: 'ց'
    SmallLetterCo,
    /// \u{582}: 'ւ'
    SmallLetterYiwn,
    /// \u{583}: 'փ'
    SmallLetterPiwr,
    /// \u{584}: 'ք'
    SmallLetterKeh,
    /// \u{585}: 'օ'
    SmallLetterOh,
    /// \u{586}: 'ֆ'
    SmallLetterFeh,
    /// \u{587}: 'և'
    SmallLigatureEchYiwn,
    /// \u{588}: 'ֈ'
    SmallLetterYiWithStroke,
    /// \u{589}: '։'
    FullStop,
    /// \u{58a}: '֊'
    Hyphen,
    /// \u{58d}: '֍'
    RightDashFacingEternitySign,
    /// \u{58e}: '֎'
    LeftDashFacingEternitySign,
}

impl Into<char> for Armenian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Armenian::CapitalLetterAyb => CAPITAL_LETTER_AYB,
            Armenian::CapitalLetterBen => CAPITAL_LETTER_BEN,
            Armenian::CapitalLetterGim => CAPITAL_LETTER_GIM,
            Armenian::CapitalLetterDa => CAPITAL_LETTER_DA,
            Armenian::CapitalLetterEch => CAPITAL_LETTER_ECH,
            Armenian::CapitalLetterZa => CAPITAL_LETTER_ZA,
            Armenian::CapitalLetterEh => CAPITAL_LETTER_EH,
            Armenian::CapitalLetterEt => CAPITAL_LETTER_ET,
            Armenian::CapitalLetterTo => CAPITAL_LETTER_TO,
            Armenian::CapitalLetterZhe => CAPITAL_LETTER_ZHE,
            Armenian::CapitalLetterIni => CAPITAL_LETTER_INI,
            Armenian::CapitalLetterLiwn => CAPITAL_LETTER_LIWN,
            Armenian::CapitalLetterXeh => CAPITAL_LETTER_XEH,
            Armenian::CapitalLetterCa => CAPITAL_LETTER_CA,
            Armenian::CapitalLetterKen => CAPITAL_LETTER_KEN,
            Armenian::CapitalLetterHo => CAPITAL_LETTER_HO,
            Armenian::CapitalLetterJa => CAPITAL_LETTER_JA,
            Armenian::CapitalLetterGhad => CAPITAL_LETTER_GHAD,
            Armenian::CapitalLetterCheh => CAPITAL_LETTER_CHEH,
            Armenian::CapitalLetterMen => CAPITAL_LETTER_MEN,
            Armenian::CapitalLetterYi => CAPITAL_LETTER_YI,
            Armenian::CapitalLetterNow => CAPITAL_LETTER_NOW,
            Armenian::CapitalLetterSha => CAPITAL_LETTER_SHA,
            Armenian::CapitalLetterVo => CAPITAL_LETTER_VO,
            Armenian::CapitalLetterCha => CAPITAL_LETTER_CHA,
            Armenian::CapitalLetterPeh => CAPITAL_LETTER_PEH,
            Armenian::CapitalLetterJheh => CAPITAL_LETTER_JHEH,
            Armenian::CapitalLetterRa => CAPITAL_LETTER_RA,
            Armenian::CapitalLetterSeh => CAPITAL_LETTER_SEH,
            Armenian::CapitalLetterVew => CAPITAL_LETTER_VEW,
            Armenian::CapitalLetterTiwn => CAPITAL_LETTER_TIWN,
            Armenian::CapitalLetterReh => CAPITAL_LETTER_REH,
            Armenian::CapitalLetterCo => CAPITAL_LETTER_CO,
            Armenian::CapitalLetterYiwn => CAPITAL_LETTER_YIWN,
            Armenian::CapitalLetterPiwr => CAPITAL_LETTER_PIWR,
            Armenian::CapitalLetterKeh => CAPITAL_LETTER_KEH,
            Armenian::CapitalLetterOh => CAPITAL_LETTER_OH,
            Armenian::CapitalLetterFeh => CAPITAL_LETTER_FEH,
            Armenian::ModifierLetterLeftHalfRing => MODIFIER_LETTER_LEFT_HALF_RING,
            Armenian::Apostrophe => APOSTROPHE,
            Armenian::EmphasisMark => EMPHASIS_MARK,
            Armenian::ExclamationMark => EXCLAMATION_MARK,
            Armenian::Comma => COMMA,
            Armenian::QuestionMark => QUESTION_MARK,
            Armenian::AbbreviationMark => ABBREVIATION_MARK,
            Armenian::SmallLetterTurnedAyb => SMALL_LETTER_TURNED_AYB,
            Armenian::SmallLetterAyb => SMALL_LETTER_AYB,
            Armenian::SmallLetterBen => SMALL_LETTER_BEN,
            Armenian::SmallLetterGim => SMALL_LETTER_GIM,
            Armenian::SmallLetterDa => SMALL_LETTER_DA,
            Armenian::SmallLetterEch => SMALL_LETTER_ECH,
            Armenian::SmallLetterZa => SMALL_LETTER_ZA,
            Armenian::SmallLetterEh => SMALL_LETTER_EH,
            Armenian::SmallLetterEt => SMALL_LETTER_ET,
            Armenian::SmallLetterTo => SMALL_LETTER_TO,
            Armenian::SmallLetterZhe => SMALL_LETTER_ZHE,
            Armenian::SmallLetterIni => SMALL_LETTER_INI,
            Armenian::SmallLetterLiwn => SMALL_LETTER_LIWN,
            Armenian::SmallLetterXeh => SMALL_LETTER_XEH,
            Armenian::SmallLetterCa => SMALL_LETTER_CA,
            Armenian::SmallLetterKen => SMALL_LETTER_KEN,
            Armenian::SmallLetterHo => SMALL_LETTER_HO,
            Armenian::SmallLetterJa => SMALL_LETTER_JA,
            Armenian::SmallLetterGhad => SMALL_LETTER_GHAD,
            Armenian::SmallLetterCheh => SMALL_LETTER_CHEH,
            Armenian::SmallLetterMen => SMALL_LETTER_MEN,
            Armenian::SmallLetterYi => SMALL_LETTER_YI,
            Armenian::SmallLetterNow => SMALL_LETTER_NOW,
            Armenian::SmallLetterSha => SMALL_LETTER_SHA,
            Armenian::SmallLetterVo => SMALL_LETTER_VO,
            Armenian::SmallLetterCha => SMALL_LETTER_CHA,
            Armenian::SmallLetterPeh => SMALL_LETTER_PEH,
            Armenian::SmallLetterJheh => SMALL_LETTER_JHEH,
            Armenian::SmallLetterRa => SMALL_LETTER_RA,
            Armenian::SmallLetterSeh => SMALL_LETTER_SEH,
            Armenian::SmallLetterVew => SMALL_LETTER_VEW,
            Armenian::SmallLetterTiwn => SMALL_LETTER_TIWN,
            Armenian::SmallLetterReh => SMALL_LETTER_REH,
            Armenian::SmallLetterCo => SMALL_LETTER_CO,
            Armenian::SmallLetterYiwn => SMALL_LETTER_YIWN,
            Armenian::SmallLetterPiwr => SMALL_LETTER_PIWR,
            Armenian::SmallLetterKeh => SMALL_LETTER_KEH,
            Armenian::SmallLetterOh => SMALL_LETTER_OH,
            Armenian::SmallLetterFeh => SMALL_LETTER_FEH,
            Armenian::SmallLigatureEchYiwn => SMALL_LIGATURE_ECH_YIWN,
            Armenian::SmallLetterYiWithStroke => SMALL_LETTER_YI_WITH_STROKE,
            Armenian::FullStop => FULL_STOP,
            Armenian::Hyphen => HYPHEN,
            Armenian::RightDashFacingEternitySign => RIGHT_DASH_FACING_ETERNITY_SIGN,
            Armenian::LeftDashFacingEternitySign => LEFT_DASH_FACING_ETERNITY_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for Armenian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CAPITAL_LETTER_AYB => Ok(Armenian::CapitalLetterAyb),
            CAPITAL_LETTER_BEN => Ok(Armenian::CapitalLetterBen),
            CAPITAL_LETTER_GIM => Ok(Armenian::CapitalLetterGim),
            CAPITAL_LETTER_DA => Ok(Armenian::CapitalLetterDa),
            CAPITAL_LETTER_ECH => Ok(Armenian::CapitalLetterEch),
            CAPITAL_LETTER_ZA => Ok(Armenian::CapitalLetterZa),
            CAPITAL_LETTER_EH => Ok(Armenian::CapitalLetterEh),
            CAPITAL_LETTER_ET => Ok(Armenian::CapitalLetterEt),
            CAPITAL_LETTER_TO => Ok(Armenian::CapitalLetterTo),
            CAPITAL_LETTER_ZHE => Ok(Armenian::CapitalLetterZhe),
            CAPITAL_LETTER_INI => Ok(Armenian::CapitalLetterIni),
            CAPITAL_LETTER_LIWN => Ok(Armenian::CapitalLetterLiwn),
            CAPITAL_LETTER_XEH => Ok(Armenian::CapitalLetterXeh),
            CAPITAL_LETTER_CA => Ok(Armenian::CapitalLetterCa),
            CAPITAL_LETTER_KEN => Ok(Armenian::CapitalLetterKen),
            CAPITAL_LETTER_HO => Ok(Armenian::CapitalLetterHo),
            CAPITAL_LETTER_JA => Ok(Armenian::CapitalLetterJa),
            CAPITAL_LETTER_GHAD => Ok(Armenian::CapitalLetterGhad),
            CAPITAL_LETTER_CHEH => Ok(Armenian::CapitalLetterCheh),
            CAPITAL_LETTER_MEN => Ok(Armenian::CapitalLetterMen),
            CAPITAL_LETTER_YI => Ok(Armenian::CapitalLetterYi),
            CAPITAL_LETTER_NOW => Ok(Armenian::CapitalLetterNow),
            CAPITAL_LETTER_SHA => Ok(Armenian::CapitalLetterSha),
            CAPITAL_LETTER_VO => Ok(Armenian::CapitalLetterVo),
            CAPITAL_LETTER_CHA => Ok(Armenian::CapitalLetterCha),
            CAPITAL_LETTER_PEH => Ok(Armenian::CapitalLetterPeh),
            CAPITAL_LETTER_JHEH => Ok(Armenian::CapitalLetterJheh),
            CAPITAL_LETTER_RA => Ok(Armenian::CapitalLetterRa),
            CAPITAL_LETTER_SEH => Ok(Armenian::CapitalLetterSeh),
            CAPITAL_LETTER_VEW => Ok(Armenian::CapitalLetterVew),
            CAPITAL_LETTER_TIWN => Ok(Armenian::CapitalLetterTiwn),
            CAPITAL_LETTER_REH => Ok(Armenian::CapitalLetterReh),
            CAPITAL_LETTER_CO => Ok(Armenian::CapitalLetterCo),
            CAPITAL_LETTER_YIWN => Ok(Armenian::CapitalLetterYiwn),
            CAPITAL_LETTER_PIWR => Ok(Armenian::CapitalLetterPiwr),
            CAPITAL_LETTER_KEH => Ok(Armenian::CapitalLetterKeh),
            CAPITAL_LETTER_OH => Ok(Armenian::CapitalLetterOh),
            CAPITAL_LETTER_FEH => Ok(Armenian::CapitalLetterFeh),
            MODIFIER_LETTER_LEFT_HALF_RING => Ok(Armenian::ModifierLetterLeftHalfRing),
            APOSTROPHE => Ok(Armenian::Apostrophe),
            EMPHASIS_MARK => Ok(Armenian::EmphasisMark),
            EXCLAMATION_MARK => Ok(Armenian::ExclamationMark),
            COMMA => Ok(Armenian::Comma),
            QUESTION_MARK => Ok(Armenian::QuestionMark),
            ABBREVIATION_MARK => Ok(Armenian::AbbreviationMark),
            SMALL_LETTER_TURNED_AYB => Ok(Armenian::SmallLetterTurnedAyb),
            SMALL_LETTER_AYB => Ok(Armenian::SmallLetterAyb),
            SMALL_LETTER_BEN => Ok(Armenian::SmallLetterBen),
            SMALL_LETTER_GIM => Ok(Armenian::SmallLetterGim),
            SMALL_LETTER_DA => Ok(Armenian::SmallLetterDa),
            SMALL_LETTER_ECH => Ok(Armenian::SmallLetterEch),
            SMALL_LETTER_ZA => Ok(Armenian::SmallLetterZa),
            SMALL_LETTER_EH => Ok(Armenian::SmallLetterEh),
            SMALL_LETTER_ET => Ok(Armenian::SmallLetterEt),
            SMALL_LETTER_TO => Ok(Armenian::SmallLetterTo),
            SMALL_LETTER_ZHE => Ok(Armenian::SmallLetterZhe),
            SMALL_LETTER_INI => Ok(Armenian::SmallLetterIni),
            SMALL_LETTER_LIWN => Ok(Armenian::SmallLetterLiwn),
            SMALL_LETTER_XEH => Ok(Armenian::SmallLetterXeh),
            SMALL_LETTER_CA => Ok(Armenian::SmallLetterCa),
            SMALL_LETTER_KEN => Ok(Armenian::SmallLetterKen),
            SMALL_LETTER_HO => Ok(Armenian::SmallLetterHo),
            SMALL_LETTER_JA => Ok(Armenian::SmallLetterJa),
            SMALL_LETTER_GHAD => Ok(Armenian::SmallLetterGhad),
            SMALL_LETTER_CHEH => Ok(Armenian::SmallLetterCheh),
            SMALL_LETTER_MEN => Ok(Armenian::SmallLetterMen),
            SMALL_LETTER_YI => Ok(Armenian::SmallLetterYi),
            SMALL_LETTER_NOW => Ok(Armenian::SmallLetterNow),
            SMALL_LETTER_SHA => Ok(Armenian::SmallLetterSha),
            SMALL_LETTER_VO => Ok(Armenian::SmallLetterVo),
            SMALL_LETTER_CHA => Ok(Armenian::SmallLetterCha),
            SMALL_LETTER_PEH => Ok(Armenian::SmallLetterPeh),
            SMALL_LETTER_JHEH => Ok(Armenian::SmallLetterJheh),
            SMALL_LETTER_RA => Ok(Armenian::SmallLetterRa),
            SMALL_LETTER_SEH => Ok(Armenian::SmallLetterSeh),
            SMALL_LETTER_VEW => Ok(Armenian::SmallLetterVew),
            SMALL_LETTER_TIWN => Ok(Armenian::SmallLetterTiwn),
            SMALL_LETTER_REH => Ok(Armenian::SmallLetterReh),
            SMALL_LETTER_CO => Ok(Armenian::SmallLetterCo),
            SMALL_LETTER_YIWN => Ok(Armenian::SmallLetterYiwn),
            SMALL_LETTER_PIWR => Ok(Armenian::SmallLetterPiwr),
            SMALL_LETTER_KEH => Ok(Armenian::SmallLetterKeh),
            SMALL_LETTER_OH => Ok(Armenian::SmallLetterOh),
            SMALL_LETTER_FEH => Ok(Armenian::SmallLetterFeh),
            SMALL_LIGATURE_ECH_YIWN => Ok(Armenian::SmallLigatureEchYiwn),
            SMALL_LETTER_YI_WITH_STROKE => Ok(Armenian::SmallLetterYiWithStroke),
            FULL_STOP => Ok(Armenian::FullStop),
            HYPHEN => Ok(Armenian::Hyphen),
            RIGHT_DASH_FACING_ETERNITY_SIGN => Ok(Armenian::RightDashFacingEternitySign),
            LEFT_DASH_FACING_ETERNITY_SIGN => Ok(Armenian::LeftDashFacingEternitySign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Armenian {
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

impl std::convert::TryFrom<u32> for Armenian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Armenian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Armenian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Armenian::CapitalLetterAyb
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Armenian::CapitalLetterAyb => "armenian capital letter ayb",
            Armenian::CapitalLetterBen => "armenian capital letter ben",
            Armenian::CapitalLetterGim => "armenian capital letter gim",
            Armenian::CapitalLetterDa => "armenian capital letter da",
            Armenian::CapitalLetterEch => "armenian capital letter ech",
            Armenian::CapitalLetterZa => "armenian capital letter za",
            Armenian::CapitalLetterEh => "armenian capital letter eh",
            Armenian::CapitalLetterEt => "armenian capital letter et",
            Armenian::CapitalLetterTo => "armenian capital letter to",
            Armenian::CapitalLetterZhe => "armenian capital letter zhe",
            Armenian::CapitalLetterIni => "armenian capital letter ini",
            Armenian::CapitalLetterLiwn => "armenian capital letter liwn",
            Armenian::CapitalLetterXeh => "armenian capital letter xeh",
            Armenian::CapitalLetterCa => "armenian capital letter ca",
            Armenian::CapitalLetterKen => "armenian capital letter ken",
            Armenian::CapitalLetterHo => "armenian capital letter ho",
            Armenian::CapitalLetterJa => "armenian capital letter ja",
            Armenian::CapitalLetterGhad => "armenian capital letter ghad",
            Armenian::CapitalLetterCheh => "armenian capital letter cheh",
            Armenian::CapitalLetterMen => "armenian capital letter men",
            Armenian::CapitalLetterYi => "armenian capital letter yi",
            Armenian::CapitalLetterNow => "armenian capital letter now",
            Armenian::CapitalLetterSha => "armenian capital letter sha",
            Armenian::CapitalLetterVo => "armenian capital letter vo",
            Armenian::CapitalLetterCha => "armenian capital letter cha",
            Armenian::CapitalLetterPeh => "armenian capital letter peh",
            Armenian::CapitalLetterJheh => "armenian capital letter jheh",
            Armenian::CapitalLetterRa => "armenian capital letter ra",
            Armenian::CapitalLetterSeh => "armenian capital letter seh",
            Armenian::CapitalLetterVew => "armenian capital letter vew",
            Armenian::CapitalLetterTiwn => "armenian capital letter tiwn",
            Armenian::CapitalLetterReh => "armenian capital letter reh",
            Armenian::CapitalLetterCo => "armenian capital letter co",
            Armenian::CapitalLetterYiwn => "armenian capital letter yiwn",
            Armenian::CapitalLetterPiwr => "armenian capital letter piwr",
            Armenian::CapitalLetterKeh => "armenian capital letter keh",
            Armenian::CapitalLetterOh => "armenian capital letter oh",
            Armenian::CapitalLetterFeh => "armenian capital letter feh",
            Armenian::ModifierLetterLeftHalfRing => "armenian modifier letter left half ring",
            Armenian::Apostrophe => "armenian apostrophe",
            Armenian::EmphasisMark => "armenian emphasis mark",
            Armenian::ExclamationMark => "armenian exclamation mark",
            Armenian::Comma => "armenian comma",
            Armenian::QuestionMark => "armenian question mark",
            Armenian::AbbreviationMark => "armenian abbreviation mark",
            Armenian::SmallLetterTurnedAyb => "armenian small letter turned ayb",
            Armenian::SmallLetterAyb => "armenian small letter ayb",
            Armenian::SmallLetterBen => "armenian small letter ben",
            Armenian::SmallLetterGim => "armenian small letter gim",
            Armenian::SmallLetterDa => "armenian small letter da",
            Armenian::SmallLetterEch => "armenian small letter ech",
            Armenian::SmallLetterZa => "armenian small letter za",
            Armenian::SmallLetterEh => "armenian small letter eh",
            Armenian::SmallLetterEt => "armenian small letter et",
            Armenian::SmallLetterTo => "armenian small letter to",
            Armenian::SmallLetterZhe => "armenian small letter zhe",
            Armenian::SmallLetterIni => "armenian small letter ini",
            Armenian::SmallLetterLiwn => "armenian small letter liwn",
            Armenian::SmallLetterXeh => "armenian small letter xeh",
            Armenian::SmallLetterCa => "armenian small letter ca",
            Armenian::SmallLetterKen => "armenian small letter ken",
            Armenian::SmallLetterHo => "armenian small letter ho",
            Armenian::SmallLetterJa => "armenian small letter ja",
            Armenian::SmallLetterGhad => "armenian small letter ghad",
            Armenian::SmallLetterCheh => "armenian small letter cheh",
            Armenian::SmallLetterMen => "armenian small letter men",
            Armenian::SmallLetterYi => "armenian small letter yi",
            Armenian::SmallLetterNow => "armenian small letter now",
            Armenian::SmallLetterSha => "armenian small letter sha",
            Armenian::SmallLetterVo => "armenian small letter vo",
            Armenian::SmallLetterCha => "armenian small letter cha",
            Armenian::SmallLetterPeh => "armenian small letter peh",
            Armenian::SmallLetterJheh => "armenian small letter jheh",
            Armenian::SmallLetterRa => "armenian small letter ra",
            Armenian::SmallLetterSeh => "armenian small letter seh",
            Armenian::SmallLetterVew => "armenian small letter vew",
            Armenian::SmallLetterTiwn => "armenian small letter tiwn",
            Armenian::SmallLetterReh => "armenian small letter reh",
            Armenian::SmallLetterCo => "armenian small letter co",
            Armenian::SmallLetterYiwn => "armenian small letter yiwn",
            Armenian::SmallLetterPiwr => "armenian small letter piwr",
            Armenian::SmallLetterKeh => "armenian small letter keh",
            Armenian::SmallLetterOh => "armenian small letter oh",
            Armenian::SmallLetterFeh => "armenian small letter feh",
            Armenian::SmallLigatureEchYiwn => "armenian small ligature ech yiwn",
            Armenian::SmallLetterYiWithStroke => "armenian small letter yi with stroke",
            Armenian::FullStop => "armenian full stop",
            Armenian::Hyphen => "armenian hyphen",
            Armenian::RightDashFacingEternitySign => "right-facing armenian eternity sign",
            Armenian::LeftDashFacingEternitySign => "left-facing armenian eternity sign",
        }
    }
}
