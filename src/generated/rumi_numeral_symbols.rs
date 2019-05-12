/// \u{10e60} → \u{10e7f}\
///\
/// 𐹠 𐹡 𐹢 𐹣 𐹤 𐹥 𐹦 𐹧 𐹨 𐹩 𐹪 𐹫 𐹬 𐹭 𐹮 𐹯\
/// 𐹰 𐹱 𐹲 𐹳 𐹴 𐹵 𐹶 𐹷 𐹸 𐹹 𐹺 𐹻 𐹼 𐹽 𐹾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10e60}: '𐹠'
    pub const RUMI_DIGIT_ONE: char = '𐹠';
    /// \u{10e61}: '𐹡'
    pub const RUMI_DIGIT_TWO: char = '𐹡';
    /// \u{10e62}: '𐹢'
    pub const RUMI_DIGIT_THREE: char = '𐹢';
    /// \u{10e63}: '𐹣'
    pub const RUMI_DIGIT_FOUR: char = '𐹣';
    /// \u{10e64}: '𐹤'
    pub const RUMI_DIGIT_FIVE: char = '𐹤';
    /// \u{10e65}: '𐹥'
    pub const RUMI_DIGIT_SIX: char = '𐹥';
    /// \u{10e66}: '𐹦'
    pub const RUMI_DIGIT_SEVEN: char = '𐹦';
    /// \u{10e67}: '𐹧'
    pub const RUMI_DIGIT_EIGHT: char = '𐹧';
    /// \u{10e68}: '𐹨'
    pub const RUMI_DIGIT_NINE: char = '𐹨';
    /// \u{10e69}: '𐹩'
    pub const RUMI_NUMBER_TEN: char = '𐹩';
    /// \u{10e6a}: '𐹪'
    pub const RUMI_NUMBER_TWENTY: char = '𐹪';
    /// \u{10e6b}: '𐹫'
    pub const RUMI_NUMBER_THIRTY: char = '𐹫';
    /// \u{10e6c}: '𐹬'
    pub const RUMI_NUMBER_FORTY: char = '𐹬';
    /// \u{10e6d}: '𐹭'
    pub const RUMI_NUMBER_FIFTY: char = '𐹭';
    /// \u{10e6e}: '𐹮'
    pub const RUMI_NUMBER_SIXTY: char = '𐹮';
    /// \u{10e6f}: '𐹯'
    pub const RUMI_NUMBER_SEVENTY: char = '𐹯';
    /// \u{10e70}: '𐹰'
    pub const RUMI_NUMBER_EIGHTY: char = '𐹰';
    /// \u{10e71}: '𐹱'
    pub const RUMI_NUMBER_NINETY: char = '𐹱';
    /// \u{10e72}: '𐹲'
    pub const RUMI_NUMBER_ONE_HUNDRED: char = '𐹲';
    /// \u{10e73}: '𐹳'
    pub const RUMI_NUMBER_TWO_HUNDRED: char = '𐹳';
    /// \u{10e74}: '𐹴'
    pub const RUMI_NUMBER_THREE_HUNDRED: char = '𐹴';
    /// \u{10e75}: '𐹵'
    pub const RUMI_NUMBER_FOUR_HUNDRED: char = '𐹵';
    /// \u{10e76}: '𐹶'
    pub const RUMI_NUMBER_FIVE_HUNDRED: char = '𐹶';
    /// \u{10e77}: '𐹷'
    pub const RUMI_NUMBER_SIX_HUNDRED: char = '𐹷';
    /// \u{10e78}: '𐹸'
    pub const RUMI_NUMBER_SEVEN_HUNDRED: char = '𐹸';
    /// \u{10e79}: '𐹹'
    pub const RUMI_NUMBER_EIGHT_HUNDRED: char = '𐹹';
    /// \u{10e7a}: '𐹺'
    pub const RUMI_NUMBER_NINE_HUNDRED: char = '𐹺';
    /// \u{10e7b}: '𐹻'
    pub const RUMI_FRACTION_ONE_HALF: char = '𐹻';
    /// \u{10e7c}: '𐹼'
    pub const RUMI_FRACTION_ONE_QUARTER: char = '𐹼';
    /// \u{10e7d}: '𐹽'
    pub const RUMI_FRACTION_ONE_THIRD: char = '𐹽';
    /// \u{10e7e}: '𐹾'
    pub const RUMI_FRACTION_TWO_THIRDS: char = '𐹾';
}

