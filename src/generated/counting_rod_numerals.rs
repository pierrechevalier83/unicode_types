/// \u{1d360} → \u{1d37f}\
///\
/// 𝍠 𝍡 𝍢 𝍣 𝍤 𝍥 𝍦 𝍧 𝍨 𝍩 𝍪 𝍫 𝍬 𝍭 𝍮 𝍯
/// 𝍰 𝍱 𝍲 𝍳 𝍴 𝍵 𝍶 𝍷 𝍸
pub mod constants {
    /// \u{1d360}: '𝍠'
    pub const COUNTING_ROD_UNIT_DIGIT_ONE: char = '𝍠';
    /// \u{1d361}: '𝍡'
    pub const COUNTING_ROD_UNIT_DIGIT_TWO: char = '𝍡';
    /// \u{1d362}: '𝍢'
    pub const COUNTING_ROD_UNIT_DIGIT_THREE: char = '𝍢';
    /// \u{1d363}: '𝍣'
    pub const COUNTING_ROD_UNIT_DIGIT_FOUR: char = '𝍣';
    /// \u{1d364}: '𝍤'
    pub const COUNTING_ROD_UNIT_DIGIT_FIVE: char = '𝍤';
    /// \u{1d365}: '𝍥'
    pub const COUNTING_ROD_UNIT_DIGIT_SIX: char = '𝍥';
    /// \u{1d366}: '𝍦'
    pub const COUNTING_ROD_UNIT_DIGIT_SEVEN: char = '𝍦';
    /// \u{1d367}: '𝍧'
    pub const COUNTING_ROD_UNIT_DIGIT_EIGHT: char = '𝍧';
    /// \u{1d368}: '𝍨'
    pub const COUNTING_ROD_UNIT_DIGIT_NINE: char = '𝍨';
    /// \u{1d369}: '𝍩'
    pub const COUNTING_ROD_TENS_DIGIT_ONE: char = '𝍩';
    /// \u{1d36a}: '𝍪'
    pub const COUNTING_ROD_TENS_DIGIT_TWO: char = '𝍪';
    /// \u{1d36b}: '𝍫'
    pub const COUNTING_ROD_TENS_DIGIT_THREE: char = '𝍫';
    /// \u{1d36c}: '𝍬'
    pub const COUNTING_ROD_TENS_DIGIT_FOUR: char = '𝍬';
    /// \u{1d36d}: '𝍭'
    pub const COUNTING_ROD_TENS_DIGIT_FIVE: char = '𝍭';
    /// \u{1d36e}: '𝍮'
    pub const COUNTING_ROD_TENS_DIGIT_SIX: char = '𝍮';
    /// \u{1d36f}: '𝍯'
    pub const COUNTING_ROD_TENS_DIGIT_SEVEN: char = '𝍯';
    /// \u{1d370}: '𝍰'
    pub const COUNTING_ROD_TENS_DIGIT_EIGHT: char = '𝍰';
    /// \u{1d371}: '𝍱'
    pub const COUNTING_ROD_TENS_DIGIT_NINE: char = '𝍱';
    /// \u{1d372}: '𝍲'
    pub const IDEOGRAPHIC_TALLY_MARK_ONE: char = '𝍲';
    /// \u{1d373}: '𝍳'
    pub const IDEOGRAPHIC_TALLY_MARK_TWO: char = '𝍳';
    /// \u{1d374}: '𝍴'
    pub const IDEOGRAPHIC_TALLY_MARK_THREE: char = '𝍴';
    /// \u{1d375}: '𝍵'
    pub const IDEOGRAPHIC_TALLY_MARK_FOUR: char = '𝍵';
    /// \u{1d376}: '𝍶'
    pub const IDEOGRAPHIC_TALLY_MARK_FIVE: char = '𝍶';
    /// \u{1d377}: '𝍷'
    pub const TALLY_MARK_ONE: char = '𝍷';
    /// \u{1d378}: '𝍸'
    pub const TALLY_MARK_FIVE: char = '𝍸';
}

