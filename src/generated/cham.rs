
/// An enum to represent all characters in the Cham block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Cham {
    /// \u{aa00}: 'ꨀ'
    LetterA,
    /// \u{aa01}: 'ꨁ'
    LetterI,
    /// \u{aa02}: 'ꨂ'
    LetterU,
    /// \u{aa03}: 'ꨃ'
    LetterE,
    /// \u{aa04}: 'ꨄ'
    LetterAi,
    /// \u{aa05}: 'ꨅ'
    LetterO,
    /// \u{aa06}: 'ꨆ'
    LetterKa,
    /// \u{aa07}: 'ꨇ'
    LetterKha,
    /// \u{aa08}: 'ꨈ'
    LetterGa,
    /// \u{aa09}: 'ꨉ'
    LetterGha,
    /// \u{aa0a}: 'ꨊ'
    LetterNgue,
    /// \u{aa0b}: 'ꨋ'
    LetterNga,
    /// \u{aa0c}: 'ꨌ'
    LetterCha,
    /// \u{aa0d}: 'ꨍ'
    LetterChha,
    /// \u{aa0e}: 'ꨎ'
    LetterJa,
    /// \u{aa0f}: 'ꨏ'
    LetterJha,
    /// \u{aa10}: 'ꨐ'
    LetterNhue,
    /// \u{aa11}: 'ꨑ'
    LetterNha,
    /// \u{aa12}: 'ꨒ'
    LetterNhja,
    /// \u{aa13}: 'ꨓ'
    LetterTa,
    /// \u{aa14}: 'ꨔ'
    LetterTha,
    /// \u{aa15}: 'ꨕ'
    LetterDa,
    /// \u{aa16}: 'ꨖ'
    LetterDha,
    /// \u{aa17}: 'ꨗ'
    LetterNue,
    /// \u{aa18}: 'ꨘ'
    LetterNa,
    /// \u{aa19}: 'ꨙ'
    LetterDda,
    /// \u{aa1a}: 'ꨚ'
    LetterPa,
    /// \u{aa1b}: 'ꨛ'
    LetterPpa,
    /// \u{aa1c}: 'ꨜ'
    LetterPha,
    /// \u{aa1d}: 'ꨝ'
    LetterBa,
    /// \u{aa1e}: 'ꨞ'
    LetterBha,
    /// \u{aa1f}: 'ꨟ'
    LetterMue,
    /// \u{aa20}: 'ꨠ'
    LetterMa,
    /// \u{aa21}: 'ꨡ'
    LetterBba,
    /// \u{aa22}: 'ꨢ'
    LetterYa,
    /// \u{aa23}: 'ꨣ'
    LetterRa,
    /// \u{aa24}: 'ꨤ'
    LetterLa,
    /// \u{aa25}: 'ꨥ'
    LetterVa,
    /// \u{aa26}: 'ꨦ'
    LetterSsa,
    /// \u{aa27}: 'ꨧ'
    LetterSa,
    /// \u{aa28}: 'ꨨ'
    LetterHa,
    /// \u{aa29}: 'ꨩ'
    VowelSignAa,
    /// \u{aa2a}: 'ꨪ'
    VowelSignI,
    /// \u{aa2b}: 'ꨫ'
    VowelSignIi,
    /// \u{aa2c}: 'ꨬ'
    VowelSignEi,
    /// \u{aa2d}: 'ꨭ'
    VowelSignU,
    /// \u{aa2e}: 'ꨮ'
    VowelSignOe,
    /// \u{aa2f}: 'ꨯ'
    VowelSignO,
    /// \u{aa30}: 'ꨰ'
    VowelSignAi,
    /// \u{aa31}: 'ꨱ'
    VowelSignAu,
    /// \u{aa32}: 'ꨲ'
    VowelSignUe,
    /// \u{aa33}: 'ꨳ'
    ConsonantSignYa,
    /// \u{aa34}: 'ꨴ'
    ConsonantSignRa,
    /// \u{aa35}: 'ꨵ'
    ConsonantSignLa,
    /// \u{aa36}: 'ꨶ'
    ConsonantSignWa,
    /// \u{aa40}: 'ꩀ'
    LetterFinalK,
    /// \u{aa41}: 'ꩁ'
    LetterFinalG,
    /// \u{aa42}: 'ꩂ'
    LetterFinalNg,
    /// \u{aa43}: 'ꩃ'
    ConsonantSignFinalNg,
    /// \u{aa44}: 'ꩄ'
    LetterFinalCh,
    /// \u{aa45}: 'ꩅ'
    LetterFinalT,
    /// \u{aa46}: 'ꩆ'
    LetterFinalN,
    /// \u{aa47}: 'ꩇ'
    LetterFinalP,
    /// \u{aa48}: 'ꩈ'
    LetterFinalY,
    /// \u{aa49}: 'ꩉ'
    LetterFinalR,
    /// \u{aa4a}: 'ꩊ'
    LetterFinalL,
    /// \u{aa4b}: 'ꩋ'
    LetterFinalSs,
    /// \u{aa4c}: 'ꩌ'
    ConsonantSignFinalM,
    /// \u{aa4d}: 'ꩍ'
    ConsonantSignFinalH,
    /// \u{aa50}: '꩐'
    DigitZero,
    /// \u{aa51}: '꩑'
    DigitOne,
    /// \u{aa52}: '꩒'
    DigitTwo,
    /// \u{aa53}: '꩓'
    DigitThree,
    /// \u{aa54}: '꩔'
    DigitFour,
    /// \u{aa55}: '꩕'
    DigitFive,
    /// \u{aa56}: '꩖'
    DigitSix,
    /// \u{aa57}: '꩗'
    DigitSeven,
    /// \u{aa58}: '꩘'
    DigitEight,
    /// \u{aa59}: '꩙'
    DigitNine,
    /// \u{aa5c}: '꩜'
    PunctuationSpiral,
    /// \u{aa5d}: '꩝'
    PunctuationDanda,
    /// \u{aa5e}: '꩞'
    PunctuationDoubleDanda,
}