/// An enum to represent all characters in the RumiNumeralSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RumiNumeralSymbols {
    /// \u{10e60}: '𐹠'
    RumiDigitOne,
    /// \u{10e61}: '𐹡'
    RumiDigitTwo,
    /// \u{10e62}: '𐹢'
    RumiDigitThree,
    /// \u{10e63}: '𐹣'
    RumiDigitFour,
    /// \u{10e64}: '𐹤'
    RumiDigitFive,
    /// \u{10e65}: '𐹥'
    RumiDigitSix,
    /// \u{10e66}: '𐹦'
    RumiDigitSeven,
    /// \u{10e67}: '𐹧'
    RumiDigitEight,
    /// \u{10e68}: '𐹨'
    RumiDigitNine,
    /// \u{10e69}: '𐹩'
    RumiNumberTen,
    /// \u{10e6a}: '𐹪'
    RumiNumberTwenty,
    /// \u{10e6b}: '𐹫'
    RumiNumberThirty,
    /// \u{10e6c}: '𐹬'
    RumiNumberForty,
    /// \u{10e6d}: '𐹭'
    RumiNumberFifty,
    /// \u{10e6e}: '𐹮'
    RumiNumberSixty,
    /// \u{10e6f}: '𐹯'
    RumiNumberSeventy,
    /// \u{10e70}: '𐹰'
    RumiNumberEighty,
    /// \u{10e71}: '𐹱'
    RumiNumberNinety,
    /// \u{10e72}: '𐹲'
    RumiNumberOneHundred,
    /// \u{10e73}: '𐹳'
    RumiNumberTwoHundred,
    /// \u{10e74}: '𐹴'
    RumiNumberThreeHundred,
    /// \u{10e75}: '𐹵'
    RumiNumberFourHundred,
    /// \u{10e76}: '𐹶'
    RumiNumberFiveHundred,
    /// \u{10e77}: '𐹷'
    RumiNumberSixHundred,
    /// \u{10e78}: '𐹸'
    RumiNumberSevenHundred,
    /// \u{10e79}: '𐹹'
    RumiNumberEightHundred,
    /// \u{10e7a}: '𐹺'
    RumiNumberNineHundred,
    /// \u{10e7b}: '𐹻'
    RumiFractionOneHalf,
    /// \u{10e7c}: '𐹼'
    RumiFractionOneQuarter,
    /// \u{10e7d}: '𐹽'
    RumiFractionOneThird,
    /// \u{10e7e}: '𐹾'
    RumiFractionTwoThirds,
}

