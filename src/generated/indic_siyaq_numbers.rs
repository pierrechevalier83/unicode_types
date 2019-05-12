
/// An enum to represent all characters in the IndicSiyaqNumbers block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IndicSiyaqNumbers {
    /// \u{1ec71}: '𞱱'
    IndicSiyaqNumberOne,
    /// \u{1ec72}: '𞱲'
    IndicSiyaqNumberTwo,
    /// \u{1ec73}: '𞱳'
    IndicSiyaqNumberThree,
    /// \u{1ec74}: '𞱴'
    IndicSiyaqNumberFour,
    /// \u{1ec75}: '𞱵'
    IndicSiyaqNumberFive,
    /// \u{1ec76}: '𞱶'
    IndicSiyaqNumberSix,
    /// \u{1ec77}: '𞱷'
    IndicSiyaqNumberSeven,
    /// \u{1ec78}: '𞱸'
    IndicSiyaqNumberEight,
    /// \u{1ec79}: '𞱹'
    IndicSiyaqNumberNine,
    /// \u{1ec7a}: '𞱺'
    IndicSiyaqNumberTen,
    /// \u{1ec7b}: '𞱻'
    IndicSiyaqNumberTwenty,
    /// \u{1ec7c}: '𞱼'
    IndicSiyaqNumberThirty,
    /// \u{1ec7d}: '𞱽'
    IndicSiyaqNumberForty,
    /// \u{1ec7e}: '𞱾'
    IndicSiyaqNumberFifty,
    /// \u{1ec7f}: '𞱿'
    IndicSiyaqNumberSixty,
    /// \u{1ec80}: '𞲀'
    IndicSiyaqNumberSeventy,
    /// \u{1ec81}: '𞲁'
    IndicSiyaqNumberEighty,
    /// \u{1ec82}: '𞲂'
    IndicSiyaqNumberNinety,
    /// \u{1ec83}: '𞲃'
    IndicSiyaqNumberOneHundred,
    /// \u{1ec84}: '𞲄'
    IndicSiyaqNumberTwoHundred,
    /// \u{1ec85}: '𞲅'
    IndicSiyaqNumberThreeHundred,
    /// \u{1ec86}: '𞲆'
    IndicSiyaqNumberFourHundred,
    /// \u{1ec87}: '𞲇'
    IndicSiyaqNumberFiveHundred,
    /// \u{1ec88}: '𞲈'
    IndicSiyaqNumberSixHundred,
    /// \u{1ec89}: '𞲉'
    IndicSiyaqNumberSevenHundred,
    /// \u{1ec8a}: '𞲊'
    IndicSiyaqNumberEightHundred,
    /// \u{1ec8b}: '𞲋'
    IndicSiyaqNumberNineHundred,
    /// \u{1ec8c}: '𞲌'
    IndicSiyaqNumberOneThousand,
    /// \u{1ec8d}: '𞲍'
    IndicSiyaqNumberTwoThousand,
    /// \u{1ec8e}: '𞲎'
    IndicSiyaqNumberThreeThousand,
    /// \u{1ec8f}: '𞲏'
    IndicSiyaqNumberFourThousand,
    /// \u{1ec90}: '𞲐'
    IndicSiyaqNumberFiveThousand,
    /// \u{1ec91}: '𞲑'
    IndicSiyaqNumberSixThousand,
    /// \u{1ec92}: '𞲒'
    IndicSiyaqNumberSevenThousand,
    /// \u{1ec93}: '𞲓'
    IndicSiyaqNumberEightThousand,
    /// \u{1ec94}: '𞲔'
    IndicSiyaqNumberNineThousand,
    /// \u{1ec95}: '𞲕'
    IndicSiyaqNumberTenThousand,
    /// \u{1ec96}: '𞲖'
    IndicSiyaqNumberTwentyThousand,
    /// \u{1ec97}: '𞲗'
    IndicSiyaqNumberThirtyThousand,
    /// \u{1ec98}: '𞲘'
    IndicSiyaqNumberFortyThousand,
    /// \u{1ec99}: '𞲙'
    IndicSiyaqNumberFiftyThousand,
    /// \u{1ec9a}: '𞲚'
    IndicSiyaqNumberSixtyThousand,
    /// \u{1ec9b}: '𞲛'
    IndicSiyaqNumberSeventyThousand,
    /// \u{1ec9c}: '𞲜'
    IndicSiyaqNumberEightyThousand,
    /// \u{1ec9d}: '𞲝'
    IndicSiyaqNumberNinetyThousand,
    /// \u{1ec9e}: '𞲞'
    IndicSiyaqNumberLakh,
    /// \u{1ec9f}: '𞲟'
    IndicSiyaqNumberLakhan,
    /// \u{1eca0}: '𞲠'
    IndicSiyaqLakhMark,
    /// \u{1eca1}: '𞲡'
    IndicSiyaqNumberKaror,
    /// \u{1eca2}: '𞲢'
    IndicSiyaqNumberKaroran,
    /// \u{1eca3}: '𞲣'
    IndicSiyaqNumberPrefixedOne,
    /// \u{1eca4}: '𞲤'
    IndicSiyaqNumberPrefixedTwo,
    /// \u{1eca5}: '𞲥'
    IndicSiyaqNumberPrefixedThree,
    /// \u{1eca6}: '𞲦'
    IndicSiyaqNumberPrefixedFour,
    /// \u{1eca7}: '𞲧'
    IndicSiyaqNumberPrefixedFive,
    /// \u{1eca8}: '𞲨'
    IndicSiyaqNumberPrefixedSix,
    /// \u{1eca9}: '𞲩'
    IndicSiyaqNumberPrefixedSeven,
    /// \u{1ecaa}: '𞲪'
    IndicSiyaqNumberPrefixedEight,
    /// \u{1ecab}: '𞲫'
    IndicSiyaqNumberPrefixedNine,
    /// \u{1ecac}: '𞲬'
    IndicSiyaqPlaceholder,
    /// \u{1ecad}: '𞲭'
    IndicSiyaqFractionOneQuarter,
    /// \u{1ecae}: '𞲮'
    IndicSiyaqFractionOneHalf,
    /// \u{1ecaf}: '𞲯'
    IndicSiyaqFractionThreeQuarters,
    /// \u{1ecb0}: '𞲰'
    IndicSiyaqRupeeMark,
    /// \u{1ecb1}: '𞲱'
    IndicSiyaqNumberAlternateOne,
    /// \u{1ecb2}: '𞲲'
    IndicSiyaqNumberAlternateTwo,
    /// \u{1ecb3}: '𞲳'
    IndicSiyaqNumberAlternateTenThousand,
    /// \u{1ecb4}: '𞲴'
    IndicSiyaqAlternateLakhMark,
}

