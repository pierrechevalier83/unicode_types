
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
        match self {
            CombiningDiacriticalMarksforSymbols::CombiningLeftHarpoonAbove => '⃐',
            CombiningDiacriticalMarksforSymbols::CombiningRightHarpoonAbove => '⃑',
            CombiningDiacriticalMarksforSymbols::CombiningLongVerticalLineOverlay => '⃒',
            CombiningDiacriticalMarksforSymbols::CombiningShortVerticalLineOverlay => '⃓',
            CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseArrowAbove => '⃔',
            CombiningDiacriticalMarksforSymbols::CombiningClockwiseArrowAbove => '⃕',
            CombiningDiacriticalMarksforSymbols::CombiningLeftArrowAbove => '⃖',
            CombiningDiacriticalMarksforSymbols::CombiningRightArrowAbove => '⃗',
            CombiningDiacriticalMarksforSymbols::CombiningRingOverlay => '⃘',
            CombiningDiacriticalMarksforSymbols::CombiningClockwiseRingOverlay => '⃙',
            CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseRingOverlay => '⃚',
            CombiningDiacriticalMarksforSymbols::CombiningThreeDotsAbove => '⃛',
            CombiningDiacriticalMarksforSymbols::CombiningFourDotsAbove => '⃜',
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircle => '⃝',
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingSquare => '⃞',
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingDiamond => '⃟',
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircleBackslash => '⃠',
            CombiningDiacriticalMarksforSymbols::CombiningLeftRightArrowAbove => '⃡',
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingScreen => '⃢',
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingKeycap => '⃣',
            CombiningDiacriticalMarksforSymbols::CombiningEnclosingUpwardPointingTriangle => '⃤',
            CombiningDiacriticalMarksforSymbols::CombiningReverseSolidusOverlay => '⃥',
            CombiningDiacriticalMarksforSymbols::CombiningDoubleVerticalStrokeOverlay => '⃦',
            CombiningDiacriticalMarksforSymbols::CombiningAnnuitySymbol => '⃧',
            CombiningDiacriticalMarksforSymbols::CombiningTripleUnderdot => '⃨',
            CombiningDiacriticalMarksforSymbols::CombiningWideBridgeAbove => '⃩',
            CombiningDiacriticalMarksforSymbols::CombiningLeftwardsArrowOverlay => '⃪',
            CombiningDiacriticalMarksforSymbols::CombiningLongDoubleSolidusOverlay => '⃫',
            CombiningDiacriticalMarksforSymbols::CombiningRightwardsHarpoonWithBarbDownwards => '⃬',
            CombiningDiacriticalMarksforSymbols::CombiningLeftwardsHarpoonWithBarbDownwards => '⃭',
            CombiningDiacriticalMarksforSymbols::CombiningLeftArrowBelow => '⃮',
            CombiningDiacriticalMarksforSymbols::CombiningRightArrowBelow => '⃯',
            CombiningDiacriticalMarksforSymbols::CombiningAsteriskAbove => '⃰',
        }
    }
}

impl std::convert::TryFrom<char> for CombiningDiacriticalMarksforSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⃐' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftHarpoonAbove),
            '⃑' => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightHarpoonAbove),
            '⃒' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLongVerticalLineOverlay),
            '⃓' => Ok(CombiningDiacriticalMarksforSymbols::CombiningShortVerticalLineOverlay),
            '⃔' => Ok(CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseArrowAbove),
            '⃕' => Ok(CombiningDiacriticalMarksforSymbols::CombiningClockwiseArrowAbove),
            '⃖' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftArrowAbove),
            '⃗' => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightArrowAbove),
            '⃘' => Ok(CombiningDiacriticalMarksforSymbols::CombiningRingOverlay),
            '⃙' => Ok(CombiningDiacriticalMarksforSymbols::CombiningClockwiseRingOverlay),
            '⃚' => Ok(CombiningDiacriticalMarksforSymbols::CombiningAnticlockwiseRingOverlay),
            '⃛' => Ok(CombiningDiacriticalMarksforSymbols::CombiningThreeDotsAbove),
            '⃜' => Ok(CombiningDiacriticalMarksforSymbols::CombiningFourDotsAbove),
            '⃝' => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircle),
            '⃞' => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingSquare),
            '⃟' => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingDiamond),
            '⃠' => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingCircleBackslash),
            '⃡' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftRightArrowAbove),
            '⃢' => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingScreen),
            '⃣' => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingKeycap),
            '⃤' => Ok(CombiningDiacriticalMarksforSymbols::CombiningEnclosingUpwardPointingTriangle),
            '⃥' => Ok(CombiningDiacriticalMarksforSymbols::CombiningReverseSolidusOverlay),
            '⃦' => Ok(CombiningDiacriticalMarksforSymbols::CombiningDoubleVerticalStrokeOverlay),
            '⃧' => Ok(CombiningDiacriticalMarksforSymbols::CombiningAnnuitySymbol),
            '⃨' => Ok(CombiningDiacriticalMarksforSymbols::CombiningTripleUnderdot),
            '⃩' => Ok(CombiningDiacriticalMarksforSymbols::CombiningWideBridgeAbove),
            '⃪' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftwardsArrowOverlay),
            '⃫' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLongDoubleSolidusOverlay),
            '⃬' => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightwardsHarpoonWithBarbDownwards),
            '⃭' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftwardsHarpoonWithBarbDownwards),
            '⃮' => Ok(CombiningDiacriticalMarksforSymbols::CombiningLeftArrowBelow),
            '⃯' => Ok(CombiningDiacriticalMarksforSymbols::CombiningRightArrowBelow),
            '⃰' => Ok(CombiningDiacriticalMarksforSymbols::CombiningAsteriskAbove),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CombiningDiacriticalMarksforSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
