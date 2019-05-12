/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{102e0}: '𐋠'
    pub const COPTIC_EPACT_THOUSANDS_MARK: char = '𐋠';
    /// \u{102e1}: '𐋡'
    pub const COPTIC_EPACT_DIGIT_ONE: char = '𐋡';
    /// \u{102e2}: '𐋢'
    pub const COPTIC_EPACT_DIGIT_TWO: char = '𐋢';
    /// \u{102e3}: '𐋣'
    pub const COPTIC_EPACT_DIGIT_THREE: char = '𐋣';
    /// \u{102e4}: '𐋤'
    pub const COPTIC_EPACT_DIGIT_FOUR: char = '𐋤';
    /// \u{102e5}: '𐋥'
    pub const COPTIC_EPACT_DIGIT_FIVE: char = '𐋥';
    /// \u{102e6}: '𐋦'
    pub const COPTIC_EPACT_DIGIT_SIX: char = '𐋦';
    /// \u{102e7}: '𐋧'
    pub const COPTIC_EPACT_DIGIT_SEVEN: char = '𐋧';
    /// \u{102e8}: '𐋨'
    pub const COPTIC_EPACT_DIGIT_EIGHT: char = '𐋨';
    /// \u{102e9}: '𐋩'
    pub const COPTIC_EPACT_DIGIT_NINE: char = '𐋩';
    /// \u{102ea}: '𐋪'
    pub const COPTIC_EPACT_NUMBER_TEN: char = '𐋪';
    /// \u{102eb}: '𐋫'
    pub const COPTIC_EPACT_NUMBER_TWENTY: char = '𐋫';
    /// \u{102ec}: '𐋬'
    pub const COPTIC_EPACT_NUMBER_THIRTY: char = '𐋬';
    /// \u{102ed}: '𐋭'
    pub const COPTIC_EPACT_NUMBER_FORTY: char = '𐋭';
    /// \u{102ee}: '𐋮'
    pub const COPTIC_EPACT_NUMBER_FIFTY: char = '𐋮';
    /// \u{102ef}: '𐋯'
    pub const COPTIC_EPACT_NUMBER_SIXTY: char = '𐋯';
    /// \u{102f0}: '𐋰'
    pub const COPTIC_EPACT_NUMBER_SEVENTY: char = '𐋰';
    /// \u{102f1}: '𐋱'
    pub const COPTIC_EPACT_NUMBER_EIGHTY: char = '𐋱';
    /// \u{102f2}: '𐋲'
    pub const COPTIC_EPACT_NUMBER_NINETY: char = '𐋲';
    /// \u{102f3}: '𐋳'
    pub const COPTIC_EPACT_NUMBER_ONE_HUNDRED: char = '𐋳';
    /// \u{102f4}: '𐋴'
    pub const COPTIC_EPACT_NUMBER_TWO_HUNDRED: char = '𐋴';
    /// \u{102f5}: '𐋵'
    pub const COPTIC_EPACT_NUMBER_THREE_HUNDRED: char = '𐋵';
    /// \u{102f6}: '𐋶'
    pub const COPTIC_EPACT_NUMBER_FOUR_HUNDRED: char = '𐋶';
    /// \u{102f7}: '𐋷'
    pub const COPTIC_EPACT_NUMBER_FIVE_HUNDRED: char = '𐋷';
    /// \u{102f8}: '𐋸'
    pub const COPTIC_EPACT_NUMBER_SIX_HUNDRED: char = '𐋸';
    /// \u{102f9}: '𐋹'
    pub const COPTIC_EPACT_NUMBER_SEVEN_HUNDRED: char = '𐋹';
    /// \u{102fa}: '𐋺'
    pub const COPTIC_EPACT_NUMBER_EIGHT_HUNDRED: char = '𐋺';
    /// \u{102fb}: '𐋻'
    pub const COPTIC_EPACT_NUMBER_NINE_HUNDRED: char = '𐋻';
}

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
        use constants::*;
        match self {
            CopticEpactNumbers::CopticEpactThousandsMark => COPTIC_EPACT_THOUSANDS_MARK,
            CopticEpactNumbers::CopticEpactDigitOne => COPTIC_EPACT_DIGIT_ONE,
            CopticEpactNumbers::CopticEpactDigitTwo => COPTIC_EPACT_DIGIT_TWO,
            CopticEpactNumbers::CopticEpactDigitThree => COPTIC_EPACT_DIGIT_THREE,
            CopticEpactNumbers::CopticEpactDigitFour => COPTIC_EPACT_DIGIT_FOUR,
            CopticEpactNumbers::CopticEpactDigitFive => COPTIC_EPACT_DIGIT_FIVE,
            CopticEpactNumbers::CopticEpactDigitSix => COPTIC_EPACT_DIGIT_SIX,
            CopticEpactNumbers::CopticEpactDigitSeven => COPTIC_EPACT_DIGIT_SEVEN,
            CopticEpactNumbers::CopticEpactDigitEight => COPTIC_EPACT_DIGIT_EIGHT,
            CopticEpactNumbers::CopticEpactDigitNine => COPTIC_EPACT_DIGIT_NINE,
            CopticEpactNumbers::CopticEpactNumberTen => COPTIC_EPACT_NUMBER_TEN,
            CopticEpactNumbers::CopticEpactNumberTwenty => COPTIC_EPACT_NUMBER_TWENTY,
            CopticEpactNumbers::CopticEpactNumberThirty => COPTIC_EPACT_NUMBER_THIRTY,
            CopticEpactNumbers::CopticEpactNumberForty => COPTIC_EPACT_NUMBER_FORTY,
            CopticEpactNumbers::CopticEpactNumberFifty => COPTIC_EPACT_NUMBER_FIFTY,
            CopticEpactNumbers::CopticEpactNumberSixty => COPTIC_EPACT_NUMBER_SIXTY,
            CopticEpactNumbers::CopticEpactNumberSeventy => COPTIC_EPACT_NUMBER_SEVENTY,
            CopticEpactNumbers::CopticEpactNumberEighty => COPTIC_EPACT_NUMBER_EIGHTY,
            CopticEpactNumbers::CopticEpactNumberNinety => COPTIC_EPACT_NUMBER_NINETY,
            CopticEpactNumbers::CopticEpactNumberOneHundred => COPTIC_EPACT_NUMBER_ONE_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberTwoHundred => COPTIC_EPACT_NUMBER_TWO_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberThreeHundred => COPTIC_EPACT_NUMBER_THREE_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberFourHundred => COPTIC_EPACT_NUMBER_FOUR_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberFiveHundred => COPTIC_EPACT_NUMBER_FIVE_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberSixHundred => COPTIC_EPACT_NUMBER_SIX_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberSevenHundred => COPTIC_EPACT_NUMBER_SEVEN_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberEightHundred => COPTIC_EPACT_NUMBER_EIGHT_HUNDRED,
            CopticEpactNumbers::CopticEpactNumberNineHundred => COPTIC_EPACT_NUMBER_NINE_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for CopticEpactNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COPTIC_EPACT_THOUSANDS_MARK => Ok(CopticEpactNumbers::CopticEpactThousandsMark),
            COPTIC_EPACT_DIGIT_ONE => Ok(CopticEpactNumbers::CopticEpactDigitOne),
            COPTIC_EPACT_DIGIT_TWO => Ok(CopticEpactNumbers::CopticEpactDigitTwo),
            COPTIC_EPACT_DIGIT_THREE => Ok(CopticEpactNumbers::CopticEpactDigitThree),
            COPTIC_EPACT_DIGIT_FOUR => Ok(CopticEpactNumbers::CopticEpactDigitFour),
            COPTIC_EPACT_DIGIT_FIVE => Ok(CopticEpactNumbers::CopticEpactDigitFive),
            COPTIC_EPACT_DIGIT_SIX => Ok(CopticEpactNumbers::CopticEpactDigitSix),
            COPTIC_EPACT_DIGIT_SEVEN => Ok(CopticEpactNumbers::CopticEpactDigitSeven),
            COPTIC_EPACT_DIGIT_EIGHT => Ok(CopticEpactNumbers::CopticEpactDigitEight),
            COPTIC_EPACT_DIGIT_NINE => Ok(CopticEpactNumbers::CopticEpactDigitNine),
            COPTIC_EPACT_NUMBER_TEN => Ok(CopticEpactNumbers::CopticEpactNumberTen),
            COPTIC_EPACT_NUMBER_TWENTY => Ok(CopticEpactNumbers::CopticEpactNumberTwenty),
            COPTIC_EPACT_NUMBER_THIRTY => Ok(CopticEpactNumbers::CopticEpactNumberThirty),
            COPTIC_EPACT_NUMBER_FORTY => Ok(CopticEpactNumbers::CopticEpactNumberForty),
            COPTIC_EPACT_NUMBER_FIFTY => Ok(CopticEpactNumbers::CopticEpactNumberFifty),
            COPTIC_EPACT_NUMBER_SIXTY => Ok(CopticEpactNumbers::CopticEpactNumberSixty),
            COPTIC_EPACT_NUMBER_SEVENTY => Ok(CopticEpactNumbers::CopticEpactNumberSeventy),
            COPTIC_EPACT_NUMBER_EIGHTY => Ok(CopticEpactNumbers::CopticEpactNumberEighty),
            COPTIC_EPACT_NUMBER_NINETY => Ok(CopticEpactNumbers::CopticEpactNumberNinety),
            COPTIC_EPACT_NUMBER_ONE_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberOneHundred),
            COPTIC_EPACT_NUMBER_TWO_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberTwoHundred),
            COPTIC_EPACT_NUMBER_THREE_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberThreeHundred),
            COPTIC_EPACT_NUMBER_FOUR_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberFourHundred),
            COPTIC_EPACT_NUMBER_FIVE_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberFiveHundred),
            COPTIC_EPACT_NUMBER_SIX_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberSixHundred),
            COPTIC_EPACT_NUMBER_SEVEN_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberSevenHundred),
            COPTIC_EPACT_NUMBER_EIGHT_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberEightHundred),
            COPTIC_EPACT_NUMBER_NINE_HUNDRED => Ok(CopticEpactNumbers::CopticEpactNumberNineHundred),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CopticEpactNumbers::CopticEpactThousandsMark => "coptic epact thousands mark",
            CopticEpactNumbers::CopticEpactDigitOne => "coptic epact digit one",
            CopticEpactNumbers::CopticEpactDigitTwo => "coptic epact digit two",
            CopticEpactNumbers::CopticEpactDigitThree => "coptic epact digit three",
            CopticEpactNumbers::CopticEpactDigitFour => "coptic epact digit four",
            CopticEpactNumbers::CopticEpactDigitFive => "coptic epact digit five",
            CopticEpactNumbers::CopticEpactDigitSix => "coptic epact digit six",
            CopticEpactNumbers::CopticEpactDigitSeven => "coptic epact digit seven",
            CopticEpactNumbers::CopticEpactDigitEight => "coptic epact digit eight",
            CopticEpactNumbers::CopticEpactDigitNine => "coptic epact digit nine",
            CopticEpactNumbers::CopticEpactNumberTen => "coptic epact number ten",
            CopticEpactNumbers::CopticEpactNumberTwenty => "coptic epact number twenty",
            CopticEpactNumbers::CopticEpactNumberThirty => "coptic epact number thirty",
            CopticEpactNumbers::CopticEpactNumberForty => "coptic epact number forty",
            CopticEpactNumbers::CopticEpactNumberFifty => "coptic epact number fifty",
            CopticEpactNumbers::CopticEpactNumberSixty => "coptic epact number sixty",
            CopticEpactNumbers::CopticEpactNumberSeventy => "coptic epact number seventy",
            CopticEpactNumbers::CopticEpactNumberEighty => "coptic epact number eighty",
            CopticEpactNumbers::CopticEpactNumberNinety => "coptic epact number ninety",
            CopticEpactNumbers::CopticEpactNumberOneHundred => "coptic epact number one hundred",
            CopticEpactNumbers::CopticEpactNumberTwoHundred => "coptic epact number two hundred",
            CopticEpactNumbers::CopticEpactNumberThreeHundred => "coptic epact number three hundred",
            CopticEpactNumbers::CopticEpactNumberFourHundred => "coptic epact number four hundred",
            CopticEpactNumbers::CopticEpactNumberFiveHundred => "coptic epact number five hundred",
            CopticEpactNumbers::CopticEpactNumberSixHundred => "coptic epact number six hundred",
            CopticEpactNumbers::CopticEpactNumberSevenHundred => "coptic epact number seven hundred",
            CopticEpactNumbers::CopticEpactNumberEightHundred => "coptic epact number eight hundred",
            CopticEpactNumbers::CopticEpactNumberNineHundred => "coptic epact number nine hundred",
        }
    }
}
