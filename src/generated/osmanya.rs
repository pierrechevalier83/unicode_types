
/// An enum to represent all characters in the Osmanya block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Osmanya {
    /// \u{10480}: '𐒀'
    LetterAlef,
    /// \u{10481}: '𐒁'
    LetterBa,
    /// \u{10482}: '𐒂'
    LetterTa,
    /// \u{10483}: '𐒃'
    LetterJa,
    /// \u{10484}: '𐒄'
    LetterXa,
    /// \u{10485}: '𐒅'
    LetterKha,
    /// \u{10486}: '𐒆'
    LetterDeel,
    /// \u{10487}: '𐒇'
    LetterRa,
    /// \u{10488}: '𐒈'
    LetterSa,
    /// \u{10489}: '𐒉'
    LetterShiin,
    /// \u{1048a}: '𐒊'
    LetterDha,
    /// \u{1048b}: '𐒋'
    LetterCayn,
    /// \u{1048c}: '𐒌'
    LetterGa,
    /// \u{1048d}: '𐒍'
    LetterFa,
    /// \u{1048e}: '𐒎'
    LetterQaaf,
    /// \u{1048f}: '𐒏'
    LetterKaaf,
    /// \u{10490}: '𐒐'
    LetterLaan,
    /// \u{10491}: '𐒑'
    LetterMiin,
    /// \u{10492}: '𐒒'
    LetterNuun,
    /// \u{10493}: '𐒓'
    LetterWaw,
    /// \u{10494}: '𐒔'
    LetterHa,
    /// \u{10495}: '𐒕'
    LetterYa,
    /// \u{10496}: '𐒖'
    LetterA,
    /// \u{10497}: '𐒗'
    LetterE,
    /// \u{10498}: '𐒘'
    LetterI,
    /// \u{10499}: '𐒙'
    LetterO,
    /// \u{1049a}: '𐒚'
    LetterU,
    /// \u{1049b}: '𐒛'
    LetterAa,
    /// \u{1049c}: '𐒜'
    LetterEe,
    /// \u{1049d}: '𐒝'
    LetterOo,
    /// \u{104a0}: '𐒠'
    DigitZero,
    /// \u{104a1}: '𐒡'
    DigitOne,
    /// \u{104a2}: '𐒢'
    DigitTwo,
    /// \u{104a3}: '𐒣'
    DigitThree,
    /// \u{104a4}: '𐒤'
    DigitFour,
    /// \u{104a5}: '𐒥'
    DigitFive,
    /// \u{104a6}: '𐒦'
    DigitSix,
    /// \u{104a7}: '𐒧'
    DigitSeven,
    /// \u{104a8}: '𐒨'
    DigitEight,
    /// \u{104a9}: '𐒩'
    DigitNine,
}

impl Into<char> for Osmanya {
    fn into(self) -> char {
        match self {
            Osmanya::LetterAlef => '𐒀',
            Osmanya::LetterBa => '𐒁',
            Osmanya::LetterTa => '𐒂',
            Osmanya::LetterJa => '𐒃',
            Osmanya::LetterXa => '𐒄',
            Osmanya::LetterKha => '𐒅',
            Osmanya::LetterDeel => '𐒆',
            Osmanya::LetterRa => '𐒇',
            Osmanya::LetterSa => '𐒈',
            Osmanya::LetterShiin => '𐒉',
            Osmanya::LetterDha => '𐒊',
            Osmanya::LetterCayn => '𐒋',
            Osmanya::LetterGa => '𐒌',
            Osmanya::LetterFa => '𐒍',
            Osmanya::LetterQaaf => '𐒎',
            Osmanya::LetterKaaf => '𐒏',
            Osmanya::LetterLaan => '𐒐',
            Osmanya::LetterMiin => '𐒑',
            Osmanya::LetterNuun => '𐒒',
            Osmanya::LetterWaw => '𐒓',
            Osmanya::LetterHa => '𐒔',
            Osmanya::LetterYa => '𐒕',
            Osmanya::LetterA => '𐒖',
            Osmanya::LetterE => '𐒗',
            Osmanya::LetterI => '𐒘',
            Osmanya::LetterO => '𐒙',
            Osmanya::LetterU => '𐒚',
            Osmanya::LetterAa => '𐒛',
            Osmanya::LetterEe => '𐒜',
            Osmanya::LetterOo => '𐒝',
            Osmanya::DigitZero => '𐒠',
            Osmanya::DigitOne => '𐒡',
            Osmanya::DigitTwo => '𐒢',
            Osmanya::DigitThree => '𐒣',
            Osmanya::DigitFour => '𐒤',
            Osmanya::DigitFive => '𐒥',
            Osmanya::DigitSix => '𐒦',
            Osmanya::DigitSeven => '𐒧',
            Osmanya::DigitEight => '𐒨',
            Osmanya::DigitNine => '𐒩',
        }
    }
}

