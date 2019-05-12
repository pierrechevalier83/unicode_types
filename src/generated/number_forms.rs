
/// An enum to represent all characters in the NumberForms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NumberForms {
    /// \u{2150}: '⅐'
    VulgarFractionOneSeventh,
    /// \u{2151}: '⅑'
    VulgarFractionOneNinth,
    /// \u{2152}: '⅒'
    VulgarFractionOneTenth,
    /// \u{2153}: '⅓'
    VulgarFractionOneThird,
    /// \u{2154}: '⅔'
    VulgarFractionTwoThirds,
    /// \u{2155}: '⅕'
    VulgarFractionOneFifth,
    /// \u{2156}: '⅖'
    VulgarFractionTwoFifths,
    /// \u{2157}: '⅗'
    VulgarFractionThreeFifths,
    /// \u{2158}: '⅘'
    VulgarFractionFourFifths,
    /// \u{2159}: '⅙'
    VulgarFractionOneSixth,
    /// \u{215a}: '⅚'
    VulgarFractionFiveSixths,
    /// \u{215b}: '⅛'
    VulgarFractionOneEighth,
    /// \u{215c}: '⅜'
    VulgarFractionThreeEighths,
    /// \u{215d}: '⅝'
    VulgarFractionFiveEighths,
    /// \u{215e}: '⅞'
    VulgarFractionSevenEighths,
    /// \u{215f}: '⅟'
    FractionNumeratorOne,
    /// \u{2160}: 'Ⅰ'
    RomanNumeralOne,
    /// \u{2161}: 'Ⅱ'
    RomanNumeralTwo,
    /// \u{2162}: 'Ⅲ'
    RomanNumeralThree,
    /// \u{2163}: 'Ⅳ'
    RomanNumeralFour,
    /// \u{2164}: 'Ⅴ'
    RomanNumeralFive,
    /// \u{2165}: 'Ⅵ'
    RomanNumeralSix,
    /// \u{2166}: 'Ⅶ'
    RomanNumeralSeven,
    /// \u{2167}: 'Ⅷ'
    RomanNumeralEight,
    /// \u{2168}: 'Ⅸ'
    RomanNumeralNine,
    /// \u{2169}: 'Ⅹ'
    RomanNumeralTen,
    /// \u{216a}: 'Ⅺ'
    RomanNumeralEleven,
    /// \u{216b}: 'Ⅻ'
    RomanNumeralTwelve,
    /// \u{216c}: 'Ⅼ'
    RomanNumeralFifty,
    /// \u{216d}: 'Ⅽ'
    RomanNumeralOneHundred,
    /// \u{216e}: 'Ⅾ'
    RomanNumeralFiveHundred,
    /// \u{216f}: 'Ⅿ'
    RomanNumeralOneThousand,
    /// \u{2170}: 'ⅰ'
    SmallRomanNumeralOne,
    /// \u{2171}: 'ⅱ'
    SmallRomanNumeralTwo,
    /// \u{2172}: 'ⅲ'
    SmallRomanNumeralThree,
    /// \u{2173}: 'ⅳ'
    SmallRomanNumeralFour,
    /// \u{2174}: 'ⅴ'
    SmallRomanNumeralFive,
    /// \u{2175}: 'ⅵ'
    SmallRomanNumeralSix,
    /// \u{2176}: 'ⅶ'
    SmallRomanNumeralSeven,
    /// \u{2177}: 'ⅷ'
    SmallRomanNumeralEight,
    /// \u{2178}: 'ⅸ'
    SmallRomanNumeralNine,
    /// \u{2179}: 'ⅹ'
    SmallRomanNumeralTen,
    /// \u{217a}: 'ⅺ'
    SmallRomanNumeralEleven,
    /// \u{217b}: 'ⅻ'
    SmallRomanNumeralTwelve,
    /// \u{217c}: 'ⅼ'
    SmallRomanNumeralFifty,
    /// \u{217d}: 'ⅽ'
    SmallRomanNumeralOneHundred,
    /// \u{217e}: 'ⅾ'
    SmallRomanNumeralFiveHundred,
    /// \u{217f}: 'ⅿ'
    SmallRomanNumeralOneThousand,
    /// \u{2180}: 'ↀ'
    RomanNumeralOneThousandCD,
    /// \u{2181}: 'ↁ'
    RomanNumeralFiveThousand,
    /// \u{2182}: 'ↂ'
    RomanNumeralTenThousand,
    /// \u{2183}: 'Ↄ'
    RomanNumeralReversedOneHundred,
    /// \u{2184}: 'ↄ'
    LatinSmallLetterReversedC,
    /// \u{2185}: 'ↅ'
    RomanNumeralSixLateForm,
    /// \u{2186}: 'ↆ'
    RomanNumeralFiftyEarlyForm,
    /// \u{2187}: 'ↇ'
    RomanNumeralFiftyThousand,
    /// \u{2188}: 'ↈ'
    RomanNumeralOneHundredThousand,
    /// \u{2189}: '↉'
    VulgarFractionZeroThirds,
    /// \u{218a}: '↊'
    TurnedDigitTwo,
    /// \u{218b}: '↋'
    TurnedDigitThree,
}

