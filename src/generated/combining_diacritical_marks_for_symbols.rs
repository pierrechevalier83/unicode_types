/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{20d0}: '⃐'
    pub const COMBINING_LEFT_HARPOON_ABOVE: char = '⃐';
    /// \u{20d1}: '⃑'
    pub const COMBINING_RIGHT_HARPOON_ABOVE: char = '⃑';
    /// \u{20d2}: '⃒'
    pub const COMBINING_LONG_VERTICAL_LINE_OVERLAY: char = '⃒';
    /// \u{20d3}: '⃓'
    pub const COMBINING_SHORT_VERTICAL_LINE_OVERLAY: char = '⃓';
    /// \u{20d4}: '⃔'
    pub const COMBINING_ANTICLOCKWISE_ARROW_ABOVE: char = '⃔';
    /// \u{20d5}: '⃕'
    pub const COMBINING_CLOCKWISE_ARROW_ABOVE: char = '⃕';
    /// \u{20d6}: '⃖'
    pub const COMBINING_LEFT_ARROW_ABOVE: char = '⃖';
    /// \u{20d7}: '⃗'
    pub const COMBINING_RIGHT_ARROW_ABOVE: char = '⃗';
    /// \u{20d8}: '⃘'
    pub const COMBINING_RING_OVERLAY: char = '⃘';
    /// \u{20d9}: '⃙'
    pub const COMBINING_CLOCKWISE_RING_OVERLAY: char = '⃙';
    /// \u{20da}: '⃚'
    pub const COMBINING_ANTICLOCKWISE_RING_OVERLAY: char = '⃚';
    /// \u{20db}: '⃛'
    pub const COMBINING_THREE_DOTS_ABOVE: char = '⃛';
    /// \u{20dc}: '⃜'
    pub const COMBINING_FOUR_DOTS_ABOVE: char = '⃜';
    /// \u{20dd}: '⃝'
    pub const COMBINING_ENCLOSING_CIRCLE: char = '⃝';
    /// \u{20de}: '⃞'
    pub const COMBINING_ENCLOSING_SQUARE: char = '⃞';
    /// \u{20df}: '⃟'
    pub const COMBINING_ENCLOSING_DIAMOND: char = '⃟';
    /// \u{20e0}: '⃠'
    pub const COMBINING_ENCLOSING_CIRCLE_BACKSLASH: char = '⃠';
    /// \u{20e1}: '⃡'
    pub const COMBINING_LEFT_RIGHT_ARROW_ABOVE: char = '⃡';
    /// \u{20e2}: '⃢'
    pub const COMBINING_ENCLOSING_SCREEN: char = '⃢';
    /// \u{20e3}: '⃣'
    pub const COMBINING_ENCLOSING_KEYCAP: char = '⃣';
    /// \u{20e4}: '⃤'
    pub const COMBINING_ENCLOSING_UPWARD_POINTING_TRIANGLE: char = '⃤';
    /// \u{20e5}: '⃥'
    pub const COMBINING_REVERSE_SOLIDUS_OVERLAY: char = '⃥';
    /// \u{20e6}: '⃦'
    pub const COMBINING_DOUBLE_VERTICAL_STROKE_OVERLAY: char = '⃦';
    /// \u{20e7}: '⃧'
    pub const COMBINING_ANNUITY_SYMBOL: char = '⃧';
    /// \u{20e8}: '⃨'
    pub const COMBINING_TRIPLE_UNDERDOT: char = '⃨';
    /// \u{20e9}: '⃩'
    pub const COMBINING_WIDE_BRIDGE_ABOVE: char = '⃩';
    /// \u{20ea}: '⃪'
    pub const COMBINING_LEFTWARDS_ARROW_OVERLAY: char = '⃪';
    /// \u{20eb}: '⃫'
    pub const COMBINING_LONG_DOUBLE_SOLIDUS_OVERLAY: char = '⃫';
    /// \u{20ec}: '⃬'
    pub const COMBINING_RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS: char = '⃬';
    /// \u{20ed}: '⃭'
    pub const COMBINING_LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS: char = '⃭';
    /// \u{20ee}: '⃮'
    pub const COMBINING_LEFT_ARROW_BELOW: char = '⃮';
    /// \u{20ef}: '⃯'
    pub const COMBINING_RIGHT_ARROW_BELOW: char = '⃯';
    /// \u{20f0}: '⃰'
    pub const COMBINING_ASTERISK_ABOVE: char = '⃰';
}