/// \u{1d360} → \u{1d37f}\
///\
/// 𝍠 𝍡 𝍢 𝍣 𝍤 𝍥 𝍦 𝍧 𝍨 𝍩 𝍪 𝍫 𝍬 𝍭 𝍮 𝍯
/// 𝍰 𝍱 𝍲 𝍳 𝍴 𝍵 𝍶 𝍷 𝍸
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CountingRodNumerals {
    /// \u{1d360}: '𝍠'
    CountingRodUnitDigitOne,
    /// \u{1d361}: '𝍡'
    CountingRodUnitDigitTwo,
    /// \u{1d362}: '𝍢'
    CountingRodUnitDigitThree,
    /// \u{1d363}: '𝍣'
    CountingRodUnitDigitFour,
    /// \u{1d364}: '𝍤'
    CountingRodUnitDigitFive,
    /// \u{1d365}: '𝍥'
    CountingRodUnitDigitSix,
    /// \u{1d366}: '𝍦'
    CountingRodUnitDigitSeven,
    /// \u{1d367}: '𝍧'
    CountingRodUnitDigitEight,
    /// \u{1d368}: '𝍨'
    CountingRodUnitDigitNine,
    /// \u{1d369}: '𝍩'
    CountingRodTensDigitOne,
    /// \u{1d36a}: '𝍪'
    CountingRodTensDigitTwo,
    /// \u{1d36b}: '𝍫'
    CountingRodTensDigitThree,
    /// \u{1d36c}: '𝍬'
    CountingRodTensDigitFour,
    /// \u{1d36d}: '𝍭'
    CountingRodTensDigitFive,
    /// \u{1d36e}: '𝍮'
    CountingRodTensDigitSix,
    /// \u{1d36f}: '𝍯'
    CountingRodTensDigitSeven,
    /// \u{1d370}: '𝍰'
    CountingRodTensDigitEight,
    /// \u{1d371}: '𝍱'
    CountingRodTensDigitNine,
    /// \u{1d372}: '𝍲'
    IdeographicTallyMarkOne,
    /// \u{1d373}: '𝍳'
    IdeographicTallyMarkTwo,
    /// \u{1d374}: '𝍴'
    IdeographicTallyMarkThree,
    /// \u{1d375}: '𝍵'
    IdeographicTallyMarkFour,
    /// \u{1d376}: '𝍶'
    IdeographicTallyMarkFive,
    /// \u{1d377}: '𝍷'
    TallyMarkOne,
    /// \u{1d378}: '𝍸'
    TallyMarkFive,
}

