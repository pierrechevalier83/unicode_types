/// \u{1ab0} → \u{1aff}\
///\
/// ᪰ ᪱ ᪲ ᪳ ᪴ ᪵ ᪶ ᪷ ᪸ ᪹ ᪺ ᪻ ᪼ ᪽ ᪾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1ab0}: '᪰'
    pub const COMBINING_DOUBLED_CIRCUMFLEX_ACCENT: char = '᪰';
    /// \u{1ab1}: '᪱'
    pub const COMBINING_DIAERESIS_DASH_RING: char = '᪱';
    /// \u{1ab2}: '᪲'
    pub const COMBINING_INFINITY: char = '᪲';
    /// \u{1ab3}: '᪳'
    pub const COMBINING_DOWNWARDS_ARROW: char = '᪳';
    /// \u{1ab4}: '᪴'
    pub const COMBINING_TRIPLE_DOT: char = '᪴';
    /// \u{1ab5}: '᪵'
    pub const COMBINING_X_DASH_X_BELOW: char = '᪵';
    /// \u{1ab6}: '᪶'
    pub const COMBINING_WIGGLY_LINE_BELOW: char = '᪶';
    /// \u{1ab7}: '᪷'
    pub const COMBINING_OPEN_MARK_BELOW: char = '᪷';
    /// \u{1ab8}: '᪸'
    pub const COMBINING_DOUBLE_OPEN_MARK_BELOW: char = '᪸';
    /// \u{1ab9}: '᪹'
    pub const COMBINING_LIGHT_CENTRALIZATION_STROKE_BELOW: char = '᪹';
    /// \u{1aba}: '᪺'
    pub const COMBINING_STRONG_CENTRALIZATION_STROKE_BELOW: char = '᪺';
    /// \u{1abb}: '᪻'
    pub const COMBINING_PARENTHESES_ABOVE: char = '᪻';
    /// \u{1abc}: '᪼'
    pub const COMBINING_DOUBLE_PARENTHESES_ABOVE: char = '᪼';
    /// \u{1abd}: '᪽'
    pub const COMBINING_PARENTHESES_BELOW: char = '᪽';
    /// \u{1abe}: '᪾'
    pub const COMBINING_PARENTHESES_OVERLAY: char = '᪾';
}

/// An enum to represent all characters in the CombiningDiacriticalMarksExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CombiningDiacriticalMarksExtended {
    /// \u{1ab0}: '᪰'
    CombiningDoubledCircumflexAccent,
    /// \u{1ab1}: '᪱'
    CombiningDiaeresisDashRing,
    /// \u{1ab2}: '᪲'
    CombiningInfinity,
    /// \u{1ab3}: '᪳'
    CombiningDownwardsArrow,
    /// \u{1ab4}: '᪴'
    CombiningTripleDot,
    /// \u{1ab5}: '᪵'
    CombiningXDashXBelow,
    /// \u{1ab6}: '᪶'
    CombiningWigglyLineBelow,
    /// \u{1ab7}: '᪷'
    CombiningOpenMarkBelow,
    /// \u{1ab8}: '᪸'
    CombiningDoubleOpenMarkBelow,
    /// \u{1ab9}: '᪹'
    CombiningLightCentralizationStrokeBelow,
    /// \u{1aba}: '᪺'
    CombiningStrongCentralizationStrokeBelow,
    /// \u{1abb}: '᪻'
    CombiningParenthesesAbove,
    /// \u{1abc}: '᪼'
    CombiningDoubleParenthesesAbove,
    /// \u{1abd}: '᪽'
    CombiningParenthesesBelow,
    /// \u{1abe}: '᪾'
    CombiningParenthesesOverlay,
}

impl Into<char> for CombiningDiacriticalMarksExtended {
    fn into(self) -> char {
        use constants::*;
        match self {
            CombiningDiacriticalMarksExtended::CombiningDoubledCircumflexAccent => COMBINING_DOUBLED_CIRCUMFLEX_ACCENT,
            CombiningDiacriticalMarksExtended::CombiningDiaeresisDashRing => COMBINING_DIAERESIS_DASH_RING,
            CombiningDiacriticalMarksExtended::CombiningInfinity => COMBINING_INFINITY,
            CombiningDiacriticalMarksExtended::CombiningDownwardsArrow => COMBINING_DOWNWARDS_ARROW,
            CombiningDiacriticalMarksExtended::CombiningTripleDot => COMBINING_TRIPLE_DOT,
            CombiningDiacriticalMarksExtended::CombiningXDashXBelow => COMBINING_X_DASH_X_BELOW,
            CombiningDiacriticalMarksExtended::CombiningWigglyLineBelow => COMBINING_WIGGLY_LINE_BELOW,
            CombiningDiacriticalMarksExtended::CombiningOpenMarkBelow => COMBINING_OPEN_MARK_BELOW,
            CombiningDiacriticalMarksExtended::CombiningDoubleOpenMarkBelow => COMBINING_DOUBLE_OPEN_MARK_BELOW,
            CombiningDiacriticalMarksExtended::CombiningLightCentralizationStrokeBelow => COMBINING_LIGHT_CENTRALIZATION_STROKE_BELOW,
            CombiningDiacriticalMarksExtended::CombiningStrongCentralizationStrokeBelow => COMBINING_STRONG_CENTRALIZATION_STROKE_BELOW,
            CombiningDiacriticalMarksExtended::CombiningParenthesesAbove => COMBINING_PARENTHESES_ABOVE,
            CombiningDiacriticalMarksExtended::CombiningDoubleParenthesesAbove => COMBINING_DOUBLE_PARENTHESES_ABOVE,
            CombiningDiacriticalMarksExtended::CombiningParenthesesBelow => COMBINING_PARENTHESES_BELOW,
            CombiningDiacriticalMarksExtended::CombiningParenthesesOverlay => COMBINING_PARENTHESES_OVERLAY,
        }
    }
}