impl Into<char> for RumiNumeralSymbols {
    fn into(self) -> char {
        use constants::*;
        match self {
            RumiNumeralSymbols::RumiDigitOne => RUMI_DIGIT_ONE,
            RumiNumeralSymbols::RumiDigitTwo => RUMI_DIGIT_TWO,
            RumiNumeralSymbols::RumiDigitThree => RUMI_DIGIT_THREE,
            RumiNumeralSymbols::RumiDigitFour => RUMI_DIGIT_FOUR,
            RumiNumeralSymbols::RumiDigitFive => RUMI_DIGIT_FIVE,
            RumiNumeralSymbols::RumiDigitSix => RUMI_DIGIT_SIX,
            RumiNumeralSymbols::RumiDigitSeven => RUMI_DIGIT_SEVEN,
            RumiNumeralSymbols::RumiDigitEight => RUMI_DIGIT_EIGHT,
            RumiNumeralSymbols::RumiDigitNine => RUMI_DIGIT_NINE,
            RumiNumeralSymbols::RumiNumberTen => RUMI_NUMBER_TEN,
            RumiNumeralSymbols::RumiNumberTwenty => RUMI_NUMBER_TWENTY,
            RumiNumeralSymbols::RumiNumberThirty => RUMI_NUMBER_THIRTY,
            RumiNumeralSymbols::RumiNumberForty => RUMI_NUMBER_FORTY,
            RumiNumeralSymbols::RumiNumberFifty => RUMI_NUMBER_FIFTY,
            RumiNumeralSymbols::RumiNumberSixty => RUMI_NUMBER_SIXTY,
            RumiNumeralSymbols::RumiNumberSeventy => RUMI_NUMBER_SEVENTY,
            RumiNumeralSymbols::RumiNumberEighty => RUMI_NUMBER_EIGHTY,
            RumiNumeralSymbols::RumiNumberNinety => RUMI_NUMBER_NINETY,
            RumiNumeralSymbols::RumiNumberOneHundred => RUMI_NUMBER_ONE_HUNDRED,
            RumiNumeralSymbols::RumiNumberTwoHundred => RUMI_NUMBER_TWO_HUNDRED,
            RumiNumeralSymbols::RumiNumberThreeHundred => RUMI_NUMBER_THREE_HUNDRED,
            RumiNumeralSymbols::RumiNumberFourHundred => RUMI_NUMBER_FOUR_HUNDRED,
            RumiNumeralSymbols::RumiNumberFiveHundred => RUMI_NUMBER_FIVE_HUNDRED,
            RumiNumeralSymbols::RumiNumberSixHundred => RUMI_NUMBER_SIX_HUNDRED,
            RumiNumeralSymbols::RumiNumberSevenHundred => RUMI_NUMBER_SEVEN_HUNDRED,
            RumiNumeralSymbols::RumiNumberEightHundred => RUMI_NUMBER_EIGHT_HUNDRED,
            RumiNumeralSymbols::RumiNumberNineHundred => RUMI_NUMBER_NINE_HUNDRED,
            RumiNumeralSymbols::RumiFractionOneHalf => RUMI_FRACTION_ONE_HALF,
            RumiNumeralSymbols::RumiFractionOneQuarter => RUMI_FRACTION_ONE_QUARTER,
            RumiNumeralSymbols::RumiFractionOneThird => RUMI_FRACTION_ONE_THIRD,
            RumiNumeralSymbols::RumiFractionTwoThirds => RUMI_FRACTION_TWO_THIRDS,
        }
    }
}

impl std::convert::TryFrom<char> for RumiNumeralSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            RUMI_DIGIT_ONE => Ok(RumiNumeralSymbols::RumiDigitOne),
            RUMI_DIGIT_TWO => Ok(RumiNumeralSymbols::RumiDigitTwo),
            RUMI_DIGIT_THREE => Ok(RumiNumeralSymbols::RumiDigitThree),
            RUMI_DIGIT_FOUR => Ok(RumiNumeralSymbols::RumiDigitFour),
            RUMI_DIGIT_FIVE => Ok(RumiNumeralSymbols::RumiDigitFive),
            RUMI_DIGIT_SIX => Ok(RumiNumeralSymbols::RumiDigitSix),
            RUMI_DIGIT_SEVEN => Ok(RumiNumeralSymbols::RumiDigitSeven),
            RUMI_DIGIT_EIGHT => Ok(RumiNumeralSymbols::RumiDigitEight),
            RUMI_DIGIT_NINE => Ok(RumiNumeralSymbols::RumiDigitNine),
            RUMI_NUMBER_TEN => Ok(RumiNumeralSymbols::RumiNumberTen),
            RUMI_NUMBER_TWENTY => Ok(RumiNumeralSymbols::RumiNumberTwenty),
            RUMI_NUMBER_THIRTY => Ok(RumiNumeralSymbols::RumiNumberThirty),
            RUMI_NUMBER_FORTY => Ok(RumiNumeralSymbols::RumiNumberForty),
            RUMI_NUMBER_FIFTY => Ok(RumiNumeralSymbols::RumiNumberFifty),
            RUMI_NUMBER_SIXTY => Ok(RumiNumeralSymbols::RumiNumberSixty),
            RUMI_NUMBER_SEVENTY => Ok(RumiNumeralSymbols::RumiNumberSeventy),
            RUMI_NUMBER_EIGHTY => Ok(RumiNumeralSymbols::RumiNumberEighty),
            RUMI_NUMBER_NINETY => Ok(RumiNumeralSymbols::RumiNumberNinety),
            RUMI_NUMBER_ONE_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberOneHundred),
            RUMI_NUMBER_TWO_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberTwoHundred),
            RUMI_NUMBER_THREE_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberThreeHundred),
            RUMI_NUMBER_FOUR_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberFourHundred),
            RUMI_NUMBER_FIVE_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberFiveHundred),
            RUMI_NUMBER_SIX_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberSixHundred),
            RUMI_NUMBER_SEVEN_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberSevenHundred),
            RUMI_NUMBER_EIGHT_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberEightHundred),
            RUMI_NUMBER_NINE_HUNDRED => Ok(RumiNumeralSymbols::RumiNumberNineHundred),
            RUMI_FRACTION_ONE_HALF => Ok(RumiNumeralSymbols::RumiFractionOneHalf),
            RUMI_FRACTION_ONE_QUARTER => Ok(RumiNumeralSymbols::RumiFractionOneQuarter),
            RUMI_FRACTION_ONE_THIRD => Ok(RumiNumeralSymbols::RumiFractionOneThird),
            RUMI_FRACTION_TWO_THIRDS => Ok(RumiNumeralSymbols::RumiFractionTwoThirds),
            _ => Err(()),
        }
    }
}

