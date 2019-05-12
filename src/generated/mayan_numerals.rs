/// \u{1d2e0} → \u{1d2ff}
///
/// 𝋠 𝋡 𝋢 𝋣 𝋤 𝋥 𝋦 𝋧 𝋨 𝋩 𝋪 𝋫 𝋬 𝋭 𝋮 𝋯\
/// 𝋰 𝋱 𝋲 𝋳\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1d2e0}: '𝋠'
    pub const MAYAN_NUMERAL_ZERO: char = '𝋠';
    /// \u{1d2e1}: '𝋡'
    pub const MAYAN_NUMERAL_ONE: char = '𝋡';
    /// \u{1d2e2}: '𝋢'
    pub const MAYAN_NUMERAL_TWO: char = '𝋢';
    /// \u{1d2e3}: '𝋣'
    pub const MAYAN_NUMERAL_THREE: char = '𝋣';
    /// \u{1d2e4}: '𝋤'
    pub const MAYAN_NUMERAL_FOUR: char = '𝋤';
    /// \u{1d2e5}: '𝋥'
    pub const MAYAN_NUMERAL_FIVE: char = '𝋥';
    /// \u{1d2e6}: '𝋦'
    pub const MAYAN_NUMERAL_SIX: char = '𝋦';
    /// \u{1d2e7}: '𝋧'
    pub const MAYAN_NUMERAL_SEVEN: char = '𝋧';
    /// \u{1d2e8}: '𝋨'
    pub const MAYAN_NUMERAL_EIGHT: char = '𝋨';
    /// \u{1d2e9}: '𝋩'
    pub const MAYAN_NUMERAL_NINE: char = '𝋩';
    /// \u{1d2ea}: '𝋪'
    pub const MAYAN_NUMERAL_TEN: char = '𝋪';
    /// \u{1d2eb}: '𝋫'
    pub const MAYAN_NUMERAL_ELEVEN: char = '𝋫';
    /// \u{1d2ec}: '𝋬'
    pub const MAYAN_NUMERAL_TWELVE: char = '𝋬';
    /// \u{1d2ed}: '𝋭'
    pub const MAYAN_NUMERAL_THIRTEEN: char = '𝋭';
    /// \u{1d2ee}: '𝋮'
    pub const MAYAN_NUMERAL_FOURTEEN: char = '𝋮';
    /// \u{1d2ef}: '𝋯'
    pub const MAYAN_NUMERAL_FIFTEEN: char = '𝋯';
    /// \u{1d2f0}: '𝋰'
    pub const MAYAN_NUMERAL_SIXTEEN: char = '𝋰';
    /// \u{1d2f1}: '𝋱'
    pub const MAYAN_NUMERAL_SEVENTEEN: char = '𝋱';
    /// \u{1d2f2}: '𝋲'
    pub const MAYAN_NUMERAL_EIGHTEEN: char = '𝋲';
    /// \u{1d2f3}: '𝋳'
    pub const MAYAN_NUMERAL_NINETEEN: char = '𝋳';
}

/// An enum to represent all characters in the MayanNumerals block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MayanNumerals {
    /// \u{1d2e0}: '𝋠'
    MayanNumeralZero,
    /// \u{1d2e1}: '𝋡'
    MayanNumeralOne,
    /// \u{1d2e2}: '𝋢'
    MayanNumeralTwo,
    /// \u{1d2e3}: '𝋣'
    MayanNumeralThree,
    /// \u{1d2e4}: '𝋤'
    MayanNumeralFour,
    /// \u{1d2e5}: '𝋥'
    MayanNumeralFive,
    /// \u{1d2e6}: '𝋦'
    MayanNumeralSix,
    /// \u{1d2e7}: '𝋧'
    MayanNumeralSeven,
    /// \u{1d2e8}: '𝋨'
    MayanNumeralEight,
    /// \u{1d2e9}: '𝋩'
    MayanNumeralNine,
    /// \u{1d2ea}: '𝋪'
    MayanNumeralTen,
    /// \u{1d2eb}: '𝋫'
    MayanNumeralEleven,
    /// \u{1d2ec}: '𝋬'
    MayanNumeralTwelve,
    /// \u{1d2ed}: '𝋭'
    MayanNumeralThirteen,
    /// \u{1d2ee}: '𝋮'
    MayanNumeralFourteen,
    /// \u{1d2ef}: '𝋯'
    MayanNumeralFifteen,
    /// \u{1d2f0}: '𝋰'
    MayanNumeralSixteen,
    /// \u{1d2f1}: '𝋱'
    MayanNumeralSeventeen,
    /// \u{1d2f2}: '𝋲'
    MayanNumeralEighteen,
    /// \u{1d2f3}: '𝋳'
    MayanNumeralNineteen,
}

