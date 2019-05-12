
/// An enum to represent all characters in the AegeanNumbers block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AegeanNumbers {
    /// \u{10100}: '𐄀'
    AegeanWordSeparatorLine,
    /// \u{10101}: '𐄁'
    AegeanWordSeparatorDot,
    /// \u{10102}: '𐄂'
    AegeanCheckMark,
    /// \u{10107}: '𐄇'
    AegeanNumberOne,
    /// \u{10108}: '𐄈'
    AegeanNumberTwo,
    /// \u{10109}: '𐄉'
    AegeanNumberThree,
    /// \u{1010a}: '𐄊'
    AegeanNumberFour,
    /// \u{1010b}: '𐄋'
    AegeanNumberFive,
    /// \u{1010c}: '𐄌'
    AegeanNumberSix,
    /// \u{1010d}: '𐄍'
    AegeanNumberSeven,
    /// \u{1010e}: '𐄎'
    AegeanNumberEight,
    /// \u{1010f}: '𐄏'
    AegeanNumberNine,
    /// \u{10110}: '𐄐'
    AegeanNumberTen,
    /// \u{10111}: '𐄑'
    AegeanNumberTwenty,
    /// \u{10112}: '𐄒'
    AegeanNumberThirty,
    /// \u{10113}: '𐄓'
    AegeanNumberForty,
    /// \u{10114}: '𐄔'
    AegeanNumberFifty,
    /// \u{10115}: '𐄕'
    AegeanNumberSixty,
    /// \u{10116}: '𐄖'
    AegeanNumberSeventy,
    /// \u{10117}: '𐄗'
    AegeanNumberEighty,
    /// \u{10118}: '𐄘'
    AegeanNumberNinety,
    /// \u{10119}: '𐄙'
    AegeanNumberOneHundred,
    /// \u{1011a}: '𐄚'
    AegeanNumberTwoHundred,
    /// \u{1011b}: '𐄛'
    AegeanNumberThreeHundred,
    /// \u{1011c}: '𐄜'
    AegeanNumberFourHundred,
    /// \u{1011d}: '𐄝'
    AegeanNumberFiveHundred,
    /// \u{1011e}: '𐄞'
    AegeanNumberSixHundred,
    /// \u{1011f}: '𐄟'
    AegeanNumberSevenHundred,
    /// \u{10120}: '𐄠'
    AegeanNumberEightHundred,
    /// \u{10121}: '𐄡'
    AegeanNumberNineHundred,
    /// \u{10122}: '𐄢'
    AegeanNumberOneThousand,
    /// \u{10123}: '𐄣'
    AegeanNumberTwoThousand,
    /// \u{10124}: '𐄤'
    AegeanNumberThreeThousand,
    /// \u{10125}: '𐄥'
    AegeanNumberFourThousand,
    /// \u{10126}: '𐄦'
    AegeanNumberFiveThousand,
    /// \u{10127}: '𐄧'
    AegeanNumberSixThousand,
    /// \u{10128}: '𐄨'
    AegeanNumberSevenThousand,
    /// \u{10129}: '𐄩'
    AegeanNumberEightThousand,
    /// \u{1012a}: '𐄪'
    AegeanNumberNineThousand,
    /// \u{1012b}: '𐄫'
    AegeanNumberTenThousand,
    /// \u{1012c}: '𐄬'
    AegeanNumberTwentyThousand,
    /// \u{1012d}: '𐄭'
    AegeanNumberThirtyThousand,
    /// \u{1012e}: '𐄮'
    AegeanNumberFortyThousand,
    /// \u{1012f}: '𐄯'
    AegeanNumberFiftyThousand,
    /// \u{10130}: '𐄰'
    AegeanNumberSixtyThousand,
    /// \u{10131}: '𐄱'
    AegeanNumberSeventyThousand,
    /// \u{10132}: '𐄲'
    AegeanNumberEightyThousand,
    /// \u{10133}: '𐄳'
    AegeanNumberNinetyThousand,
    /// \u{10137}: '𐄷'
    AegeanWeightBaseUnit,
    /// \u{10138}: '𐄸'
    AegeanWeightFirstSubunit,
    /// \u{10139}: '𐄹'
    AegeanWeightSecondSubunit,
    /// \u{1013a}: '𐄺'
    AegeanWeightThirdSubunit,
    /// \u{1013b}: '𐄻'
    AegeanWeightFourthSubunit,
    /// \u{1013c}: '𐄼'
    AegeanDryMeasureFirstSubunit,
    /// \u{1013d}: '𐄽'
    AegeanLiquidMeasureFirstSubunit,
    /// \u{1013e}: '𐄾'
    AegeanMeasureSecondSubunit,
}

