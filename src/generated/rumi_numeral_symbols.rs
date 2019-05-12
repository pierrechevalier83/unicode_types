
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
        match self {
            RumiNumeralSymbols::RumiDigitOne => '𐹠',
            RumiNumeralSymbols::RumiDigitTwo => '𐹡',
            RumiNumeralSymbols::RumiDigitThree => '𐹢',
            RumiNumeralSymbols::RumiDigitFour => '𐹣',
            RumiNumeralSymbols::RumiDigitFive => '𐹤',
            RumiNumeralSymbols::RumiDigitSix => '𐹥',
            RumiNumeralSymbols::RumiDigitSeven => '𐹦',
            RumiNumeralSymbols::RumiDigitEight => '𐹧',
            RumiNumeralSymbols::RumiDigitNine => '𐹨',
            RumiNumeralSymbols::RumiNumberTen => '𐹩',
            RumiNumeralSymbols::RumiNumberTwenty => '𐹪',
            RumiNumeralSymbols::RumiNumberThirty => '𐹫',
            RumiNumeralSymbols::RumiNumberForty => '𐹬',
            RumiNumeralSymbols::RumiNumberFifty => '𐹭',
            RumiNumeralSymbols::RumiNumberSixty => '𐹮',
            RumiNumeralSymbols::RumiNumberSeventy => '𐹯',
            RumiNumeralSymbols::RumiNumberEighty => '𐹰',
            RumiNumeralSymbols::RumiNumberNinety => '𐹱',
            RumiNumeralSymbols::RumiNumberOneHundred => '𐹲',
            RumiNumeralSymbols::RumiNumberTwoHundred => '𐹳',
            RumiNumeralSymbols::RumiNumberThreeHundred => '𐹴',
            RumiNumeralSymbols::RumiNumberFourHundred => '𐹵',
            RumiNumeralSymbols::RumiNumberFiveHundred => '𐹶',
            RumiNumeralSymbols::RumiNumberSixHundred => '𐹷',
            RumiNumeralSymbols::RumiNumberSevenHundred => '𐹸',
            RumiNumeralSymbols::RumiNumberEightHundred => '𐹹',
            RumiNumeralSymbols::RumiNumberNineHundred => '𐹺',
            RumiNumeralSymbols::RumiFractionOneHalf => '𐹻',
            RumiNumeralSymbols::RumiFractionOneQuarter => '𐹼',
            RumiNumeralSymbols::RumiFractionOneThird => '𐹽',
            RumiNumeralSymbols::RumiFractionTwoThirds => '𐹾',
        }
    }
}

impl std::convert::TryFrom<char> for RumiNumeralSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐹠' => Ok(RumiNumeralSymbols::RumiDigitOne),
            '𐹡' => Ok(RumiNumeralSymbols::RumiDigitTwo),
            '𐹢' => Ok(RumiNumeralSymbols::RumiDigitThree),
            '𐹣' => Ok(RumiNumeralSymbols::RumiDigitFour),
            '𐹤' => Ok(RumiNumeralSymbols::RumiDigitFive),
            '𐹥' => Ok(RumiNumeralSymbols::RumiDigitSix),
            '𐹦' => Ok(RumiNumeralSymbols::RumiDigitSeven),
            '𐹧' => Ok(RumiNumeralSymbols::RumiDigitEight),
            '𐹨' => Ok(RumiNumeralSymbols::RumiDigitNine),
            '𐹩' => Ok(RumiNumeralSymbols::RumiNumberTen),
            '𐹪' => Ok(RumiNumeralSymbols::RumiNumberTwenty),
            '𐹫' => Ok(RumiNumeralSymbols::RumiNumberThirty),
            '𐹬' => Ok(RumiNumeralSymbols::RumiNumberForty),
            '𐹭' => Ok(RumiNumeralSymbols::RumiNumberFifty),
            '𐹮' => Ok(RumiNumeralSymbols::RumiNumberSixty),
            '𐹯' => Ok(RumiNumeralSymbols::RumiNumberSeventy),
            '𐹰' => Ok(RumiNumeralSymbols::RumiNumberEighty),
            '𐹱' => Ok(RumiNumeralSymbols::RumiNumberNinety),
            '𐹲' => Ok(RumiNumeralSymbols::RumiNumberOneHundred),
            '𐹳' => Ok(RumiNumeralSymbols::RumiNumberTwoHundred),
            '𐹴' => Ok(RumiNumeralSymbols::RumiNumberThreeHundred),
            '𐹵' => Ok(RumiNumeralSymbols::RumiNumberFourHundred),
            '𐹶' => Ok(RumiNumeralSymbols::RumiNumberFiveHundred),
            '𐹷' => Ok(RumiNumeralSymbols::RumiNumberSixHundred),
            '𐹸' => Ok(RumiNumeralSymbols::RumiNumberSevenHundred),
            '𐹹' => Ok(RumiNumeralSymbols::RumiNumberEightHundred),
            '𐹺' => Ok(RumiNumeralSymbols::RumiNumberNineHundred),
            '𐹻' => Ok(RumiNumeralSymbols::RumiFractionOneHalf),
            '𐹼' => Ok(RumiNumeralSymbols::RumiFractionOneQuarter),
            '𐹽' => Ok(RumiNumeralSymbols::RumiFractionOneThird),
            '𐹾' => Ok(RumiNumeralSymbols::RumiFractionTwoThirds),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("RumiNumeralSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
