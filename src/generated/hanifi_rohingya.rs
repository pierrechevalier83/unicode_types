/// \u{10d00} → \u{10d3f}
///
/// 𐴀 𐴁 𐴂 𐴃 𐴄 𐴅 𐴆 𐴇 𐴈 𐴉 𐴊 𐴋 𐴌 𐴍 𐴎 𐴏\
/// 𐴐 𐴑 𐴒 𐴓 𐴔 𐴕 𐴖 𐴗 𐴘 𐴙 𐴚 𐴛 𐴜 𐴝 𐴞 𐴟\
/// 𐴠 𐴡 𐴢 𐴣 𐴤 𐴥 𐴦 𐴧 𐴰 𐴱 𐴲 𐴳 𐴴 𐴵 𐴶 𐴷\
/// 𐴸 𐴹\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10d00}: '𐴀'
    pub const LETTER_A: char = '𐴀';
    /// \u{10d01}: '𐴁'
    pub const LETTER_BA: char = '𐴁';
    /// \u{10d02}: '𐴂'
    pub const LETTER_PA: char = '𐴂';
    /// \u{10d03}: '𐴃'
    pub const LETTER_TA: char = '𐴃';
    /// \u{10d04}: '𐴄'
    pub const LETTER_TTA: char = '𐴄';
    /// \u{10d05}: '𐴅'
    pub const LETTER_JA: char = '𐴅';
    /// \u{10d06}: '𐴆'
    pub const LETTER_CA: char = '𐴆';
    /// \u{10d07}: '𐴇'
    pub const LETTER_HA: char = '𐴇';
    /// \u{10d08}: '𐴈'
    pub const LETTER_KHA: char = '𐴈';
    /// \u{10d09}: '𐴉'
    pub const LETTER_FA: char = '𐴉';
    /// \u{10d0a}: '𐴊'
    pub const LETTER_DA: char = '𐴊';
    /// \u{10d0b}: '𐴋'
    pub const LETTER_DDA: char = '𐴋';
    /// \u{10d0c}: '𐴌'
    pub const LETTER_RA: char = '𐴌';
    /// \u{10d0d}: '𐴍'
    pub const LETTER_RRA: char = '𐴍';
    /// \u{10d0e}: '𐴎'
    pub const LETTER_ZA: char = '𐴎';
    /// \u{10d0f}: '𐴏'
    pub const LETTER_SA: char = '𐴏';
    /// \u{10d10}: '𐴐'
    pub const LETTER_SHA: char = '𐴐';
    /// \u{10d11}: '𐴑'
    pub const LETTER_KA: char = '𐴑';
    /// \u{10d12}: '𐴒'
    pub const LETTER_GA: char = '𐴒';
    /// \u{10d13}: '𐴓'
    pub const LETTER_LA: char = '𐴓';
    /// \u{10d14}: '𐴔'
    pub const LETTER_MA: char = '𐴔';
    /// \u{10d15}: '𐴕'
    pub const LETTER_NA: char = '𐴕';
    /// \u{10d16}: '𐴖'
    pub const LETTER_WA: char = '𐴖';
    /// \u{10d17}: '𐴗'
    pub const LETTER_KINNA_WA: char = '𐴗';
    /// \u{10d18}: '𐴘'
    pub const LETTER_YA: char = '𐴘';
    /// \u{10d19}: '𐴙'
    pub const LETTER_KINNA_YA: char = '𐴙';
    /// \u{10d1a}: '𐴚'
    pub const LETTER_NGA: char = '𐴚';
    /// \u{10d1b}: '𐴛'
    pub const LETTER_NYA: char = '𐴛';
    /// \u{10d1c}: '𐴜'
    pub const LETTER_VA: char = '𐴜';
    /// \u{10d1d}: '𐴝'
    pub const VOWEL_A: char = '𐴝';
    /// \u{10d1e}: '𐴞'
    pub const VOWEL_I: char = '𐴞';
    /// \u{10d1f}: '𐴟'
    pub const VOWEL_U: char = '𐴟';
    /// \u{10d20}: '𐴠'
    pub const VOWEL_E: char = '𐴠';
    /// \u{10d21}: '𐴡'
    pub const VOWEL_O: char = '𐴡';
    /// \u{10d22}: '𐴢'
    pub const MARK_SAKIN: char = '𐴢';
    /// \u{10d23}: '𐴣'
    pub const MARK_NA_KHONNA: char = '𐴣';
    /// \u{10d24}: '𐴤'
    pub const SIGN_HARBAHAY: char = '𐴤';
    /// \u{10d25}: '𐴥'
    pub const SIGN_TAHALA: char = '𐴥';
    /// \u{10d26}: '𐴦'
    pub const SIGN_TANA: char = '𐴦';
    /// \u{10d27}: '𐴧'
    pub const SIGN_TASSI: char = '𐴧';
    /// \u{10d30}: '𐴰'
    pub const DIGIT_ZERO: char = '𐴰';
    /// \u{10d31}: '𐴱'
    pub const DIGIT_ONE: char = '𐴱';
    /// \u{10d32}: '𐴲'
    pub const DIGIT_TWO: char = '𐴲';
    /// \u{10d33}: '𐴳'
    pub const DIGIT_THREE: char = '𐴳';
    /// \u{10d34}: '𐴴'
    pub const DIGIT_FOUR: char = '𐴴';
    /// \u{10d35}: '𐴵'
    pub const DIGIT_FIVE: char = '𐴵';
    /// \u{10d36}: '𐴶'
    pub const DIGIT_SIX: char = '𐴶';
    /// \u{10d37}: '𐴷'
    pub const DIGIT_SEVEN: char = '𐴷';
    /// \u{10d38}: '𐴸'
    pub const DIGIT_EIGHT: char = '𐴸';
    /// \u{10d39}: '𐴹'
    pub const DIGIT_NINE: char = '𐴹';
}