impl Into<char> for Cham {
    fn into(self) -> char {
        match self {
            Cham::LetterA => 'ꨀ',
            Cham::LetterI => 'ꨁ',
            Cham::LetterU => 'ꨂ',
            Cham::LetterE => 'ꨃ',
            Cham::LetterAi => 'ꨄ',
            Cham::LetterO => 'ꨅ',
            Cham::LetterKa => 'ꨆ',
            Cham::LetterKha => 'ꨇ',
            Cham::LetterGa => 'ꨈ',
            Cham::LetterGha => 'ꨉ',
            Cham::LetterNgue => 'ꨊ',
            Cham::LetterNga => 'ꨋ',
            Cham::LetterCha => 'ꨌ',
            Cham::LetterChha => 'ꨍ',
            Cham::LetterJa => 'ꨎ',
            Cham::LetterJha => 'ꨏ',
            Cham::LetterNhue => 'ꨐ',
            Cham::LetterNha => 'ꨑ',
            Cham::LetterNhja => 'ꨒ',
            Cham::LetterTa => 'ꨓ',
            Cham::LetterTha => 'ꨔ',
            Cham::LetterDa => 'ꨕ',
            Cham::LetterDha => 'ꨖ',
            Cham::LetterNue => 'ꨗ',
            Cham::LetterNa => 'ꨘ',
            Cham::LetterDda => 'ꨙ',
            Cham::LetterPa => 'ꨚ',
            Cham::LetterPpa => 'ꨛ',
            Cham::LetterPha => 'ꨜ',
            Cham::LetterBa => 'ꨝ',
            Cham::LetterBha => 'ꨞ',
            Cham::LetterMue => 'ꨟ',
            Cham::LetterMa => 'ꨠ',
            Cham::LetterBba => 'ꨡ',
            Cham::LetterYa => 'ꨢ',
            Cham::LetterRa => 'ꨣ',
            Cham::LetterLa => 'ꨤ',
            Cham::LetterVa => 'ꨥ',
            Cham::LetterSsa => 'ꨦ',
            Cham::LetterSa => 'ꨧ',
            Cham::LetterHa => 'ꨨ',
            Cham::VowelSignAa => 'ꨩ',
            Cham::VowelSignI => 'ꨪ',
            Cham::VowelSignIi => 'ꨫ',
            Cham::VowelSignEi => 'ꨬ',
            Cham::VowelSignU => 'ꨭ',
            Cham::VowelSignOe => 'ꨮ',
            Cham::VowelSignO => 'ꨯ',
            Cham::VowelSignAi => 'ꨰ',
            Cham::VowelSignAu => 'ꨱ',
            Cham::VowelSignUe => 'ꨲ',
            Cham::ConsonantSignYa => 'ꨳ',
            Cham::ConsonantSignRa => 'ꨴ',
            Cham::ConsonantSignLa => 'ꨵ',
            Cham::ConsonantSignWa => 'ꨶ',
            Cham::LetterFinalK => 'ꩀ',
            Cham::LetterFinalG => 'ꩁ',
            Cham::LetterFinalNg => 'ꩂ',
            Cham::ConsonantSignFinalNg => 'ꩃ',
            Cham::LetterFinalCh => 'ꩄ',
            Cham::LetterFinalT => 'ꩅ',
            Cham::LetterFinalN => 'ꩆ',
            Cham::LetterFinalP => 'ꩇ',
            Cham::LetterFinalY => 'ꩈ',
            Cham::LetterFinalR => 'ꩉ',
            Cham::LetterFinalL => 'ꩊ',
            Cham::LetterFinalSs => 'ꩋ',
            Cham::ConsonantSignFinalM => 'ꩌ',
            Cham::ConsonantSignFinalH => 'ꩍ',
            Cham::DigitZero => '꩐',
            Cham::DigitOne => '꩑',
            Cham::DigitTwo => '꩒',
            Cham::DigitThree => '꩓',
            Cham::DigitFour => '꩔',
            Cham::DigitFive => '꩕',
            Cham::DigitSix => '꩖',
            Cham::DigitSeven => '꩗',
            Cham::DigitEight => '꩘',
            Cham::DigitNine => '꩙',
            Cham::PunctuationSpiral => '꩜',
            Cham::PunctuationDanda => '꩝',
            Cham::PunctuationDoubleDanda => '꩞',
        }
    }
}