impl Into<char> for AegeanNumbers {
    fn into(self) -> char {
        match self {
            AegeanNumbers::AegeanWordSeparatorLine => '𐄀',
            AegeanNumbers::AegeanWordSeparatorDot => '𐄁',
            AegeanNumbers::AegeanCheckMark => '𐄂',
            AegeanNumbers::AegeanNumberOne => '𐄇',
            AegeanNumbers::AegeanNumberTwo => '𐄈',
            AegeanNumbers::AegeanNumberThree => '𐄉',
            AegeanNumbers::AegeanNumberFour => '𐄊',
            AegeanNumbers::AegeanNumberFive => '𐄋',
            AegeanNumbers::AegeanNumberSix => '𐄌',
            AegeanNumbers::AegeanNumberSeven => '𐄍',
            AegeanNumbers::AegeanNumberEight => '𐄎',
            AegeanNumbers::AegeanNumberNine => '𐄏',
            AegeanNumbers::AegeanNumberTen => '𐄐',
            AegeanNumbers::AegeanNumberTwenty => '𐄑',
            AegeanNumbers::AegeanNumberThirty => '𐄒',
            AegeanNumbers::AegeanNumberForty => '𐄓',
            AegeanNumbers::AegeanNumberFifty => '𐄔',
            AegeanNumbers::AegeanNumberSixty => '𐄕',
            AegeanNumbers::AegeanNumberSeventy => '𐄖',
            AegeanNumbers::AegeanNumberEighty => '𐄗',
            AegeanNumbers::AegeanNumberNinety => '𐄘',
            AegeanNumbers::AegeanNumberOneHundred => '𐄙',
            AegeanNumbers::AegeanNumberTwoHundred => '𐄚',
            AegeanNumbers::AegeanNumberThreeHundred => '𐄛',
            AegeanNumbers::AegeanNumberFourHundred => '𐄜',
            AegeanNumbers::AegeanNumberFiveHundred => '𐄝',
            AegeanNumbers::AegeanNumberSixHundred => '𐄞',
            AegeanNumbers::AegeanNumberSevenHundred => '𐄟',
            AegeanNumbers::AegeanNumberEightHundred => '𐄠',
            AegeanNumbers::AegeanNumberNineHundred => '𐄡',
            AegeanNumbers::AegeanNumberOneThousand => '𐄢',
            AegeanNumbers::AegeanNumberTwoThousand => '𐄣',
            AegeanNumbers::AegeanNumberThreeThousand => '𐄤',
            AegeanNumbers::AegeanNumberFourThousand => '𐄥',
            AegeanNumbers::AegeanNumberFiveThousand => '𐄦',
            AegeanNumbers::AegeanNumberSixThousand => '𐄧',
            AegeanNumbers::AegeanNumberSevenThousand => '𐄨',
            AegeanNumbers::AegeanNumberEightThousand => '𐄩',
            AegeanNumbers::AegeanNumberNineThousand => '𐄪',
            AegeanNumbers::AegeanNumberTenThousand => '𐄫',
            AegeanNumbers::AegeanNumberTwentyThousand => '𐄬',
            AegeanNumbers::AegeanNumberThirtyThousand => '𐄭',
            AegeanNumbers::AegeanNumberFortyThousand => '𐄮',
            AegeanNumbers::AegeanNumberFiftyThousand => '𐄯',
            AegeanNumbers::AegeanNumberSixtyThousand => '𐄰',
            AegeanNumbers::AegeanNumberSeventyThousand => '𐄱',
            AegeanNumbers::AegeanNumberEightyThousand => '𐄲',
            AegeanNumbers::AegeanNumberNinetyThousand => '𐄳',
            AegeanNumbers::AegeanWeightBaseUnit => '𐄷',
            AegeanNumbers::AegeanWeightFirstSubunit => '𐄸',
            AegeanNumbers::AegeanWeightSecondSubunit => '𐄹',
            AegeanNumbers::AegeanWeightThirdSubunit => '𐄺',
            AegeanNumbers::AegeanWeightFourthSubunit => '𐄻',
            AegeanNumbers::AegeanDryMeasureFirstSubunit => '𐄼',
            AegeanNumbers::AegeanLiquidMeasureFirstSubunit => '𐄽',
            AegeanNumbers::AegeanMeasureSecondSubunit => '𐄾',
        }
    }
}

