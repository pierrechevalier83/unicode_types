/// \u{10480} → \u{104af}\
///\
/// 𐒀 𐒁 𐒂 𐒃 𐒄 𐒅 𐒆 𐒇 𐒈 𐒉 𐒊 𐒋 𐒌 𐒍 𐒎 𐒏\
/// 𐒐 𐒑 𐒒 𐒓 𐒔 𐒕 𐒖 𐒗 𐒘 𐒙 𐒚 𐒛 𐒜 𐒝 𐒠 𐒡\
/// 𐒢 𐒣 𐒤 𐒥 𐒦 𐒧 𐒨 𐒩\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10480}: '𐒀'
    pub const LETTER_ALEF: char = '𐒀';
    /// \u{10481}: '𐒁'
    pub const LETTER_BA: char = '𐒁';
    /// \u{10482}: '𐒂'
    pub const LETTER_TA: char = '𐒂';
    /// \u{10483}: '𐒃'
    pub const LETTER_JA: char = '𐒃';
    /// \u{10484}: '𐒄'
    pub const LETTER_XA: char = '𐒄';
    /// \u{10485}: '𐒅'
    pub const LETTER_KHA: char = '𐒅';
    /// \u{10486}: '𐒆'
    pub const LETTER_DEEL: char = '𐒆';
    /// \u{10487}: '𐒇'
    pub const LETTER_RA: char = '𐒇';
    /// \u{10488}: '𐒈'
    pub const LETTER_SA: char = '𐒈';
    /// \u{10489}: '𐒉'
    pub const LETTER_SHIIN: char = '𐒉';
    /// \u{1048a}: '𐒊'
    pub const LETTER_DHA: char = '𐒊';
    /// \u{1048b}: '𐒋'
    pub const LETTER_CAYN: char = '𐒋';
    /// \u{1048c}: '𐒌'
    pub const LETTER_GA: char = '𐒌';
    /// \u{1048d}: '𐒍'
    pub const LETTER_FA: char = '𐒍';
    /// \u{1048e}: '𐒎'
    pub const LETTER_QAAF: char = '𐒎';
    /// \u{1048f}: '𐒏'
    pub const LETTER_KAAF: char = '𐒏';
    /// \u{10490}: '𐒐'
    pub const LETTER_LAAN: char = '𐒐';
    /// \u{10491}: '𐒑'
    pub const LETTER_MIIN: char = '𐒑';
    /// \u{10492}: '𐒒'
    pub const LETTER_NUUN: char = '𐒒';
    /// \u{10493}: '𐒓'
    pub const LETTER_WAW: char = '𐒓';
    /// \u{10494}: '𐒔'
    pub const LETTER_HA: char = '𐒔';
    /// \u{10495}: '𐒕'
    pub const LETTER_YA: char = '𐒕';
    /// \u{10496}: '𐒖'
    pub const LETTER_A: char = '𐒖';
    /// \u{10497}: '𐒗'
    pub const LETTER_E: char = '𐒗';
    /// \u{10498}: '𐒘'
    pub const LETTER_I: char = '𐒘';
    /// \u{10499}: '𐒙'
    pub const LETTER_O: char = '𐒙';
    /// \u{1049a}: '𐒚'
    pub const LETTER_U: char = '𐒚';
    /// \u{1049b}: '𐒛'
    pub const LETTER_AA: char = '𐒛';
    /// \u{1049c}: '𐒜'
    pub const LETTER_EE: char = '𐒜';
    /// \u{1049d}: '𐒝'
    pub const LETTER_OO: char = '𐒝';
    /// \u{104a0}: '𐒠'
    pub const DIGIT_ZERO: char = '𐒠';
    /// \u{104a1}: '𐒡'
    pub const DIGIT_ONE: char = '𐒡';
    /// \u{104a2}: '𐒢'
    pub const DIGIT_TWO: char = '𐒢';
    /// \u{104a3}: '𐒣'
    pub const DIGIT_THREE: char = '𐒣';
    /// \u{104a4}: '𐒤'
    pub const DIGIT_FOUR: char = '𐒤';
    /// \u{104a5}: '𐒥'
    pub const DIGIT_FIVE: char = '𐒥';
    /// \u{104a6}: '𐒦'
    pub const DIGIT_SIX: char = '𐒦';
    /// \u{104a7}: '𐒧'
    pub const DIGIT_SEVEN: char = '𐒧';
    /// \u{104a8}: '𐒨'
    pub const DIGIT_EIGHT: char = '𐒨';
    /// \u{104a9}: '𐒩'
    pub const DIGIT_NINE: char = '𐒩';
}

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
        use constants::*;
        match self {
            Osmanya::LetterAlef => LETTER_ALEF,
            Osmanya::LetterBa => LETTER_BA,
            Osmanya::LetterTa => LETTER_TA,
            Osmanya::LetterJa => LETTER_JA,
            Osmanya::LetterXa => LETTER_XA,
            Osmanya::LetterKha => LETTER_KHA,
            Osmanya::LetterDeel => LETTER_DEEL,
            Osmanya::LetterRa => LETTER_RA,
            Osmanya::LetterSa => LETTER_SA,
            Osmanya::LetterShiin => LETTER_SHIIN,
            Osmanya::LetterDha => LETTER_DHA,
            Osmanya::LetterCayn => LETTER_CAYN,
            Osmanya::LetterGa => LETTER_GA,
            Osmanya::LetterFa => LETTER_FA,
            Osmanya::LetterQaaf => LETTER_QAAF,
            Osmanya::LetterKaaf => LETTER_KAAF,
            Osmanya::LetterLaan => LETTER_LAAN,
            Osmanya::LetterMiin => LETTER_MIIN,
            Osmanya::LetterNuun => LETTER_NUUN,
            Osmanya::LetterWaw => LETTER_WAW,
            Osmanya::LetterHa => LETTER_HA,
            Osmanya::LetterYa => LETTER_YA,
            Osmanya::LetterA => LETTER_A,
            Osmanya::LetterE => LETTER_E,
            Osmanya::LetterI => LETTER_I,
            Osmanya::LetterO => LETTER_O,
            Osmanya::LetterU => LETTER_U,
            Osmanya::LetterAa => LETTER_AA,
            Osmanya::LetterEe => LETTER_EE,
            Osmanya::LetterOo => LETTER_OO,
            Osmanya::DigitZero => DIGIT_ZERO,
            Osmanya::DigitOne => DIGIT_ONE,
            Osmanya::DigitTwo => DIGIT_TWO,
            Osmanya::DigitThree => DIGIT_THREE,
            Osmanya::DigitFour => DIGIT_FOUR,
            Osmanya::DigitFive => DIGIT_FIVE,
            Osmanya::DigitSix => DIGIT_SIX,
            Osmanya::DigitSeven => DIGIT_SEVEN,
            Osmanya::DigitEight => DIGIT_EIGHT,
            Osmanya::DigitNine => DIGIT_NINE,
        }
    }
}

