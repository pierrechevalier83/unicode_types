
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
        match self {
            Armenian::CapitalLetterAyb => 'Ա',
            Armenian::CapitalLetterBen => 'Բ',
            Armenian::CapitalLetterGim => 'Գ',
            Armenian::CapitalLetterDa => 'Դ',
            Armenian::CapitalLetterEch => 'Ե',
            Armenian::CapitalLetterZa => 'Զ',
            Armenian::CapitalLetterEh => 'Է',
            Armenian::CapitalLetterEt => 'Ը',
            Armenian::CapitalLetterTo => 'Թ',
            Armenian::CapitalLetterZhe => 'Ժ',
            Armenian::CapitalLetterIni => 'Ի',
            Armenian::CapitalLetterLiwn => 'Լ',
            Armenian::CapitalLetterXeh => 'Խ',
            Armenian::CapitalLetterCa => 'Ծ',
            Armenian::CapitalLetterKen => 'Կ',
            Armenian::CapitalLetterHo => 'Հ',
            Armenian::CapitalLetterJa => 'Ձ',
            Armenian::CapitalLetterGhad => 'Ղ',
            Armenian::CapitalLetterCheh => 'Ճ',
            Armenian::CapitalLetterMen => 'Մ',
            Armenian::CapitalLetterYi => 'Յ',
            Armenian::CapitalLetterNow => 'Ն',
            Armenian::CapitalLetterSha => 'Շ',
            Armenian::CapitalLetterVo => 'Ո',
            Armenian::CapitalLetterCha => 'Չ',
            Armenian::CapitalLetterPeh => 'Պ',
            Armenian::CapitalLetterJheh => 'Ջ',
            Armenian::CapitalLetterRa => 'Ռ',
            Armenian::CapitalLetterSeh => 'Ս',
            Armenian::CapitalLetterVew => 'Վ',
            Armenian::CapitalLetterTiwn => 'Տ',
            Armenian::CapitalLetterReh => 'Ր',
            Armenian::CapitalLetterCo => 'Ց',
            Armenian::CapitalLetterYiwn => 'Ւ',
            Armenian::CapitalLetterPiwr => 'Փ',
            Armenian::CapitalLetterKeh => 'Ք',
            Armenian::CapitalLetterOh => 'Օ',
            Armenian::CapitalLetterFeh => 'Ֆ',
            Armenian::ModifierLetterLeftHalfRing => 'ՙ',
            Armenian::Apostrophe => '՚',
            Armenian::EmphasisMark => '՛',
            Armenian::ExclamationMark => '՜',
            Armenian::Comma => '՝',
            Armenian::QuestionMark => '՞',
            Armenian::AbbreviationMark => '՟',
            Armenian::SmallLetterTurnedAyb => 'ՠ',
            Armenian::SmallLetterAyb => 'ա',
            Armenian::SmallLetterBen => 'բ',
            Armenian::SmallLetterGim => 'գ',
            Armenian::SmallLetterDa => 'դ',
            Armenian::SmallLetterEch => 'ե',
            Armenian::SmallLetterZa => 'զ',
            Armenian::SmallLetterEh => 'է',
            Armenian::SmallLetterEt => 'ը',
            Armenian::SmallLetterTo => 'թ',
            Armenian::SmallLetterZhe => 'ժ',
            Armenian::SmallLetterIni => 'ի',
            Armenian::SmallLetterLiwn => 'լ',
            Armenian::SmallLetterXeh => 'խ',
            Armenian::SmallLetterCa => 'ծ',
            Armenian::SmallLetterKen => 'կ',
            Armenian::SmallLetterHo => 'հ',
            Armenian::SmallLetterJa => 'ձ',
            Armenian::SmallLetterGhad => 'ղ',
            Armenian::SmallLetterCheh => 'ճ',
            Armenian::SmallLetterMen => 'մ',
            Armenian::SmallLetterYi => 'յ',
            Armenian::SmallLetterNow => 'ն',
            Armenian::SmallLetterSha => 'շ',
            Armenian::SmallLetterVo => 'ո',
            Armenian::SmallLetterCha => 'չ',
            Armenian::SmallLetterPeh => 'պ',
            Armenian::SmallLetterJheh => 'ջ',
            Armenian::SmallLetterRa => 'ռ',
            Armenian::SmallLetterSeh => 'ս',
            Armenian::SmallLetterVew => 'վ',
            Armenian::SmallLetterTiwn => 'տ',
            Armenian::SmallLetterReh => 'ր',
            Armenian::SmallLetterCo => 'ց',
            Armenian::SmallLetterYiwn => 'ւ',
            Armenian::SmallLetterPiwr => 'փ',
            Armenian::SmallLetterKeh => 'ք',
            Armenian::SmallLetterOh => 'օ',
            Armenian::SmallLetterFeh => 'ֆ',
            Armenian::SmallLigatureEchYiwn => 'և',
            Armenian::SmallLetterYiWithStroke => 'ֈ',
            Armenian::FullStop => '։',
            Armenian::Hyphen => '֊',
            Armenian::RightDashFacingEternitySign => '֍',
            Armenian::LeftDashFacingEternitySign => '֎',
        }
    }
}

