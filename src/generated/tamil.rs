
/// An enum to represent all characters in the Tamil block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tamil {
    /// \u{b82}: 'ஂ'
    SignAnusvara,
    /// \u{b83}: 'ஃ'
    SignVisarga,
    /// \u{b85}: 'அ'
    LetterA,
    /// \u{b86}: 'ஆ'
    LetterAa,
    /// \u{b87}: 'இ'
    LetterI,
    /// \u{b88}: 'ஈ'
    LetterIi,
    /// \u{b89}: 'உ'
    LetterU,
    /// \u{b8a}: 'ஊ'
    LetterUu,
    /// \u{b8e}: 'எ'
    LetterE,
    /// \u{b8f}: 'ஏ'
    LetterEe,
    /// \u{b90}: 'ஐ'
    LetterAi,
    /// \u{b92}: 'ஒ'
    LetterO,
    /// \u{b93}: 'ஓ'
    LetterOo,
    /// \u{b94}: 'ஔ'
    LetterAu,
    /// \u{b95}: 'க'
    LetterKa,
    /// \u{b99}: 'ங'
    LetterNga,
    /// \u{b9a}: 'ச'
    LetterCa,
    /// \u{b9c}: 'ஜ'
    LetterJa,
    /// \u{b9e}: 'ஞ'
    LetterNya,
    /// \u{b9f}: 'ட'
    LetterTta,
    /// \u{ba3}: 'ண'
    LetterNna,
    /// \u{ba4}: 'த'
    LetterTa,
    /// \u{ba8}: 'ந'
    LetterNa,
    /// \u{ba9}: 'ன'
    LetterNnna,
    /// \u{baa}: 'ப'
    LetterPa,
    /// \u{bae}: 'ம'
    LetterMa,
    /// \u{baf}: 'ய'
    LetterYa,
    /// \u{bb0}: 'ர'
    LetterRa,
    /// \u{bb1}: 'ற'
    LetterRra,
    /// \u{bb2}: 'ல'
    LetterLa,
    /// \u{bb3}: 'ள'
    LetterLla,
    /// \u{bb4}: 'ழ'
    LetterLlla,
    /// \u{bb5}: 'வ'
    LetterVa,
    /// \u{bb6}: 'ஶ'
    LetterSha,
    /// \u{bb7}: 'ஷ'
    LetterSsa,
    /// \u{bb8}: 'ஸ'
    LetterSa,
    /// \u{bb9}: 'ஹ'
    LetterHa,
    /// \u{bbe}: 'ா'
    VowelSignAa,
    /// \u{bbf}: 'ி'
    VowelSignI,
    /// \u{bc0}: 'ீ'
    VowelSignIi,
    /// \u{bc1}: 'ு'
    VowelSignU,
    /// \u{bc2}: 'ூ'
    VowelSignUu,
    /// \u{bc6}: 'ெ'
    VowelSignE,
    /// \u{bc7}: 'ே'
    VowelSignEe,
    /// \u{bc8}: 'ை'
    VowelSignAi,
    /// \u{bca}: 'ொ'
    VowelSignO,
    /// \u{bcb}: 'ோ'
    VowelSignOo,
    /// \u{bcc}: 'ௌ'
    VowelSignAu,
    /// \u{bcd}: '்'
    SignVirama,
    /// \u{bd0}: 'ௐ'
    Om,
    /// \u{bd7}: 'ௗ'
    AuLengthMark,
    /// \u{be6}: '௦'
    DigitZero,
    /// \u{be7}: '௧'
    DigitOne,
    /// \u{be8}: '௨'
    DigitTwo,
    /// \u{be9}: '௩'
    DigitThree,
    /// \u{bea}: '௪'
    DigitFour,
    /// \u{beb}: '௫'
    DigitFive,
    /// \u{bec}: '௬'
    DigitSix,
    /// \u{bed}: '௭'
    DigitSeven,
    /// \u{bee}: '௮'
    DigitEight,
    /// \u{bef}: '௯'
    DigitNine,
    /// \u{bf0}: '௰'
    NumberTen,
    /// \u{bf1}: '௱'
    NumberOneHundred,
    /// \u{bf2}: '௲'
    NumberOneThousand,
    /// \u{bf3}: '௳'
    DaySign,
    /// \u{bf4}: '௴'
    MonthSign,
    /// \u{bf5}: '௵'
    YearSign,
    /// \u{bf6}: '௶'
    DebitSign,
    /// \u{bf7}: '௷'
    CreditSign,
    /// \u{bf8}: '௸'
    AsAboveSign,
    /// \u{bf9}: '௹'
    RupeeSign,
    /// \u{bfa}: '௺'
    NumberSign,
}

