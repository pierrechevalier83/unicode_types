
/// An enum to represent all characters in the Sundanese block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sundanese {
    /// \u{1b80}: 'ᮀ'
    SignPanyecek,
    /// \u{1b81}: 'ᮁ'
    SignPanglayar,
    /// \u{1b82}: 'ᮂ'
    SignPangwisad,
    /// \u{1b83}: 'ᮃ'
    LetterA,
    /// \u{1b84}: 'ᮄ'
    LetterI,
    /// \u{1b85}: 'ᮅ'
    LetterU,
    /// \u{1b86}: 'ᮆ'
    LetterAe,
    /// \u{1b87}: 'ᮇ'
    LetterO,
    /// \u{1b88}: 'ᮈ'
    LetterE,
    /// \u{1b89}: 'ᮉ'
    LetterEu,
    /// \u{1b8a}: 'ᮊ'
    LetterKa,
    /// \u{1b8b}: 'ᮋ'
    LetterQa,
    /// \u{1b8c}: 'ᮌ'
    LetterGa,
    /// \u{1b8d}: 'ᮍ'
    LetterNga,
    /// \u{1b8e}: 'ᮎ'
    LetterCa,
    /// \u{1b8f}: 'ᮏ'
    LetterJa,
    /// \u{1b90}: 'ᮐ'
    LetterZa,
    /// \u{1b91}: 'ᮑ'
    LetterNya,
    /// \u{1b92}: 'ᮒ'
    LetterTa,
    /// \u{1b93}: 'ᮓ'
    LetterDa,
    /// \u{1b94}: 'ᮔ'
    LetterNa,
    /// \u{1b95}: 'ᮕ'
    LetterPa,
    /// \u{1b96}: 'ᮖ'
    LetterFa,
    /// \u{1b97}: 'ᮗ'
    LetterVa,
    /// \u{1b98}: 'ᮘ'
    LetterBa,
    /// \u{1b99}: 'ᮙ'
    LetterMa,
    /// \u{1b9a}: 'ᮚ'
    LetterYa,
    /// \u{1b9b}: 'ᮛ'
    LetterRa,
    /// \u{1b9c}: 'ᮜ'
    LetterLa,
    /// \u{1b9d}: 'ᮝ'
    LetterWa,
    /// \u{1b9e}: 'ᮞ'
    LetterSa,
    /// \u{1b9f}: 'ᮟ'
    LetterXa,
    /// \u{1ba0}: 'ᮠ'
    LetterHa,
    /// \u{1ba1}: 'ᮡ'
    ConsonantSignPamingkal,
    /// \u{1ba2}: 'ᮢ'
    ConsonantSignPanyakra,
    /// \u{1ba3}: 'ᮣ'
    ConsonantSignPanyiku,
    /// \u{1ba4}: 'ᮤ'
    VowelSignPanghulu,
    /// \u{1ba5}: 'ᮥ'
    VowelSignPanyuku,
    /// \u{1ba6}: 'ᮦ'
    VowelSignPanaelaeng,
    /// \u{1ba7}: 'ᮧ'
    VowelSignPanolong,
    /// \u{1ba8}: 'ᮨ'
    VowelSignPamepet,
    /// \u{1ba9}: 'ᮩ'
    VowelSignPaneuleung,
    /// \u{1baa}: '᮪'
    SignPamaaeh,
    /// \u{1bab}: '᮫'
    SignVirama,
    /// \u{1bac}: 'ᮬ'
    ConsonantSignPasanganMa,
    /// \u{1bad}: 'ᮭ'
    ConsonantSignPasanganWa,
    /// \u{1bae}: 'ᮮ'
    LetterKha,
    /// \u{1baf}: 'ᮯ'
    LetterSya,
    /// \u{1bb0}: '᮰'
    DigitZero,
    /// \u{1bb1}: '᮱'
    DigitOne,
    /// \u{1bb2}: '᮲'
    DigitTwo,
    /// \u{1bb3}: '᮳'
    DigitThree,
    /// \u{1bb4}: '᮴'
    DigitFour,
    /// \u{1bb5}: '᮵'
    DigitFive,
    /// \u{1bb6}: '᮶'
    DigitSix,
    /// \u{1bb7}: '᮷'
    DigitSeven,
    /// \u{1bb8}: '᮸'
    DigitEight,
    /// \u{1bb9}: '᮹'
    DigitNine,
    /// \u{1bba}: 'ᮺ'
    Avagraha,
    /// \u{1bbb}: 'ᮻ'
    LetterReu,
    /// \u{1bbc}: 'ᮼ'
    LetterLeu,
    /// \u{1bbd}: 'ᮽ'
    LetterBha,
    /// \u{1bbe}: 'ᮾ'
    LetterFinalK,
}

