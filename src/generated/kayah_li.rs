
/// An enum to represent all characters in the KayahLi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KayahLi {
    /// \u{a900}: '꤀'
    DigitZero,
    /// \u{a901}: '꤁'
    DigitOne,
    /// \u{a902}: '꤂'
    DigitTwo,
    /// \u{a903}: '꤃'
    DigitThree,
    /// \u{a904}: '꤄'
    DigitFour,
    /// \u{a905}: '꤅'
    DigitFive,
    /// \u{a906}: '꤆'
    DigitSix,
    /// \u{a907}: '꤇'
    DigitSeven,
    /// \u{a908}: '꤈'
    DigitEight,
    /// \u{a909}: '꤉'
    DigitNine,
    /// \u{a90a}: 'ꤊ'
    LetterKa,
    /// \u{a90b}: 'ꤋ'
    LetterKha,
    /// \u{a90c}: 'ꤌ'
    LetterGa,
    /// \u{a90d}: 'ꤍ'
    LetterNga,
    /// \u{a90e}: 'ꤎ'
    LetterSa,
    /// \u{a90f}: 'ꤏ'
    LetterSha,
    /// \u{a910}: 'ꤐ'
    LetterZa,
    /// \u{a911}: 'ꤑ'
    LetterNya,
    /// \u{a912}: 'ꤒ'
    LetterTa,
    /// \u{a913}: 'ꤓ'
    LetterHta,
    /// \u{a914}: 'ꤔ'
    LetterNa,
    /// \u{a915}: 'ꤕ'
    LetterPa,
    /// \u{a916}: 'ꤖ'
    LetterPha,
    /// \u{a917}: 'ꤗ'
    LetterMa,
    /// \u{a918}: 'ꤘ'
    LetterDa,
    /// \u{a919}: 'ꤙ'
    LetterBa,
    /// \u{a91a}: 'ꤚ'
    LetterRa,
    /// \u{a91b}: 'ꤛ'
    LetterYa,
    /// \u{a91c}: 'ꤜ'
    LetterLa,
    /// \u{a91d}: 'ꤝ'
    LetterWa,
    /// \u{a91e}: 'ꤞ'
    LetterTha,
    /// \u{a91f}: 'ꤟ'
    LetterHa,
    /// \u{a920}: 'ꤠ'
    LetterVa,
    /// \u{a921}: 'ꤡ'
    LetterCa,
    /// \u{a922}: 'ꤢ'
    LetterA,
    /// \u{a923}: 'ꤣ'
    LetterOe,
    /// \u{a924}: 'ꤤ'
    LetterI,
    /// \u{a925}: 'ꤥ'
    LetterOo,
    /// \u{a926}: 'ꤦ'
    VowelUe,
    /// \u{a927}: 'ꤧ'
    VowelE,
    /// \u{a928}: 'ꤨ'
    VowelU,
    /// \u{a929}: 'ꤩ'
    VowelEe,
    /// \u{a92a}: 'ꤪ'
    VowelO,
    /// \u{a92b}: '꤫'
    TonePlophu,
    /// \u{a92c}: '꤬'
    ToneCalya,
    /// \u{a92d}: '꤭'
    ToneCalyaPlophu,
    /// \u{a92e}: '꤮'
    SignCwi,
}