impl std::convert::TryFrom<char> for AegeanNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐄀' => Ok(AegeanNumbers::AegeanWordSeparatorLine),
            '𐄁' => Ok(AegeanNumbers::AegeanWordSeparatorDot),
            '𐄂' => Ok(AegeanNumbers::AegeanCheckMark),
            '𐄇' => Ok(AegeanNumbers::AegeanNumberOne),
            '𐄈' => Ok(AegeanNumbers::AegeanNumberTwo),
            '𐄉' => Ok(AegeanNumbers::AegeanNumberThree),
            '𐄊' => Ok(AegeanNumbers::AegeanNumberFour),
            '𐄋' => Ok(AegeanNumbers::AegeanNumberFive),
            '𐄌' => Ok(AegeanNumbers::AegeanNumberSix),
            '𐄍' => Ok(AegeanNumbers::AegeanNumberSeven),
            '𐄎' => Ok(AegeanNumbers::AegeanNumberEight),
            '𐄏' => Ok(AegeanNumbers::AegeanNumberNine),
            '𐄐' => Ok(AegeanNumbers::AegeanNumberTen),
            '𐄑' => Ok(AegeanNumbers::AegeanNumberTwenty),
            '𐄒' => Ok(AegeanNumbers::AegeanNumberThirty),
            '𐄓' => Ok(AegeanNumbers::AegeanNumberForty),
            '𐄔' => Ok(AegeanNumbers::AegeanNumberFifty),
            '𐄕' => Ok(AegeanNumbers::AegeanNumberSixty),
            '𐄖' => Ok(AegeanNumbers::AegeanNumberSeventy),
            '𐄗' => Ok(AegeanNumbers::AegeanNumberEighty),
            '𐄘' => Ok(AegeanNumbers::AegeanNumberNinety),
            '𐄙' => Ok(AegeanNumbers::AegeanNumberOneHundred),
            '𐄚' => Ok(AegeanNumbers::AegeanNumberTwoHundred),
            '𐄛' => Ok(AegeanNumbers::AegeanNumberThreeHundred),
            '𐄜' => Ok(AegeanNumbers::AegeanNumberFourHundred),
            '𐄝' => Ok(AegeanNumbers::AegeanNumberFiveHundred),
            '𐄞' => Ok(AegeanNumbers::AegeanNumberSixHundred),
            '𐄟' => Ok(AegeanNumbers::AegeanNumberSevenHundred),
            '𐄠' => Ok(AegeanNumbers::AegeanNumberEightHundred),
            '𐄡' => Ok(AegeanNumbers::AegeanNumberNineHundred),
            '𐄢' => Ok(AegeanNumbers::AegeanNumberOneThousand),
            '𐄣' => Ok(AegeanNumbers::AegeanNumberTwoThousand),
            '𐄤' => Ok(AegeanNumbers::AegeanNumberThreeThousand),
            '𐄥' => Ok(AegeanNumbers::AegeanNumberFourThousand),
            '𐄦' => Ok(AegeanNumbers::AegeanNumberFiveThousand),
            '𐄧' => Ok(AegeanNumbers::AegeanNumberSixThousand),
            '𐄨' => Ok(AegeanNumbers::AegeanNumberSevenThousand),
            '𐄩' => Ok(AegeanNumbers::AegeanNumberEightThousand),
            '𐄪' => Ok(AegeanNumbers::AegeanNumberNineThousand),
            '𐄫' => Ok(AegeanNumbers::AegeanNumberTenThousand),
            '𐄬' => Ok(AegeanNumbers::AegeanNumberTwentyThousand),
            '𐄭' => Ok(AegeanNumbers::AegeanNumberThirtyThousand),
            '𐄮' => Ok(AegeanNumbers::AegeanNumberFortyThousand),
            '𐄯' => Ok(AegeanNumbers::AegeanNumberFiftyThousand),
            '𐄰' => Ok(AegeanNumbers::AegeanNumberSixtyThousand),
            '𐄱' => Ok(AegeanNumbers::AegeanNumberSeventyThousand),
            '𐄲' => Ok(AegeanNumbers::AegeanNumberEightyThousand),
            '𐄳' => Ok(AegeanNumbers::AegeanNumberNinetyThousand),
            '𐄷' => Ok(AegeanNumbers::AegeanWeightBaseUnit),
            '𐄸' => Ok(AegeanNumbers::AegeanWeightFirstSubunit),
            '𐄹' => Ok(AegeanNumbers::AegeanWeightSecondSubunit),
            '𐄺' => Ok(AegeanNumbers::AegeanWeightThirdSubunit),
            '𐄻' => Ok(AegeanNumbers::AegeanWeightFourthSubunit),
            '𐄼' => Ok(AegeanNumbers::AegeanDryMeasureFirstSubunit),
            '𐄽' => Ok(AegeanNumbers::AegeanLiquidMeasureFirstSubunit),
            '𐄾' => Ok(AegeanNumbers::AegeanMeasureSecondSubunit),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AegeanNumbers {
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

impl std::convert::TryFrom<u32> for AegeanNumbers {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AegeanNumbers {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AegeanNumbers {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        AegeanNumbers::AegeanWordSeparatorLine
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("AegeanNumbers{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