impl Into<char> for MayanNumerals {
    fn into(self) -> char {
        use constants::*;
        match self {
            MayanNumerals::MayanNumeralZero => MAYAN_NUMERAL_ZERO,
            MayanNumerals::MayanNumeralOne => MAYAN_NUMERAL_ONE,
            MayanNumerals::MayanNumeralTwo => MAYAN_NUMERAL_TWO,
            MayanNumerals::MayanNumeralThree => MAYAN_NUMERAL_THREE,
            MayanNumerals::MayanNumeralFour => MAYAN_NUMERAL_FOUR,
            MayanNumerals::MayanNumeralFive => MAYAN_NUMERAL_FIVE,
            MayanNumerals::MayanNumeralSix => MAYAN_NUMERAL_SIX,
            MayanNumerals::MayanNumeralSeven => MAYAN_NUMERAL_SEVEN,
            MayanNumerals::MayanNumeralEight => MAYAN_NUMERAL_EIGHT,
            MayanNumerals::MayanNumeralNine => MAYAN_NUMERAL_NINE,
            MayanNumerals::MayanNumeralTen => MAYAN_NUMERAL_TEN,
            MayanNumerals::MayanNumeralEleven => MAYAN_NUMERAL_ELEVEN,
            MayanNumerals::MayanNumeralTwelve => MAYAN_NUMERAL_TWELVE,
            MayanNumerals::MayanNumeralThirteen => MAYAN_NUMERAL_THIRTEEN,
            MayanNumerals::MayanNumeralFourteen => MAYAN_NUMERAL_FOURTEEN,
            MayanNumerals::MayanNumeralFifteen => MAYAN_NUMERAL_FIFTEEN,
            MayanNumerals::MayanNumeralSixteen => MAYAN_NUMERAL_SIXTEEN,
            MayanNumerals::MayanNumeralSeventeen => MAYAN_NUMERAL_SEVENTEEN,
            MayanNumerals::MayanNumeralEighteen => MAYAN_NUMERAL_EIGHTEEN,
            MayanNumerals::MayanNumeralNineteen => MAYAN_NUMERAL_NINETEEN,
        }
    }
}

impl std::convert::TryFrom<char> for MayanNumerals {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MAYAN_NUMERAL_ZERO => Ok(MayanNumerals::MayanNumeralZero),
            MAYAN_NUMERAL_ONE => Ok(MayanNumerals::MayanNumeralOne),
            MAYAN_NUMERAL_TWO => Ok(MayanNumerals::MayanNumeralTwo),
            MAYAN_NUMERAL_THREE => Ok(MayanNumerals::MayanNumeralThree),
            MAYAN_NUMERAL_FOUR => Ok(MayanNumerals::MayanNumeralFour),
            MAYAN_NUMERAL_FIVE => Ok(MayanNumerals::MayanNumeralFive),
            MAYAN_NUMERAL_SIX => Ok(MayanNumerals::MayanNumeralSix),
            MAYAN_NUMERAL_SEVEN => Ok(MayanNumerals::MayanNumeralSeven),
            MAYAN_NUMERAL_EIGHT => Ok(MayanNumerals::MayanNumeralEight),
            MAYAN_NUMERAL_NINE => Ok(MayanNumerals::MayanNumeralNine),
            MAYAN_NUMERAL_TEN => Ok(MayanNumerals::MayanNumeralTen),
            MAYAN_NUMERAL_ELEVEN => Ok(MayanNumerals::MayanNumeralEleven),
            MAYAN_NUMERAL_TWELVE => Ok(MayanNumerals::MayanNumeralTwelve),
            MAYAN_NUMERAL_THIRTEEN => Ok(MayanNumerals::MayanNumeralThirteen),
            MAYAN_NUMERAL_FOURTEEN => Ok(MayanNumerals::MayanNumeralFourteen),
            MAYAN_NUMERAL_FIFTEEN => Ok(MayanNumerals::MayanNumeralFifteen),
            MAYAN_NUMERAL_SIXTEEN => Ok(MayanNumerals::MayanNumeralSixteen),
            MAYAN_NUMERAL_SEVENTEEN => Ok(MayanNumerals::MayanNumeralSeventeen),
            MAYAN_NUMERAL_EIGHTEEN => Ok(MayanNumerals::MayanNumeralEighteen),
            MAYAN_NUMERAL_NINETEEN => Ok(MayanNumerals::MayanNumeralNineteen),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MayanNumerals {
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

impl std::convert::TryFrom<u32> for MayanNumerals {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MayanNumerals {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MayanNumerals {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MayanNumerals::MayanNumeralZero
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MayanNumerals::MayanNumeralZero => "mayan numeral zero",
            MayanNumerals::MayanNumeralOne => "mayan numeral one",
            MayanNumerals::MayanNumeralTwo => "mayan numeral two",
            MayanNumerals::MayanNumeralThree => "mayan numeral three",
            MayanNumerals::MayanNumeralFour => "mayan numeral four",
            MayanNumerals::MayanNumeralFive => "mayan numeral five",
            MayanNumerals::MayanNumeralSix => "mayan numeral six",
            MayanNumerals::MayanNumeralSeven => "mayan numeral seven",
            MayanNumerals::MayanNumeralEight => "mayan numeral eight",
            MayanNumerals::MayanNumeralNine => "mayan numeral nine",
            MayanNumerals::MayanNumeralTen => "mayan numeral ten",
            MayanNumerals::MayanNumeralEleven => "mayan numeral eleven",
            MayanNumerals::MayanNumeralTwelve => "mayan numeral twelve",
            MayanNumerals::MayanNumeralThirteen => "mayan numeral thirteen",
            MayanNumerals::MayanNumeralFourteen => "mayan numeral fourteen",
            MayanNumerals::MayanNumeralFifteen => "mayan numeral fifteen",
            MayanNumerals::MayanNumeralSixteen => "mayan numeral sixteen",
            MayanNumerals::MayanNumeralSeventeen => "mayan numeral seventeen",
            MayanNumerals::MayanNumeralEighteen => "mayan numeral eighteen",
            MayanNumerals::MayanNumeralNineteen => "mayan numeral nineteen",
        }
    }
}
