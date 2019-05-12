
/// An enum to represent all characters in the CopticEpactNumbers block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CopticEpactNumbers {
    /// \u{102e0}: '𐋠'
    CopticEpactThousandsMark,
    /// \u{102e1}: '𐋡'
    CopticEpactDigitOne,
    /// \u{102e2}: '𐋢'
    CopticEpactDigitTwo,
    /// \u{102e3}: '𐋣'
    CopticEpactDigitThree,
    /// \u{102e4}: '𐋤'
    CopticEpactDigitFour,
    /// \u{102e5}: '𐋥'
    CopticEpactDigitFive,
    /// \u{102e6}: '𐋦'
    CopticEpactDigitSix,
    /// \u{102e7}: '𐋧'
    CopticEpactDigitSeven,
    /// \u{102e8}: '𐋨'
    CopticEpactDigitEight,
    /// \u{102e9}: '𐋩'
    CopticEpactDigitNine,
    /// \u{102ea}: '𐋪'
    CopticEpactNumberTen,
    /// \u{102eb}: '𐋫'
    CopticEpactNumberTwenty,
    /// \u{102ec}: '𐋬'
    CopticEpactNumberThirty,
    /// \u{102ed}: '𐋭'
    CopticEpactNumberForty,
    /// \u{102ee}: '𐋮'
    CopticEpactNumberFifty,
    /// \u{102ef}: '𐋯'
    CopticEpactNumberSixty,
    /// \u{102f0}: '𐋰'
    CopticEpactNumberSeventy,
    /// \u{102f1}: '𐋱'
    CopticEpactNumberEighty,
    /// \u{102f2}: '𐋲'
    CopticEpactNumberNinety,
    /// \u{102f3}: '𐋳'
    CopticEpactNumberOneHundred,
    /// \u{102f4}: '𐋴'
    CopticEpactNumberTwoHundred,
    /// \u{102f5}: '𐋵'
    CopticEpactNumberThreeHundred,
    /// \u{102f6}: '𐋶'
    CopticEpactNumberFourHundred,
    /// \u{102f7}: '𐋷'
    CopticEpactNumberFiveHundred,
    /// \u{102f8}: '𐋸'
    CopticEpactNumberSixHundred,
    /// \u{102f9}: '𐋹'
    CopticEpactNumberSevenHundred,
    /// \u{102fa}: '𐋺'
    CopticEpactNumberEightHundred,
    /// \u{102fb}: '𐋻'
    CopticEpactNumberNineHundred,
}

impl Into<char> for CopticEpactNumbers {
    fn into(self) -> char {
        match self {
            CopticEpactNumbers::CopticEpactThousandsMark => '𐋠',
            CopticEpactNumbers::CopticEpactDigitOne => '𐋡',
            CopticEpactNumbers::CopticEpactDigitTwo => '𐋢',
            CopticEpactNumbers::CopticEpactDigitThree => '𐋣',
            CopticEpactNumbers::CopticEpactDigitFour => '𐋤',
            CopticEpactNumbers::CopticEpactDigitFive => '𐋥',
            CopticEpactNumbers::CopticEpactDigitSix => '𐋦',
            CopticEpactNumbers::CopticEpactDigitSeven => '𐋧',
            CopticEpactNumbers::CopticEpactDigitEight => '𐋨',
            CopticEpactNumbers::CopticEpactDigitNine => '𐋩',
            CopticEpactNumbers::CopticEpactNumberTen => '𐋪',
            CopticEpactNumbers::CopticEpactNumberTwenty => '𐋫',
            CopticEpactNumbers::CopticEpactNumberThirty => '𐋬',
            CopticEpactNumbers::CopticEpactNumberForty => '𐋭',
            CopticEpactNumbers::CopticEpactNumberFifty => '𐋮',
            CopticEpactNumbers::CopticEpactNumberSixty => '𐋯',
            CopticEpactNumbers::CopticEpactNumberSeventy => '𐋰',
            CopticEpactNumbers::CopticEpactNumberEighty => '𐋱',
            CopticEpactNumbers::CopticEpactNumberNinety => '𐋲',
            CopticEpactNumbers::CopticEpactNumberOneHundred => '𐋳',
            CopticEpactNumbers::CopticEpactNumberTwoHundred => '𐋴',
            CopticEpactNumbers::CopticEpactNumberThreeHundred => '𐋵',
            CopticEpactNumbers::CopticEpactNumberFourHundred => '𐋶',
            CopticEpactNumbers::CopticEpactNumberFiveHundred => '𐋷',
            CopticEpactNumbers::CopticEpactNumberSixHundred => '𐋸',
            CopticEpactNumbers::CopticEpactNumberSevenHundred => '𐋹',
            CopticEpactNumbers::CopticEpactNumberEightHundred => '𐋺',
            CopticEpactNumbers::CopticEpactNumberNineHundred => '𐋻',
        }
    }
}