impl Into<char> for IndicSiyaqNumbers {
    fn into(self) -> char {
        match self {
            IndicSiyaqNumbers::IndicSiyaqNumberOne => '𞱱',
            IndicSiyaqNumbers::IndicSiyaqNumberTwo => '𞱲',
            IndicSiyaqNumbers::IndicSiyaqNumberThree => '𞱳',
            IndicSiyaqNumbers::IndicSiyaqNumberFour => '𞱴',
            IndicSiyaqNumbers::IndicSiyaqNumberFive => '𞱵',
            IndicSiyaqNumbers::IndicSiyaqNumberSix => '𞱶',
            IndicSiyaqNumbers::IndicSiyaqNumberSeven => '𞱷',
            IndicSiyaqNumbers::IndicSiyaqNumberEight => '𞱸',
            IndicSiyaqNumbers::IndicSiyaqNumberNine => '𞱹',
            IndicSiyaqNumbers::IndicSiyaqNumberTen => '𞱺',
            IndicSiyaqNumbers::IndicSiyaqNumberTwenty => '𞱻',
            IndicSiyaqNumbers::IndicSiyaqNumberThirty => '𞱼',
            IndicSiyaqNumbers::IndicSiyaqNumberForty => '𞱽',
            IndicSiyaqNumbers::IndicSiyaqNumberFifty => '𞱾',
            IndicSiyaqNumbers::IndicSiyaqNumberSixty => '𞱿',
            IndicSiyaqNumbers::IndicSiyaqNumberSeventy => '𞲀',
            IndicSiyaqNumbers::IndicSiyaqNumberEighty => '𞲁',
            IndicSiyaqNumbers::IndicSiyaqNumberNinety => '𞲂',
            IndicSiyaqNumbers::IndicSiyaqNumberOneHundred => '𞲃',
            IndicSiyaqNumbers::IndicSiyaqNumberTwoHundred => '𞲄',
            IndicSiyaqNumbers::IndicSiyaqNumberThreeHundred => '𞲅',
            IndicSiyaqNumbers::IndicSiyaqNumberFourHundred => '𞲆',
            IndicSiyaqNumbers::IndicSiyaqNumberFiveHundred => '𞲇',
            IndicSiyaqNumbers::IndicSiyaqNumberSixHundred => '𞲈',
            IndicSiyaqNumbers::IndicSiyaqNumberSevenHundred => '𞲉',
            IndicSiyaqNumbers::IndicSiyaqNumberEightHundred => '𞲊',
            IndicSiyaqNumbers::IndicSiyaqNumberNineHundred => '𞲋',
            IndicSiyaqNumbers::IndicSiyaqNumberOneThousand => '𞲌',
            IndicSiyaqNumbers::IndicSiyaqNumberTwoThousand => '𞲍',
            IndicSiyaqNumbers::IndicSiyaqNumberThreeThousand => '𞲎',
            IndicSiyaqNumbers::IndicSiyaqNumberFourThousand => '𞲏',
            IndicSiyaqNumbers::IndicSiyaqNumberFiveThousand => '𞲐',
            IndicSiyaqNumbers::IndicSiyaqNumberSixThousand => '𞲑',
            IndicSiyaqNumbers::IndicSiyaqNumberSevenThousand => '𞲒',
            IndicSiyaqNumbers::IndicSiyaqNumberEightThousand => '𞲓',
            IndicSiyaqNumbers::IndicSiyaqNumberNineThousand => '𞲔',
            IndicSiyaqNumbers::IndicSiyaqNumberTenThousand => '𞲕',
            IndicSiyaqNumbers::IndicSiyaqNumberTwentyThousand => '𞲖',
            IndicSiyaqNumbers::IndicSiyaqNumberThirtyThousand => '𞲗',
            IndicSiyaqNumbers::IndicSiyaqNumberFortyThousand => '𞲘',
            IndicSiyaqNumbers::IndicSiyaqNumberFiftyThousand => '𞲙',
            IndicSiyaqNumbers::IndicSiyaqNumberSixtyThousand => '𞲚',
            IndicSiyaqNumbers::IndicSiyaqNumberSeventyThousand => '𞲛',
            IndicSiyaqNumbers::IndicSiyaqNumberEightyThousand => '𞲜',
            IndicSiyaqNumbers::IndicSiyaqNumberNinetyThousand => '𞲝',
            IndicSiyaqNumbers::IndicSiyaqNumberLakh => '𞲞',
            IndicSiyaqNumbers::IndicSiyaqNumberLakhan => '𞲟',
            IndicSiyaqNumbers::IndicSiyaqLakhMark => '𞲠',
            IndicSiyaqNumbers::IndicSiyaqNumberKaror => '𞲡',
            IndicSiyaqNumbers::IndicSiyaqNumberKaroran => '𞲢',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedOne => '𞲣',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedTwo => '𞲤',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedThree => '𞲥',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedFour => '𞲦',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedFive => '𞲧',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedSix => '𞲨',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedSeven => '𞲩',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedEight => '𞲪',
            IndicSiyaqNumbers::IndicSiyaqNumberPrefixedNine => '𞲫',
            IndicSiyaqNumbers::IndicSiyaqPlaceholder => '𞲬',
            IndicSiyaqNumbers::IndicSiyaqFractionOneQuarter => '𞲭',
            IndicSiyaqNumbers::IndicSiyaqFractionOneHalf => '𞲮',
            IndicSiyaqNumbers::IndicSiyaqFractionThreeQuarters => '𞲯',
            IndicSiyaqNumbers::IndicSiyaqRupeeMark => '𞲰',
            IndicSiyaqNumbers::IndicSiyaqNumberAlternateOne => '𞲱',
            IndicSiyaqNumbers::IndicSiyaqNumberAlternateTwo => '𞲲',
            IndicSiyaqNumbers::IndicSiyaqNumberAlternateTenThousand => '𞲳',
            IndicSiyaqNumbers::IndicSiyaqAlternateLakhMark => '𞲴',
        }
    }
}