impl Into<char> for KayahLi {
    fn into(self) -> char {
        match self {
            KayahLi::DigitZero => '꤀',
            KayahLi::DigitOne => '꤁',
            KayahLi::DigitTwo => '꤂',
            KayahLi::DigitThree => '꤃',
            KayahLi::DigitFour => '꤄',
            KayahLi::DigitFive => '꤅',
            KayahLi::DigitSix => '꤆',
            KayahLi::DigitSeven => '꤇',
            KayahLi::DigitEight => '꤈',
            KayahLi::DigitNine => '꤉',
            KayahLi::LetterKa => 'ꤊ',
            KayahLi::LetterKha => 'ꤋ',
            KayahLi::LetterGa => 'ꤌ',
            KayahLi::LetterNga => 'ꤍ',
            KayahLi::LetterSa => 'ꤎ',
            KayahLi::LetterSha => 'ꤏ',
            KayahLi::LetterZa => 'ꤐ',
            KayahLi::LetterNya => 'ꤑ',
            KayahLi::LetterTa => 'ꤒ',
            KayahLi::LetterHta => 'ꤓ',
            KayahLi::LetterNa => 'ꤔ',
            KayahLi::LetterPa => 'ꤕ',
            KayahLi::LetterPha => 'ꤖ',
            KayahLi::LetterMa => 'ꤗ',
            KayahLi::LetterDa => 'ꤘ',
            KayahLi::LetterBa => 'ꤙ',
            KayahLi::LetterRa => 'ꤚ',
            KayahLi::LetterYa => 'ꤛ',
            KayahLi::LetterLa => 'ꤜ',
            KayahLi::LetterWa => 'ꤝ',
            KayahLi::LetterTha => 'ꤞ',
            KayahLi::LetterHa => 'ꤟ',
            KayahLi::LetterVa => 'ꤠ',
            KayahLi::LetterCa => 'ꤡ',
            KayahLi::LetterA => 'ꤢ',
            KayahLi::LetterOe => 'ꤣ',
            KayahLi::LetterI => 'ꤤ',
            KayahLi::LetterOo => 'ꤥ',
            KayahLi::VowelUe => 'ꤦ',
            KayahLi::VowelE => 'ꤧ',
            KayahLi::VowelU => 'ꤨ',
            KayahLi::VowelEe => 'ꤩ',
            KayahLi::VowelO => 'ꤪ',
            KayahLi::TonePlophu => '꤫',
            KayahLi::ToneCalya => '꤬',
            KayahLi::ToneCalyaPlophu => '꤭',
            KayahLi::SignCwi => '꤮',
        }
    }
}

impl std::convert::TryFrom<char> for KayahLi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '꤀' => Ok(KayahLi::DigitZero),
            '꤁' => Ok(KayahLi::DigitOne),
            '꤂' => Ok(KayahLi::DigitTwo),
            '꤃' => Ok(KayahLi::DigitThree),
            '꤄' => Ok(KayahLi::DigitFour),
            '꤅' => Ok(KayahLi::DigitFive),
            '꤆' => Ok(KayahLi::DigitSix),
            '꤇' => Ok(KayahLi::DigitSeven),
            '꤈' => Ok(KayahLi::DigitEight),
            '꤉' => Ok(KayahLi::DigitNine),
            'ꤊ' => Ok(KayahLi::LetterKa),
            'ꤋ' => Ok(KayahLi::LetterKha),
            'ꤌ' => Ok(KayahLi::LetterGa),
            'ꤍ' => Ok(KayahLi::LetterNga),
            'ꤎ' => Ok(KayahLi::LetterSa),
            'ꤏ' => Ok(KayahLi::LetterSha),
            'ꤐ' => Ok(KayahLi::LetterZa),
            'ꤑ' => Ok(KayahLi::LetterNya),
            'ꤒ' => Ok(KayahLi::LetterTa),
            'ꤓ' => Ok(KayahLi::LetterHta),
            'ꤔ' => Ok(KayahLi::LetterNa),
            'ꤕ' => Ok(KayahLi::LetterPa),
            'ꤖ' => Ok(KayahLi::LetterPha),
            'ꤗ' => Ok(KayahLi::LetterMa),
            'ꤘ' => Ok(KayahLi::LetterDa),
            'ꤙ' => Ok(KayahLi::LetterBa),
            'ꤚ' => Ok(KayahLi::LetterRa),
            'ꤛ' => Ok(KayahLi::LetterYa),
            'ꤜ' => Ok(KayahLi::LetterLa),
            'ꤝ' => Ok(KayahLi::LetterWa),
            'ꤞ' => Ok(KayahLi::LetterTha),
            'ꤟ' => Ok(KayahLi::LetterHa),
            'ꤠ' => Ok(KayahLi::LetterVa),
            'ꤡ' => Ok(KayahLi::LetterCa),
            'ꤢ' => Ok(KayahLi::LetterA),
            'ꤣ' => Ok(KayahLi::LetterOe),
            'ꤤ' => Ok(KayahLi::LetterI),
            'ꤥ' => Ok(KayahLi::LetterOo),
            'ꤦ' => Ok(KayahLi::VowelUe),
            'ꤧ' => Ok(KayahLi::VowelE),
            'ꤨ' => Ok(KayahLi::VowelU),
            'ꤩ' => Ok(KayahLi::VowelEe),
            'ꤪ' => Ok(KayahLi::VowelO),
            '꤫' => Ok(KayahLi::TonePlophu),
            '꤬' => Ok(KayahLi::ToneCalya),
            '꤭' => Ok(KayahLi::ToneCalyaPlophu),
            '꤮' => Ok(KayahLi::SignCwi),
            _ => Err(()),
        }
    }
}