/// An enum to represent all characters in the HanifiRohingya block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HanifiRohingya {
    /// \u{10d00}: '𐴀'
    LetterA,
    /// \u{10d01}: '𐴁'
    LetterBa,
    /// \u{10d02}: '𐴂'
    LetterPa,
    /// \u{10d03}: '𐴃'
    LetterTa,
    /// \u{10d04}: '𐴄'
    LetterTta,
    /// \u{10d05}: '𐴅'
    LetterJa,
    /// \u{10d06}: '𐴆'
    LetterCa,
    /// \u{10d07}: '𐴇'
    LetterHa,
    /// \u{10d08}: '𐴈'
    LetterKha,
    /// \u{10d09}: '𐴉'
    LetterFa,
    /// \u{10d0a}: '𐴊'
    LetterDa,
    /// \u{10d0b}: '𐴋'
    LetterDda,
    /// \u{10d0c}: '𐴌'
    LetterRa,
    /// \u{10d0d}: '𐴍'
    LetterRra,
    /// \u{10d0e}: '𐴎'
    LetterZa,
    /// \u{10d0f}: '𐴏'
    LetterSa,
    /// \u{10d10}: '𐴐'
    LetterSha,
    /// \u{10d11}: '𐴑'
    LetterKa,
    /// \u{10d12}: '𐴒'
    LetterGa,
    /// \u{10d13}: '𐴓'
    LetterLa,
    /// \u{10d14}: '𐴔'
    LetterMa,
    /// \u{10d15}: '𐴕'
    LetterNa,
    /// \u{10d16}: '𐴖'
    LetterWa,
    /// \u{10d17}: '𐴗'
    LetterKinnaWa,
    /// \u{10d18}: '𐴘'
    LetterYa,
    /// \u{10d19}: '𐴙'
    LetterKinnaYa,
    /// \u{10d1a}: '𐴚'
    LetterNga,
    /// \u{10d1b}: '𐴛'
    LetterNya,
    /// \u{10d1c}: '𐴜'
    LetterVa,
    /// \u{10d1d}: '𐴝'
    VowelA,
    /// \u{10d1e}: '𐴞'
    VowelI,
    /// \u{10d1f}: '𐴟'
    VowelU,
    /// \u{10d20}: '𐴠'
    VowelE,
    /// \u{10d21}: '𐴡'
    VowelO,
    /// \u{10d22}: '𐴢'
    MarkSakin,
    /// \u{10d23}: '𐴣'
    MarkNaKhonna,
    /// \u{10d24}: '𐴤'
    SignHarbahay,
    /// \u{10d25}: '𐴥'
    SignTahala,
    /// \u{10d26}: '𐴦'
    SignTana,
    /// \u{10d27}: '𐴧'
    SignTassi,
    /// \u{10d30}: '𐴰'
    DigitZero,
    /// \u{10d31}: '𐴱'
    DigitOne,
    /// \u{10d32}: '𐴲'
    DigitTwo,
    /// \u{10d33}: '𐴳'
    DigitThree,
    /// \u{10d34}: '𐴴'
    DigitFour,
    /// \u{10d35}: '𐴵'
    DigitFive,
    /// \u{10d36}: '𐴶'
    DigitSix,
    /// \u{10d37}: '𐴷'
    DigitSeven,
    /// \u{10d38}: '𐴸'
    DigitEight,
    /// \u{10d39}: '𐴹'
    DigitNine,
}

