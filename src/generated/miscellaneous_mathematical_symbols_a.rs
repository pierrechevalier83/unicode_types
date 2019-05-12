
/// An enum to represent all characters in the MiscellaneousMathematicalSymbolsA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MiscellaneousMathematicalSymbolsA {
    /// \u{27c0}: '⟀'
    ThreeDimensionalAngle,
    /// \u{27c1}: '⟁'
    WhiteTriangleContainingSmallWhiteTriangle,
    /// \u{27c2}: '⟂'
    Perpendicular,
    /// \u{27c3}: '⟃'
    OpenSubset,
    /// \u{27c4}: '⟄'
    OpenSuperset,
    /// \u{27c5}: '⟅'
    LeftSDashShapedBagDelimiter,
    /// \u{27c6}: '⟆'
    RightSDashShapedBagDelimiter,
    /// \u{27c7}: '⟇'
    OrWithDotInside,
    /// \u{27c8}: '⟈'
    ReverseSolidusPrecedingSubset,
    /// \u{27c9}: '⟉'
    SupersetPrecedingSolidus,
    /// \u{27ca}: '⟊'
    VerticalBarWithHorizontalStroke,
    /// \u{27cb}: '⟋'
    MathematicalRisingDiagonal,
    /// \u{27cc}: '⟌'
    LongDivision,
    /// \u{27cd}: '⟍'
    MathematicalFallingDiagonal,
    /// \u{27ce}: '⟎'
    SquaredLogicalAnd,
    /// \u{27cf}: '⟏'
    SquaredLogicalOr,
    /// \u{27d0}: '⟐'
    WhiteDiamondWithCentredDot,
    /// \u{27d1}: '⟑'
    AndWithDot,
    /// \u{27d2}: '⟒'
    ElementOfOpeningUpwards,
    /// \u{27d3}: '⟓'
    LowerRightCornerWithDot,
    /// \u{27d4}: '⟔'
    UpperLeftCornerWithDot,
    /// \u{27d5}: '⟕'
    LeftOuterJoin,
    /// \u{27d6}: '⟖'
    RightOuterJoin,
    /// \u{27d7}: '⟗'
    FullOuterJoin,
    /// \u{27d8}: '⟘'
    LargeUpTack,
    /// \u{27d9}: '⟙'
    LargeDownTack,
    /// \u{27da}: '⟚'
    LeftAndRightDoubleTurnstile,
    /// \u{27db}: '⟛'
    LeftAndRightTack,
    /// \u{27dc}: '⟜'
    LeftMultimap,
    /// \u{27dd}: '⟝'
    LongRightTack,
    /// \u{27de}: '⟞'
    LongLeftTack,
    /// \u{27df}: '⟟'
    UpTackWithCircleAbove,
    /// \u{27e0}: '⟠'
    LozengeDividedByHorizontalRule,
    /// \u{27e1}: '⟡'
    WhiteConcaveDashSidedDiamond,
    /// \u{27e2}: '⟢'
    WhiteConcaveDashSidedDiamondWithLeftwardsTick,
    /// \u{27e3}: '⟣'
    WhiteConcaveDashSidedDiamondWithRightwardsTick,
    /// \u{27e4}: '⟤'
    WhiteSquareWithLeftwardsTick,
    /// \u{27e5}: '⟥'
    WhiteSquareWithRightwardsTick,
    /// \u{27e6}: '⟦'
    MathematicalLeftWhiteSquareBracket,
    /// \u{27e7}: '⟧'
    MathematicalRightWhiteSquareBracket,
    /// \u{27e8}: '⟨'
    MathematicalLeftAngleBracket,
    /// \u{27e9}: '⟩'
    MathematicalRightAngleBracket,
    /// \u{27ea}: '⟪'
    MathematicalLeftDoubleAngleBracket,
    /// \u{27eb}: '⟫'
    MathematicalRightDoubleAngleBracket,
    /// \u{27ec}: '⟬'
    MathematicalLeftWhiteTortoiseShellBracket,
    /// \u{27ed}: '⟭'
    MathematicalRightWhiteTortoiseShellBracket,
    /// \u{27ee}: '⟮'
    MathematicalLeftFlattenedParenthesis,
}

