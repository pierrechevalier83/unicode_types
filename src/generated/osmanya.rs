/// \u{10480} → \u{104af}\
///\
/// 𐒀 𐒁 𐒂 𐒃 𐒄 𐒅 𐒆 𐒇 𐒈 𐒉 𐒊 𐒋 𐒌 𐒍 𐒎 𐒏
/// 𐒐 𐒑 𐒒 𐒓 𐒔 𐒕 𐒖 𐒗 𐒘 𐒙 𐒚 𐒛 𐒜 𐒝 𐒠 𐒡
/// 𐒢 𐒣 𐒤 𐒥 𐒦 𐒧 𐒨 𐒩
pub mod constants {
    /// \u{10480}: '𐒀'
    pub const OSMANYA_LETTER_ALEF: char = '𐒀';
    /// \u{10481}: '𐒁'
    pub const OSMANYA_LETTER_BA: char = '𐒁';
    /// \u{10482}: '𐒂'
    pub const OSMANYA_LETTER_TA: char = '𐒂';
    /// \u{10483}: '𐒃'
    pub const OSMANYA_LETTER_JA: char = '𐒃';
    /// \u{10484}: '𐒄'
    pub const OSMANYA_LETTER_XA: char = '𐒄';
    /// \u{10485}: '𐒅'
    pub const OSMANYA_LETTER_KHA: char = '𐒅';
    /// \u{10486}: '𐒆'
    pub const OSMANYA_LETTER_DEEL: char = '𐒆';
    /// \u{10487}: '𐒇'
    pub const OSMANYA_LETTER_RA: char = '𐒇';
    /// \u{10488}: '𐒈'
    pub const OSMANYA_LETTER_SA: char = '𐒈';
    /// \u{10489}: '𐒉'
    pub const OSMANYA_LETTER_SHIIN: char = '𐒉';
    /// \u{1048a}: '𐒊'
    pub const OSMANYA_LETTER_DHA: char = '𐒊';
    /// \u{1048b}: '𐒋'
    pub const OSMANYA_LETTER_CAYN: char = '𐒋';
    /// \u{1048c}: '𐒌'
    pub const OSMANYA_LETTER_GA: char = '𐒌';
    /// \u{1048d}: '𐒍'
    pub const OSMANYA_LETTER_FA: char = '𐒍';
    /// \u{1048e}: '𐒎'
    pub const OSMANYA_LETTER_QAAF: char = '𐒎';
    /// \u{1048f}: '𐒏'
    pub const OSMANYA_LETTER_KAAF: char = '𐒏';
    /// \u{10490}: '𐒐'
    pub const OSMANYA_LETTER_LAAN: char = '𐒐';
    /// \u{10491}: '𐒑'
    pub const OSMANYA_LETTER_MIIN: char = '𐒑';
    /// \u{10492}: '𐒒'
    pub const OSMANYA_LETTER_NUUN: char = '𐒒';
    /// \u{10493}: '𐒓'
    pub const OSMANYA_LETTER_WAW: char = '𐒓';
    /// \u{10494}: '𐒔'
    pub const OSMANYA_LETTER_HA: char = '𐒔';
    /// \u{10495}: '𐒕'
    pub const OSMANYA_LETTER_YA: char = '𐒕';
    /// \u{10496}: '𐒖'
    pub const OSMANYA_LETTER_A: char = '𐒖';
    /// \u{10497}: '𐒗'
    pub const OSMANYA_LETTER_E: char = '𐒗';
    /// \u{10498}: '𐒘'
    pub const OSMANYA_LETTER_I: char = '𐒘';
    /// \u{10499}: '𐒙'
    pub const OSMANYA_LETTER_O: char = '𐒙';
    /// \u{1049a}: '𐒚'
    pub const OSMANYA_LETTER_U: char = '𐒚';
    /// \u{1049b}: '𐒛'
    pub const OSMANYA_LETTER_AA: char = '𐒛';
    /// \u{1049c}: '𐒜'
    pub const OSMANYA_LETTER_EE: char = '𐒜';
    /// \u{1049d}: '𐒝'
    pub const OSMANYA_LETTER_OO: char = '𐒝';
    /// \u{104a0}: '𐒠'
    pub const OSMANYA_DIGIT_ZERO: char = '𐒠';
    /// \u{104a1}: '𐒡'
    pub const OSMANYA_DIGIT_ONE: char = '𐒡';
    /// \u{104a2}: '𐒢'
    pub const OSMANYA_DIGIT_TWO: char = '𐒢';
    /// \u{104a3}: '𐒣'
    pub const OSMANYA_DIGIT_THREE: char = '𐒣';
    /// \u{104a4}: '𐒤'
    pub const OSMANYA_DIGIT_FOUR: char = '𐒤';
    /// \u{104a5}: '𐒥'
    pub const OSMANYA_DIGIT_FIVE: char = '𐒥';
    /// \u{104a6}: '𐒦'
    pub const OSMANYA_DIGIT_SIX: char = '𐒦';
    /// \u{104a7}: '𐒧'
    pub const OSMANYA_DIGIT_SEVEN: char = '𐒧';
    /// \u{104a8}: '𐒨'
    pub const OSMANYA_DIGIT_EIGHT: char = '𐒨';
    /// \u{104a9}: '𐒩'
    pub const OSMANYA_DIGIT_NINE: char = '𐒩';
}

