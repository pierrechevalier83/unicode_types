/// \u{27f0} → \u{27ff}\
///\
/// ⟰ ⟱ ⟲ ⟳ ⟴ ⟵ ⟶ ⟷ ⟸ ⟹ ⟺ ⟻ ⟼ ⟽ ⟾
pub mod constants {
    /// \u{27f0}: '⟰'
    pub const UPWARDS_QUADRUPLE_ARROW: char = '⟰';
    /// \u{27f1}: '⟱'
    pub const DOWNWARDS_QUADRUPLE_ARROW: char = '⟱';
    /// \u{27f2}: '⟲'
    pub const ANTICLOCKWISE_GAPPED_CIRCLE_ARROW: char = '⟲';
    /// \u{27f3}: '⟳'
    pub const CLOCKWISE_GAPPED_CIRCLE_ARROW: char = '⟳';
    /// \u{27f4}: '⟴'
    pub const RIGHT_ARROW_WITH_CIRCLED_PLUS: char = '⟴';
    /// \u{27f5}: '⟵'
    pub const LONG_LEFTWARDS_ARROW: char = '⟵';
    /// \u{27f6}: '⟶'
    pub const LONG_RIGHTWARDS_ARROW: char = '⟶';
    /// \u{27f7}: '⟷'
    pub const LONG_LEFT_RIGHT_ARROW: char = '⟷';
    /// \u{27f8}: '⟸'
    pub const LONG_LEFTWARDS_DOUBLE_ARROW: char = '⟸';
    /// \u{27f9}: '⟹'
    pub const LONG_RIGHTWARDS_DOUBLE_ARROW: char = '⟹';
    /// \u{27fa}: '⟺'
    pub const LONG_LEFT_RIGHT_DOUBLE_ARROW: char = '⟺';
    /// \u{27fb}: '⟻'
    pub const LONG_LEFTWARDS_ARROW_FROM_BAR: char = '⟻';
    /// \u{27fc}: '⟼'
    pub const LONG_RIGHTWARDS_ARROW_FROM_BAR: char = '⟼';
    /// \u{27fd}: '⟽'
    pub const LONG_LEFTWARDS_DOUBLE_ARROW_FROM_BAR: char = '⟽';
    /// \u{27fe}: '⟾'
    pub const LONG_RIGHTWARDS_DOUBLE_ARROW_FROM_BAR: char = '⟾';
}

/// \u{27f0} → \u{27ff}\
///\
/// ⟰ ⟱ ⟲ ⟳ ⟴ ⟵ ⟶ ⟷ ⟸ ⟹ ⟺ ⟻ ⟼ ⟽ ⟾
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SupplementalArrowsA {
    /// \u{27f0}: '⟰'
    UpwardsQuadrupleArrow,
    /// \u{27f1}: '⟱'
    DownwardsQuadrupleArrow,
    /// \u{27f2}: '⟲'
    AnticlockwiseGappedCircleArrow,
    /// \u{27f3}: '⟳'
    ClockwiseGappedCircleArrow,
    /// \u{27f4}: '⟴'
    RightArrowWithCircledPlus,
    /// \u{27f5}: '⟵'
    LongLeftwardsArrow,
    /// \u{27f6}: '⟶'
    LongRightwardsArrow,
    /// \u{27f7}: '⟷'
    LongLeftRightArrow,
    /// \u{27f8}: '⟸'
    LongLeftwardsDoubleArrow,
    /// \u{27f9}: '⟹'
    LongRightwardsDoubleArrow,
    /// \u{27fa}: '⟺'
    LongLeftRightDoubleArrow,
    /// \u{27fb}: '⟻'
    LongLeftwardsArrowFromBar,
    /// \u{27fc}: '⟼'
    LongRightwardsArrowFromBar,
    /// \u{27fd}: '⟽'
    LongLeftwardsDoubleArrowFromBar,
    /// \u{27fe}: '⟾'
    LongRightwardsDoubleArrowFromBar,
}

impl Into<char> for SupplementalArrowsA {
    fn into(self) -> char {
        use constants::*;
        match self {
            SupplementalArrowsA::UpwardsQuadrupleArrow => UPWARDS_QUADRUPLE_ARROW,
            SupplementalArrowsA::DownwardsQuadrupleArrow => DOWNWARDS_QUADRUPLE_ARROW,
            SupplementalArrowsA::AnticlockwiseGappedCircleArrow => ANTICLOCKWISE_GAPPED_CIRCLE_ARROW,
            SupplementalArrowsA::ClockwiseGappedCircleArrow => CLOCKWISE_GAPPED_CIRCLE_ARROW,
            SupplementalArrowsA::RightArrowWithCircledPlus => RIGHT_ARROW_WITH_CIRCLED_PLUS,
            SupplementalArrowsA::LongLeftwardsArrow => LONG_LEFTWARDS_ARROW,
            SupplementalArrowsA::LongRightwardsArrow => LONG_RIGHTWARDS_ARROW,
            SupplementalArrowsA::LongLeftRightArrow => LONG_LEFT_RIGHT_ARROW,
            SupplementalArrowsA::LongLeftwardsDoubleArrow => LONG_LEFTWARDS_DOUBLE_ARROW,
            SupplementalArrowsA::LongRightwardsDoubleArrow => LONG_RIGHTWARDS_DOUBLE_ARROW,
            SupplementalArrowsA::LongLeftRightDoubleArrow => LONG_LEFT_RIGHT_DOUBLE_ARROW,
            SupplementalArrowsA::LongLeftwardsArrowFromBar => LONG_LEFTWARDS_ARROW_FROM_BAR,
            SupplementalArrowsA::LongRightwardsArrowFromBar => LONG_RIGHTWARDS_ARROW_FROM_BAR,
            SupplementalArrowsA::LongLeftwardsDoubleArrowFromBar => LONG_LEFTWARDS_DOUBLE_ARROW_FROM_BAR,
            SupplementalArrowsA::LongRightwardsDoubleArrowFromBar => LONG_RIGHTWARDS_DOUBLE_ARROW_FROM_BAR,
        }
    }
}

