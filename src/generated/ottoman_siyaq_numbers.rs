
/// An enum to represent all characters in the OttomanSiyaqNumbers block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OttomanSiyaqNumbers {
    /// \u{1ed01}: '𞴁'
    OttomanSiyaqNumberOne,
    /// \u{1ed02}: '𞴂'
    OttomanSiyaqNumberTwo,
    /// \u{1ed03}: '𞴃'
    OttomanSiyaqNumberThree,
    /// \u{1ed04}: '𞴄'
    OttomanSiyaqNumberFour,
    /// \u{1ed05}: '𞴅'
    OttomanSiyaqNumberFive,
    /// \u{1ed06}: '𞴆'
    OttomanSiyaqNumberSix,
    /// \u{1ed07}: '𞴇'
    OttomanSiyaqNumberSeven,
    /// \u{1ed08}: '𞴈'
    OttomanSiyaqNumberEight,
    /// \u{1ed09}: '𞴉'
    OttomanSiyaqNumberNine,
    /// \u{1ed0a}: '𞴊'
    OttomanSiyaqNumberTen,
    /// \u{1ed0b}: '𞴋'
    OttomanSiyaqNumberTwenty,
    /// \u{1ed0c}: '𞴌'
    OttomanSiyaqNumberThirty,
    /// \u{1ed0d}: '𞴍'
    OttomanSiyaqNumberForty,
    /// \u{1ed0e}: '𞴎'
    OttomanSiyaqNumberFifty,
    /// \u{1ed0f}: '𞴏'
    OttomanSiyaqNumberSixty,
    /// \u{1ed10}: '𞴐'
    OttomanSiyaqNumberSeventy,
    /// \u{1ed11}: '𞴑'
    OttomanSiyaqNumberEighty,
    /// \u{1ed12}: '𞴒'
    OttomanSiyaqNumberNinety,
    /// \u{1ed13}: '𞴓'
    OttomanSiyaqNumberOneHundred,
    /// \u{1ed14}: '𞴔'
    OttomanSiyaqNumberTwoHundred,
    /// \u{1ed15}: '𞴕'
    OttomanSiyaqNumberThreeHundred,
    /// \u{1ed16}: '𞴖'
    OttomanSiyaqNumberFourHundred,
    /// \u{1ed17}: '𞴗'
    OttomanSiyaqNumberFiveHundred,
    /// \u{1ed18}: '𞴘'
    OttomanSiyaqNumberSixHundred,
    /// \u{1ed19}: '𞴙'
    OttomanSiyaqNumberSevenHundred,
    /// \u{1ed1a}: '𞴚'
    OttomanSiyaqNumberEightHundred,
    /// \u{1ed1b}: '𞴛'
    OttomanSiyaqNumberNineHundred,
    /// \u{1ed1c}: '𞴜'
    OttomanSiyaqNumberOneThousand,
    /// \u{1ed1d}: '𞴝'
    OttomanSiyaqNumberTwoThousand,
    /// \u{1ed1e}: '𞴞'
    OttomanSiyaqNumberThreeThousand,
    /// \u{1ed1f}: '𞴟'
    OttomanSiyaqNumberFourThousand,
    /// \u{1ed20}: '𞴠'
    OttomanSiyaqNumberFiveThousand,
    /// \u{1ed21}: '𞴡'
    OttomanSiyaqNumberSixThousand,
    /// \u{1ed22}: '𞴢'
    OttomanSiyaqNumberSevenThousand,
    /// \u{1ed23}: '𞴣'
    OttomanSiyaqNumberEightThousand,
    /// \u{1ed24}: '𞴤'
    OttomanSiyaqNumberNineThousand,
    /// \u{1ed25}: '𞴥'
    OttomanSiyaqNumberTenThousand,
    /// \u{1ed26}: '𞴦'
    OttomanSiyaqNumberTwentyThousand,
    /// \u{1ed27}: '𞴧'
    OttomanSiyaqNumberThirtyThousand,
    /// \u{1ed28}: '𞴨'
    OttomanSiyaqNumberFortyThousand,
    /// \u{1ed29}: '𞴩'
    OttomanSiyaqNumberFiftyThousand,
    /// \u{1ed2a}: '𞴪'
    OttomanSiyaqNumberSixtyThousand,
    /// \u{1ed2b}: '𞴫'
    OttomanSiyaqNumberSeventyThousand,
    /// \u{1ed2c}: '𞴬'
    OttomanSiyaqNumberEightyThousand,
    /// \u{1ed2d}: '𞴭'
    OttomanSiyaqNumberNinetyThousand,
    /// \u{1ed2e}: '𞴮'
    OttomanSiyaqMarratan,
    /// \u{1ed2f}: '𞴯'
    OttomanSiyaqAlternateNumberTwo,
    /// \u{1ed30}: '𞴰'
    OttomanSiyaqAlternateNumberThree,
    /// \u{1ed31}: '𞴱'
    OttomanSiyaqAlternateNumberFour,
    /// \u{1ed32}: '𞴲'
    OttomanSiyaqAlternateNumberFive,
    /// \u{1ed33}: '𞴳'
    OttomanSiyaqAlternateNumberSix,
    /// \u{1ed34}: '𞴴'
    OttomanSiyaqAlternateNumberSeven,
    /// \u{1ed35}: '𞴵'
    OttomanSiyaqAlternateNumberEight,
    /// \u{1ed36}: '𞴶'
    OttomanSiyaqAlternateNumberNine,
    /// \u{1ed37}: '𞴷'
    OttomanSiyaqAlternateNumberTen,
    /// \u{1ed38}: '𞴸'
    OttomanSiyaqAlternateNumberFourHundred,
    /// \u{1ed39}: '𞴹'
    OttomanSiyaqAlternateNumberSixHundred,
    /// \u{1ed3a}: '𞴺'
    OttomanSiyaqAlternateNumberTwoThousand,
    /// \u{1ed3b}: '𞴻'
    OttomanSiyaqAlternateNumberTenThousand,
    /// \u{1ed3c}: '𞴼'
    OttomanSiyaqFractionOneHalf,
    /// \u{1ed3d}: '𞴽'
    OttomanSiyaqFractionOneSixth,
}