/// An enum to represent all characters in the CombiningDiacriticalMarksforSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CombiningDiacriticalMarksforSymbols {
    /// \u{20d0}: '⃐'
    CombiningLeftHarpoonAbove,
    /// \u{20d1}: '⃑'
    CombiningRightHarpoonAbove,
    /// \u{20d2}: '⃒'
    CombiningLongVerticalLineOverlay,
    /// \u{20d3}: '⃓'
    CombiningShortVerticalLineOverlay,
    /// \u{20d4}: '⃔'
    CombiningAnticlockwiseArrowAbove,
    /// \u{20d5}: '⃕'
    CombiningClockwiseArrowAbove,
    /// \u{20d6}: '⃖'
    CombiningLeftArrowAbove,
    /// \u{20d7}: '⃗'
    CombiningRightArrowAbove,
    /// \u{20d8}: '⃘'
    CombiningRingOverlay,
    /// \u{20d9}: '⃙'
    CombiningClockwiseRingOverlay,
    /// \u{20da}: '⃚'
    CombiningAnticlockwiseRingOverlay,
    /// \u{20db}: '⃛'
    CombiningThreeDotsAbove,
    /// \u{20dc}: '⃜'
    CombiningFourDotsAbove,
    /// \u{20dd}: '⃝'
    CombiningEnclosingCircle,
    /// \u{20de}: '⃞'
    CombiningEnclosingSquare,
    /// \u{20df}: '⃟'
    CombiningEnclosingDiamond,
    /// \u{20e0}: '⃠'
    CombiningEnclosingCircleBackslash,
    /// \u{20e1}: '⃡'
    CombiningLeftRightArrowAbove,
    /// \u{20e2}: '⃢'
    CombiningEnclosingScreen,
    /// \u{20e3}: '⃣'
    CombiningEnclosingKeycap,
    /// \u{20e4}: '⃤'
    CombiningEnclosingUpwardPointingTriangle,
    /// \u{20e5}: '⃥'
    CombiningReverseSolidusOverlay,
    /// \u{20e6}: '⃦'
    CombiningDoubleVerticalStrokeOverlay,
    /// \u{20e7}: '⃧'
    CombiningAnnuitySymbol,
    /// \u{20e8}: '⃨'
    CombiningTripleUnderdot,
    /// \u{20e9}: '⃩'
    CombiningWideBridgeAbove,
    /// \u{20ea}: '⃪'
    CombiningLeftwardsArrowOverlay,
    /// \u{20eb}: '⃫'
    CombiningLongDoubleSolidusOverlay,
    /// \u{20ec}: '⃬'
    CombiningRightwardsHarpoonWithBarbDownwards,
    /// \u{20ed}: '⃭'
    CombiningLeftwardsHarpoonWithBarbDownwards,
    /// \u{20ee}: '⃮'
    CombiningLeftArrowBelow,
    /// \u{20ef}: '⃯'
    CombiningRightArrowBelow,
    /// \u{20f0}: '⃰'
    CombiningAsteriskAbove,
}

