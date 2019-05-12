
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
        match self {
            CombiningDiacriticalMarksExtended::CombiningDoubledCircumflexAccent => '᪰',
            CombiningDiacriticalMarksExtended::CombiningDiaeresisDashRing => '᪱',
            CombiningDiacriticalMarksExtended::CombiningInfinity => '᪲',
            CombiningDiacriticalMarksExtended::CombiningDownwardsArrow => '᪳',
            CombiningDiacriticalMarksExtended::CombiningTripleDot => '᪴',
            CombiningDiacriticalMarksExtended::CombiningXDashXBelow => '᪵',
            CombiningDiacriticalMarksExtended::CombiningWigglyLineBelow => '᪶',
            CombiningDiacriticalMarksExtended::CombiningOpenMarkBelow => '᪷',
            CombiningDiacriticalMarksExtended::CombiningDoubleOpenMarkBelow => '᪸',
            CombiningDiacriticalMarksExtended::CombiningLightCentralizationStrokeBelow => '᪹',
            CombiningDiacriticalMarksExtended::CombiningStrongCentralizationStrokeBelow => '᪺',
            CombiningDiacriticalMarksExtended::CombiningParenthesesAbove => '᪻',
            CombiningDiacriticalMarksExtended::CombiningDoubleParenthesesAbove => '᪼',
            CombiningDiacriticalMarksExtended::CombiningParenthesesBelow => '᪽',
            CombiningDiacriticalMarksExtended::CombiningParenthesesOverlay => '᪾',
        }
    }
}

impl std::convert::TryFrom<char> for CombiningDiacriticalMarksExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᪰' => Ok(CombiningDiacriticalMarksExtended::CombiningDoubledCircumflexAccent),
            '᪱' => Ok(CombiningDiacriticalMarksExtended::CombiningDiaeresisDashRing),
            '᪲' => Ok(CombiningDiacriticalMarksExtended::CombiningInfinity),
            '᪳' => Ok(CombiningDiacriticalMarksExtended::CombiningDownwardsArrow),
            '᪴' => Ok(CombiningDiacriticalMarksExtended::CombiningTripleDot),
            '᪵' => Ok(CombiningDiacriticalMarksExtended::CombiningXDashXBelow),
            '᪶' => Ok(CombiningDiacriticalMarksExtended::CombiningWigglyLineBelow),
            '᪷' => Ok(CombiningDiacriticalMarksExtended::CombiningOpenMarkBelow),
            '᪸' => Ok(CombiningDiacriticalMarksExtended::CombiningDoubleOpenMarkBelow),
            '᪹' => Ok(CombiningDiacriticalMarksExtended::CombiningLightCentralizationStrokeBelow),
            '᪺' => Ok(CombiningDiacriticalMarksExtended::CombiningStrongCentralizationStrokeBelow),
            '᪻' => Ok(CombiningDiacriticalMarksExtended::CombiningParenthesesAbove),
            '᪼' => Ok(CombiningDiacriticalMarksExtended::CombiningDoubleParenthesesAbove),
            '᪽' => Ok(CombiningDiacriticalMarksExtended::CombiningParenthesesBelow),
            '᪾' => Ok(CombiningDiacriticalMarksExtended::CombiningParenthesesOverlay),
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