impl Into<char> for Tamil {
    fn into(self) -> char {
        match self {
            Tamil::SignAnusvara => 'ஂ',
            Tamil::SignVisarga => 'ஃ',
            Tamil::LetterA => 'அ',
            Tamil::LetterAa => 'ஆ',
            Tamil::LetterI => 'இ',
            Tamil::LetterIi => 'ஈ',
            Tamil::LetterU => 'உ',
            Tamil::LetterUu => 'ஊ',
            Tamil::LetterE => 'எ',
            Tamil::LetterEe => 'ஏ',
            Tamil::LetterAi => 'ஐ',
            Tamil::LetterO => 'ஒ',
            Tamil::LetterOo => 'ஓ',
            Tamil::LetterAu => 'ஔ',
            Tamil::LetterKa => 'க',
            Tamil::LetterNga => 'ங',
            Tamil::LetterCa => 'ச',
            Tamil::LetterJa => 'ஜ',
            Tamil::LetterNya => 'ஞ',
            Tamil::LetterTta => 'ட',
            Tamil::LetterNna => 'ண',
            Tamil::LetterTa => 'த',
            Tamil::LetterNa => 'ந',
            Tamil::LetterNnna => 'ன',
            Tamil::LetterPa => 'ப',
            Tamil::LetterMa => 'ம',
            Tamil::LetterYa => 'ய',
            Tamil::LetterRa => 'ர',
            Tamil::LetterRra => 'ற',
            Tamil::LetterLa => 'ல',
            Tamil::LetterLla => 'ள',
            Tamil::LetterLlla => 'ழ',
            Tamil::LetterVa => 'வ',
            Tamil::LetterSha => 'ஶ',
            Tamil::LetterSsa => 'ஷ',
            Tamil::LetterSa => 'ஸ',
            Tamil::LetterHa => 'ஹ',
            Tamil::VowelSignAa => 'ா',
            Tamil::VowelSignI => 'ி',
            Tamil::VowelSignIi => 'ீ',
            Tamil::VowelSignU => 'ு',
            Tamil::VowelSignUu => 'ூ',
            Tamil::VowelSignE => 'ெ',
            Tamil::VowelSignEe => 'ே',
            Tamil::VowelSignAi => 'ை',
            Tamil::VowelSignO => 'ொ',
            Tamil::VowelSignOo => 'ோ',
            Tamil::VowelSignAu => 'ௌ',
            Tamil::SignVirama => '்',
            Tamil::Om => 'ௐ',
            Tamil::AuLengthMark => 'ௗ',
            Tamil::DigitZero => '௦',
            Tamil::DigitOne => '௧',
            Tamil::DigitTwo => '௨',
            Tamil::DigitThree => '௩',
            Tamil::DigitFour => '௪',
            Tamil::DigitFive => '௫',
            Tamil::DigitSix => '௬',
            Tamil::DigitSeven => '௭',
            Tamil::DigitEight => '௮',
            Tamil::DigitNine => '௯',
            Tamil::NumberTen => '௰',
            Tamil::NumberOneHundred => '௱',
            Tamil::NumberOneThousand => '௲',
            Tamil::DaySign => '௳',
            Tamil::MonthSign => '௴',
            Tamil::YearSign => '௵',
            Tamil::DebitSign => '௶',
            Tamil::CreditSign => '௷',
            Tamil::AsAboveSign => '௸',
            Tamil::RupeeSign => '௹',
            Tamil::NumberSign => '௺',
        }
    }
}