impl Into<char> for Sundanese {
    fn into(self) -> char {
        match self {
            Sundanese::SignPanyecek => 'ᮀ',
            Sundanese::SignPanglayar => 'ᮁ',
            Sundanese::SignPangwisad => 'ᮂ',
            Sundanese::LetterA => 'ᮃ',
            Sundanese::LetterI => 'ᮄ',
            Sundanese::LetterU => 'ᮅ',
            Sundanese::LetterAe => 'ᮆ',
            Sundanese::LetterO => 'ᮇ',
            Sundanese::LetterE => 'ᮈ',
            Sundanese::LetterEu => 'ᮉ',
            Sundanese::LetterKa => 'ᮊ',
            Sundanese::LetterQa => 'ᮋ',
            Sundanese::LetterGa => 'ᮌ',
            Sundanese::LetterNga => 'ᮍ',
            Sundanese::LetterCa => 'ᮎ',
            Sundanese::LetterJa => 'ᮏ',
            Sundanese::LetterZa => 'ᮐ',
            Sundanese::LetterNya => 'ᮑ',
            Sundanese::LetterTa => 'ᮒ',
            Sundanese::LetterDa => 'ᮓ',
            Sundanese::LetterNa => 'ᮔ',
            Sundanese::LetterPa => 'ᮕ',
            Sundanese::LetterFa => 'ᮖ',
            Sundanese::LetterVa => 'ᮗ',
            Sundanese::LetterBa => 'ᮘ',
            Sundanese::LetterMa => 'ᮙ',
            Sundanese::LetterYa => 'ᮚ',
            Sundanese::LetterRa => 'ᮛ',
            Sundanese::LetterLa => 'ᮜ',
            Sundanese::LetterWa => 'ᮝ',
            Sundanese::LetterSa => 'ᮞ',
            Sundanese::LetterXa => 'ᮟ',
            Sundanese::LetterHa => 'ᮠ',
            Sundanese::ConsonantSignPamingkal => 'ᮡ',
            Sundanese::ConsonantSignPanyakra => 'ᮢ',
            Sundanese::ConsonantSignPanyiku => 'ᮣ',
            Sundanese::VowelSignPanghulu => 'ᮤ',
            Sundanese::VowelSignPanyuku => 'ᮥ',
            Sundanese::VowelSignPanaelaeng => 'ᮦ',
            Sundanese::VowelSignPanolong => 'ᮧ',
            Sundanese::VowelSignPamepet => 'ᮨ',
            Sundanese::VowelSignPaneuleung => 'ᮩ',
            Sundanese::SignPamaaeh => '᮪',
            Sundanese::SignVirama => '᮫',
            Sundanese::ConsonantSignPasanganMa => 'ᮬ',
            Sundanese::ConsonantSignPasanganWa => 'ᮭ',
            Sundanese::LetterKha => 'ᮮ',
            Sundanese::LetterSya => 'ᮯ',
            Sundanese::DigitZero => '᮰',
            Sundanese::DigitOne => '᮱',
            Sundanese::DigitTwo => '᮲',
            Sundanese::DigitThree => '᮳',
            Sundanese::DigitFour => '᮴',
            Sundanese::DigitFive => '᮵',
            Sundanese::DigitSix => '᮶',
            Sundanese::DigitSeven => '᮷',
            Sundanese::DigitEight => '᮸',
            Sundanese::DigitNine => '᮹',
            Sundanese::Avagraha => 'ᮺ',
            Sundanese::LetterReu => 'ᮻ',
            Sundanese::LetterLeu => 'ᮼ',
            Sundanese::LetterBha => 'ᮽ',
            Sundanese::LetterFinalK => 'ᮾ',
        }
    }
}