impl std::convert::TryFrom<char> for Osmanya {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEF => Ok(Osmanya::LetterAlef),
            LETTER_BA => Ok(Osmanya::LetterBa),
            LETTER_TA => Ok(Osmanya::LetterTa),
            LETTER_JA => Ok(Osmanya::LetterJa),
            LETTER_XA => Ok(Osmanya::LetterXa),
            LETTER_KHA => Ok(Osmanya::LetterKha),
            LETTER_DEEL => Ok(Osmanya::LetterDeel),
            LETTER_RA => Ok(Osmanya::LetterRa),
            LETTER_SA => Ok(Osmanya::LetterSa),
            LETTER_SHIIN => Ok(Osmanya::LetterShiin),
            LETTER_DHA => Ok(Osmanya::LetterDha),
            LETTER_CAYN => Ok(Osmanya::LetterCayn),
            LETTER_GA => Ok(Osmanya::LetterGa),
            LETTER_FA => Ok(Osmanya::LetterFa),
            LETTER_QAAF => Ok(Osmanya::LetterQaaf),
            LETTER_KAAF => Ok(Osmanya::LetterKaaf),
            LETTER_LAAN => Ok(Osmanya::LetterLaan),
            LETTER_MIIN => Ok(Osmanya::LetterMiin),
            LETTER_NUUN => Ok(Osmanya::LetterNuun),
            LETTER_WAW => Ok(Osmanya::LetterWaw),
            LETTER_HA => Ok(Osmanya::LetterHa),
            LETTER_YA => Ok(Osmanya::LetterYa),
            LETTER_A => Ok(Osmanya::LetterA),
            LETTER_E => Ok(Osmanya::LetterE),
            LETTER_I => Ok(Osmanya::LetterI),
            LETTER_O => Ok(Osmanya::LetterO),
            LETTER_U => Ok(Osmanya::LetterU),
            LETTER_AA => Ok(Osmanya::LetterAa),
            LETTER_EE => Ok(Osmanya::LetterEe),
            LETTER_OO => Ok(Osmanya::LetterOo),
            DIGIT_ZERO => Ok(Osmanya::DigitZero),
            DIGIT_ONE => Ok(Osmanya::DigitOne),
            DIGIT_TWO => Ok(Osmanya::DigitTwo),
            DIGIT_THREE => Ok(Osmanya::DigitThree),
            DIGIT_FOUR => Ok(Osmanya::DigitFour),
            DIGIT_FIVE => Ok(Osmanya::DigitFive),
            DIGIT_SIX => Ok(Osmanya::DigitSix),
            DIGIT_SEVEN => Ok(Osmanya::DigitSeven),
            DIGIT_EIGHT => Ok(Osmanya::DigitEight),
            DIGIT_NINE => Ok(Osmanya::DigitNine),
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
