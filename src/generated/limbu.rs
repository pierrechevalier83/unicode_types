
/// An enum to represent all characters in the Limbu block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Limbu {
    /// \u{1900}: 'ᤀ'
    VowelDashCarrierLetter,
    /// \u{1901}: 'ᤁ'
    LetterKa,
    /// \u{1902}: 'ᤂ'
    LetterKha,
    /// \u{1903}: 'ᤃ'
    LetterGa,
    /// \u{1904}: 'ᤄ'
    LetterGha,
    /// \u{1905}: 'ᤅ'
    LetterNga,
    /// \u{1906}: 'ᤆ'
    LetterCa,
    /// \u{1907}: 'ᤇ'
    LetterCha,
    /// \u{1908}: 'ᤈ'
    LetterJa,
    /// \u{1909}: 'ᤉ'
    LetterJha,
    /// \u{190a}: 'ᤊ'
    LetterYan,
    /// \u{190b}: 'ᤋ'
    LetterTa,
    /// \u{190c}: 'ᤌ'
    LetterTha,
    /// \u{190d}: 'ᤍ'
    LetterDa,
    /// \u{190e}: 'ᤎ'
    LetterDha,
    /// \u{190f}: 'ᤏ'
    LetterNa,
    /// \u{1910}: 'ᤐ'
    LetterPa,
    /// \u{1911}: 'ᤑ'
    LetterPha,
    /// \u{1912}: 'ᤒ'
    LetterBa,
    /// \u{1913}: 'ᤓ'
    LetterBha,
    /// \u{1914}: 'ᤔ'
    LetterMa,
    /// \u{1915}: 'ᤕ'
    LetterYa,
    /// \u{1916}: 'ᤖ'
    LetterRa,
    /// \u{1917}: 'ᤗ'
    LetterLa,
    /// \u{1918}: 'ᤘ'
    LetterWa,
    /// \u{1919}: 'ᤙ'
    LetterSha,
    /// \u{191a}: 'ᤚ'
    LetterSsa,
    /// \u{191b}: 'ᤛ'
    LetterSa,
    /// \u{191c}: 'ᤜ'
    LetterHa,
    /// \u{191d}: 'ᤝ'
    LetterGyan,
    /// \u{191e}: 'ᤞ'
    LetterTra,
    /// \u{1920}: 'ᤠ'
    VowelSignA,
    /// \u{1921}: 'ᤡ'
    VowelSignI,
    /// \u{1922}: 'ᤢ'
    VowelSignU,
    /// \u{1923}: 'ᤣ'
    VowelSignEe,
    /// \u{1924}: 'ᤤ'
    VowelSignAi,
    /// \u{1925}: 'ᤥ'
    VowelSignOo,
    /// \u{1926}: 'ᤦ'
    VowelSignAu,
    /// \u{1927}: 'ᤧ'
    VowelSignE,
    /// \u{1928}: 'ᤨ'
    VowelSignO,
    /// \u{1929}: 'ᤩ'
    SubjoinedLetterYa,
    /// \u{192a}: 'ᤪ'
    SubjoinedLetterRa,
    /// \u{192b}: 'ᤫ'
    SubjoinedLetterWa,
    /// \u{1930}: 'ᤰ'
    SmallLetterKa,
    /// \u{1931}: 'ᤱ'
    SmallLetterNga,
    /// \u{1932}: 'ᤲ'
    SmallLetterAnusvara,
    /// \u{1933}: 'ᤳ'
    SmallLetterTa,
    /// \u{1934}: 'ᤴ'
    SmallLetterNa,
    /// \u{1935}: 'ᤵ'
    SmallLetterPa,
    /// \u{1936}: 'ᤶ'
    SmallLetterMa,
    /// \u{1937}: 'ᤷ'
    SmallLetterRa,
    /// \u{1938}: 'ᤸ'
    SmallLetterLa,
    /// \u{1939}: '᤹'
    SignMukphreng,
    /// \u{193a}: '᤺'
    SignKemphreng,
    /// \u{193b}: '᤻'
    SignSaDashI,
    /// \u{1940}: '᥀'
    SignLoo,
    /// \u{1944}: '᥄'
    ExclamationMark,
    /// \u{1945}: '᥅'
    QuestionMark,
    /// \u{1946}: '᥆'
    DigitZero,
    /// \u{1947}: '᥇'
    DigitOne,
    /// \u{1948}: '᥈'
    DigitTwo,
    /// \u{1949}: '᥉'
    DigitThree,
    /// \u{194a}: '᥊'
    DigitFour,
    /// \u{194b}: '᥋'
    DigitFive,
    /// \u{194c}: '᥌'
    DigitSix,
    /// \u{194d}: '᥍'
    DigitSeven,
    /// \u{194e}: '᥎'
    DigitEight,
}