impl std::convert::TryFrom<char> for CombiningDiacriticalMarksExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COMBINING_DOUBLED_CIRCUMFLEX_ACCENT => Ok(CombiningDiacriticalMarksExtended::CombiningDoubledCircumflexAccent),
            COMBINING_DIAERESIS_DASH_RING => Ok(CombiningDiacriticalMarksExtended::CombiningDiaeresisDashRing),
            COMBINING_INFINITY => Ok(CombiningDiacriticalMarksExtended::CombiningInfinity),
            COMBINING_DOWNWARDS_ARROW => Ok(CombiningDiacriticalMarksExtended::CombiningDownwardsArrow),
            COMBINING_TRIPLE_DOT => Ok(CombiningDiacriticalMarksExtended::CombiningTripleDot),
            COMBINING_X_DASH_X_BELOW => Ok(CombiningDiacriticalMarksExtended::CombiningXDashXBelow),
            COMBINING_WIGGLY_LINE_BELOW => Ok(CombiningDiacriticalMarksExtended::CombiningWigglyLineBelow),
            COMBINING_OPEN_MARK_BELOW => Ok(CombiningDiacriticalMarksExtended::CombiningOpenMarkBelow),
            COMBINING_DOUBLE_OPEN_MARK_BELOW => Ok(CombiningDiacriticalMarksExtended::CombiningDoubleOpenMarkBelow),
            COMBINING_LIGHT_CENTRALIZATION_STROKE_BELOW => Ok(CombiningDiacriticalMarksExtended::CombiningLightCentralizationStrokeBelow),
            COMBINING_STRONG_CENTRALIZATION_STROKE_BELOW => Ok(CombiningDiacriticalMarksExtended::CombiningStrongCentralizationStrokeBelow),
            COMBINING_PARENTHESES_ABOVE => Ok(CombiningDiacriticalMarksExtended::CombiningParenthesesAbove),
            COMBINING_DOUBLE_PARENTHESES_ABOVE => Ok(CombiningDiacriticalMarksExtended::CombiningDoubleParenthesesAbove),
            COMBINING_PARENTHESES_BELOW => Ok(CombiningDiacriticalMarksExtended::CombiningParenthesesBelow),
            COMBINING_PARENTHESES_OVERLAY => Ok(CombiningDiacriticalMarksExtended::CombiningParenthesesOverlay),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CombiningDiacriticalMarksExtended {
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

impl std::convert::TryFrom<u32> for CombiningDiacriticalMarksExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CombiningDiacriticalMarksExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CombiningDiacriticalMarksExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CombiningDiacriticalMarksExtended::CombiningDoubledCircumflexAccent
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CombiningDiacriticalMarksExtended::CombiningDoubledCircumflexAccent => "combining doubled circumflex accent",
            CombiningDiacriticalMarksExtended::CombiningDiaeresisDashRing => "combining diaeresis-ring",
            CombiningDiacriticalMarksExtended::CombiningInfinity => "combining infinity",
            CombiningDiacriticalMarksExtended::CombiningDownwardsArrow => "combining downwards arrow",
            CombiningDiacriticalMarksExtended::CombiningTripleDot => "combining triple dot",
            CombiningDiacriticalMarksExtended::CombiningXDashXBelow => "combining x-x below",
            CombiningDiacriticalMarksExtended::CombiningWigglyLineBelow => "combining wiggly line below",
            CombiningDiacriticalMarksExtended::CombiningOpenMarkBelow => "combining open mark below",
            CombiningDiacriticalMarksExtended::CombiningDoubleOpenMarkBelow => "combining double open mark below",
            CombiningDiacriticalMarksExtended::CombiningLightCentralizationStrokeBelow => "combining light centralization stroke below",
            CombiningDiacriticalMarksExtended::CombiningStrongCentralizationStrokeBelow => "combining strong centralization stroke below",
            CombiningDiacriticalMarksExtended::CombiningParenthesesAbove => "combining parentheses above",
            CombiningDiacriticalMarksExtended::CombiningDoubleParenthesesAbove => "combining double parentheses above",
            CombiningDiacriticalMarksExtended::CombiningParenthesesBelow => "combining parentheses below",
            CombiningDiacriticalMarksExtended::CombiningParenthesesOverlay => "combining parentheses overlay",
        }
    }
}