impl Into<u32> for KayahLi {
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

impl std::convert::TryFrom<u32> for KayahLi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for KayahLi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl KayahLi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        KayahLi::DigitZero
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            KayahLi::DigitZero => "kayah li digit zero",
            KayahLi::DigitOne => "kayah li digit one",
            KayahLi::DigitTwo => "kayah li digit two",
            KayahLi::DigitThree => "kayah li digit three",
            KayahLi::DigitFour => "kayah li digit four",
            KayahLi::DigitFive => "kayah li digit five",
            KayahLi::DigitSix => "kayah li digit six",
            KayahLi::DigitSeven => "kayah li digit seven",
            KayahLi::DigitEight => "kayah li digit eight",
            KayahLi::DigitNine => "kayah li digit nine",
            KayahLi::LetterKa => "kayah li letter ka",
            KayahLi::LetterKha => "kayah li letter kha",
            KayahLi::LetterGa => "kayah li letter ga",
            KayahLi::LetterNga => "kayah li letter nga",
            KayahLi::LetterSa => "kayah li letter sa",
            KayahLi::LetterSha => "kayah li letter sha",
            KayahLi::LetterZa => "kayah li letter za",
            KayahLi::LetterNya => "kayah li letter nya",
            KayahLi::LetterTa => "kayah li letter ta",
            KayahLi::LetterHta => "kayah li letter hta",
            KayahLi::LetterNa => "kayah li letter na",
            KayahLi::LetterPa => "kayah li letter pa",
            KayahLi::LetterPha => "kayah li letter pha",
            KayahLi::LetterMa => "kayah li letter ma",
            KayahLi::LetterDa => "kayah li letter da",
            KayahLi::LetterBa => "kayah li letter ba",
            KayahLi::LetterRa => "kayah li letter ra",
            KayahLi::LetterYa => "kayah li letter ya",
            KayahLi::LetterLa => "kayah li letter la",
            KayahLi::LetterWa => "kayah li letter wa",
            KayahLi::LetterTha => "kayah li letter tha",
            KayahLi::LetterHa => "kayah li letter ha",
            KayahLi::LetterVa => "kayah li letter va",
            KayahLi::LetterCa => "kayah li letter ca",
            KayahLi::LetterA => "kayah li letter a",
            KayahLi::LetterOe => "kayah li letter oe",
            KayahLi::LetterI => "kayah li letter i",
            KayahLi::LetterOo => "kayah li letter oo",
            KayahLi::VowelUe => "kayah li vowel ue",
            KayahLi::VowelE => "kayah li vowel e",
            KayahLi::VowelU => "kayah li vowel u",
            KayahLi::VowelEe => "kayah li vowel ee",
            KayahLi::VowelO => "kayah li vowel o",
            KayahLi::TonePlophu => "kayah li tone plophu",
            KayahLi::ToneCalya => "kayah li tone calya",
            KayahLi::ToneCalyaPlophu => "kayah li tone calya plophu",
            KayahLi::SignCwi => "kayah li sign cwi",
        }
    }
}
