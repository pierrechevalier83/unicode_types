/// \u{fe00} → \u{fe0f}
///
/// ︀ ︁ ︂ ︃ ︄ ︅ ︆ ︇ ︈ ︉ ︊ ︋ ︌ ︍ ︎\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{fe00}: '︀'
    pub const VARIATION_SELECTOR_DASH_1: char = '︀';
    /// \u{fe01}: '︁'
    pub const VARIATION_SELECTOR_DASH_2: char = '︁';
    /// \u{fe02}: '︂'
    pub const VARIATION_SELECTOR_DASH_3: char = '︂';
    /// \u{fe03}: '︃'
    pub const VARIATION_SELECTOR_DASH_4: char = '︃';
    /// \u{fe04}: '︄'
    pub const VARIATION_SELECTOR_DASH_5: char = '︄';
    /// \u{fe05}: '︅'
    pub const VARIATION_SELECTOR_DASH_6: char = '︅';
    /// \u{fe06}: '︆'
    pub const VARIATION_SELECTOR_DASH_7: char = '︆';
    /// \u{fe07}: '︇'
    pub const VARIATION_SELECTOR_DASH_8: char = '︇';
    /// \u{fe08}: '︈'
    pub const VARIATION_SELECTOR_DASH_9: char = '︈';
    /// \u{fe09}: '︉'
    pub const VARIATION_SELECTOR_DASH_10: char = '︉';
    /// \u{fe0a}: '︊'
    pub const VARIATION_SELECTOR_DASH_11: char = '︊';
    /// \u{fe0b}: '︋'
    pub const VARIATION_SELECTOR_DASH_12: char = '︋';
    /// \u{fe0c}: '︌'
    pub const VARIATION_SELECTOR_DASH_13: char = '︌';
    /// \u{fe0d}: '︍'
    pub const VARIATION_SELECTOR_DASH_14: char = '︍';
    /// \u{fe0e}: '︎'
    pub const VARIATION_SELECTOR_DASH_15: char = '︎';
}

/// An enum to represent all characters in the VariationSelectors block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum VariationSelectors {
    /// \u{fe00}: '︀'
    VariationSelectorDash1,
    /// \u{fe01}: '︁'
    VariationSelectorDash2,
    /// \u{fe02}: '︂'
    VariationSelectorDash3,
    /// \u{fe03}: '︃'
    VariationSelectorDash4,
    /// \u{fe04}: '︄'
    VariationSelectorDash5,
    /// \u{fe05}: '︅'
    VariationSelectorDash6,
    /// \u{fe06}: '︆'
    VariationSelectorDash7,
    /// \u{fe07}: '︇'
    VariationSelectorDash8,
    /// \u{fe08}: '︈'
    VariationSelectorDash9,
    /// \u{fe09}: '︉'
    VariationSelectorDash10,
    /// \u{fe0a}: '︊'
    VariationSelectorDash11,
    /// \u{fe0b}: '︋'
    VariationSelectorDash12,
    /// \u{fe0c}: '︌'
    VariationSelectorDash13,
    /// \u{fe0d}: '︍'
    VariationSelectorDash14,
    /// \u{fe0e}: '︎'
    VariationSelectorDash15,
}

