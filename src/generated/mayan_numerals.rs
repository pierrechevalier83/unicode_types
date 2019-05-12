
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
        match self {
            MayanNumerals::MayanNumeralZero => '𝋠',
            MayanNumerals::MayanNumeralOne => '𝋡',
            MayanNumerals::MayanNumeralTwo => '𝋢',
            MayanNumerals::MayanNumeralThree => '𝋣',
            MayanNumerals::MayanNumeralFour => '𝋤',
            MayanNumerals::MayanNumeralFive => '𝋥',
            MayanNumerals::MayanNumeralSix => '𝋦',
            MayanNumerals::MayanNumeralSeven => '𝋧',
            MayanNumerals::MayanNumeralEight => '𝋨',
            MayanNumerals::MayanNumeralNine => '𝋩',
            MayanNumerals::MayanNumeralTen => '𝋪',
            MayanNumerals::MayanNumeralEleven => '𝋫',
            MayanNumerals::MayanNumeralTwelve => '𝋬',
            MayanNumerals::MayanNumeralThirteen => '𝋭',
            MayanNumerals::MayanNumeralFourteen => '𝋮',
            MayanNumerals::MayanNumeralFifteen => '𝋯',
            MayanNumerals::MayanNumeralSixteen => '𝋰',
            MayanNumerals::MayanNumeralSeventeen => '𝋱',
            MayanNumerals::MayanNumeralEighteen => '𝋲',
            MayanNumerals::MayanNumeralNineteen => '𝋳',
        }
    }
}

impl std::convert::TryFrom<char> for MayanNumerals {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𝋠' => Ok(MayanNumerals::MayanNumeralZero),
            '𝋡' => Ok(MayanNumerals::MayanNumeralOne),
            '𝋢' => Ok(MayanNumerals::MayanNumeralTwo),
            '𝋣' => Ok(MayanNumerals::MayanNumeralThree),
            '𝋤' => Ok(MayanNumerals::MayanNumeralFour),
            '𝋥' => Ok(MayanNumerals::MayanNumeralFive),
            '𝋦' => Ok(MayanNumerals::MayanNumeralSix),
            '𝋧' => Ok(MayanNumerals::MayanNumeralSeven),
            '𝋨' => Ok(MayanNumerals::MayanNumeralEight),
            '𝋩' => Ok(MayanNumerals::MayanNumeralNine),
            '𝋪' => Ok(MayanNumerals::MayanNumeralTen),
            '𝋫' => Ok(MayanNumerals::MayanNumeralEleven),
            '𝋬' => Ok(MayanNumerals::MayanNumeralTwelve),
            '𝋭' => Ok(MayanNumerals::MayanNumeralThirteen),
            '𝋮' => Ok(MayanNumerals::MayanNumeralFourteen),
            '𝋯' => Ok(MayanNumerals::MayanNumeralFifteen),
            '𝋰' => Ok(MayanNumerals::MayanNumeralSixteen),
            '𝋱' => Ok(MayanNumerals::MayanNumeralSeventeen),
            '𝋲' => Ok(MayanNumerals::MayanNumeralEighteen),
            '𝋳' => Ok(MayanNumerals::MayanNumeralNineteen),
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