impl std::convert::TryFrom<char> for CopticEpactNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐋠' => Ok(CopticEpactNumbers::CopticEpactThousandsMark),
            '𐋡' => Ok(CopticEpactNumbers::CopticEpactDigitOne),
            '𐋢' => Ok(CopticEpactNumbers::CopticEpactDigitTwo),
            '𐋣' => Ok(CopticEpactNumbers::CopticEpactDigitThree),
            '𐋤' => Ok(CopticEpactNumbers::CopticEpactDigitFour),
            '𐋥' => Ok(CopticEpactNumbers::CopticEpactDigitFive),
            '𐋦' => Ok(CopticEpactNumbers::CopticEpactDigitSix),
            '𐋧' => Ok(CopticEpactNumbers::CopticEpactDigitSeven),
            '𐋨' => Ok(CopticEpactNumbers::CopticEpactDigitEight),
            '𐋩' => Ok(CopticEpactNumbers::CopticEpactDigitNine),
            '𐋪' => Ok(CopticEpactNumbers::CopticEpactNumberTen),
            '𐋫' => Ok(CopticEpactNumbers::CopticEpactNumberTwenty),
            '𐋬' => Ok(CopticEpactNumbers::CopticEpactNumberThirty),
            '𐋭' => Ok(CopticEpactNumbers::CopticEpactNumberForty),
            '𐋮' => Ok(CopticEpactNumbers::CopticEpactNumberFifty),
            '𐋯' => Ok(CopticEpactNumbers::CopticEpactNumberSixty),
            '𐋰' => Ok(CopticEpactNumbers::CopticEpactNumberSeventy),
            '𐋱' => Ok(CopticEpactNumbers::CopticEpactNumberEighty),
            '𐋲' => Ok(CopticEpactNumbers::CopticEpactNumberNinety),
            '𐋳' => Ok(CopticEpactNumbers::CopticEpactNumberOneHundred),
            '𐋴' => Ok(CopticEpactNumbers::CopticEpactNumberTwoHundred),
            '𐋵' => Ok(CopticEpactNumbers::CopticEpactNumberThreeHundred),
            '𐋶' => Ok(CopticEpactNumbers::CopticEpactNumberFourHundred),
            '𐋷' => Ok(CopticEpactNumbers::CopticEpactNumberFiveHundred),
            '𐋸' => Ok(CopticEpactNumbers::CopticEpactNumberSixHundred),
            '𐋹' => Ok(CopticEpactNumbers::CopticEpactNumberSevenHundred),
            '𐋺' => Ok(CopticEpactNumbers::CopticEpactNumberEightHundred),
            '𐋻' => Ok(CopticEpactNumbers::CopticEpactNumberNineHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CopticEpactNumbers {
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

impl std::convert::TryFrom<u32> for CopticEpactNumbers {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CopticEpactNumbers {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CopticEpactNumbers {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CopticEpactNumbers::CopticEpactThousandsMark
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CopticEpactNumbers{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