impl std::convert::TryFrom<char> for Armenian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ա' => Ok(Armenian::CapitalLetterAyb),
            'Բ' => Ok(Armenian::CapitalLetterBen),
            'Գ' => Ok(Armenian::CapitalLetterGim),
            'Դ' => Ok(Armenian::CapitalLetterDa),
            'Ե' => Ok(Armenian::CapitalLetterEch),
            'Զ' => Ok(Armenian::CapitalLetterZa),
            'Է' => Ok(Armenian::CapitalLetterEh),
            'Ը' => Ok(Armenian::CapitalLetterEt),
            'Թ' => Ok(Armenian::CapitalLetterTo),
            'Ժ' => Ok(Armenian::CapitalLetterZhe),
            'Ի' => Ok(Armenian::CapitalLetterIni),
            'Լ' => Ok(Armenian::CapitalLetterLiwn),
            'Խ' => Ok(Armenian::CapitalLetterXeh),
            'Ծ' => Ok(Armenian::CapitalLetterCa),
            'Կ' => Ok(Armenian::CapitalLetterKen),
            'Հ' => Ok(Armenian::CapitalLetterHo),
            'Ձ' => Ok(Armenian::CapitalLetterJa),
            'Ղ' => Ok(Armenian::CapitalLetterGhad),
            'Ճ' => Ok(Armenian::CapitalLetterCheh),
            'Մ' => Ok(Armenian::CapitalLetterMen),
            'Յ' => Ok(Armenian::CapitalLetterYi),
            'Ն' => Ok(Armenian::CapitalLetterNow),
            'Շ' => Ok(Armenian::CapitalLetterSha),
            'Ո' => Ok(Armenian::CapitalLetterVo),
            'Չ' => Ok(Armenian::CapitalLetterCha),
            'Պ' => Ok(Armenian::CapitalLetterPeh),
            'Ջ' => Ok(Armenian::CapitalLetterJheh),
            'Ռ' => Ok(Armenian::CapitalLetterRa),
            'Ս' => Ok(Armenian::CapitalLetterSeh),
            'Վ' => Ok(Armenian::CapitalLetterVew),
            'Տ' => Ok(Armenian::CapitalLetterTiwn),
            'Ր' => Ok(Armenian::CapitalLetterReh),
            'Ց' => Ok(Armenian::CapitalLetterCo),
            'Ւ' => Ok(Armenian::CapitalLetterYiwn),
            'Փ' => Ok(Armenian::CapitalLetterPiwr),
            'Ք' => Ok(Armenian::CapitalLetterKeh),
            'Օ' => Ok(Armenian::CapitalLetterOh),
            'Ֆ' => Ok(Armenian::CapitalLetterFeh),
            'ՙ' => Ok(Armenian::ModifierLetterLeftHalfRing),
            '՚' => Ok(Armenian::Apostrophe),
            '՛' => Ok(Armenian::EmphasisMark),
            '՜' => Ok(Armenian::ExclamationMark),
            '՝' => Ok(Armenian::Comma),
            '՞' => Ok(Armenian::QuestionMark),
            '՟' => Ok(Armenian::AbbreviationMark),
            'ՠ' => Ok(Armenian::SmallLetterTurnedAyb),
            'ա' => Ok(Armenian::SmallLetterAyb),
            'բ' => Ok(Armenian::SmallLetterBen),
            'գ' => Ok(Armenian::SmallLetterGim),
            'դ' => Ok(Armenian::SmallLetterDa),
            'ե' => Ok(Armenian::SmallLetterEch),
            'զ' => Ok(Armenian::SmallLetterZa),
            'է' => Ok(Armenian::SmallLetterEh),
            'ը' => Ok(Armenian::SmallLetterEt),
            'թ' => Ok(Armenian::SmallLetterTo),
            'ժ' => Ok(Armenian::SmallLetterZhe),
            'ի' => Ok(Armenian::SmallLetterIni),
            'լ' => Ok(Armenian::SmallLetterLiwn),
            'խ' => Ok(Armenian::SmallLetterXeh),
            'ծ' => Ok(Armenian::SmallLetterCa),
            'կ' => Ok(Armenian::SmallLetterKen),
            'հ' => Ok(Armenian::SmallLetterHo),
            'ձ' => Ok(Armenian::SmallLetterJa),
            'ղ' => Ok(Armenian::SmallLetterGhad),
            'ճ' => Ok(Armenian::SmallLetterCheh),
            'մ' => Ok(Armenian::SmallLetterMen),
            'յ' => Ok(Armenian::SmallLetterYi),
            'ն' => Ok(Armenian::SmallLetterNow),
            'շ' => Ok(Armenian::SmallLetterSha),
            'ո' => Ok(Armenian::SmallLetterVo),
            'չ' => Ok(Armenian::SmallLetterCha),
            'պ' => Ok(Armenian::SmallLetterPeh),
            'ջ' => Ok(Armenian::SmallLetterJheh),
            'ռ' => Ok(Armenian::SmallLetterRa),
            'ս' => Ok(Armenian::SmallLetterSeh),
            'վ' => Ok(Armenian::SmallLetterVew),
            'տ' => Ok(Armenian::SmallLetterTiwn),
            'ր' => Ok(Armenian::SmallLetterReh),
            'ց' => Ok(Armenian::SmallLetterCo),
            'ւ' => Ok(Armenian::SmallLetterYiwn),
            'փ' => Ok(Armenian::SmallLetterPiwr),
            'ք' => Ok(Armenian::SmallLetterKeh),
            'օ' => Ok(Armenian::SmallLetterOh),
            'ֆ' => Ok(Armenian::SmallLetterFeh),
            'և' => Ok(Armenian::SmallLigatureEchYiwn),
            'ֈ' => Ok(Armenian::SmallLetterYiWithStroke),
            '։' => Ok(Armenian::FullStop),
            '֊' => Ok(Armenian::Hyphen),
            '֍' => Ok(Armenian::RightDashFacingEternitySign),
            '֎' => Ok(Armenian::LeftDashFacingEternitySign),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Armenian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
