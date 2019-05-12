/// \u{a830} → \u{a83f}\
///\
/// ꠰ ꠱ ꠲ ꠳ ꠴ ꠵ ꠶ ꠷ ꠸ ꠹\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{a830}: '꠰'
    pub const NORTH_INDIC_FRACTION_ONE_QUARTER: char = '꠰';
    /// \u{a831}: '꠱'
    pub const NORTH_INDIC_FRACTION_ONE_HALF: char = '꠱';
    /// \u{a832}: '꠲'
    pub const NORTH_INDIC_FRACTION_THREE_QUARTERS: char = '꠲';
    /// \u{a833}: '꠳'
    pub const NORTH_INDIC_FRACTION_ONE_SIXTEENTH: char = '꠳';
    /// \u{a834}: '꠴'
    pub const NORTH_INDIC_FRACTION_ONE_EIGHTH: char = '꠴';
    /// \u{a835}: '꠵'
    pub const NORTH_INDIC_FRACTION_THREE_SIXTEENTHS: char = '꠵';
    /// \u{a836}: '꠶'
    pub const NORTH_INDIC_QUARTER_MARK: char = '꠶';
    /// \u{a837}: '꠷'
    pub const NORTH_INDIC_PLACEHOLDER_MARK: char = '꠷';
    /// \u{a838}: '꠸'
    pub const NORTH_INDIC_RUPEE_MARK: char = '꠸';
    /// \u{a839}: '꠹'
    pub const NORTH_INDIC_QUANTITY_MARK: char = '꠹';
}

/// An enum to represent all characters in the CommonIndicNumberForms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CommonIndicNumberForms {
    /// \u{a830}: '꠰'
    NorthIndicFractionOneQuarter,
    /// \u{a831}: '꠱'
    NorthIndicFractionOneHalf,
    /// \u{a832}: '꠲'
    NorthIndicFractionThreeQuarters,
    /// \u{a833}: '꠳'
    NorthIndicFractionOneSixteenth,
    /// \u{a834}: '꠴'
    NorthIndicFractionOneEighth,
    /// \u{a835}: '꠵'
    NorthIndicFractionThreeSixteenths,
    /// \u{a836}: '꠶'
    NorthIndicQuarterMark,
    /// \u{a837}: '꠷'
    NorthIndicPlaceholderMark,
    /// \u{a838}: '꠸'
    NorthIndicRupeeMark,
    /// \u{a839}: '꠹'
    NorthIndicQuantityMark,
}

impl Into<char> for CommonIndicNumberForms {
    fn into(self) -> char {
        use constants::*;
        match self {
            CommonIndicNumberForms::NorthIndicFractionOneQuarter => NORTH_INDIC_FRACTION_ONE_QUARTER,
            CommonIndicNumberForms::NorthIndicFractionOneHalf => NORTH_INDIC_FRACTION_ONE_HALF,
            CommonIndicNumberForms::NorthIndicFractionThreeQuarters => NORTH_INDIC_FRACTION_THREE_QUARTERS,
            CommonIndicNumberForms::NorthIndicFractionOneSixteenth => NORTH_INDIC_FRACTION_ONE_SIXTEENTH,
            CommonIndicNumberForms::NorthIndicFractionOneEighth => NORTH_INDIC_FRACTION_ONE_EIGHTH,
            CommonIndicNumberForms::NorthIndicFractionThreeSixteenths => NORTH_INDIC_FRACTION_THREE_SIXTEENTHS,
            CommonIndicNumberForms::NorthIndicQuarterMark => NORTH_INDIC_QUARTER_MARK,
            CommonIndicNumberForms::NorthIndicPlaceholderMark => NORTH_INDIC_PLACEHOLDER_MARK,
            CommonIndicNumberForms::NorthIndicRupeeMark => NORTH_INDIC_RUPEE_MARK,
            CommonIndicNumberForms::NorthIndicQuantityMark => NORTH_INDIC_QUANTITY_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for CommonIndicNumberForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            NORTH_INDIC_FRACTION_ONE_QUARTER => Ok(CommonIndicNumberForms::NorthIndicFractionOneQuarter),
            NORTH_INDIC_FRACTION_ONE_HALF => Ok(CommonIndicNumberForms::NorthIndicFractionOneHalf),
            NORTH_INDIC_FRACTION_THREE_QUARTERS => Ok(CommonIndicNumberForms::NorthIndicFractionThreeQuarters),
            NORTH_INDIC_FRACTION_ONE_SIXTEENTH => Ok(CommonIndicNumberForms::NorthIndicFractionOneSixteenth),
            NORTH_INDIC_FRACTION_ONE_EIGHTH => Ok(CommonIndicNumberForms::NorthIndicFractionOneEighth),
            NORTH_INDIC_FRACTION_THREE_SIXTEENTHS => Ok(CommonIndicNumberForms::NorthIndicFractionThreeSixteenths),
            NORTH_INDIC_QUARTER_MARK => Ok(CommonIndicNumberForms::NorthIndicQuarterMark),
            NORTH_INDIC_PLACEHOLDER_MARK => Ok(CommonIndicNumberForms::NorthIndicPlaceholderMark),
            NORTH_INDIC_RUPEE_MARK => Ok(CommonIndicNumberForms::NorthIndicRupeeMark),
            NORTH_INDIC_QUANTITY_MARK => Ok(CommonIndicNumberForms::NorthIndicQuantityMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CommonIndicNumberForms {
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

impl std::convert::TryFrom<u32> for CommonIndicNumberForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CommonIndicNumberForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CommonIndicNumberForms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CommonIndicNumberForms::NorthIndicFractionOneQuarter
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CommonIndicNumberForms::NorthIndicFractionOneQuarter => "north indic fraction one quarter",
            CommonIndicNumberForms::NorthIndicFractionOneHalf => "north indic fraction one half",
            CommonIndicNumberForms::NorthIndicFractionThreeQuarters => "north indic fraction three quarters",
            CommonIndicNumberForms::NorthIndicFractionOneSixteenth => "north indic fraction one sixteenth",
            CommonIndicNumberForms::NorthIndicFractionOneEighth => "north indic fraction one eighth",
            CommonIndicNumberForms::NorthIndicFractionThreeSixteenths => "north indic fraction three sixteenths",
            CommonIndicNumberForms::NorthIndicQuarterMark => "north indic quarter mark",
            CommonIndicNumberForms::NorthIndicPlaceholderMark => "north indic placeholder mark",
            CommonIndicNumberForms::NorthIndicRupeeMark => "north indic rupee mark",
            CommonIndicNumberForms::NorthIndicQuantityMark => "north indic quantity mark",
        }
    }
}