impl std::convert::TryFrom<char> for IndicSiyaqNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞱱' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberOne),
            '𞱲' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberTwo),
            '𞱳' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberThree),
            '𞱴' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFour),
            '𞱵' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFive),
            '𞱶' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSix),
            '𞱷' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSeven),
            '𞱸' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberEight),
            '𞱹' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberNine),
            '𞱺' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberTen),
            '𞱻' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberTwenty),
            '𞱼' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberThirty),
            '𞱽' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberForty),
            '𞱾' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFifty),
            '𞱿' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSixty),
            '𞲀' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSeventy),
            '𞲁' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberEighty),
            '𞲂' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberNinety),
            '𞲃' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberOneHundred),
            '𞲄' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberTwoHundred),
            '𞲅' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberThreeHundred),
            '𞲆' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFourHundred),
            '𞲇' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFiveHundred),
            '𞲈' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSixHundred),
            '𞲉' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSevenHundred),
            '𞲊' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberEightHundred),
            '𞲋' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberNineHundred),
            '𞲌' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberOneThousand),
            '𞲍' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberTwoThousand),
            '𞲎' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberThreeThousand),
            '𞲏' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFourThousand),
            '𞲐' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFiveThousand),
            '𞲑' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSixThousand),
            '𞲒' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSevenThousand),
            '𞲓' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberEightThousand),
            '𞲔' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberNineThousand),
            '𞲕' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberTenThousand),
            '𞲖' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberTwentyThousand),
            '𞲗' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberThirtyThousand),
            '𞲘' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFortyThousand),
            '𞲙' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberFiftyThousand),
            '𞲚' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSixtyThousand),
            '𞲛' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberSeventyThousand),
            '𞲜' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberEightyThousand),
            '𞲝' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberNinetyThousand),
            '𞲞' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberLakh),
            '𞲟' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberLakhan),
            '𞲠' => Ok(IndicSiyaqNumbers::IndicSiyaqLakhMark),
            '𞲡' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberKaror),
            '𞲢' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberKaroran),
            '𞲣' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedOne),
            '𞲤' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedTwo),
            '𞲥' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedThree),
            '𞲦' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedFour),
            '𞲧' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedFive),
            '𞲨' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedSix),
            '𞲩' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedSeven),
            '𞲪' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedEight),
            '𞲫' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberPrefixedNine),
            '𞲬' => Ok(IndicSiyaqNumbers::IndicSiyaqPlaceholder),
            '𞲭' => Ok(IndicSiyaqNumbers::IndicSiyaqFractionOneQuarter),
            '𞲮' => Ok(IndicSiyaqNumbers::IndicSiyaqFractionOneHalf),
            '𞲯' => Ok(IndicSiyaqNumbers::IndicSiyaqFractionThreeQuarters),
            '𞲰' => Ok(IndicSiyaqNumbers::IndicSiyaqRupeeMark),
            '𞲱' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberAlternateOne),
            '𞲲' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberAlternateTwo),
            '𞲳' => Ok(IndicSiyaqNumbers::IndicSiyaqNumberAlternateTenThousand),
            '𞲴' => Ok(IndicSiyaqNumbers::IndicSiyaqAlternateLakhMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for IndicSiyaqNumbers {
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

impl std::convert::TryFrom<u32> for IndicSiyaqNumbers {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for IndicSiyaqNumbers {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl IndicSiyaqNumbers {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        IndicSiyaqNumbers::IndicSiyaqNumberOne
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("IndicSiyaqNumbers{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