impl Into<char> for HanifiRohingya {
    fn into(self) -> char {
        use constants::*;
        match self {
            HanifiRohingya::LetterA => LETTER_A,
            HanifiRohingya::LetterBa => LETTER_BA,
            HanifiRohingya::LetterPa => LETTER_PA,
            HanifiRohingya::LetterTa => LETTER_TA,
            HanifiRohingya::LetterTta => LETTER_TTA,
            HanifiRohingya::LetterJa => LETTER_JA,
            HanifiRohingya::LetterCa => LETTER_CA,
            HanifiRohingya::LetterHa => LETTER_HA,
            HanifiRohingya::LetterKha => LETTER_KHA,
            HanifiRohingya::LetterFa => LETTER_FA,
            HanifiRohingya::LetterDa => LETTER_DA,
            HanifiRohingya::LetterDda => LETTER_DDA,
            HanifiRohingya::LetterRa => LETTER_RA,
            HanifiRohingya::LetterRra => LETTER_RRA,
            HanifiRohingya::LetterZa => LETTER_ZA,
            HanifiRohingya::LetterSa => LETTER_SA,
            HanifiRohingya::LetterSha => LETTER_SHA,
            HanifiRohingya::LetterKa => LETTER_KA,
            HanifiRohingya::LetterGa => LETTER_GA,
            HanifiRohingya::LetterLa => LETTER_LA,
            HanifiRohingya::LetterMa => LETTER_MA,
            HanifiRohingya::LetterNa => LETTER_NA,
            HanifiRohingya::LetterWa => LETTER_WA,
            HanifiRohingya::LetterKinnaWa => LETTER_KINNA_WA,
            HanifiRohingya::LetterYa => LETTER_YA,
            HanifiRohingya::LetterKinnaYa => LETTER_KINNA_YA,
            HanifiRohingya::LetterNga => LETTER_NGA,
            HanifiRohingya::LetterNya => LETTER_NYA,
            HanifiRohingya::LetterVa => LETTER_VA,
            HanifiRohingya::VowelA => VOWEL_A,
            HanifiRohingya::VowelI => VOWEL_I,
            HanifiRohingya::VowelU => VOWEL_U,
            HanifiRohingya::VowelE => VOWEL_E,
            HanifiRohingya::VowelO => VOWEL_O,
            HanifiRohingya::MarkSakin => MARK_SAKIN,
            HanifiRohingya::MarkNaKhonna => MARK_NA_KHONNA,
            HanifiRohingya::SignHarbahay => SIGN_HARBAHAY,
            HanifiRohingya::SignTahala => SIGN_TAHALA,
            HanifiRohingya::SignTana => SIGN_TANA,
            HanifiRohingya::SignTassi => SIGN_TASSI,
            HanifiRohingya::DigitZero => DIGIT_ZERO,
            HanifiRohingya::DigitOne => DIGIT_ONE,
            HanifiRohingya::DigitTwo => DIGIT_TWO,
            HanifiRohingya::DigitThree => DIGIT_THREE,
            HanifiRohingya::DigitFour => DIGIT_FOUR,
            HanifiRohingya::DigitFive => DIGIT_FIVE,
            HanifiRohingya::DigitSix => DIGIT_SIX,
            HanifiRohingya::DigitSeven => DIGIT_SEVEN,
            HanifiRohingya::DigitEight => DIGIT_EIGHT,
            HanifiRohingya::DigitNine => DIGIT_NINE,
        }
    }
}