impl Into<char> for NumberForms {
    fn into(self) -> char {
        match self {
            NumberForms::VulgarFractionOneSeventh => '⅐',
            NumberForms::VulgarFractionOneNinth => '⅑',
            NumberForms::VulgarFractionOneTenth => '⅒',
            NumberForms::VulgarFractionOneThird => '⅓',
            NumberForms::VulgarFractionTwoThirds => '⅔',
            NumberForms::VulgarFractionOneFifth => '⅕',
            NumberForms::VulgarFractionTwoFifths => '⅖',
            NumberForms::VulgarFractionThreeFifths => '⅗',
            NumberForms::VulgarFractionFourFifths => '⅘',
            NumberForms::VulgarFractionOneSixth => '⅙',
            NumberForms::VulgarFractionFiveSixths => '⅚',
            NumberForms::VulgarFractionOneEighth => '⅛',
            NumberForms::VulgarFractionThreeEighths => '⅜',
            NumberForms::VulgarFractionFiveEighths => '⅝',
            NumberForms::VulgarFractionSevenEighths => '⅞',
            NumberForms::FractionNumeratorOne => '⅟',
            NumberForms::RomanNumeralOne => 'Ⅰ',
            NumberForms::RomanNumeralTwo => 'Ⅱ',
            NumberForms::RomanNumeralThree => 'Ⅲ',
            NumberForms::RomanNumeralFour => 'Ⅳ',
            NumberForms::RomanNumeralFive => 'Ⅴ',
            NumberForms::RomanNumeralSix => 'Ⅵ',
            NumberForms::RomanNumeralSeven => 'Ⅶ',
            NumberForms::RomanNumeralEight => 'Ⅷ',
            NumberForms::RomanNumeralNine => 'Ⅸ',
            NumberForms::RomanNumeralTen => 'Ⅹ',
            NumberForms::RomanNumeralEleven => 'Ⅺ',
            NumberForms::RomanNumeralTwelve => 'Ⅻ',
            NumberForms::RomanNumeralFifty => 'Ⅼ',
            NumberForms::RomanNumeralOneHundred => 'Ⅽ',
            NumberForms::RomanNumeralFiveHundred => 'Ⅾ',
            NumberForms::RomanNumeralOneThousand => 'Ⅿ',
            NumberForms::SmallRomanNumeralOne => 'ⅰ',
            NumberForms::SmallRomanNumeralTwo => 'ⅱ',
            NumberForms::SmallRomanNumeralThree => 'ⅲ',
            NumberForms::SmallRomanNumeralFour => 'ⅳ',
            NumberForms::SmallRomanNumeralFive => 'ⅴ',
            NumberForms::SmallRomanNumeralSix => 'ⅵ',
            NumberForms::SmallRomanNumeralSeven => 'ⅶ',
            NumberForms::SmallRomanNumeralEight => 'ⅷ',
            NumberForms::SmallRomanNumeralNine => 'ⅸ',
            NumberForms::SmallRomanNumeralTen => 'ⅹ',
            NumberForms::SmallRomanNumeralEleven => 'ⅺ',
            NumberForms::SmallRomanNumeralTwelve => 'ⅻ',
            NumberForms::SmallRomanNumeralFifty => 'ⅼ',
            NumberForms::SmallRomanNumeralOneHundred => 'ⅽ',
            NumberForms::SmallRomanNumeralFiveHundred => 'ⅾ',
            NumberForms::SmallRomanNumeralOneThousand => 'ⅿ',
            NumberForms::RomanNumeralOneThousandCD => 'ↀ',
            NumberForms::RomanNumeralFiveThousand => 'ↁ',
            NumberForms::RomanNumeralTenThousand => 'ↂ',
            NumberForms::RomanNumeralReversedOneHundred => 'Ↄ',
            NumberForms::LatinSmallLetterReversedC => 'ↄ',
            NumberForms::RomanNumeralSixLateForm => 'ↅ',
            NumberForms::RomanNumeralFiftyEarlyForm => 'ↆ',
            NumberForms::RomanNumeralFiftyThousand => 'ↇ',
            NumberForms::RomanNumeralOneHundredThousand => 'ↈ',
            NumberForms::VulgarFractionZeroThirds => '↉',
            NumberForms::TurnedDigitTwo => '↊',
            NumberForms::TurnedDigitThree => '↋',
        }
    }
}