impl Into<char> for Limbu {
    fn into(self) -> char {
        match self {
            Limbu::VowelDashCarrierLetter => 'ᤀ',
            Limbu::LetterKa => 'ᤁ',
            Limbu::LetterKha => 'ᤂ',
            Limbu::LetterGa => 'ᤃ',
            Limbu::LetterGha => 'ᤄ',
            Limbu::LetterNga => 'ᤅ',
            Limbu::LetterCa => 'ᤆ',
            Limbu::LetterCha => 'ᤇ',
            Limbu::LetterJa => 'ᤈ',
            Limbu::LetterJha => 'ᤉ',
            Limbu::LetterYan => 'ᤊ',
            Limbu::LetterTa => 'ᤋ',
            Limbu::LetterTha => 'ᤌ',
            Limbu::LetterDa => 'ᤍ',
            Limbu::LetterDha => 'ᤎ',
            Limbu::LetterNa => 'ᤏ',
            Limbu::LetterPa => 'ᤐ',
            Limbu::LetterPha => 'ᤑ',
            Limbu::LetterBa => 'ᤒ',
            Limbu::LetterBha => 'ᤓ',
            Limbu::LetterMa => 'ᤔ',
            Limbu::LetterYa => 'ᤕ',
            Limbu::LetterRa => 'ᤖ',
            Limbu::LetterLa => 'ᤗ',
            Limbu::LetterWa => 'ᤘ',
            Limbu::LetterSha => 'ᤙ',
            Limbu::LetterSsa => 'ᤚ',
            Limbu::LetterSa => 'ᤛ',
            Limbu::LetterHa => 'ᤜ',
            Limbu::LetterGyan => 'ᤝ',
            Limbu::LetterTra => 'ᤞ',
            Limbu::VowelSignA => 'ᤠ',
            Limbu::VowelSignI => 'ᤡ',
            Limbu::VowelSignU => 'ᤢ',
            Limbu::VowelSignEe => 'ᤣ',
            Limbu::VowelSignAi => 'ᤤ',
            Limbu::VowelSignOo => 'ᤥ',
            Limbu::VowelSignAu => 'ᤦ',
            Limbu::VowelSignE => 'ᤧ',
            Limbu::VowelSignO => 'ᤨ',
            Limbu::SubjoinedLetterYa => 'ᤩ',
            Limbu::SubjoinedLetterRa => 'ᤪ',
            Limbu::SubjoinedLetterWa => 'ᤫ',
            Limbu::SmallLetterKa => 'ᤰ',
            Limbu::SmallLetterNga => 'ᤱ',
            Limbu::SmallLetterAnusvara => 'ᤲ',
            Limbu::SmallLetterTa => 'ᤳ',
            Limbu::SmallLetterNa => 'ᤴ',
            Limbu::SmallLetterPa => 'ᤵ',
            Limbu::SmallLetterMa => 'ᤶ',
            Limbu::SmallLetterRa => 'ᤷ',
            Limbu::SmallLetterLa => 'ᤸ',
            Limbu::SignMukphreng => '᤹',
            Limbu::SignKemphreng => '᤺',
            Limbu::SignSaDashI => '᤻',
            Limbu::SignLoo => '᥀',
            Limbu::ExclamationMark => '᥄',
            Limbu::QuestionMark => '᥅',
            Limbu::DigitZero => '᥆',
            Limbu::DigitOne => '᥇',
            Limbu::DigitTwo => '᥈',
            Limbu::DigitThree => '᥉',
            Limbu::DigitFour => '᥊',
            Limbu::DigitFive => '᥋',
            Limbu::DigitSix => '᥌',
            Limbu::DigitSeven => '᥍',
            Limbu::DigitEight => '᥎',
        }
    }
}