/// \u{10480} → \u{104af}\
///\
/// 𐒀 𐒁 𐒂 𐒃 𐒄 𐒅 𐒆 𐒇 𐒈 𐒉 𐒊 𐒋 𐒌 𐒍 𐒎 𐒏
/// 𐒐 𐒑 𐒒 𐒓 𐒔 𐒕 𐒖 𐒗 𐒘 𐒙 𐒚 𐒛 𐒜 𐒝 𐒠 𐒡
/// 𐒢 𐒣 𐒤 𐒥 𐒦 𐒧 𐒨 𐒩
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Osmanya {
    /// \u{10480}: '𐒀'
    OsmanyaLetterAlef,
    /// \u{10481}: '𐒁'
    OsmanyaLetterBa,
    /// \u{10482}: '𐒂'
    OsmanyaLetterTa,
    /// \u{10483}: '𐒃'
    OsmanyaLetterJa,
    /// \u{10484}: '𐒄'
    OsmanyaLetterXa,
    /// \u{10485}: '𐒅'
    OsmanyaLetterKha,
    /// \u{10486}: '𐒆'
    OsmanyaLetterDeel,
    /// \u{10487}: '𐒇'
    OsmanyaLetterRa,
    /// \u{10488}: '𐒈'
    OsmanyaLetterSa,
    /// \u{10489}: '𐒉'
    OsmanyaLetterShiin,
    /// \u{1048a}: '𐒊'
    OsmanyaLetterDha,
    /// \u{1048b}: '𐒋'
    OsmanyaLetterCayn,
    /// \u{1048c}: '𐒌'
    OsmanyaLetterGa,
    /// \u{1048d}: '𐒍'
    OsmanyaLetterFa,
    /// \u{1048e}: '𐒎'
    OsmanyaLetterQaaf,
    /// \u{1048f}: '𐒏'
    OsmanyaLetterKaaf,
    /// \u{10490}: '𐒐'
    OsmanyaLetterLaan,
    /// \u{10491}: '𐒑'
    OsmanyaLetterMiin,
    /// \u{10492}: '𐒒'
    OsmanyaLetterNuun,
    /// \u{10493}: '𐒓'
    OsmanyaLetterWaw,
    /// \u{10494}: '𐒔'
    OsmanyaLetterHa,
    /// \u{10495}: '𐒕'
    OsmanyaLetterYa,
    /// \u{10496}: '𐒖'
    OsmanyaLetterA,
    /// \u{10497}: '𐒗'
    OsmanyaLetterE,
    /// \u{10498}: '𐒘'
    OsmanyaLetterI,
    /// \u{10499}: '𐒙'
    OsmanyaLetterO,
    /// \u{1049a}: '𐒚'
    OsmanyaLetterU,
    /// \u{1049b}: '𐒛'
    OsmanyaLetterAa,
    /// \u{1049c}: '𐒜'
    OsmanyaLetterEe,
    /// \u{1049d}: '𐒝'
    OsmanyaLetterOo,
    /// \u{104a0}: '𐒠'
    OsmanyaDigitZero,
    /// \u{104a1}: '𐒡'
    OsmanyaDigitOne,
    /// \u{104a2}: '𐒢'
    OsmanyaDigitTwo,
    /// \u{104a3}: '𐒣'
    OsmanyaDigitThree,
    /// \u{104a4}: '𐒤'
    OsmanyaDigitFour,
    /// \u{104a5}: '𐒥'
    OsmanyaDigitFive,
    /// \u{104a6}: '𐒦'
    OsmanyaDigitSix,
    /// \u{104a7}: '𐒧'
    OsmanyaDigitSeven,
    /// \u{104a8}: '𐒨'
    OsmanyaDigitEight,
    /// \u{104a9}: '𐒩'
    OsmanyaDigitNine,
}

