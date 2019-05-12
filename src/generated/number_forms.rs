
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("NumberForms{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