impl Into<char> for VariationSelectors {
    fn into(self) -> char {
        use constants::*;
        match self {
            VariationSelectors::VariationSelectorDash1 => VARIATION_SELECTOR_DASH_1,
            VariationSelectors::VariationSelectorDash2 => VARIATION_SELECTOR_DASH_2,
            VariationSelectors::VariationSelectorDash3 => VARIATION_SELECTOR_DASH_3,
            VariationSelectors::VariationSelectorDash4 => VARIATION_SELECTOR_DASH_4,
            VariationSelectors::VariationSelectorDash5 => VARIATION_SELECTOR_DASH_5,
            VariationSelectors::VariationSelectorDash6 => VARIATION_SELECTOR_DASH_6,
            VariationSelectors::VariationSelectorDash7 => VARIATION_SELECTOR_DASH_7,
            VariationSelectors::VariationSelectorDash8 => VARIATION_SELECTOR_DASH_8,
            VariationSelectors::VariationSelectorDash9 => VARIATION_SELECTOR_DASH_9,
            VariationSelectors::VariationSelectorDash10 => VARIATION_SELECTOR_DASH_10,
            VariationSelectors::VariationSelectorDash11 => VARIATION_SELECTOR_DASH_11,
            VariationSelectors::VariationSelectorDash12 => VARIATION_SELECTOR_DASH_12,
            VariationSelectors::VariationSelectorDash13 => VARIATION_SELECTOR_DASH_13,
            VariationSelectors::VariationSelectorDash14 => VARIATION_SELECTOR_DASH_14,
            VariationSelectors::VariationSelectorDash15 => VARIATION_SELECTOR_DASH_15,
        }
    }
}

impl std::convert::TryFrom<char> for VariationSelectors {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            VARIATION_SELECTOR_DASH_1 => Ok(VariationSelectors::VariationSelectorDash1),
            VARIATION_SELECTOR_DASH_2 => Ok(VariationSelectors::VariationSelectorDash2),
            VARIATION_SELECTOR_DASH_3 => Ok(VariationSelectors::VariationSelectorDash3),
            VARIATION_SELECTOR_DASH_4 => Ok(VariationSelectors::VariationSelectorDash4),
            VARIATION_SELECTOR_DASH_5 => Ok(VariationSelectors::VariationSelectorDash5),
            VARIATION_SELECTOR_DASH_6 => Ok(VariationSelectors::VariationSelectorDash6),
            VARIATION_SELECTOR_DASH_7 => Ok(VariationSelectors::VariationSelectorDash7),
            VARIATION_SELECTOR_DASH_8 => Ok(VariationSelectors::VariationSelectorDash8),
            VARIATION_SELECTOR_DASH_9 => Ok(VariationSelectors::VariationSelectorDash9),
            VARIATION_SELECTOR_DASH_10 => Ok(VariationSelectors::VariationSelectorDash10),
            VARIATION_SELECTOR_DASH_11 => Ok(VariationSelectors::VariationSelectorDash11),
            VARIATION_SELECTOR_DASH_12 => Ok(VariationSelectors::VariationSelectorDash12),
            VARIATION_SELECTOR_DASH_13 => Ok(VariationSelectors::VariationSelectorDash13),
            VARIATION_SELECTOR_DASH_14 => Ok(VariationSelectors::VariationSelectorDash14),
            VARIATION_SELECTOR_DASH_15 => Ok(VariationSelectors::VariationSelectorDash15),
            _ => Err(()),
        }
    }
}

impl Into<u32> for VariationSelectors {
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

impl std::convert::TryFrom<u32> for VariationSelectors {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for VariationSelectors {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl VariationSelectors {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        VariationSelectors::VariationSelectorDash1
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            VariationSelectors::VariationSelectorDash1 => "variation selector-1",
            VariationSelectors::VariationSelectorDash2 => "variation selector-2",
            VariationSelectors::VariationSelectorDash3 => "variation selector-3",
            VariationSelectors::VariationSelectorDash4 => "variation selector-4",
            VariationSelectors::VariationSelectorDash5 => "variation selector-5",
            VariationSelectors::VariationSelectorDash6 => "variation selector-6",
            VariationSelectors::VariationSelectorDash7 => "variation selector-7",
            VariationSelectors::VariationSelectorDash8 => "variation selector-8",
            VariationSelectors::VariationSelectorDash9 => "variation selector-9",
            VariationSelectors::VariationSelectorDash10 => "variation selector-10",
            VariationSelectors::VariationSelectorDash11 => "variation selector-11",
            VariationSelectors::VariationSelectorDash12 => "variation selector-12",
            VariationSelectors::VariationSelectorDash13 => "variation selector-13",
            VariationSelectors::VariationSelectorDash14 => "variation selector-14",
            VariationSelectors::VariationSelectorDash15 => "variation selector-15",
        }
    }
}