impl std::convert::TryFrom<char> for NumberForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⅐' => Ok(NumberForms::VulgarFractionOneSeventh),
            '⅑' => Ok(NumberForms::VulgarFractionOneNinth),
            '⅒' => Ok(NumberForms::VulgarFractionOneTenth),
            '⅓' => Ok(NumberForms::VulgarFractionOneThird),
            '⅔' => Ok(NumberForms::VulgarFractionTwoThirds),
            '⅕' => Ok(NumberForms::VulgarFractionOneFifth),
            '⅖' => Ok(NumberForms::VulgarFractionTwoFifths),
            '⅗' => Ok(NumberForms::VulgarFractionThreeFifths),
            '⅘' => Ok(NumberForms::VulgarFractionFourFifths),
            '⅙' => Ok(NumberForms::VulgarFractionOneSixth),
            '⅚' => Ok(NumberForms::VulgarFractionFiveSixths),
            '⅛' => Ok(NumberForms::VulgarFractionOneEighth),
            '⅜' => Ok(NumberForms::VulgarFractionThreeEighths),
            '⅝' => Ok(NumberForms::VulgarFractionFiveEighths),
            '⅞' => Ok(NumberForms::VulgarFractionSevenEighths),
            '⅟' => Ok(NumberForms::FractionNumeratorOne),
            'Ⅰ' => Ok(NumberForms::RomanNumeralOne),
            'Ⅱ' => Ok(NumberForms::RomanNumeralTwo),
            'Ⅲ' => Ok(NumberForms::RomanNumeralThree),
            'Ⅳ' => Ok(NumberForms::RomanNumeralFour),
            'Ⅴ' => Ok(NumberForms::RomanNumeralFive),
            'Ⅵ' => Ok(NumberForms::RomanNumeralSix),
            'Ⅶ' => Ok(NumberForms::RomanNumeralSeven),
            'Ⅷ' => Ok(NumberForms::RomanNumeralEight),
            'Ⅸ' => Ok(NumberForms::RomanNumeralNine),
            'Ⅹ' => Ok(NumberForms::RomanNumeralTen),
            'Ⅺ' => Ok(NumberForms::RomanNumeralEleven),
            'Ⅻ' => Ok(NumberForms::RomanNumeralTwelve),
            'Ⅼ' => Ok(NumberForms::RomanNumeralFifty),
            'Ⅽ' => Ok(NumberForms::RomanNumeralOneHundred),
            'Ⅾ' => Ok(NumberForms::RomanNumeralFiveHundred),
            'Ⅿ' => Ok(NumberForms::RomanNumeralOneThousand),
            'ⅰ' => Ok(NumberForms::SmallRomanNumeralOne),
            'ⅱ' => Ok(NumberForms::SmallRomanNumeralTwo),
            'ⅲ' => Ok(NumberForms::SmallRomanNumeralThree),
            'ⅳ' => Ok(NumberForms::SmallRomanNumeralFour),
            'ⅴ' => Ok(NumberForms::SmallRomanNumeralFive),
            'ⅵ' => Ok(NumberForms::SmallRomanNumeralSix),
            'ⅶ' => Ok(NumberForms::SmallRomanNumeralSeven),
            'ⅷ' => Ok(NumberForms::SmallRomanNumeralEight),
            'ⅸ' => Ok(NumberForms::SmallRomanNumeralNine),
            'ⅹ' => Ok(NumberForms::SmallRomanNumeralTen),
            'ⅺ' => Ok(NumberForms::SmallRomanNumeralEleven),
            'ⅻ' => Ok(NumberForms::SmallRomanNumeralTwelve),
            'ⅼ' => Ok(NumberForms::SmallRomanNumeralFifty),
            'ⅽ' => Ok(NumberForms::SmallRomanNumeralOneHundred),
            'ⅾ' => Ok(NumberForms::SmallRomanNumeralFiveHundred),
            'ⅿ' => Ok(NumberForms::SmallRomanNumeralOneThousand),
            'ↀ' => Ok(NumberForms::RomanNumeralOneThousandCD),
            'ↁ' => Ok(NumberForms::RomanNumeralFiveThousand),
            'ↂ' => Ok(NumberForms::RomanNumeralTenThousand),
            'Ↄ' => Ok(NumberForms::RomanNumeralReversedOneHundred),
            'ↄ' => Ok(NumberForms::LatinSmallLetterReversedC),
            'ↅ' => Ok(NumberForms::RomanNumeralSixLateForm),
            'ↆ' => Ok(NumberForms::RomanNumeralFiftyEarlyForm),
            'ↇ' => Ok(NumberForms::RomanNumeralFiftyThousand),
            'ↈ' => Ok(NumberForms::RomanNumeralOneHundredThousand),
            '↉' => Ok(NumberForms::VulgarFractionZeroThirds),
            '↊' => Ok(NumberForms::TurnedDigitTwo),
            '↋' => Ok(NumberForms::TurnedDigitThree),
            _ => Err(()),
        }
    }
}