impl std::convert::TryFrom<char> for SupplementalArrowsA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            UPWARDS_QUADRUPLE_ARROW => Ok(SupplementalArrowsA::UpwardsQuadrupleArrow),
            DOWNWARDS_QUADRUPLE_ARROW => Ok(SupplementalArrowsA::DownwardsQuadrupleArrow),
            ANTICLOCKWISE_GAPPED_CIRCLE_ARROW => Ok(SupplementalArrowsA::AnticlockwiseGappedCircleArrow),
            CLOCKWISE_GAPPED_CIRCLE_ARROW => Ok(SupplementalArrowsA::ClockwiseGappedCircleArrow),
            RIGHT_ARROW_WITH_CIRCLED_PLUS => Ok(SupplementalArrowsA::RightArrowWithCircledPlus),
            LONG_LEFTWARDS_ARROW => Ok(SupplementalArrowsA::LongLeftwardsArrow),
            LONG_RIGHTWARDS_ARROW => Ok(SupplementalArrowsA::LongRightwardsArrow),
            LONG_LEFT_RIGHT_ARROW => Ok(SupplementalArrowsA::LongLeftRightArrow),
            LONG_LEFTWARDS_DOUBLE_ARROW => Ok(SupplementalArrowsA::LongLeftwardsDoubleArrow),
            LONG_RIGHTWARDS_DOUBLE_ARROW => Ok(SupplementalArrowsA::LongRightwardsDoubleArrow),
            LONG_LEFT_RIGHT_DOUBLE_ARROW => Ok(SupplementalArrowsA::LongLeftRightDoubleArrow),
            LONG_LEFTWARDS_ARROW_FROM_BAR => Ok(SupplementalArrowsA::LongLeftwardsArrowFromBar),
            LONG_RIGHTWARDS_ARROW_FROM_BAR => Ok(SupplementalArrowsA::LongRightwardsArrowFromBar),
            LONG_LEFTWARDS_DOUBLE_ARROW_FROM_BAR => Ok(SupplementalArrowsA::LongLeftwardsDoubleArrowFromBar),
            LONG_RIGHTWARDS_DOUBLE_ARROW_FROM_BAR => Ok(SupplementalArrowsA::LongRightwardsDoubleArrowFromBar),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SupplementalArrowsA {
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

impl std::convert::TryFrom<u32> for SupplementalArrowsA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SupplementalArrowsA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SupplementalArrowsA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SupplementalArrowsA::UpwardsQuadrupleArrow
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SupplementalArrowsA::UpwardsQuadrupleArrow => "upwards quadruple arrow",
            SupplementalArrowsA::DownwardsQuadrupleArrow => "downwards quadruple arrow",
            SupplementalArrowsA::AnticlockwiseGappedCircleArrow => "anticlockwise gapped circle arrow",
            SupplementalArrowsA::ClockwiseGappedCircleArrow => "clockwise gapped circle arrow",
            SupplementalArrowsA::RightArrowWithCircledPlus => "right arrow with circled plus",
            SupplementalArrowsA::LongLeftwardsArrow => "long leftwards arrow",
            SupplementalArrowsA::LongRightwardsArrow => "long rightwards arrow",
            SupplementalArrowsA::LongLeftRightArrow => "long left right arrow",
            SupplementalArrowsA::LongLeftwardsDoubleArrow => "long leftwards double arrow",
            SupplementalArrowsA::LongRightwardsDoubleArrow => "long rightwards double arrow",
            SupplementalArrowsA::LongLeftRightDoubleArrow => "long left right double arrow",
            SupplementalArrowsA::LongLeftwardsArrowFromBar => "long leftwards arrow from bar",
            SupplementalArrowsA::LongRightwardsArrowFromBar => "long rightwards arrow from bar",
            SupplementalArrowsA::LongLeftwardsDoubleArrowFromBar => "long leftwards double arrow from bar",
            SupplementalArrowsA::LongRightwardsDoubleArrowFromBar => "long rightwards double arrow from bar",
        }
    }
}