impl Into<char> for OttomanSiyaqNumbers {
    fn into(self) -> char {
        match self {
            OttomanSiyaqNumbers::OttomanSiyaqNumberOne => '𞴁',
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwo => '𞴂',
            OttomanSiyaqNumbers::OttomanSiyaqNumberThree => '𞴃',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFour => '𞴄',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFive => '𞴅',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSix => '𞴆',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSeven => '𞴇',
            OttomanSiyaqNumbers::OttomanSiyaqNumberEight => '𞴈',
            OttomanSiyaqNumbers::OttomanSiyaqNumberNine => '𞴉',
            OttomanSiyaqNumbers::OttomanSiyaqNumberTen => '𞴊',
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwenty => '𞴋',
            OttomanSiyaqNumbers::OttomanSiyaqNumberThirty => '𞴌',
            OttomanSiyaqNumbers::OttomanSiyaqNumberForty => '𞴍',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFifty => '𞴎',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixty => '𞴏',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSeventy => '𞴐',
            OttomanSiyaqNumbers::OttomanSiyaqNumberEighty => '𞴑',
            OttomanSiyaqNumbers::OttomanSiyaqNumberNinety => '𞴒',
            OttomanSiyaqNumbers::OttomanSiyaqNumberOneHundred => '𞴓',
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwoHundred => '𞴔',
            OttomanSiyaqNumbers::OttomanSiyaqNumberThreeHundred => '𞴕',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFourHundred => '𞴖',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFiveHundred => '𞴗',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixHundred => '𞴘',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSevenHundred => '𞴙',
            OttomanSiyaqNumbers::OttomanSiyaqNumberEightHundred => '𞴚',
            OttomanSiyaqNumbers::OttomanSiyaqNumberNineHundred => '𞴛',
            OttomanSiyaqNumbers::OttomanSiyaqNumberOneThousand => '𞴜',
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwoThousand => '𞴝',
            OttomanSiyaqNumbers::OttomanSiyaqNumberThreeThousand => '𞴞',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFourThousand => '𞴟',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFiveThousand => '𞴠',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixThousand => '𞴡',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSevenThousand => '𞴢',
            OttomanSiyaqNumbers::OttomanSiyaqNumberEightThousand => '𞴣',
            OttomanSiyaqNumbers::OttomanSiyaqNumberNineThousand => '𞴤',
            OttomanSiyaqNumbers::OttomanSiyaqNumberTenThousand => '𞴥',
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwentyThousand => '𞴦',
            OttomanSiyaqNumbers::OttomanSiyaqNumberThirtyThousand => '𞴧',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFortyThousand => '𞴨',
            OttomanSiyaqNumbers::OttomanSiyaqNumberFiftyThousand => '𞴩',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixtyThousand => '𞴪',
            OttomanSiyaqNumbers::OttomanSiyaqNumberSeventyThousand => '𞴫',
            OttomanSiyaqNumbers::OttomanSiyaqNumberEightyThousand => '𞴬',
            OttomanSiyaqNumbers::OttomanSiyaqNumberNinetyThousand => '𞴭',
            OttomanSiyaqNumbers::OttomanSiyaqMarratan => '𞴮',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTwo => '𞴯',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberThree => '𞴰',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFour => '𞴱',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFive => '𞴲',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSix => '𞴳',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSeven => '𞴴',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberEight => '𞴵',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberNine => '𞴶',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTen => '𞴷',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFourHundred => '𞴸',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSixHundred => '𞴹',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTwoThousand => '𞴺',
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTenThousand => '𞴻',
            OttomanSiyaqNumbers::OttomanSiyaqFractionOneHalf => '𞴼',
            OttomanSiyaqNumbers::OttomanSiyaqFractionOneSixth => '𞴽',
        }
    }
}