impl std::convert::TryFrom<char> for Cham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꨀ' => Ok(Cham::LetterA),
            'ꨁ' => Ok(Cham::LetterI),
            'ꨂ' => Ok(Cham::LetterU),
            'ꨃ' => Ok(Cham::LetterE),
            'ꨄ' => Ok(Cham::LetterAi),
            'ꨅ' => Ok(Cham::LetterO),
            'ꨆ' => Ok(Cham::LetterKa),
            'ꨇ' => Ok(Cham::LetterKha),
            'ꨈ' => Ok(Cham::LetterGa),
            'ꨉ' => Ok(Cham::LetterGha),
            'ꨊ' => Ok(Cham::LetterNgue),
            'ꨋ' => Ok(Cham::LetterNga),
            'ꨌ' => Ok(Cham::LetterCha),
            'ꨍ' => Ok(Cham::LetterChha),
            'ꨎ' => Ok(Cham::LetterJa),
            'ꨏ' => Ok(Cham::LetterJha),
            'ꨐ' => Ok(Cham::LetterNhue),
            'ꨑ' => Ok(Cham::LetterNha),
            'ꨒ' => Ok(Cham::LetterNhja),
            'ꨓ' => Ok(Cham::LetterTa),
            'ꨔ' => Ok(Cham::LetterTha),
            'ꨕ' => Ok(Cham::LetterDa),
            'ꨖ' => Ok(Cham::LetterDha),
            'ꨗ' => Ok(Cham::LetterNue),
            'ꨘ' => Ok(Cham::LetterNa),
            'ꨙ' => Ok(Cham::LetterDda),
            'ꨚ' => Ok(Cham::LetterPa),
            'ꨛ' => Ok(Cham::LetterPpa),
            'ꨜ' => Ok(Cham::LetterPha),
            'ꨝ' => Ok(Cham::LetterBa),
            'ꨞ' => Ok(Cham::LetterBha),
            'ꨟ' => Ok(Cham::LetterMue),
            'ꨠ' => Ok(Cham::LetterMa),
            'ꨡ' => Ok(Cham::LetterBba),
            'ꨢ' => Ok(Cham::LetterYa),
            'ꨣ' => Ok(Cham::LetterRa),
            'ꨤ' => Ok(Cham::LetterLa),
            'ꨥ' => Ok(Cham::LetterVa),
            'ꨦ' => Ok(Cham::LetterSsa),
            'ꨧ' => Ok(Cham::LetterSa),
            'ꨨ' => Ok(Cham::LetterHa),
            'ꨩ' => Ok(Cham::VowelSignAa),
            'ꨪ' => Ok(Cham::VowelSignI),
            'ꨫ' => Ok(Cham::VowelSignIi),
            'ꨬ' => Ok(Cham::VowelSignEi),
            'ꨭ' => Ok(Cham::VowelSignU),
            'ꨮ' => Ok(Cham::VowelSignOe),
            'ꨯ' => Ok(Cham::VowelSignO),
            'ꨰ' => Ok(Cham::VowelSignAi),
            'ꨱ' => Ok(Cham::VowelSignAu),
            'ꨲ' => Ok(Cham::VowelSignUe),
            'ꨳ' => Ok(Cham::ConsonantSignYa),
            'ꨴ' => Ok(Cham::ConsonantSignRa),
            'ꨵ' => Ok(Cham::ConsonantSignLa),
            'ꨶ' => Ok(Cham::ConsonantSignWa),
            'ꩀ' => Ok(Cham::LetterFinalK),
            'ꩁ' => Ok(Cham::LetterFinalG),
            'ꩂ' => Ok(Cham::LetterFinalNg),
            'ꩃ' => Ok(Cham::ConsonantSignFinalNg),
            'ꩄ' => Ok(Cham::LetterFinalCh),
            'ꩅ' => Ok(Cham::LetterFinalT),
            'ꩆ' => Ok(Cham::LetterFinalN),
            'ꩇ' => Ok(Cham::LetterFinalP),
            'ꩈ' => Ok(Cham::LetterFinalY),
            'ꩉ' => Ok(Cham::LetterFinalR),
            'ꩊ' => Ok(Cham::LetterFinalL),
            'ꩋ' => Ok(Cham::LetterFinalSs),
            'ꩌ' => Ok(Cham::ConsonantSignFinalM),
            'ꩍ' => Ok(Cham::ConsonantSignFinalH),
            '꩐' => Ok(Cham::DigitZero),
            '꩑' => Ok(Cham::DigitOne),
            '꩒' => Ok(Cham::DigitTwo),
            '꩓' => Ok(Cham::DigitThree),
            '꩔' => Ok(Cham::DigitFour),
            '꩕' => Ok(Cham::DigitFive),
            '꩖' => Ok(Cham::DigitSix),
            '꩗' => Ok(Cham::DigitSeven),
            '꩘' => Ok(Cham::DigitEight),
            '꩙' => Ok(Cham::DigitNine),
            '꩜' => Ok(Cham::PunctuationSpiral),
            '꩝' => Ok(Cham::PunctuationDanda),
            '꩞' => Ok(Cham::PunctuationDoubleDanda),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Cham {
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

impl std::convert::TryFrom<u32> for Cham {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Cham {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Cham {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Cham::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Cham{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