impl Into<char> for Osmanya {
    fn into(self) -> char {
        use constants::*;
        match self {
            Osmanya::OsmanyaLetterAlef => OSMANYA_LETTER_ALEF,
            Osmanya::OsmanyaLetterBa => OSMANYA_LETTER_BA,
            Osmanya::OsmanyaLetterTa => OSMANYA_LETTER_TA,
            Osmanya::OsmanyaLetterJa => OSMANYA_LETTER_JA,
            Osmanya::OsmanyaLetterXa => OSMANYA_LETTER_XA,
            Osmanya::OsmanyaLetterKha => OSMANYA_LETTER_KHA,
            Osmanya::OsmanyaLetterDeel => OSMANYA_LETTER_DEEL,
            Osmanya::OsmanyaLetterRa => OSMANYA_LETTER_RA,
            Osmanya::OsmanyaLetterSa => OSMANYA_LETTER_SA,
            Osmanya::OsmanyaLetterShiin => OSMANYA_LETTER_SHIIN,
            Osmanya::OsmanyaLetterDha => OSMANYA_LETTER_DHA,
            Osmanya::OsmanyaLetterCayn => OSMANYA_LETTER_CAYN,
            Osmanya::OsmanyaLetterGa => OSMANYA_LETTER_GA,
            Osmanya::OsmanyaLetterFa => OSMANYA_LETTER_FA,
            Osmanya::OsmanyaLetterQaaf => OSMANYA_LETTER_QAAF,
            Osmanya::OsmanyaLetterKaaf => OSMANYA_LETTER_KAAF,
            Osmanya::OsmanyaLetterLaan => OSMANYA_LETTER_LAAN,
            Osmanya::OsmanyaLetterMiin => OSMANYA_LETTER_MIIN,
            Osmanya::OsmanyaLetterNuun => OSMANYA_LETTER_NUUN,
            Osmanya::OsmanyaLetterWaw => OSMANYA_LETTER_WAW,
            Osmanya::OsmanyaLetterHa => OSMANYA_LETTER_HA,
            Osmanya::OsmanyaLetterYa => OSMANYA_LETTER_YA,
            Osmanya::OsmanyaLetterA => OSMANYA_LETTER_A,
            Osmanya::OsmanyaLetterE => OSMANYA_LETTER_E,
            Osmanya::OsmanyaLetterI => OSMANYA_LETTER_I,
            Osmanya::OsmanyaLetterO => OSMANYA_LETTER_O,
            Osmanya::OsmanyaLetterU => OSMANYA_LETTER_U,
            Osmanya::OsmanyaLetterAa => OSMANYA_LETTER_AA,
            Osmanya::OsmanyaLetterEe => OSMANYA_LETTER_EE,
            Osmanya::OsmanyaLetterOo => OSMANYA_LETTER_OO,
            Osmanya::OsmanyaDigitZero => OSMANYA_DIGIT_ZERO,
            Osmanya::OsmanyaDigitOne => OSMANYA_DIGIT_ONE,
            Osmanya::OsmanyaDigitTwo => OSMANYA_DIGIT_TWO,
            Osmanya::OsmanyaDigitThree => OSMANYA_DIGIT_THREE,
            Osmanya::OsmanyaDigitFour => OSMANYA_DIGIT_FOUR,
            Osmanya::OsmanyaDigitFive => OSMANYA_DIGIT_FIVE,
            Osmanya::OsmanyaDigitSix => OSMANYA_DIGIT_SIX,
            Osmanya::OsmanyaDigitSeven => OSMANYA_DIGIT_SEVEN,
            Osmanya::OsmanyaDigitEight => OSMANYA_DIGIT_EIGHT,
            Osmanya::OsmanyaDigitNine => OSMANYA_DIGIT_NINE,
        }
    }
}