impl Into<u32> for NumberForms {
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

impl std::convert::TryFrom<u32> for NumberForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for NumberForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl NumberForms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        NumberForms::VulgarFractionOneSeventh
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            NumberForms::VulgarFractionOneSeventh => "vulgar fraction one seventh",
            NumberForms::VulgarFractionOneNinth => "vulgar fraction one ninth",
            NumberForms::VulgarFractionOneTenth => "vulgar fraction one tenth",
            NumberForms::VulgarFractionOneThird => "vulgar fraction one third",
            NumberForms::VulgarFractionTwoThirds => "vulgar fraction two thirds",
            NumberForms::VulgarFractionOneFifth => "vulgar fraction one fifth",
            NumberForms::VulgarFractionTwoFifths => "vulgar fraction two fifths",
            NumberForms::VulgarFractionThreeFifths => "vulgar fraction three fifths",
            NumberForms::VulgarFractionFourFifths => "vulgar fraction four fifths",
            NumberForms::VulgarFractionOneSixth => "vulgar fraction one sixth",
            NumberForms::VulgarFractionFiveSixths => "vulgar fraction five sixths",
            NumberForms::VulgarFractionOneEighth => "vulgar fraction one eighth",
            NumberForms::VulgarFractionThreeEighths => "vulgar fraction three eighths",
            NumberForms::VulgarFractionFiveEighths => "vulgar fraction five eighths",
            NumberForms::VulgarFractionSevenEighths => "vulgar fraction seven eighths",
            NumberForms::FractionNumeratorOne => "fraction numerator one",
            NumberForms::RomanNumeralOne => "roman numeral one",
            NumberForms::RomanNumeralTwo => "roman numeral two",
            NumberForms::RomanNumeralThree => "roman numeral three",
            NumberForms::RomanNumeralFour => "roman numeral four",
            NumberForms::RomanNumeralFive => "roman numeral five",
            NumberForms::RomanNumeralSix => "roman numeral six",
            NumberForms::RomanNumeralSeven => "roman numeral seven",
            NumberForms::RomanNumeralEight => "roman numeral eight",
            NumberForms::RomanNumeralNine => "roman numeral nine",
            NumberForms::RomanNumeralTen => "roman numeral ten",
            NumberForms::RomanNumeralEleven => "roman numeral eleven",
            NumberForms::RomanNumeralTwelve => "roman numeral twelve",
            NumberForms::RomanNumeralFifty => "roman numeral fifty",
            NumberForms::RomanNumeralOneHundred => "roman numeral one hundred",
            NumberForms::RomanNumeralFiveHundred => "roman numeral five hundred",
            NumberForms::RomanNumeralOneThousand => "roman numeral one thousand",
            NumberForms::SmallRomanNumeralOne => "small roman numeral one",
            NumberForms::SmallRomanNumeralTwo => "small roman numeral two",
            NumberForms::SmallRomanNumeralThree => "small roman numeral three",
            NumberForms::SmallRomanNumeralFour => "small roman numeral four",
            NumberForms::SmallRomanNumeralFive => "small roman numeral five",
            NumberForms::SmallRomanNumeralSix => "small roman numeral six",
            NumberForms::SmallRomanNumeralSeven => "small roman numeral seven",
            NumberForms::SmallRomanNumeralEight => "small roman numeral eight",
            NumberForms::SmallRomanNumeralNine => "small roman numeral nine",
            NumberForms::SmallRomanNumeralTen => "small roman numeral ten",
            NumberForms::SmallRomanNumeralEleven => "small roman numeral eleven",
            NumberForms::SmallRomanNumeralTwelve => "small roman numeral twelve",
            NumberForms::SmallRomanNumeralFifty => "small roman numeral fifty",
            NumberForms::SmallRomanNumeralOneHundred => "small roman numeral one hundred",
            NumberForms::SmallRomanNumeralFiveHundred => "small roman numeral five hundred",
            NumberForms::SmallRomanNumeralOneThousand => "small roman numeral one thousand",
            NumberForms::RomanNumeralOneThousandCD => "roman numeral one thousand c d",
            NumberForms::RomanNumeralFiveThousand => "roman numeral five thousand",
            NumberForms::RomanNumeralTenThousand => "roman numeral ten thousand",
            NumberForms::RomanNumeralReversedOneHundred => "roman numeral reversed one hundred",
            NumberForms::LatinSmallLetterReversedC => "latin small letter reversed c",
            NumberForms::RomanNumeralSixLateForm => "roman numeral six late form",
            NumberForms::RomanNumeralFiftyEarlyForm => "roman numeral fifty early form",
            NumberForms::RomanNumeralFiftyThousand => "roman numeral fifty thousand",
            NumberForms::RomanNumeralOneHundredThousand => "roman numeral one hundred thousand",
            NumberForms::VulgarFractionZeroThirds => "vulgar fraction zero thirds",
            NumberForms::TurnedDigitTwo => "turned digit two",
            NumberForms::TurnedDigitThree => "turned digit three",
        }
    }
}