impl std::convert::TryFrom<char> for HanifiRohingya {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(HanifiRohingya::LetterA),
            LETTER_BA => Ok(HanifiRohingya::LetterBa),
            LETTER_PA => Ok(HanifiRohingya::LetterPa),
            LETTER_TA => Ok(HanifiRohingya::LetterTa),
            LETTER_TTA => Ok(HanifiRohingya::LetterTta),
            LETTER_JA => Ok(HanifiRohingya::LetterJa),
            LETTER_CA => Ok(HanifiRohingya::LetterCa),
            LETTER_HA => Ok(HanifiRohingya::LetterHa),
            LETTER_KHA => Ok(HanifiRohingya::LetterKha),
            LETTER_FA => Ok(HanifiRohingya::LetterFa),
            LETTER_DA => Ok(HanifiRohingya::LetterDa),
            LETTER_DDA => Ok(HanifiRohingya::LetterDda),
            LETTER_RA => Ok(HanifiRohingya::LetterRa),
            LETTER_RRA => Ok(HanifiRohingya::LetterRra),
            LETTER_ZA => Ok(HanifiRohingya::LetterZa),
            LETTER_SA => Ok(HanifiRohingya::LetterSa),
            LETTER_SHA => Ok(HanifiRohingya::LetterSha),
            LETTER_KA => Ok(HanifiRohingya::LetterKa),
            LETTER_GA => Ok(HanifiRohingya::LetterGa),
            LETTER_LA => Ok(HanifiRohingya::LetterLa),
            LETTER_MA => Ok(HanifiRohingya::LetterMa),
            LETTER_NA => Ok(HanifiRohingya::LetterNa),
            LETTER_WA => Ok(HanifiRohingya::LetterWa),
            LETTER_KINNA_WA => Ok(HanifiRohingya::LetterKinnaWa),
            LETTER_YA => Ok(HanifiRohingya::LetterYa),
            LETTER_KINNA_YA => Ok(HanifiRohingya::LetterKinnaYa),
            LETTER_NGA => Ok(HanifiRohingya::LetterNga),
            LETTER_NYA => Ok(HanifiRohingya::LetterNya),
            LETTER_VA => Ok(HanifiRohingya::LetterVa),
            VOWEL_A => Ok(HanifiRohingya::VowelA),
            VOWEL_I => Ok(HanifiRohingya::VowelI),
            VOWEL_U => Ok(HanifiRohingya::VowelU),
            VOWEL_E => Ok(HanifiRohingya::VowelE),
            VOWEL_O => Ok(HanifiRohingya::VowelO),
            MARK_SAKIN => Ok(HanifiRohingya::MarkSakin),
            MARK_NA_KHONNA => Ok(HanifiRohingya::MarkNaKhonna),
            SIGN_HARBAHAY => Ok(HanifiRohingya::SignHarbahay),
            SIGN_TAHALA => Ok(HanifiRohingya::SignTahala),
            SIGN_TANA => Ok(HanifiRohingya::SignTana),
            SIGN_TASSI => Ok(HanifiRohingya::SignTassi),
            DIGIT_ZERO => Ok(HanifiRohingya::DigitZero),
            DIGIT_ONE => Ok(HanifiRohingya::DigitOne),
            DIGIT_TWO => Ok(HanifiRohingya::DigitTwo),
            DIGIT_THREE => Ok(HanifiRohingya::DigitThree),
            DIGIT_FOUR => Ok(HanifiRohingya::DigitFour),
            DIGIT_FIVE => Ok(HanifiRohingya::DigitFive),
            DIGIT_SIX => Ok(HanifiRohingya::DigitSix),
            DIGIT_SEVEN => Ok(HanifiRohingya::DigitSeven),
            DIGIT_EIGHT => Ok(HanifiRohingya::DigitEight),
            DIGIT_NINE => Ok(HanifiRohingya::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HanifiRohingya {
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

impl std::convert::TryFrom<u32> for HanifiRohingya {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HanifiRohingya {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HanifiRohingya {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HanifiRohingya::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            HanifiRohingya::LetterA => "hanifi rohingya letter a",
            HanifiRohingya::LetterBa => "hanifi rohingya letter ba",
            HanifiRohingya::LetterPa => "hanifi rohingya letter pa",
            HanifiRohingya::LetterTa => "hanifi rohingya letter ta",
            HanifiRohingya::LetterTta => "hanifi rohingya letter tta",
            HanifiRohingya::LetterJa => "hanifi rohingya letter ja",
            HanifiRohingya::LetterCa => "hanifi rohingya letter ca",
            HanifiRohingya::LetterHa => "hanifi rohingya letter ha",
            HanifiRohingya::LetterKha => "hanifi rohingya letter kha",
            HanifiRohingya::LetterFa => "hanifi rohingya letter fa",
            HanifiRohingya::LetterDa => "hanifi rohingya letter da",
            HanifiRohingya::LetterDda => "hanifi rohingya letter dda",
            HanifiRohingya::LetterRa => "hanifi rohingya letter ra",
            HanifiRohingya::LetterRra => "hanifi rohingya letter rra",
            HanifiRohingya::LetterZa => "hanifi rohingya letter za",
            HanifiRohingya::LetterSa => "hanifi rohingya letter sa",
            HanifiRohingya::LetterSha => "hanifi rohingya letter sha",
            HanifiRohingya::LetterKa => "hanifi rohingya letter ka",
            HanifiRohingya::LetterGa => "hanifi rohingya letter ga",
            HanifiRohingya::LetterLa => "hanifi rohingya letter la",
            HanifiRohingya::LetterMa => "hanifi rohingya letter ma",
            HanifiRohingya::LetterNa => "hanifi rohingya letter na",
            HanifiRohingya::LetterWa => "hanifi rohingya letter wa",
            HanifiRohingya::LetterKinnaWa => "hanifi rohingya letter kinna wa",
            HanifiRohingya::LetterYa => "hanifi rohingya letter ya",
            HanifiRohingya::LetterKinnaYa => "hanifi rohingya letter kinna ya",
            HanifiRohingya::LetterNga => "hanifi rohingya letter nga",
            HanifiRohingya::LetterNya => "hanifi rohingya letter nya",
            HanifiRohingya::LetterVa => "hanifi rohingya letter va",
            HanifiRohingya::VowelA => "hanifi rohingya vowel a",
            HanifiRohingya::VowelI => "hanifi rohingya vowel i",
            HanifiRohingya::VowelU => "hanifi rohingya vowel u",
            HanifiRohingya::VowelE => "hanifi rohingya vowel e",
            HanifiRohingya::VowelO => "hanifi rohingya vowel o",
            HanifiRohingya::MarkSakin => "hanifi rohingya mark sakin",
            HanifiRohingya::MarkNaKhonna => "hanifi rohingya mark na khonna",
            HanifiRohingya::SignHarbahay => "hanifi rohingya sign harbahay",
            HanifiRohingya::SignTahala => "hanifi rohingya sign tahala",
            HanifiRohingya::SignTana => "hanifi rohingya sign tana",
            HanifiRohingya::SignTassi => "hanifi rohingya sign tassi",
            HanifiRohingya::DigitZero => "hanifi rohingya digit zero",
            HanifiRohingya::DigitOne => "hanifi rohingya digit one",
            HanifiRohingya::DigitTwo => "hanifi rohingya digit two",
            HanifiRohingya::DigitThree => "hanifi rohingya digit three",
            HanifiRohingya::DigitFour => "hanifi rohingya digit four",
            HanifiRohingya::DigitFive => "hanifi rohingya digit five",
            HanifiRohingya::DigitSix => "hanifi rohingya digit six",
            HanifiRohingya::DigitSeven => "hanifi rohingya digit seven",
            HanifiRohingya::DigitEight => "hanifi rohingya digit eight",
            HanifiRohingya::DigitNine => "hanifi rohingya digit nine",
        }
    }
}