impl Into<char> for CombiningDiacriticalMarksforSymbols {
    fn into(self) -> char {
        use constants::*;
        match self {
            CombiningDiacriticalMarksforSymbols::CombiningLeftHarpoonAbove => COMBINING_LEFT_HARPOON_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningRightHarpoonAbove => COMBINING_RIGHT_HARPOON_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningLongVerticalLineOverlay => COMBINING_LONG_VERTICAL_LINE_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningShortVerticalLineOverlay => COMBINING_SHORT_VERTICAL_LINE_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseArrowAbove => COMBINING_ANTICLOCKWISE_ARROW_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningClockwiseArrowAbove => COMBINING_CLOCKWISE_ARROW_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningLeftArrowAbove => COMBINING_LEFT_ARROW_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningRightArrowAbove => COMBINING_RIGHT_ARROW_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningRingOverlay => COMBINING_RING_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningClockwiseRingOverlay => COMBINING_CLOCKWISE_RING_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseRingOverlay => COMBINING_ANTICLOCKWISE_RING_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningThreeDotsAbove => COMBINING_THREE_DOTS_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningFourDotsAbove => COMBINING_FOUR_DOTS_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircle => COMBINING_ENCLOSING_CIRCLE,
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingSquare => COMBINING_ENCLOSING_SQUARE,
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingDiamond => COMBINING_ENCLOSING_DIAMOND,
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircleBackslash => COMBINING_ENCLOSING_CIRCLE_BACKSLASH,
            CombiningDiacriticalMarksforSymbols::CombiningLeftRightArrowAbove => COMBINING_LEFT_RIGHT_ARROW_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingScreen => COMBINING_ENCLOSING_SCREEN,
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingKeycap => COMBINING_ENCLOSING_KEYCAP,
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingUpwardPointingTriangle => COMBINING_ENCLOSING_UPWARD_POINTING_TRIANGLE,
            CombiningDiacriticalMarksforSymbols::CombiningReverseSolidusOverlay => COMBINING_REVERSE_SOLIDUS_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningDoubleVerticalStrokeOverlay => COMBINING_DOUBLE_VERTICAL_STROKE_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningAnnuitySymbol => COMBINING_ANNUITY_SYMBOL,
            CombiningDiacriticalMarksforSymbols::CombiningTripleUnderdot => COMBINING_TRIPLE_UNDERDOT,
            CombiningDiacriticalMarksforSymbols::CombiningWideBridgeAbove => COMBINING_WIDE_BRIDGE_ABOVE,
            CombiningDiacriticalMarksforSymbols::CombiningLeftwardsArrowOverlay => COMBINING_LEFTWARDS_ARROW_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningLongDoubleSolidusOverlay => COMBINING_LONG_DOUBLE_SOLIDUS_OVERLAY,
            CombiningDiacriticalMarksforSymbols::CombiningRightwardsHarpoonWithBarbDownwards => COMBINING_RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS,
            CombiningDiacriticalMarksforSymbols::CombiningLeftwardsHarpoonWithBarbDownwards => COMBINING_LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS,
            CombiningDiacriticalMarksforSymbols::CombiningLeftArrowBelow => COMBINING_LEFT_ARROW_BELOW,
            CombiningDiacriticalMarksforSymbols::CombiningRightArrowBelow => COMBINING_RIGHT_ARROW_BELOW,
            CombiningDiacriticalMarksforSymbols::CombiningAsteriskAbove => COMBINING_ASTERISK_ABOVE,
        }
    }
}