impl std::convert::TryFrom<char> for Osmanya {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            OSMANYA_LETTER_ALEF => Ok(Osmanya::OsmanyaLetterAlef),
            OSMANYA_LETTER_BA => Ok(Osmanya::OsmanyaLetterBa),
            OSMANYA_LETTER_TA => Ok(Osmanya::OsmanyaLetterTa),
            OSMANYA_LETTER_JA => Ok(Osmanya::OsmanyaLetterJa),
            OSMANYA_LETTER_XA => Ok(Osmanya::OsmanyaLetterXa),
            OSMANYA_LETTER_KHA => Ok(Osmanya::OsmanyaLetterKha),
            OSMANYA_LETTER_DEEL => Ok(Osmanya::OsmanyaLetterDeel),
            OSMANYA_LETTER_RA => Ok(Osmanya::OsmanyaLetterRa),
            OSMANYA_LETTER_SA => Ok(Osmanya::OsmanyaLetterSa),
            OSMANYA_LETTER_SHIIN => Ok(Osmanya::OsmanyaLetterShiin),
            OSMANYA_LETTER_DHA => Ok(Osmanya::OsmanyaLetterDha),
            OSMANYA_LETTER_CAYN => Ok(Osmanya::OsmanyaLetterCayn),
            OSMANYA_LETTER_GA => Ok(Osmanya::OsmanyaLetterGa),
            OSMANYA_LETTER_FA => Ok(Osmanya::OsmanyaLetterFa),
            OSMANYA_LETTER_QAAF => Ok(Osmanya::OsmanyaLetterQaaf),
            OSMANYA_LETTER_KAAF => Ok(Osmanya::OsmanyaLetterKaaf),
            OSMANYA_LETTER_LAAN => Ok(Osmanya::OsmanyaLetterLaan),
            OSMANYA_LETTER_MIIN => Ok(Osmanya::OsmanyaLetterMiin),
            OSMANYA_LETTER_NUUN => Ok(Osmanya::OsmanyaLetterNuun),
            OSMANYA_LETTER_WAW => Ok(Osmanya::OsmanyaLetterWaw),
            OSMANYA_LETTER_HA => Ok(Osmanya::OsmanyaLetterHa),
            OSMANYA_LETTER_YA => Ok(Osmanya::OsmanyaLetterYa),
            OSMANYA_LETTER_A => Ok(Osmanya::OsmanyaLetterA),
            OSMANYA_LETTER_E => Ok(Osmanya::OsmanyaLetterE),
            OSMANYA_LETTER_I => Ok(Osmanya::OsmanyaLetterI),
            OSMANYA_LETTER_O => Ok(Osmanya::OsmanyaLetterO),
            OSMANYA_LETTER_U => Ok(Osmanya::OsmanyaLetterU),
            OSMANYA_LETTER_AA => Ok(Osmanya::OsmanyaLetterAa),
            OSMANYA_LETTER_EE => Ok(Osmanya::OsmanyaLetterEe),
            OSMANYA_LETTER_OO => Ok(Osmanya::OsmanyaLetterOo),
            OSMANYA_DIGIT_ZERO => Ok(Osmanya::OsmanyaDigitZero),
            OSMANYA_DIGIT_ONE => Ok(Osmanya::OsmanyaDigitOne),
            OSMANYA_DIGIT_TWO => Ok(Osmanya::OsmanyaDigitTwo),
            OSMANYA_DIGIT_THREE => Ok(Osmanya::OsmanyaDigitThree),
            OSMANYA_DIGIT_FOUR => Ok(Osmanya::OsmanyaDigitFour),
            OSMANYA_DIGIT_FIVE => Ok(Osmanya::OsmanyaDigitFive),
            OSMANYA_DIGIT_SIX => Ok(Osmanya::OsmanyaDigitSix),
            OSMANYA_DIGIT_SEVEN => Ok(Osmanya::OsmanyaDigitSeven),
            OSMANYA_DIGIT_EIGHT => Ok(Osmanya::OsmanyaDigitEight),
            OSMANYA_DIGIT_NINE => Ok(Osmanya::OsmanyaDigitNine),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Osmanya::OsmanyaLetterAlef
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Osmanya::OsmanyaLetterAlef => "osmanya letter alef",
            Osmanya::OsmanyaLetterBa => "osmanya letter ba",
            Osmanya::OsmanyaLetterTa => "osmanya letter ta",
            Osmanya::OsmanyaLetterJa => "osmanya letter ja",
            Osmanya::OsmanyaLetterXa => "osmanya letter xa",
            Osmanya::OsmanyaLetterKha => "osmanya letter kha",
            Osmanya::OsmanyaLetterDeel => "osmanya letter deel",
            Osmanya::OsmanyaLetterRa => "osmanya letter ra",
            Osmanya::OsmanyaLetterSa => "osmanya letter sa",
            Osmanya::OsmanyaLetterShiin => "osmanya letter shiin",
            Osmanya::OsmanyaLetterDha => "osmanya letter dha",
            Osmanya::OsmanyaLetterCayn => "osmanya letter cayn",
            Osmanya::OsmanyaLetterGa => "osmanya letter ga",
            Osmanya::OsmanyaLetterFa => "osmanya letter fa",
            Osmanya::OsmanyaLetterQaaf => "osmanya letter qaaf",
            Osmanya::OsmanyaLetterKaaf => "osmanya letter kaaf",
            Osmanya::OsmanyaLetterLaan => "osmanya letter laan",
            Osmanya::OsmanyaLetterMiin => "osmanya letter miin",
            Osmanya::OsmanyaLetterNuun => "osmanya letter nuun",
            Osmanya::OsmanyaLetterWaw => "osmanya letter waw",
            Osmanya::OsmanyaLetterHa => "osmanya letter ha",
            Osmanya::OsmanyaLetterYa => "osmanya letter ya",
            Osmanya::OsmanyaLetterA => "osmanya letter a",
            Osmanya::OsmanyaLetterE => "osmanya letter e",
            Osmanya::OsmanyaLetterI => "osmanya letter i",
            Osmanya::OsmanyaLetterO => "osmanya letter o",
            Osmanya::OsmanyaLetterU => "osmanya letter u",
            Osmanya::OsmanyaLetterAa => "osmanya letter aa",
            Osmanya::OsmanyaLetterEe => "osmanya letter ee",
            Osmanya::OsmanyaLetterOo => "osmanya letter oo",
            Osmanya::OsmanyaDigitZero => "osmanya digit zero",
            Osmanya::OsmanyaDigitOne => "osmanya digit one",
            Osmanya::OsmanyaDigitTwo => "osmanya digit two",
            Osmanya::OsmanyaDigitThree => "osmanya digit three",
            Osmanya::OsmanyaDigitFour => "osmanya digit four",
            Osmanya::OsmanyaDigitFive => "osmanya digit five",
            Osmanya::OsmanyaDigitSix => "osmanya digit six",
            Osmanya::OsmanyaDigitSeven => "osmanya digit seven",
            Osmanya::OsmanyaDigitEight => "osmanya digit eight",
            Osmanya::OsmanyaDigitNine => "osmanya digit nine",
        }
    }
}