impl Into<u32> for RumiNumeralSymbols {
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

impl std::convert::TryFrom<u32> for RumiNumeralSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for RumiNumeralSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl RumiNumeralSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        RumiNumeralSymbols::RumiDigitOne
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            RumiNumeralSymbols::RumiDigitOne => "rumi digit one",
            RumiNumeralSymbols::RumiDigitTwo => "rumi digit two",
            RumiNumeralSymbols::RumiDigitThree => "rumi digit three",
            RumiNumeralSymbols::RumiDigitFour => "rumi digit four",
            RumiNumeralSymbols::RumiDigitFive => "rumi digit five",
            RumiNumeralSymbols::RumiDigitSix => "rumi digit six",
            RumiNumeralSymbols::RumiDigitSeven => "rumi digit seven",
            RumiNumeralSymbols::RumiDigitEight => "rumi digit eight",
            RumiNumeralSymbols::RumiDigitNine => "rumi digit nine",
            RumiNumeralSymbols::RumiNumberTen => "rumi number ten",
            RumiNumeralSymbols::RumiNumberTwenty => "rumi number twenty",
            RumiNumeralSymbols::RumiNumberThirty => "rumi number thirty",
            RumiNumeralSymbols::RumiNumberForty => "rumi number forty",
            RumiNumeralSymbols::RumiNumberFifty => "rumi number fifty",
            RumiNumeralSymbols::RumiNumberSixty => "rumi number sixty",
            RumiNumeralSymbols::RumiNumberSeventy => "rumi number seventy",
            RumiNumeralSymbols::RumiNumberEighty => "rumi number eighty",
            RumiNumeralSymbols::RumiNumberNinety => "rumi number ninety",
            RumiNumeralSymbols::RumiNumberOneHundred => "rumi number one hundred",
            RumiNumeralSymbols::RumiNumberTwoHundred => "rumi number two hundred",
            RumiNumeralSymbols::RumiNumberThreeHundred => "rumi number three hundred",
            RumiNumeralSymbols::RumiNumberFourHundred => "rumi number four hundred",
            RumiNumeralSymbols::RumiNumberFiveHundred => "rumi number five hundred",
            RumiNumeralSymbols::RumiNumberSixHundred => "rumi number six hundred",
            RumiNumeralSymbols::RumiNumberSevenHundred => "rumi number seven hundred",
            RumiNumeralSymbols::RumiNumberEightHundred => "rumi number eight hundred",
            RumiNumeralSymbols::RumiNumberNineHundred => "rumi number nine hundred",
            RumiNumeralSymbols::RumiFractionOneHalf => "rumi fraction one half",
            RumiNumeralSymbols::RumiFractionOneQuarter => "rumi fraction one quarter",
            RumiNumeralSymbols::RumiFractionOneThird => "rumi fraction one third",
            RumiNumeralSymbols::RumiFractionTwoThirds => "rumi fraction two thirds",
        }
    }
}