impl std::convert::TryFrom<char> for Limbu {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᤀ' => Ok(Limbu::VowelDashCarrierLetter),
            'ᤁ' => Ok(Limbu::LetterKa),
            'ᤂ' => Ok(Limbu::LetterKha),
            'ᤃ' => Ok(Limbu::LetterGa),
            'ᤄ' => Ok(Limbu::LetterGha),
            'ᤅ' => Ok(Limbu::LetterNga),
            'ᤆ' => Ok(Limbu::LetterCa),
            'ᤇ' => Ok(Limbu::LetterCha),
            'ᤈ' => Ok(Limbu::LetterJa),
            'ᤉ' => Ok(Limbu::LetterJha),
            'ᤊ' => Ok(Limbu::LetterYan),
            'ᤋ' => Ok(Limbu::LetterTa),
            'ᤌ' => Ok(Limbu::LetterTha),
            'ᤍ' => Ok(Limbu::LetterDa),
            'ᤎ' => Ok(Limbu::LetterDha),
            'ᤏ' => Ok(Limbu::LetterNa),
            'ᤐ' => Ok(Limbu::LetterPa),
            'ᤑ' => Ok(Limbu::LetterPha),
            'ᤒ' => Ok(Limbu::LetterBa),
            'ᤓ' => Ok(Limbu::LetterBha),
            'ᤔ' => Ok(Limbu::LetterMa),
            'ᤕ' => Ok(Limbu::LetterYa),
            'ᤖ' => Ok(Limbu::LetterRa),
            'ᤗ' => Ok(Limbu::LetterLa),
            'ᤘ' => Ok(Limbu::LetterWa),
            'ᤙ' => Ok(Limbu::LetterSha),
            'ᤚ' => Ok(Limbu::LetterSsa),
            'ᤛ' => Ok(Limbu::LetterSa),
            'ᤜ' => Ok(Limbu::LetterHa),
            'ᤝ' => Ok(Limbu::LetterGyan),
            'ᤞ' => Ok(Limbu::LetterTra),
            'ᤠ' => Ok(Limbu::VowelSignA),
            'ᤡ' => Ok(Limbu::VowelSignI),
            'ᤢ' => Ok(Limbu::VowelSignU),
            'ᤣ' => Ok(Limbu::VowelSignEe),
            'ᤤ' => Ok(Limbu::VowelSignAi),
            'ᤥ' => Ok(Limbu::VowelSignOo),
            'ᤦ' => Ok(Limbu::VowelSignAu),
            'ᤧ' => Ok(Limbu::VowelSignE),
            'ᤨ' => Ok(Limbu::VowelSignO),
            'ᤩ' => Ok(Limbu::SubjoinedLetterYa),
            'ᤪ' => Ok(Limbu::SubjoinedLetterRa),
            'ᤫ' => Ok(Limbu::SubjoinedLetterWa),
            'ᤰ' => Ok(Limbu::SmallLetterKa),
            'ᤱ' => Ok(Limbu::SmallLetterNga),
            'ᤲ' => Ok(Limbu::SmallLetterAnusvara),
            'ᤳ' => Ok(Limbu::SmallLetterTa),
            'ᤴ' => Ok(Limbu::SmallLetterNa),
            'ᤵ' => Ok(Limbu::SmallLetterPa),
            'ᤶ' => Ok(Limbu::SmallLetterMa),
            'ᤷ' => Ok(Limbu::SmallLetterRa),
            'ᤸ' => Ok(Limbu::SmallLetterLa),
            '᤹' => Ok(Limbu::SignMukphreng),
            '᤺' => Ok(Limbu::SignKemphreng),
            '᤻' => Ok(Limbu::SignSaDashI),
            '᥀' => Ok(Limbu::SignLoo),
            '᥄' => Ok(Limbu::ExclamationMark),
            '᥅' => Ok(Limbu::QuestionMark),
            '᥆' => Ok(Limbu::DigitZero),
            '᥇' => Ok(Limbu::DigitOne),
            '᥈' => Ok(Limbu::DigitTwo),
            '᥉' => Ok(Limbu::DigitThree),
            '᥊' => Ok(Limbu::DigitFour),
            '᥋' => Ok(Limbu::DigitFive),
            '᥌' => Ok(Limbu::DigitSix),
            '᥍' => Ok(Limbu::DigitSeven),
            '᥎' => Ok(Limbu::DigitEight),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Limbu {
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

impl std::convert::TryFrom<u32> for Limbu {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Limbu {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Limbu {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Limbu::VowelDashCarrierLetter
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Limbu::VowelDashCarrierLetter => "limbu vowel-carrier letter",
            Limbu::LetterKa => "limbu letter ka",
            Limbu::LetterKha => "limbu letter kha",
            Limbu::LetterGa => "limbu letter ga",
            Limbu::LetterGha => "limbu letter gha",
            Limbu::LetterNga => "limbu letter nga",
            Limbu::LetterCa => "limbu letter ca",
            Limbu::LetterCha => "limbu letter cha",
            Limbu::LetterJa => "limbu letter ja",
            Limbu::LetterJha => "limbu letter jha",
            Limbu::LetterYan => "limbu letter yan",
            Limbu::LetterTa => "limbu letter ta",
            Limbu::LetterTha => "limbu letter tha",
            Limbu::LetterDa => "limbu letter da",
            Limbu::LetterDha => "limbu letter dha",
            Limbu::LetterNa => "limbu letter na",
            Limbu::LetterPa => "limbu letter pa",
            Limbu::LetterPha => "limbu letter pha",
            Limbu::LetterBa => "limbu letter ba",
            Limbu::LetterBha => "limbu letter bha",
            Limbu::LetterMa => "limbu letter ma",
            Limbu::LetterYa => "limbu letter ya",
            Limbu::LetterRa => "limbu letter ra",
            Limbu::LetterLa => "limbu letter la",
            Limbu::LetterWa => "limbu letter wa",
            Limbu::LetterSha => "limbu letter sha",
            Limbu::LetterSsa => "limbu letter ssa",
            Limbu::LetterSa => "limbu letter sa",
            Limbu::LetterHa => "limbu letter ha",
            Limbu::LetterGyan => "limbu letter gyan",
            Limbu::LetterTra => "limbu letter tra",
            Limbu::VowelSignA => "limbu vowel sign a",
            Limbu::VowelSignI => "limbu vowel sign i",
            Limbu::VowelSignU => "limbu vowel sign u",
            Limbu::VowelSignEe => "limbu vowel sign ee",
            Limbu::VowelSignAi => "limbu vowel sign ai",
            Limbu::VowelSignOo => "limbu vowel sign oo",
            Limbu::VowelSignAu => "limbu vowel sign au",
            Limbu::VowelSignE => "limbu vowel sign e",
            Limbu::VowelSignO => "limbu vowel sign o",
            Limbu::SubjoinedLetterYa => "limbu subjoined letter ya",
            Limbu::SubjoinedLetterRa => "limbu subjoined letter ra",
            Limbu::SubjoinedLetterWa => "limbu subjoined letter wa",
            Limbu::SmallLetterKa => "limbu small letter ka",
            Limbu::SmallLetterNga => "limbu small letter nga",
            Limbu::SmallLetterAnusvara => "limbu small letter anusvara",
            Limbu::SmallLetterTa => "limbu small letter ta",
            Limbu::SmallLetterNa => "limbu small letter na",
            Limbu::SmallLetterPa => "limbu small letter pa",
            Limbu::SmallLetterMa => "limbu small letter ma",
            Limbu::SmallLetterRa => "limbu small letter ra",
            Limbu::SmallLetterLa => "limbu small letter la",
            Limbu::SignMukphreng => "limbu sign mukphreng",
            Limbu::SignKemphreng => "limbu sign kemphreng",
            Limbu::SignSaDashI => "limbu sign sa-i",
            Limbu::SignLoo => "limbu sign loo",
            Limbu::ExclamationMark => "limbu exclamation mark",
            Limbu::QuestionMark => "limbu question mark",
            Limbu::DigitZero => "limbu digit zero",
            Limbu::DigitOne => "limbu digit one",
            Limbu::DigitTwo => "limbu digit two",
            Limbu::DigitThree => "limbu digit three",
            Limbu::DigitFour => "limbu digit four",
            Limbu::DigitFive => "limbu digit five",
            Limbu::DigitSix => "limbu digit six",
            Limbu::DigitSeven => "limbu digit seven",
            Limbu::DigitEight => "limbu digit eight",
        }
    }
}
