
/// An enum to represent all characters in the SupplementalArrowsA block.
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
        match self {
            SupplementalArrowsA::UpwardsQuadrupleArrow => '⟰',
            SupplementalArrowsA::DownwardsQuadrupleArrow => '⟱',
            SupplementalArrowsA::AnticlockwiseGappedCircleArrow => '⟲',
            SupplementalArrowsA::ClockwiseGappedCircleArrow => '⟳',
            SupplementalArrowsA::RightArrowWithCircledPlus => '⟴',
            SupplementalArrowsA::LongLeftwardsArrow => '⟵',
            SupplementalArrowsA::LongRightwardsArrow => '⟶',
            SupplementalArrowsA::LongLeftRightArrow => '⟷',
            SupplementalArrowsA::LongLeftwardsDoubleArrow => '⟸',
            SupplementalArrowsA::LongRightwardsDoubleArrow => '⟹',
            SupplementalArrowsA::LongLeftRightDoubleArrow => '⟺',
            SupplementalArrowsA::LongLeftwardsArrowFromBar => '⟻',
            SupplementalArrowsA::LongRightwardsArrowFromBar => '⟼',
            SupplementalArrowsA::LongLeftwardsDoubleArrowFromBar => '⟽',
            SupplementalArrowsA::LongRightwardsDoubleArrowFromBar => '⟾',
        }
    }
}

impl std::convert::TryFrom<char> for SupplementalArrowsA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⟰' => Ok(SupplementalArrowsA::UpwardsQuadrupleArrow),
            '⟱' => Ok(SupplementalArrowsA::DownwardsQuadrupleArrow),
            '⟲' => Ok(SupplementalArrowsA::AnticlockwiseGappedCircleArrow),
            '⟳' => Ok(SupplementalArrowsA::ClockwiseGappedCircleArrow),
            '⟴' => Ok(SupplementalArrowsA::RightArrowWithCircledPlus),
            '⟵' => Ok(SupplementalArrowsA::LongLeftwardsArrow),
            '⟶' => Ok(SupplementalArrowsA::LongRightwardsArrow),
            '⟷' => Ok(SupplementalArrowsA::LongLeftRightArrow),
            '⟸' => Ok(SupplementalArrowsA::LongLeftwardsDoubleArrow),
            '⟹' => Ok(SupplementalArrowsA::LongRightwardsDoubleArrow),
            '⟺' => Ok(SupplementalArrowsA::LongLeftRightDoubleArrow),
            '⟻' => Ok(SupplementalArrowsA::LongLeftwardsArrowFromBar),
            '⟼' => Ok(SupplementalArrowsA::LongRightwardsArrowFromBar),
            '⟽' => Ok(SupplementalArrowsA::LongLeftwardsDoubleArrowFromBar),
            '⟾' => Ok(SupplementalArrowsA::LongRightwardsDoubleArrowFromBar),
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