impl Into<char> for CountingRodNumerals {
    fn into(self) -> char {
        use constants::*;
        match self {
            CountingRodNumerals::CountingRodUnitDigitOne => COUNTING_ROD_UNIT_DIGIT_ONE,
            CountingRodNumerals::CountingRodUnitDigitTwo => COUNTING_ROD_UNIT_DIGIT_TWO,
            CountingRodNumerals::CountingRodUnitDigitThree => COUNTING_ROD_UNIT_DIGIT_THREE,
            CountingRodNumerals::CountingRodUnitDigitFour => COUNTING_ROD_UNIT_DIGIT_FOUR,
            CountingRodNumerals::CountingRodUnitDigitFive => COUNTING_ROD_UNIT_DIGIT_FIVE,
            CountingRodNumerals::CountingRodUnitDigitSix => COUNTING_ROD_UNIT_DIGIT_SIX,
            CountingRodNumerals::CountingRodUnitDigitSeven => COUNTING_ROD_UNIT_DIGIT_SEVEN,
            CountingRodNumerals::CountingRodUnitDigitEight => COUNTING_ROD_UNIT_DIGIT_EIGHT,
            CountingRodNumerals::CountingRodUnitDigitNine => COUNTING_ROD_UNIT_DIGIT_NINE,
            CountingRodNumerals::CountingRodTensDigitOne => COUNTING_ROD_TENS_DIGIT_ONE,
            CountingRodNumerals::CountingRodTensDigitTwo => COUNTING_ROD_TENS_DIGIT_TWO,
            CountingRodNumerals::CountingRodTensDigitThree => COUNTING_ROD_TENS_DIGIT_THREE,
            CountingRodNumerals::CountingRodTensDigitFour => COUNTING_ROD_TENS_DIGIT_FOUR,
            CountingRodNumerals::CountingRodTensDigitFive => COUNTING_ROD_TENS_DIGIT_FIVE,
            CountingRodNumerals::CountingRodTensDigitSix => COUNTING_ROD_TENS_DIGIT_SIX,
            CountingRodNumerals::CountingRodTensDigitSeven => COUNTING_ROD_TENS_DIGIT_SEVEN,
            CountingRodNumerals::CountingRodTensDigitEight => COUNTING_ROD_TENS_DIGIT_EIGHT,
            CountingRodNumerals::CountingRodTensDigitNine => COUNTING_ROD_TENS_DIGIT_NINE,
            CountingRodNumerals::IdeographicTallyMarkOne => IDEOGRAPHIC_TALLY_MARK_ONE,
            CountingRodNumerals::IdeographicTallyMarkTwo => IDEOGRAPHIC_TALLY_MARK_TWO,
            CountingRodNumerals::IdeographicTallyMarkThree => IDEOGRAPHIC_TALLY_MARK_THREE,
            CountingRodNumerals::IdeographicTallyMarkFour => IDEOGRAPHIC_TALLY_MARK_FOUR,
            CountingRodNumerals::IdeographicTallyMarkFive => IDEOGRAPHIC_TALLY_MARK_FIVE,
            CountingRodNumerals::TallyMarkOne => TALLY_MARK_ONE,
            CountingRodNumerals::TallyMarkFive => TALLY_MARK_FIVE,
        }
    }
}