impl std::convert::TryFrom<char> for Tamil {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ஂ' => Ok(Tamil::SignAnusvara),
            'ஃ' => Ok(Tamil::SignVisarga),
            'அ' => Ok(Tamil::LetterA),
            'ஆ' => Ok(Tamil::LetterAa),
            'இ' => Ok(Tamil::LetterI),
            'ஈ' => Ok(Tamil::LetterIi),
            'உ' => Ok(Tamil::LetterU),
            'ஊ' => Ok(Tamil::LetterUu),
            'எ' => Ok(Tamil::LetterE),
            'ஏ' => Ok(Tamil::LetterEe),
            'ஐ' => Ok(Tamil::LetterAi),
            'ஒ' => Ok(Tamil::LetterO),
            'ஓ' => Ok(Tamil::LetterOo),
            'ஔ' => Ok(Tamil::LetterAu),
            'க' => Ok(Tamil::LetterKa),
            'ங' => Ok(Tamil::LetterNga),
            'ச' => Ok(Tamil::LetterCa),
            'ஜ' => Ok(Tamil::LetterJa),
            'ஞ' => Ok(Tamil::LetterNya),
            'ட' => Ok(Tamil::LetterTta),
            'ண' => Ok(Tamil::LetterNna),
            'த' => Ok(Tamil::LetterTa),
            'ந' => Ok(Tamil::LetterNa),
            'ன' => Ok(Tamil::LetterNnna),
            'ப' => Ok(Tamil::LetterPa),
            'ம' => Ok(Tamil::LetterMa),
            'ய' => Ok(Tamil::LetterYa),
            'ர' => Ok(Tamil::LetterRa),
            'ற' => Ok(Tamil::LetterRra),
            'ல' => Ok(Tamil::LetterLa),
            'ள' => Ok(Tamil::LetterLla),
            'ழ' => Ok(Tamil::LetterLlla),
            'வ' => Ok(Tamil::LetterVa),
            'ஶ' => Ok(Tamil::LetterSha),
            'ஷ' => Ok(Tamil::LetterSsa),
            'ஸ' => Ok(Tamil::LetterSa),
            'ஹ' => Ok(Tamil::LetterHa),
            'ா' => Ok(Tamil::VowelSignAa),
            'ி' => Ok(Tamil::VowelSignI),
            'ீ' => Ok(Tamil::VowelSignIi),
            'ு' => Ok(Tamil::VowelSignU),
            'ூ' => Ok(Tamil::VowelSignUu),
            'ெ' => Ok(Tamil::VowelSignE),
            'ே' => Ok(Tamil::VowelSignEe),
            'ை' => Ok(Tamil::VowelSignAi),
            'ொ' => Ok(Tamil::VowelSignO),
            'ோ' => Ok(Tamil::VowelSignOo),
            'ௌ' => Ok(Tamil::VowelSignAu),
            '்' => Ok(Tamil::SignVirama),
            'ௐ' => Ok(Tamil::Om),
            'ௗ' => Ok(Tamil::AuLengthMark),
            '௦' => Ok(Tamil::DigitZero),
            '௧' => Ok(Tamil::DigitOne),
            '௨' => Ok(Tamil::DigitTwo),
            '௩' => Ok(Tamil::DigitThree),
            '௪' => Ok(Tamil::DigitFour),
            '௫' => Ok(Tamil::DigitFive),
            '௬' => Ok(Tamil::DigitSix),
            '௭' => Ok(Tamil::DigitSeven),
            '௮' => Ok(Tamil::DigitEight),
            '௯' => Ok(Tamil::DigitNine),
            '௰' => Ok(Tamil::NumberTen),
            '௱' => Ok(Tamil::NumberOneHundred),
            '௲' => Ok(Tamil::NumberOneThousand),
            '௳' => Ok(Tamil::DaySign),
            '௴' => Ok(Tamil::MonthSign),
            '௵' => Ok(Tamil::YearSign),
            '௶' => Ok(Tamil::DebitSign),
            '௷' => Ok(Tamil::CreditSign),
            '௸' => Ok(Tamil::AsAboveSign),
            '௹' => Ok(Tamil::RupeeSign),
            '௺' => Ok(Tamil::NumberSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tamil {
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

impl std::convert::TryFrom<u32> for Tamil {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tamil {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tamil {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tamil::SignAnusvara
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Tamil{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