impl std::convert::TryFrom<char> for CombiningDiacriticalMarksforSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COMBINING_LEFT_HARPOON_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftHarpoonAbove),
            COMBINING_RIGHT_HARPOON_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightHarpoonAbove),
            COMBINING_LONG_VERTICAL_LINE_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningLongVerticalLineOverlay),
            COMBINING_SHORT_VERTICAL_LINE_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningShortVerticalLineOverlay),
            COMBINING_ANTICLOCKWISE_ARROW_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseArrowAbove),
            COMBINING_CLOCKWISE_ARROW_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningClockwiseArrowAbove),
            COMBINING_LEFT_ARROW_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftArrowAbove),
            COMBINING_RIGHT_ARROW_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightArrowAbove),
            COMBINING_RING_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningRingOverlay),
            COMBINING_CLOCKWISE_RING_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningClockwiseRingOverlay),
            COMBINING_ANTICLOCKWISE_RING_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseRingOverlay),
            COMBINING_THREE_DOTS_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningThreeDotsAbove),
            COMBINING_FOUR_DOTS_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningFourDotsAbove),
            COMBINING_ENCLOSING_CIRCLE => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircle),
            COMBINING_ENCLOSING_SQUARE => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingSquare),
            COMBINING_ENCLOSING_DIAMOND => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingDiamond),
            COMBINING_ENCLOSING_CIRCLE_BACKSLASH => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircleBackslash),
            COMBINING_LEFT_RIGHT_ARROW_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftRightArrowAbove),
            COMBINING_ENCLOSING_SCREEN => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingScreen),
            COMBINING_ENCLOSING_KEYCAP => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingKeycap),
            COMBINING_ENCLOSING_UPWARD_POINTING_TRIANGLE => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingUpwardPointingTriangle),
            COMBINING_REVERSE_SOLIDUS_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningReverseSolidusOverlay),
            COMBINING_DOUBLE_VERTICAL_STROKE_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningDoubleVerticalStrokeOverlay),
            COMBINING_ANNUITY_SYMBOL => Ok(CombiningDiacriticalMarksforSymbols::CombiningAnnuitySymbol),
            COMBINING_TRIPLE_UNDERDOT => Ok(CombiningDiacriticalMarksforSymbols::CombiningTripleUnderdot),
            COMBINING_WIDE_BRIDGE_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningWideBridgeAbove),
            COMBINING_LEFTWARDS_ARROW_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftwardsArrowOverlay),
            COMBINING_LONG_DOUBLE_SOLIDUS_OVERLAY => Ok(CombiningDiacriticalMarksforSymbols::CombiningLongDoubleSolidusOverlay),
            COMBINING_RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightwardsHarpoonWithBarbDownwards),
            COMBINING_LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftwardsHarpoonWithBarbDownwards),
            COMBINING_LEFT_ARROW_BELOW => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftArrowBelow),
            COMBINING_RIGHT_ARROW_BELOW => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightArrowBelow),
            COMBINING_ASTERISK_ABOVE => Ok(CombiningDiacriticalMarksforSymbols::CombiningAsteriskAbove),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CombiningDiacriticalMarksforSymbols {
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

impl std::convert::TryFrom<u32> for CombiningDiacriticalMarksforSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CombiningDiacriticalMarksforSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CombiningDiacriticalMarksforSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CombiningDiacriticalMarksforSymbols::CombiningLeftHarpoonAbove
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CombiningDiacriticalMarksforSymbols::CombiningLeftHarpoonAbove => "combining left harpoon above",
            CombiningDiacriticalMarksforSymbols::CombiningRightHarpoonAbove => "combining right harpoon above",
            CombiningDiacriticalMarksforSymbols::CombiningLongVerticalLineOverlay => "combining long vertical line overlay",
            CombiningDiacriticalMarksforSymbols::CombiningShortVerticalLineOverlay => "combining short vertical line overlay",
            CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseArrowAbove => "combining anticlockwise arrow above",
            CombiningDiacriticalMarksforSymbols::CombiningClockwiseArrowAbove => "combining clockwise arrow above",
            CombiningDiacriticalMarksforSymbols::CombiningLeftArrowAbove => "combining left arrow above",
            CombiningDiacriticalMarksforSymbols::CombiningRightArrowAbove => "combining right arrow above",
            CombiningDiacriticalMarksforSymbols::CombiningRingOverlay => "combining ring overlay",
            CombiningDiacriticalMarksforSymbols::CombiningClockwiseRingOverlay => "combining clockwise ring overlay",
            CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseRingOverlay => "combining anticlockwise ring overlay",
            CombiningDiacriticalMarksforSymbols::CombiningThreeDotsAbove => "combining three dots above",
            CombiningDiacriticalMarksforSymbols::CombiningFourDotsAbove => "combining four dots above",
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircle => "combining enclosing circle",
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingSquare => "combining enclosing square",
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingDiamond => "combining enclosing diamond",
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircleBackslash => "combining enclosing circle backslash",
            CombiningDiacriticalMarksforSymbols::CombiningLeftRightArrowAbove => "combining left right arrow above",
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingScreen => "combining enclosing screen",
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingKeycap => "combining enclosing keycap",
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingUpwardPointingTriangle => "combining enclosing upward pointing triangle",
            CombiningDiacriticalMarksforSymbols::CombiningReverseSolidusOverlay => "combining reverse solidus overlay",
            CombiningDiacriticalMarksforSymbols::CombiningDoubleVerticalStrokeOverlay => "combining double vertical stroke overlay",
            CombiningDiacriticalMarksforSymbols::CombiningAnnuitySymbol => "combining annuity symbol",
            CombiningDiacriticalMarksforSymbols::CombiningTripleUnderdot => "combining triple underdot",
            CombiningDiacriticalMarksforSymbols::CombiningWideBridgeAbove => "combining wide bridge above",
            CombiningDiacriticalMarksforSymbols::CombiningLeftwardsArrowOverlay => "combining leftwards arrow overlay",
            CombiningDiacriticalMarksforSymbols::CombiningLongDoubleSolidusOverlay => "combining long double solidus overlay",
            CombiningDiacriticalMarksforSymbols::CombiningRightwardsHarpoonWithBarbDownwards => "combining rightwards harpoon with barb downwards",
            CombiningDiacriticalMarksforSymbols::CombiningLeftwardsHarpoonWithBarbDownwards => "combining leftwards harpoon with barb downwards",
            CombiningDiacriticalMarksforSymbols::CombiningLeftArrowBelow => "combining left arrow below",
            CombiningDiacriticalMarksforSymbols::CombiningRightArrowBelow => "combining right arrow below",
            CombiningDiacriticalMarksforSymbols::CombiningAsteriskAbove => "combining asterisk above",
        }
    }
}