impl std::convert::TryFrom<char> for OttomanSiyaqNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞴁' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberOne),
            '𞴂' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberTwo),
            '𞴃' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberThree),
            '𞴄' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFour),
            '𞴅' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFive),
            '𞴆' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSix),
            '𞴇' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSeven),
            '𞴈' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberEight),
            '𞴉' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberNine),
            '𞴊' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberTen),
            '𞴋' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberTwenty),
            '𞴌' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberThirty),
            '𞴍' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberForty),
            '𞴎' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFifty),
            '𞴏' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSixty),
            '𞴐' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSeventy),
            '𞴑' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberEighty),
            '𞴒' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberNinety),
            '𞴓' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberOneHundred),
            '𞴔' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberTwoHundred),
            '𞴕' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberThreeHundred),
            '𞴖' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFourHundred),
            '𞴗' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFiveHundred),
            '𞴘' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSixHundred),
            '𞴙' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSevenHundred),
            '𞴚' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberEightHundred),
            '𞴛' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberNineHundred),
            '𞴜' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberOneThousand),
            '𞴝' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberTwoThousand),
            '𞴞' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberThreeThousand),
            '𞴟' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFourThousand),
            '𞴠' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFiveThousand),
            '𞴡' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSixThousand),
            '𞴢' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSevenThousand),
            '𞴣' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberEightThousand),
            '𞴤' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberNineThousand),
            '𞴥' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberTenThousand),
            '𞴦' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberTwentyThousand),
            '𞴧' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberThirtyThousand),
            '𞴨' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFortyThousand),
            '𞴩' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberFiftyThousand),
            '𞴪' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSixtyThousand),
            '𞴫' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberSeventyThousand),
            '𞴬' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberEightyThousand),
            '𞴭' => Ok(OttomanSiyaqNumbers::OttomanSiyaqNumberNinetyThousand),
            '𞴮' => Ok(OttomanSiyaqNumbers::OttomanSiyaqMarratan),
            '𞴯' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTwo),
            '𞴰' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberThree),
            '𞴱' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFour),
            '𞴲' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFive),
            '𞴳' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSix),
            '𞴴' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSeven),
            '𞴵' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberEight),
            '𞴶' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberNine),
            '𞴷' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTen),
            '𞴸' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFourHundred),
            '𞴹' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSixHundred),
            '𞴺' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTwoThousand),
            '𞴻' => Ok(OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTenThousand),
            '𞴼' => Ok(OttomanSiyaqNumbers::OttomanSiyaqFractionOneHalf),
            '𞴽' => Ok(OttomanSiyaqNumbers::OttomanSiyaqFractionOneSixth),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OttomanSiyaqNumbers {
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

impl std::convert::TryFrom<u32> for OttomanSiyaqNumbers {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OttomanSiyaqNumbers {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OttomanSiyaqNumbers {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OttomanSiyaqNumbers::OttomanSiyaqNumberOne
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OttomanSiyaqNumbers::OttomanSiyaqNumberOne => "ottoman siyaq number one",
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwo => "ottoman siyaq number two",
            OttomanSiyaqNumbers::OttomanSiyaqNumberThree => "ottoman siyaq number three",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFour => "ottoman siyaq number four",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFive => "ottoman siyaq number five",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSix => "ottoman siyaq number six",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSeven => "ottoman siyaq number seven",
            OttomanSiyaqNumbers::OttomanSiyaqNumberEight => "ottoman siyaq number eight",
            OttomanSiyaqNumbers::OttomanSiyaqNumberNine => "ottoman siyaq number nine",
            OttomanSiyaqNumbers::OttomanSiyaqNumberTen => "ottoman siyaq number ten",
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwenty => "ottoman siyaq number twenty",
            OttomanSiyaqNumbers::OttomanSiyaqNumberThirty => "ottoman siyaq number thirty",
            OttomanSiyaqNumbers::OttomanSiyaqNumberForty => "ottoman siyaq number forty",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFifty => "ottoman siyaq number fifty",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixty => "ottoman siyaq number sixty",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSeventy => "ottoman siyaq number seventy",
            OttomanSiyaqNumbers::OttomanSiyaqNumberEighty => "ottoman siyaq number eighty",
            OttomanSiyaqNumbers::OttomanSiyaqNumberNinety => "ottoman siyaq number ninety",
            OttomanSiyaqNumbers::OttomanSiyaqNumberOneHundred => "ottoman siyaq number one hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwoHundred => "ottoman siyaq number two hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberThreeHundred => "ottoman siyaq number three hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFourHundred => "ottoman siyaq number four hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFiveHundred => "ottoman siyaq number five hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixHundred => "ottoman siyaq number six hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSevenHundred => "ottoman siyaq number seven hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberEightHundred => "ottoman siyaq number eight hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberNineHundred => "ottoman siyaq number nine hundred",
            OttomanSiyaqNumbers::OttomanSiyaqNumberOneThousand => "ottoman siyaq number one thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwoThousand => "ottoman siyaq number two thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberThreeThousand => "ottoman siyaq number three thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFourThousand => "ottoman siyaq number four thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFiveThousand => "ottoman siyaq number five thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixThousand => "ottoman siyaq number six thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSevenThousand => "ottoman siyaq number seven thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberEightThousand => "ottoman siyaq number eight thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberNineThousand => "ottoman siyaq number nine thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberTenThousand => "ottoman siyaq number ten thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberTwentyThousand => "ottoman siyaq number twenty thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberThirtyThousand => "ottoman siyaq number thirty thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFortyThousand => "ottoman siyaq number forty thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberFiftyThousand => "ottoman siyaq number fifty thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSixtyThousand => "ottoman siyaq number sixty thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberSeventyThousand => "ottoman siyaq number seventy thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberEightyThousand => "ottoman siyaq number eighty thousand",
            OttomanSiyaqNumbers::OttomanSiyaqNumberNinetyThousand => "ottoman siyaq number ninety thousand",
            OttomanSiyaqNumbers::OttomanSiyaqMarratan => "ottoman siyaq marratan",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTwo => "ottoman siyaq alternate number two",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberThree => "ottoman siyaq alternate number three",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFour => "ottoman siyaq alternate number four",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFive => "ottoman siyaq alternate number five",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSix => "ottoman siyaq alternate number six",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSeven => "ottoman siyaq alternate number seven",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberEight => "ottoman siyaq alternate number eight",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberNine => "ottoman siyaq alternate number nine",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTen => "ottoman siyaq alternate number ten",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberFourHundred => "ottoman siyaq alternate number four hundred",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberSixHundred => "ottoman siyaq alternate number six hundred",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTwoThousand => "ottoman siyaq alternate number two thousand",
            OttomanSiyaqNumbers::OttomanSiyaqAlternateNumberTenThousand => "ottoman siyaq alternate number ten thousand",
            OttomanSiyaqNumbers::OttomanSiyaqFractionOneHalf => "ottoman siyaq fraction one half",
            OttomanSiyaqNumbers::OttomanSiyaqFractionOneSixth => "ottoman siyaq fraction one sixth",
        }
    }
}