impl std::convert::TryFrom<char> for Osmanya {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐒀' => Ok(Osmanya::LetterAlef),
            '𐒁' => Ok(Osmanya::LetterBa),
            '𐒂' => Ok(Osmanya::LetterTa),
            '𐒃' => Ok(Osmanya::LetterJa),
            '𐒄' => Ok(Osmanya::LetterXa),
            '𐒅' => Ok(Osmanya::LetterKha),
            '𐒆' => Ok(Osmanya::LetterDeel),
            '𐒇' => Ok(Osmanya::LetterRa),
            '𐒈' => Ok(Osmanya::LetterSa),
            '𐒉' => Ok(Osmanya::LetterShiin),
            '𐒊' => Ok(Osmanya::LetterDha),
            '𐒋' => Ok(Osmanya::LetterCayn),
            '𐒌' => Ok(Osmanya::LetterGa),
            '𐒍' => Ok(Osmanya::LetterFa),
            '𐒎' => Ok(Osmanya::LetterQaaf),
            '𐒏' => Ok(Osmanya::LetterKaaf),
            '𐒐' => Ok(Osmanya::LetterLaan),
            '𐒑' => Ok(Osmanya::LetterMiin),
            '𐒒' => Ok(Osmanya::LetterNuun),
            '𐒓' => Ok(Osmanya::LetterWaw),
            '𐒔' => Ok(Osmanya::LetterHa),
            '𐒕' => Ok(Osmanya::LetterYa),
            '𐒖' => Ok(Osmanya::LetterA),
            '𐒗' => Ok(Osmanya::LetterE),
            '𐒘' => Ok(Osmanya::LetterI),
            '𐒙' => Ok(Osmanya::LetterO),
            '𐒚' => Ok(Osmanya::LetterU),
            '𐒛' => Ok(Osmanya::LetterAa),
            '𐒜' => Ok(Osmanya::LetterEe),
            '𐒝' => Ok(Osmanya::LetterOo),
            '𐒠' => Ok(Osmanya::DigitZero),
            '𐒡' => Ok(Osmanya::DigitOne),
            '𐒢' => Ok(Osmanya::DigitTwo),
            '𐒣' => Ok(Osmanya::DigitThree),
            '𐒤' => Ok(Osmanya::DigitFour),
            '𐒥' => Ok(Osmanya::DigitFive),
            '𐒦' => Ok(Osmanya::DigitSix),
            '𐒧' => Ok(Osmanya::DigitSeven),
            '𐒨' => Ok(Osmanya::DigitEight),
            '𐒩' => Ok(Osmanya::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Osmanya {
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

impl std::convert::TryFrom<u32> for Osmanya {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Osmanya {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Osmanya {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Osmanya::LetterAlef
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Osmanya::LetterAlef => "osmanya letter alef",
            Osmanya::LetterBa => "osmanya letter ba",
            Osmanya::LetterTa => "osmanya letter ta",
            Osmanya::LetterJa => "osmanya letter ja",
            Osmanya::LetterXa => "osmanya letter xa",
            Osmanya::LetterKha => "osmanya letter kha",
            Osmanya::LetterDeel => "osmanya letter deel",
            Osmanya::LetterRa => "osmanya letter ra",
            Osmanya::LetterSa => "osmanya letter sa",
            Osmanya::LetterShiin => "osmanya letter shiin",
            Osmanya::LetterDha => "osmanya letter dha",
            Osmanya::LetterCayn => "osmanya letter cayn",
            Osmanya::LetterGa => "osmanya letter ga",
            Osmanya::LetterFa => "osmanya letter fa",
            Osmanya::LetterQaaf => "osmanya letter qaaf",
            Osmanya::LetterKaaf => "osmanya letter kaaf",
            Osmanya::LetterLaan => "osmanya letter laan",
            Osmanya::LetterMiin => "osmanya letter miin",
            Osmanya::LetterNuun => "osmanya letter nuun",
            Osmanya::LetterWaw => "osmanya letter waw",
            Osmanya::LetterHa => "osmanya letter ha",
            Osmanya::LetterYa => "osmanya letter ya",
            Osmanya::LetterA => "osmanya letter a",
            Osmanya::LetterE => "osmanya letter e",
            Osmanya::LetterI => "osmanya letter i",
            Osmanya::LetterO => "osmanya letter o",
            Osmanya::LetterU => "osmanya letter u",
            Osmanya::LetterAa => "osmanya letter aa",
            Osmanya::LetterEe => "osmanya letter ee",
            Osmanya::LetterOo => "osmanya letter oo",
            Osmanya::DigitZero => "osmanya digit zero",
            Osmanya::DigitOne => "osmanya digit one",
            Osmanya::DigitTwo => "osmanya digit two",
            Osmanya::DigitThree => "osmanya digit three",
            Osmanya::DigitFour => "osmanya digit four",
            Osmanya::DigitFive => "osmanya digit five",
            Osmanya::DigitSix => "osmanya digit six",
            Osmanya::DigitSeven => "osmanya digit seven",
            Osmanya::DigitEight => "osmanya digit eight",
            Osmanya::DigitNine => "osmanya digit nine",
        }
    }
}