impl Into<char> for MiscellaneousMathematicalSymbolsA {
    fn into(self) -> char {
        match self {
            MiscellaneousMathematicalSymbolsA::ThreeDimensionalAngle => '⟀',
            MiscellaneousMathematicalSymbolsA::WhiteTriangleContainingSmallWhiteTriangle => '⟁',
            MiscellaneousMathematicalSymbolsA::Perpendicular => '⟂',
            MiscellaneousMathematicalSymbolsA::OpenSubset => '⟃',
            MiscellaneousMathematicalSymbolsA::OpenSuperset => '⟄',
            MiscellaneousMathematicalSymbolsA::LeftSDashShapedBagDelimiter => '⟅',
            MiscellaneousMathematicalSymbolsA::RightSDashShapedBagDelimiter => '⟆',
            MiscellaneousMathematicalSymbolsA::OrWithDotInside => '⟇',
            MiscellaneousMathematicalSymbolsA::ReverseSolidusPrecedingSubset => '⟈',
            MiscellaneousMathematicalSymbolsA::SupersetPrecedingSolidus => '⟉',
            MiscellaneousMathematicalSymbolsA::VerticalBarWithHorizontalStroke => '⟊',
            MiscellaneousMathematicalSymbolsA::MathematicalRisingDiagonal => '⟋',
            MiscellaneousMathematicalSymbolsA::LongDivision => '⟌',
            MiscellaneousMathematicalSymbolsA::MathematicalFallingDiagonal => '⟍',
            MiscellaneousMathematicalSymbolsA::SquaredLogicalAnd => '⟎',
            MiscellaneousMathematicalSymbolsA::SquaredLogicalOr => '⟏',
            MiscellaneousMathematicalSymbolsA::WhiteDiamondWithCentredDot => '⟐',
            MiscellaneousMathematicalSymbolsA::AndWithDot => '⟑',
            MiscellaneousMathematicalSymbolsA::ElementOfOpeningUpwards => '⟒',
            MiscellaneousMathematicalSymbolsA::LowerRightCornerWithDot => '⟓',
            MiscellaneousMathematicalSymbolsA::UpperLeftCornerWithDot => '⟔',
            MiscellaneousMathematicalSymbolsA::LeftOuterJoin => '⟕',
            MiscellaneousMathematicalSymbolsA::RightOuterJoin => '⟖',
            MiscellaneousMathematicalSymbolsA::FullOuterJoin => '⟗',
            MiscellaneousMathematicalSymbolsA::LargeUpTack => '⟘',
            MiscellaneousMathematicalSymbolsA::LargeDownTack => '⟙',
            MiscellaneousMathematicalSymbolsA::LeftAndRightDoubleTurnstile => '⟚',
            MiscellaneousMathematicalSymbolsA::LeftAndRightTack => '⟛',
            MiscellaneousMathematicalSymbolsA::LeftMultimap => '⟜',
            MiscellaneousMathematicalSymbolsA::LongRightTack => '⟝',
            MiscellaneousMathematicalSymbolsA::LongLeftTack => '⟞',
            MiscellaneousMathematicalSymbolsA::UpTackWithCircleAbove => '⟟',
            MiscellaneousMathematicalSymbolsA::LozengeDividedByHorizontalRule => '⟠',
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamond => '⟡',
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithLeftwardsTick => '⟢',
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithRightwardsTick => '⟣',
            MiscellaneousMathematicalSymbolsA::WhiteSquareWithLeftwardsTick => '⟤',
            MiscellaneousMathematicalSymbolsA::WhiteSquareWithRightwardsTick => '⟥',
            MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteSquareBracket => '⟦',
            MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteSquareBracket => '⟧',
            MiscellaneousMathematicalSymbolsA::MathematicalLeftAngleBracket => '⟨',
            MiscellaneousMathematicalSymbolsA::MathematicalRightAngleBracket => '⟩',
            MiscellaneousMathematicalSymbolsA::MathematicalLeftDoubleAngleBracket => '⟪',
            MiscellaneousMathematicalSymbolsA::MathematicalRightDoubleAngleBracket => '⟫',
            MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteTortoiseShellBracket => '⟬',
            MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteTortoiseShellBracket => '⟭',
            MiscellaneousMathematicalSymbolsA::MathematicalLeftFlattenedParenthesis => '⟮',
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousMathematicalSymbolsA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⟀' => Ok(MiscellaneousMathematicalSymbolsA::ThreeDimensionalAngle),
            '⟁' => Ok(MiscellaneousMathematicalSymbolsA::WhiteTriangleContainingSmallWhiteTriangle),
            '⟂' => Ok(MiscellaneousMathematicalSymbolsA::Perpendicular),
            '⟃' => Ok(MiscellaneousMathematicalSymbolsA::OpenSubset),
            '⟄' => Ok(MiscellaneousMathematicalSymbolsA::OpenSuperset),
            '⟅' => Ok(MiscellaneousMathematicalSymbolsA::LeftSDashShapedBagDelimiter),
            '⟆' => Ok(MiscellaneousMathematicalSymbolsA::RightSDashShapedBagDelimiter),
            '⟇' => Ok(MiscellaneousMathematicalSymbolsA::OrWithDotInside),
            '⟈' => Ok(MiscellaneousMathematicalSymbolsA::ReverseSolidusPrecedingSubset),
            '⟉' => Ok(MiscellaneousMathematicalSymbolsA::SupersetPrecedingSolidus),
            '⟊' => Ok(MiscellaneousMathematicalSymbolsA::VerticalBarWithHorizontalStroke),
            '⟋' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRisingDiagonal),
            '⟌' => Ok(MiscellaneousMathematicalSymbolsA::LongDivision),
            '⟍' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalFallingDiagonal),
            '⟎' => Ok(MiscellaneousMathematicalSymbolsA::SquaredLogicalAnd),
            '⟏' => Ok(MiscellaneousMathematicalSymbolsA::SquaredLogicalOr),
            '⟐' => Ok(MiscellaneousMathematicalSymbolsA::WhiteDiamondWithCentredDot),
            '⟑' => Ok(MiscellaneousMathematicalSymbolsA::AndWithDot),
            '⟒' => Ok(MiscellaneousMathematicalSymbolsA::ElementOfOpeningUpwards),
            '⟓' => Ok(MiscellaneousMathematicalSymbolsA::LowerRightCornerWithDot),
            '⟔' => Ok(MiscellaneousMathematicalSymbolsA::UpperLeftCornerWithDot),
            '⟕' => Ok(MiscellaneousMathematicalSymbolsA::LeftOuterJoin),
            '⟖' => Ok(MiscellaneousMathematicalSymbolsA::RightOuterJoin),
            '⟗' => Ok(MiscellaneousMathematicalSymbolsA::FullOuterJoin),
            '⟘' => Ok(MiscellaneousMathematicalSymbolsA::LargeUpTack),
            '⟙' => Ok(MiscellaneousMathematicalSymbolsA::LargeDownTack),
            '⟚' => Ok(MiscellaneousMathematicalSymbolsA::LeftAndRightDoubleTurnstile),
            '⟛' => Ok(MiscellaneousMathematicalSymbolsA::LeftAndRightTack),
            '⟜' => Ok(MiscellaneousMathematicalSymbolsA::LeftMultimap),
            '⟝' => Ok(MiscellaneousMathematicalSymbolsA::LongRightTack),
            '⟞' => Ok(MiscellaneousMathematicalSymbolsA::LongLeftTack),
            '⟟' => Ok(MiscellaneousMathematicalSymbolsA::UpTackWithCircleAbove),
            '⟠' => Ok(MiscellaneousMathematicalSymbolsA::LozengeDividedByHorizontalRule),
            '⟡' => Ok(MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamond),
            '⟢' => Ok(MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithLeftwardsTick),
            '⟣' => Ok(MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithRightwardsTick),
            '⟤' => Ok(MiscellaneousMathematicalSymbolsA::WhiteSquareWithLeftwardsTick),
            '⟥' => Ok(MiscellaneousMathematicalSymbolsA::WhiteSquareWithRightwardsTick),
            '⟦' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteSquareBracket),
            '⟧' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteSquareBracket),
            '⟨' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftAngleBracket),
            '⟩' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightAngleBracket),
            '⟪' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftDoubleAngleBracket),
            '⟫' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightDoubleAngleBracket),
            '⟬' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteTortoiseShellBracket),
            '⟭' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteTortoiseShellBracket),
            '⟮' => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftFlattenedParenthesis),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MiscellaneousMathematicalSymbolsA {
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

impl std::convert::TryFrom<u32> for MiscellaneousMathematicalSymbolsA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MiscellaneousMathematicalSymbolsA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MiscellaneousMathematicalSymbolsA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MiscellaneousMathematicalSymbolsA::ThreeDimensionalAngle
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MiscellaneousMathematicalSymbolsA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