impl std::convert::TryFrom<char> for Sundanese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᮀ' => Ok(Sundanese::SignPanyecek),
            'ᮁ' => Ok(Sundanese::SignPanglayar),
            'ᮂ' => Ok(Sundanese::SignPangwisad),
            'ᮃ' => Ok(Sundanese::LetterA),
            'ᮄ' => Ok(Sundanese::LetterI),
            'ᮅ' => Ok(Sundanese::LetterU),
            'ᮆ' => Ok(Sundanese::LetterAe),
            'ᮇ' => Ok(Sundanese::LetterO),
            'ᮈ' => Ok(Sundanese::LetterE),
            'ᮉ' => Ok(Sundanese::LetterEu),
            'ᮊ' => Ok(Sundanese::LetterKa),
            'ᮋ' => Ok(Sundanese::LetterQa),
            'ᮌ' => Ok(Sundanese::LetterGa),
            'ᮍ' => Ok(Sundanese::LetterNga),
            'ᮎ' => Ok(Sundanese::LetterCa),
            'ᮏ' => Ok(Sundanese::LetterJa),
            'ᮐ' => Ok(Sundanese::LetterZa),
            'ᮑ' => Ok(Sundanese::LetterNya),
            'ᮒ' => Ok(Sundanese::LetterTa),
            'ᮓ' => Ok(Sundanese::LetterDa),
            'ᮔ' => Ok(Sundanese::LetterNa),
            'ᮕ' => Ok(Sundanese::LetterPa),
            'ᮖ' => Ok(Sundanese::LetterFa),
            'ᮗ' => Ok(Sundanese::LetterVa),
            'ᮘ' => Ok(Sundanese::LetterBa),
            'ᮙ' => Ok(Sundanese::LetterMa),
            'ᮚ' => Ok(Sundanese::LetterYa),
            'ᮛ' => Ok(Sundanese::LetterRa),
            'ᮜ' => Ok(Sundanese::LetterLa),
            'ᮝ' => Ok(Sundanese::LetterWa),
            'ᮞ' => Ok(Sundanese::LetterSa),
            'ᮟ' => Ok(Sundanese::LetterXa),
            'ᮠ' => Ok(Sundanese::LetterHa),
            'ᮡ' => Ok(Sundanese::ConsonantSignPamingkal),
            'ᮢ' => Ok(Sundanese::ConsonantSignPanyakra),
            'ᮣ' => Ok(Sundanese::ConsonantSignPanyiku),
            'ᮤ' => Ok(Sundanese::VowelSignPanghulu),
            'ᮥ' => Ok(Sundanese::VowelSignPanyuku),
            'ᮦ' => Ok(Sundanese::VowelSignPanaelaeng),
            'ᮧ' => Ok(Sundanese::VowelSignPanolong),
            'ᮨ' => Ok(Sundanese::VowelSignPamepet),
            'ᮩ' => Ok(Sundanese::VowelSignPaneuleung),
            '᮪' => Ok(Sundanese::SignPamaaeh),
            '᮫' => Ok(Sundanese::SignVirama),
            'ᮬ' => Ok(Sundanese::ConsonantSignPasanganMa),
            'ᮭ' => Ok(Sundanese::ConsonantSignPasanganWa),
            'ᮮ' => Ok(Sundanese::LetterKha),
            'ᮯ' => Ok(Sundanese::LetterSya),
            '᮰' => Ok(Sundanese::DigitZero),
            '᮱' => Ok(Sundanese::DigitOne),
            '᮲' => Ok(Sundanese::DigitTwo),
            '᮳' => Ok(Sundanese::DigitThree),
            '᮴' => Ok(Sundanese::DigitFour),
            '᮵' => Ok(Sundanese::DigitFive),
            '᮶' => Ok(Sundanese::DigitSix),
            '᮷' => Ok(Sundanese::DigitSeven),
            '᮸' => Ok(Sundanese::DigitEight),
            '᮹' => Ok(Sundanese::DigitNine),
            'ᮺ' => Ok(Sundanese::Avagraha),
            'ᮻ' => Ok(Sundanese::LetterReu),
            'ᮼ' => Ok(Sundanese::LetterLeu),
            'ᮽ' => Ok(Sundanese::LetterBha),
            'ᮾ' => Ok(Sundanese::LetterFinalK),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Sundanese {
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

impl std::convert::TryFrom<u32> for Sundanese {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Sundanese {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Sundanese {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Sundanese::SignPanyecek
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Sundanese::SignPanyecek => "sundanese sign panyecek",
            Sundanese::SignPanglayar => "sundanese sign panglayar",
            Sundanese::SignPangwisad => "sundanese sign pangwisad",
            Sundanese::LetterA => "sundanese letter a",
            Sundanese::LetterI => "sundanese letter i",
            Sundanese::LetterU => "sundanese letter u",
            Sundanese::LetterAe => "sundanese letter ae",
            Sundanese::LetterO => "sundanese letter o",
            Sundanese::LetterE => "sundanese letter e",
            Sundanese::LetterEu => "sundanese letter eu",
            Sundanese::LetterKa => "sundanese letter ka",
            Sundanese::LetterQa => "sundanese letter qa",
            Sundanese::LetterGa => "sundanese letter ga",
            Sundanese::LetterNga => "sundanese letter nga",
            Sundanese::LetterCa => "sundanese letter ca",
            Sundanese::LetterJa => "sundanese letter ja",
            Sundanese::LetterZa => "sundanese letter za",
            Sundanese::LetterNya => "sundanese letter nya",
            Sundanese::LetterTa => "sundanese letter ta",
            Sundanese::LetterDa => "sundanese letter da",
            Sundanese::LetterNa => "sundanese letter na",
            Sundanese::LetterPa => "sundanese letter pa",
            Sundanese::LetterFa => "sundanese letter fa",
            Sundanese::LetterVa => "sundanese letter va",
            Sundanese::LetterBa => "sundanese letter ba",
            Sundanese::LetterMa => "sundanese letter ma",
            Sundanese::LetterYa => "sundanese letter ya",
            Sundanese::LetterRa => "sundanese letter ra",
            Sundanese::LetterLa => "sundanese letter la",
            Sundanese::LetterWa => "sundanese letter wa",
            Sundanese::LetterSa => "sundanese letter sa",
            Sundanese::LetterXa => "sundanese letter xa",
            Sundanese::LetterHa => "sundanese letter ha",
            Sundanese::ConsonantSignPamingkal => "sundanese consonant sign pamingkal",
            Sundanese::ConsonantSignPanyakra => "sundanese consonant sign panyakra",
            Sundanese::ConsonantSignPanyiku => "sundanese consonant sign panyiku",
            Sundanese::VowelSignPanghulu => "sundanese vowel sign panghulu",
            Sundanese::VowelSignPanyuku => "sundanese vowel sign panyuku",
            Sundanese::VowelSignPanaelaeng => "sundanese vowel sign panaelaeng",
            Sundanese::VowelSignPanolong => "sundanese vowel sign panolong",
            Sundanese::VowelSignPamepet => "sundanese vowel sign pamepet",
            Sundanese::VowelSignPaneuleung => "sundanese vowel sign paneuleung",
            Sundanese::SignPamaaeh => "sundanese sign pamaaeh",
            Sundanese::SignVirama => "sundanese sign virama",
            Sundanese::ConsonantSignPasanganMa => "sundanese consonant sign pasangan ma",
            Sundanese::ConsonantSignPasanganWa => "sundanese consonant sign pasangan wa",
            Sundanese::LetterKha => "sundanese letter kha",
            Sundanese::LetterSya => "sundanese letter sya",
            Sundanese::DigitZero => "sundanese digit zero",
            Sundanese::DigitOne => "sundanese digit one",
            Sundanese::DigitTwo => "sundanese digit two",
            Sundanese::DigitThree => "sundanese digit three",
            Sundanese::DigitFour => "sundanese digit four",
            Sundanese::DigitFive => "sundanese digit five",
            Sundanese::DigitSix => "sundanese digit six",
            Sundanese::DigitSeven => "sundanese digit seven",
            Sundanese::DigitEight => "sundanese digit eight",
            Sundanese::DigitNine => "sundanese digit nine",
            Sundanese::Avagraha => "sundanese avagraha",
            Sundanese::LetterReu => "sundanese letter reu",
            Sundanese::LetterLeu => "sundanese letter leu",
            Sundanese::LetterBha => "sundanese letter bha",
            Sundanese::LetterFinalK => "sundanese letter final k",
        }
    }
}