impl std::convert::TryFrom<char> for CountingRodNumerals {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COUNTING_ROD_UNIT_DIGIT_ONE => Ok(CountingRodNumerals::CountingRodUnitDigitOne),
            COUNTING_ROD_UNIT_DIGIT_TWO => Ok(CountingRodNumerals::CountingRodUnitDigitTwo),
            COUNTING_ROD_UNIT_DIGIT_THREE => Ok(CountingRodNumerals::CountingRodUnitDigitThree),
            COUNTING_ROD_UNIT_DIGIT_FOUR => Ok(CountingRodNumerals::CountingRodUnitDigitFour),
            COUNTING_ROD_UNIT_DIGIT_FIVE => Ok(CountingRodNumerals::CountingRodUnitDigitFive),
            COUNTING_ROD_UNIT_DIGIT_SIX => Ok(CountingRodNumerals::CountingRodUnitDigitSix),
            COUNTING_ROD_UNIT_DIGIT_SEVEN => Ok(CountingRodNumerals::CountingRodUnitDigitSeven),
            COUNTING_ROD_UNIT_DIGIT_EIGHT => Ok(CountingRodNumerals::CountingRodUnitDigitEight),
            COUNTING_ROD_UNIT_DIGIT_NINE => Ok(CountingRodNumerals::CountingRodUnitDigitNine),
            COUNTING_ROD_TENS_DIGIT_ONE => Ok(CountingRodNumerals::CountingRodTensDigitOne),
            COUNTING_ROD_TENS_DIGIT_TWO => Ok(CountingRodNumerals::CountingRodTensDigitTwo),
            COUNTING_ROD_TENS_DIGIT_THREE => Ok(CountingRodNumerals::CountingRodTensDigitThree),
            COUNTING_ROD_TENS_DIGIT_FOUR => Ok(CountingRodNumerals::CountingRodTensDigitFour),
            COUNTING_ROD_TENS_DIGIT_FIVE => Ok(CountingRodNumerals::CountingRodTensDigitFive),
            COUNTING_ROD_TENS_DIGIT_SIX => Ok(CountingRodNumerals::CountingRodTensDigitSix),
            COUNTING_ROD_TENS_DIGIT_SEVEN => Ok(CountingRodNumerals::CountingRodTensDigitSeven),
            COUNTING_ROD_TENS_DIGIT_EIGHT => Ok(CountingRodNumerals::CountingRodTensDigitEight),
            COUNTING_ROD_TENS_DIGIT_NINE => Ok(CountingRodNumerals::CountingRodTensDigitNine),
            IDEOGRAPHIC_TALLY_MARK_ONE => Ok(CountingRodNumerals::IdeographicTallyMarkOne),
            IDEOGRAPHIC_TALLY_MARK_TWO => Ok(CountingRodNumerals::IdeographicTallyMarkTwo),
            IDEOGRAPHIC_TALLY_MARK_THREE => Ok(CountingRodNumerals::IdeographicTallyMarkThree),
            IDEOGRAPHIC_TALLY_MARK_FOUR => Ok(CountingRodNumerals::IdeographicTallyMarkFour),
            IDEOGRAPHIC_TALLY_MARK_FIVE => Ok(CountingRodNumerals::IdeographicTallyMarkFive),
            TALLY_MARK_ONE => Ok(CountingRodNumerals::TallyMarkOne),
            TALLY_MARK_FIVE => Ok(CountingRodNumerals::TallyMarkFive),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CountingRodNumerals {
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

impl std::convert::TryFrom<u32> for CountingRodNumerals {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CountingRodNumerals {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CountingRodNumerals {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CountingRodNumerals::CountingRodUnitDigitOne
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CountingRodNumerals::CountingRodUnitDigitOne => "counting rod unit digit one",
            CountingRodNumerals::CountingRodUnitDigitTwo => "counting rod unit digit two",
            CountingRodNumerals::CountingRodUnitDigitThree => "counting rod unit digit three",
            CountingRodNumerals::CountingRodUnitDigitFour => "counting rod unit digit four",
            CountingRodNumerals::CountingRodUnitDigitFive => "counting rod unit digit five",
            CountingRodNumerals::CountingRodUnitDigitSix => "counting rod unit digit six",
            CountingRodNumerals::CountingRodUnitDigitSeven => "counting rod unit digit seven",
            CountingRodNumerals::CountingRodUnitDigitEight => "counting rod unit digit eight",
            CountingRodNumerals::CountingRodUnitDigitNine => "counting rod unit digit nine",
            CountingRodNumerals::CountingRodTensDigitOne => "counting rod tens digit one",
            CountingRodNumerals::CountingRodTensDigitTwo => "counting rod tens digit two",
            CountingRodNumerals::CountingRodTensDigitThree => "counting rod tens digit three",
            CountingRodNumerals::CountingRodTensDigitFour => "counting rod tens digit four",
            CountingRodNumerals::CountingRodTensDigitFive => "counting rod tens digit five",
            CountingRodNumerals::CountingRodTensDigitSix => "counting rod tens digit six",
            CountingRodNumerals::CountingRodTensDigitSeven => "counting rod tens digit seven",
            CountingRodNumerals::CountingRodTensDigitEight => "counting rod tens digit eight",
            CountingRodNumerals::CountingRodTensDigitNine => "counting rod tens digit nine",
            CountingRodNumerals::IdeographicTallyMarkOne => "ideographic tally mark one",
            CountingRodNumerals::IdeographicTallyMarkTwo => "ideographic tally mark two",
            CountingRodNumerals::IdeographicTallyMarkThree => "ideographic tally mark three",
            CountingRodNumerals::IdeographicTallyMarkFour => "ideographic tally mark four",
            CountingRodNumerals::IdeographicTallyMarkFive => "ideographic tally mark five",
            CountingRodNumerals::TallyMarkOne => "tally mark one",
            CountingRodNumerals::TallyMarkFive => "tally mark five",
        }
    }
}
