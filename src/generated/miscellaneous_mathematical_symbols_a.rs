/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{27c0}: '⟀'
    pub const THREE_DIMENSIONAL_ANGLE: char = '⟀';
    /// \u{27c1}: '⟁'
    pub const WHITE_TRIANGLE_CONTAINING_SMALL_WHITE_TRIANGLE: char = '⟁';
    /// \u{27c2}: '⟂'
    pub const PERPENDICULAR: char = '⟂';
    /// \u{27c3}: '⟃'
    pub const OPEN_SUBSET: char = '⟃';
    /// \u{27c4}: '⟄'
    pub const OPEN_SUPERSET: char = '⟄';
    /// \u{27c5}: '⟅'
    pub const LEFT_S_DASH_SHAPED_BAG_DELIMITER: char = '⟅';
    /// \u{27c6}: '⟆'
    pub const RIGHT_S_DASH_SHAPED_BAG_DELIMITER: char = '⟆';
    /// \u{27c7}: '⟇'
    pub const OR_WITH_DOT_INSIDE: char = '⟇';
    /// \u{27c8}: '⟈'
    pub const REVERSE_SOLIDUS_PRECEDING_SUBSET: char = '⟈';
    /// \u{27c9}: '⟉'
    pub const SUPERSET_PRECEDING_SOLIDUS: char = '⟉';
    /// \u{27ca}: '⟊'
    pub const VERTICAL_BAR_WITH_HORIZONTAL_STROKE: char = '⟊';
    /// \u{27cb}: '⟋'
    pub const MATHEMATICAL_RISING_DIAGONAL: char = '⟋';
    /// \u{27cc}: '⟌'
    pub const LONG_DIVISION: char = '⟌';
    /// \u{27cd}: '⟍'
    pub const MATHEMATICAL_FALLING_DIAGONAL: char = '⟍';
    /// \u{27ce}: '⟎'
    pub const SQUARED_LOGICAL_AND: char = '⟎';
    /// \u{27cf}: '⟏'
    pub const SQUARED_LOGICAL_OR: char = '⟏';
    /// \u{27d0}: '⟐'
    pub const WHITE_DIAMOND_WITH_CENTRED_DOT: char = '⟐';
    /// \u{27d1}: '⟑'
    pub const AND_WITH_DOT: char = '⟑';
    /// \u{27d2}: '⟒'
    pub const ELEMENT_OF_OPENING_UPWARDS: char = '⟒';
    /// \u{27d3}: '⟓'
    pub const LOWER_RIGHT_CORNER_WITH_DOT: char = '⟓';
    /// \u{27d4}: '⟔'
    pub const UPPER_LEFT_CORNER_WITH_DOT: char = '⟔';
    /// \u{27d5}: '⟕'
    pub const LEFT_OUTER_JOIN: char = '⟕';
    /// \u{27d6}: '⟖'
    pub const RIGHT_OUTER_JOIN: char = '⟖';
    /// \u{27d7}: '⟗'
    pub const FULL_OUTER_JOIN: char = '⟗';
    /// \u{27d8}: '⟘'
    pub const LARGE_UP_TACK: char = '⟘';
    /// \u{27d9}: '⟙'
    pub const LARGE_DOWN_TACK: char = '⟙';
    /// \u{27da}: '⟚'
    pub const LEFT_AND_RIGHT_DOUBLE_TURNSTILE: char = '⟚';
    /// \u{27db}: '⟛'
    pub const LEFT_AND_RIGHT_TACK: char = '⟛';
    /// \u{27dc}: '⟜'
    pub const LEFT_MULTIMAP: char = '⟜';
    /// \u{27dd}: '⟝'
    pub const LONG_RIGHT_TACK: char = '⟝';
    /// \u{27de}: '⟞'
    pub const LONG_LEFT_TACK: char = '⟞';
    /// \u{27df}: '⟟'
    pub const UP_TACK_WITH_CIRCLE_ABOVE: char = '⟟';
    /// \u{27e0}: '⟠'
    pub const LOZENGE_DIVIDED_BY_HORIZONTAL_RULE: char = '⟠';
    /// \u{27e1}: '⟡'
    pub const WHITE_CONCAVE_DASH_SIDED_DIAMOND: char = '⟡';
    /// \u{27e2}: '⟢'
    pub const WHITE_CONCAVE_DASH_SIDED_DIAMOND_WITH_LEFTWARDS_TICK: char = '⟢';
    /// \u{27e3}: '⟣'
    pub const WHITE_CONCAVE_DASH_SIDED_DIAMOND_WITH_RIGHTWARDS_TICK: char = '⟣';
    /// \u{27e4}: '⟤'
    pub const WHITE_SQUARE_WITH_LEFTWARDS_TICK: char = '⟤';
    /// \u{27e5}: '⟥'
    pub const WHITE_SQUARE_WITH_RIGHTWARDS_TICK: char = '⟥';
    /// \u{27e6}: '⟦'
    pub const MATHEMATICAL_LEFT_WHITE_SQUARE_BRACKET: char = '⟦';
    /// \u{27e7}: '⟧'
    pub const MATHEMATICAL_RIGHT_WHITE_SQUARE_BRACKET: char = '⟧';
    /// \u{27e8}: '⟨'
    pub const MATHEMATICAL_LEFT_ANGLE_BRACKET: char = '⟨';
    /// \u{27e9}: '⟩'
    pub const MATHEMATICAL_RIGHT_ANGLE_BRACKET: char = '⟩';
    /// \u{27ea}: '⟪'
    pub const MATHEMATICAL_LEFT_DOUBLE_ANGLE_BRACKET: char = '⟪';
    /// \u{27eb}: '⟫'
    pub const MATHEMATICAL_RIGHT_DOUBLE_ANGLE_BRACKET: char = '⟫';
    /// \u{27ec}: '⟬'
    pub const MATHEMATICAL_LEFT_WHITE_TORTOISE_SHELL_BRACKET: char = '⟬';
    /// \u{27ed}: '⟭'
    pub const MATHEMATICAL_RIGHT_WHITE_TORTOISE_SHELL_BRACKET: char = '⟭';
    /// \u{27ee}: '⟮'
    pub const MATHEMATICAL_LEFT_FLATTENED_PARENTHESIS: char = '⟮';
}

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
        use constants::*;
        match self {
            MiscellaneousMathematicalSymbolsA::ThreeDimensionalAngle => THREE_DIMENSIONAL_ANGLE,
            MiscellaneousMathematicalSymbolsA::WhiteTriangleContainingSmallWhiteTriangle => WHITE_TRIANGLE_CONTAINING_SMALL_WHITE_TRIANGLE,
            MiscellaneousMathematicalSymbolsA::Perpendicular => PERPENDICULAR,
            MiscellaneousMathematicalSymbolsA::OpenSubset => OPEN_SUBSET,
            MiscellaneousMathematicalSymbolsA::OpenSuperset => OPEN_SUPERSET,
            MiscellaneousMathematicalSymbolsA::LeftSDashShapedBagDelimiter => LEFT_S_DASH_SHAPED_BAG_DELIMITER,
            MiscellaneousMathematicalSymbolsA::RightSDashShapedBagDelimiter => RIGHT_S_DASH_SHAPED_BAG_DELIMITER,
            MiscellaneousMathematicalSymbolsA::OrWithDotInside => OR_WITH_DOT_INSIDE,
            MiscellaneousMathematicalSymbolsA::ReverseSolidusPrecedingSubset => REVERSE_SOLIDUS_PRECEDING_SUBSET,
            MiscellaneousMathematicalSymbolsA::SupersetPrecedingSolidus => SUPERSET_PRECEDING_SOLIDUS,
            MiscellaneousMathematicalSymbolsA::VerticalBarWithHorizontalStroke => VERTICAL_BAR_WITH_HORIZONTAL_STROKE,
            MiscellaneousMathematicalSymbolsA::MathematicalRisingDiagonal => MATHEMATICAL_RISING_DIAGONAL,
            MiscellaneousMathematicalSymbolsA::LongDivision => LONG_DIVISION,
            MiscellaneousMathematicalSymbolsA::MathematicalFallingDiagonal => MATHEMATICAL_FALLING_DIAGONAL,
            MiscellaneousMathematicalSymbolsA::SquaredLogicalAnd => SQUARED_LOGICAL_AND,
            MiscellaneousMathematicalSymbolsA::SquaredLogicalOr => SQUARED_LOGICAL_OR,
            MiscellaneousMathematicalSymbolsA::WhiteDiamondWithCentredDot => WHITE_DIAMOND_WITH_CENTRED_DOT,
            MiscellaneousMathematicalSymbolsA::AndWithDot => AND_WITH_DOT,
            MiscellaneousMathematicalSymbolsA::ElementOfOpeningUpwards => ELEMENT_OF_OPENING_UPWARDS,
            MiscellaneousMathematicalSymbolsA::LowerRightCornerWithDot => LOWER_RIGHT_CORNER_WITH_DOT,
            MiscellaneousMathematicalSymbolsA::UpperLeftCornerWithDot => UPPER_LEFT_CORNER_WITH_DOT,
            MiscellaneousMathematicalSymbolsA::LeftOuterJoin => LEFT_OUTER_JOIN,
            MiscellaneousMathematicalSymbolsA::RightOuterJoin => RIGHT_OUTER_JOIN,
            MiscellaneousMathematicalSymbolsA::FullOuterJoin => FULL_OUTER_JOIN,
            MiscellaneousMathematicalSymbolsA::LargeUpTack => LARGE_UP_TACK,
            MiscellaneousMathematicalSymbolsA::LargeDownTack => LARGE_DOWN_TACK,
            MiscellaneousMathematicalSymbolsA::LeftAndRightDoubleTurnstile => LEFT_AND_RIGHT_DOUBLE_TURNSTILE,
            MiscellaneousMathematicalSymbolsA::LeftAndRightTack => LEFT_AND_RIGHT_TACK,
            MiscellaneousMathematicalSymbolsA::LeftMultimap => LEFT_MULTIMAP,
            MiscellaneousMathematicalSymbolsA::LongRightTack => LONG_RIGHT_TACK,
            MiscellaneousMathematicalSymbolsA::LongLeftTack => LONG_LEFT_TACK,
            MiscellaneousMathematicalSymbolsA::UpTackWithCircleAbove => UP_TACK_WITH_CIRCLE_ABOVE,
            MiscellaneousMathematicalSymbolsA::LozengeDividedByHorizontalRule => LOZENGE_DIVIDED_BY_HORIZONTAL_RULE,
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamond => WHITE_CONCAVE_DASH_SIDED_DIAMOND,
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithLeftwardsTick => WHITE_CONCAVE_DASH_SIDED_DIAMOND_WITH_LEFTWARDS_TICK,
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithRightwardsTick => WHITE_CONCAVE_DASH_SIDED_DIAMOND_WITH_RIGHTWARDS_TICK,
            MiscellaneousMathematicalSymbolsA::WhiteSquareWithLeftwardsTick => WHITE_SQUARE_WITH_LEFTWARDS_TICK,
            MiscellaneousMathematicalSymbolsA::WhiteSquareWithRightwardsTick => WHITE_SQUARE_WITH_RIGHTWARDS_TICK,
            MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteSquareBracket => MATHEMATICAL_LEFT_WHITE_SQUARE_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteSquareBracket => MATHEMATICAL_RIGHT_WHITE_SQUARE_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalLeftAngleBracket => MATHEMATICAL_LEFT_ANGLE_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalRightAngleBracket => MATHEMATICAL_RIGHT_ANGLE_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalLeftDoubleAngleBracket => MATHEMATICAL_LEFT_DOUBLE_ANGLE_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalRightDoubleAngleBracket => MATHEMATICAL_RIGHT_DOUBLE_ANGLE_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteTortoiseShellBracket => MATHEMATICAL_LEFT_WHITE_TORTOISE_SHELL_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteTortoiseShellBracket => MATHEMATICAL_RIGHT_WHITE_TORTOISE_SHELL_BRACKET,
            MiscellaneousMathematicalSymbolsA::MathematicalLeftFlattenedParenthesis => MATHEMATICAL_LEFT_FLATTENED_PARENTHESIS,
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousMathematicalSymbolsA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            THREE_DIMENSIONAL_ANGLE => Ok(MiscellaneousMathematicalSymbolsA::ThreeDimensionalAngle),
            WHITE_TRIANGLE_CONTAINING_SMALL_WHITE_TRIANGLE => Ok(MiscellaneousMathematicalSymbolsA::WhiteTriangleContainingSmallWhiteTriangle),
            PERPENDICULAR => Ok(MiscellaneousMathematicalSymbolsA::Perpendicular),
            OPEN_SUBSET => Ok(MiscellaneousMathematicalSymbolsA::OpenSubset),
            OPEN_SUPERSET => Ok(MiscellaneousMathematicalSymbolsA::OpenSuperset),
            LEFT_S_DASH_SHAPED_BAG_DELIMITER => Ok(MiscellaneousMathematicalSymbolsA::LeftSDashShapedBagDelimiter),
            RIGHT_S_DASH_SHAPED_BAG_DELIMITER => Ok(MiscellaneousMathematicalSymbolsA::RightSDashShapedBagDelimiter),
            OR_WITH_DOT_INSIDE => Ok(MiscellaneousMathematicalSymbolsA::OrWithDotInside),
            REVERSE_SOLIDUS_PRECEDING_SUBSET => Ok(MiscellaneousMathematicalSymbolsA::ReverseSolidusPrecedingSubset),
            SUPERSET_PRECEDING_SOLIDUS => Ok(MiscellaneousMathematicalSymbolsA::SupersetPrecedingSolidus),
            VERTICAL_BAR_WITH_HORIZONTAL_STROKE => Ok(MiscellaneousMathematicalSymbolsA::VerticalBarWithHorizontalStroke),
            MATHEMATICAL_RISING_DIAGONAL => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRisingDiagonal),
            LONG_DIVISION => Ok(MiscellaneousMathematicalSymbolsA::LongDivision),
            MATHEMATICAL_FALLING_DIAGONAL => Ok(MiscellaneousMathematicalSymbolsA::MathematicalFallingDiagonal),
            SQUARED_LOGICAL_AND => Ok(MiscellaneousMathematicalSymbolsA::SquaredLogicalAnd),
            SQUARED_LOGICAL_OR => Ok(MiscellaneousMathematicalSymbolsA::SquaredLogicalOr),
            WHITE_DIAMOND_WITH_CENTRED_DOT => Ok(MiscellaneousMathematicalSymbolsA::WhiteDiamondWithCentredDot),
            AND_WITH_DOT => Ok(MiscellaneousMathematicalSymbolsA::AndWithDot),
            ELEMENT_OF_OPENING_UPWARDS => Ok(MiscellaneousMathematicalSymbolsA::ElementOfOpeningUpwards),
            LOWER_RIGHT_CORNER_WITH_DOT => Ok(MiscellaneousMathematicalSymbolsA::LowerRightCornerWithDot),
            UPPER_LEFT_CORNER_WITH_DOT => Ok(MiscellaneousMathematicalSymbolsA::UpperLeftCornerWithDot),
            LEFT_OUTER_JOIN => Ok(MiscellaneousMathematicalSymbolsA::LeftOuterJoin),
            RIGHT_OUTER_JOIN => Ok(MiscellaneousMathematicalSymbolsA::RightOuterJoin),
            FULL_OUTER_JOIN => Ok(MiscellaneousMathematicalSymbolsA::FullOuterJoin),
            LARGE_UP_TACK => Ok(MiscellaneousMathematicalSymbolsA::LargeUpTack),
            LARGE_DOWN_TACK => Ok(MiscellaneousMathematicalSymbolsA::LargeDownTack),
            LEFT_AND_RIGHT_DOUBLE_TURNSTILE => Ok(MiscellaneousMathematicalSymbolsA::LeftAndRightDoubleTurnstile),
            LEFT_AND_RIGHT_TACK => Ok(MiscellaneousMathematicalSymbolsA::LeftAndRightTack),
            LEFT_MULTIMAP => Ok(MiscellaneousMathematicalSymbolsA::LeftMultimap),
            LONG_RIGHT_TACK => Ok(MiscellaneousMathematicalSymbolsA::LongRightTack),
            LONG_LEFT_TACK => Ok(MiscellaneousMathematicalSymbolsA::LongLeftTack),
            UP_TACK_WITH_CIRCLE_ABOVE => Ok(MiscellaneousMathematicalSymbolsA::UpTackWithCircleAbove),
            LOZENGE_DIVIDED_BY_HORIZONTAL_RULE => Ok(MiscellaneousMathematicalSymbolsA::LozengeDividedByHorizontalRule),
            WHITE_CONCAVE_DASH_SIDED_DIAMOND => Ok(MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamond),
            WHITE_CONCAVE_DASH_SIDED_DIAMOND_WITH_LEFTWARDS_TICK => Ok(MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithLeftwardsTick),
            WHITE_CONCAVE_DASH_SIDED_DIAMOND_WITH_RIGHTWARDS_TICK => Ok(MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithRightwardsTick),
            WHITE_SQUARE_WITH_LEFTWARDS_TICK => Ok(MiscellaneousMathematicalSymbolsA::WhiteSquareWithLeftwardsTick),
            WHITE_SQUARE_WITH_RIGHTWARDS_TICK => Ok(MiscellaneousMathematicalSymbolsA::WhiteSquareWithRightwardsTick),
            MATHEMATICAL_LEFT_WHITE_SQUARE_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteSquareBracket),
            MATHEMATICAL_RIGHT_WHITE_SQUARE_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteSquareBracket),
            MATHEMATICAL_LEFT_ANGLE_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftAngleBracket),
            MATHEMATICAL_RIGHT_ANGLE_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightAngleBracket),
            MATHEMATICAL_LEFT_DOUBLE_ANGLE_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftDoubleAngleBracket),
            MATHEMATICAL_RIGHT_DOUBLE_ANGLE_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightDoubleAngleBracket),
            MATHEMATICAL_LEFT_WHITE_TORTOISE_SHELL_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteTortoiseShellBracket),
            MATHEMATICAL_RIGHT_WHITE_TORTOISE_SHELL_BRACKET => Ok(MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteTortoiseShellBracket),
            MATHEMATICAL_LEFT_FLATTENED_PARENTHESIS => Ok(MiscellaneousMathematicalSymbolsA::MathematicalLeftFlattenedParenthesis),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MiscellaneousMathematicalSymbolsA::ThreeDimensionalAngle => "three dimensional angle",
            MiscellaneousMathematicalSymbolsA::WhiteTriangleContainingSmallWhiteTriangle => "white triangle containing small white triangle",
            MiscellaneousMathematicalSymbolsA::Perpendicular => "perpendicular",
            MiscellaneousMathematicalSymbolsA::OpenSubset => "open subset",
            MiscellaneousMathematicalSymbolsA::OpenSuperset => "open superset",
            MiscellaneousMathematicalSymbolsA::LeftSDashShapedBagDelimiter => "left s-shaped bag delimiter",
            MiscellaneousMathematicalSymbolsA::RightSDashShapedBagDelimiter => "right s-shaped bag delimiter",
            MiscellaneousMathematicalSymbolsA::OrWithDotInside => "or with dot inside",
            MiscellaneousMathematicalSymbolsA::ReverseSolidusPrecedingSubset => "reverse solidus preceding subset",
            MiscellaneousMathematicalSymbolsA::SupersetPrecedingSolidus => "superset preceding solidus",
            MiscellaneousMathematicalSymbolsA::VerticalBarWithHorizontalStroke => "vertical bar with horizontal stroke",
            MiscellaneousMathematicalSymbolsA::MathematicalRisingDiagonal => "mathematical rising diagonal",
            MiscellaneousMathematicalSymbolsA::LongDivision => "long division",
            MiscellaneousMathematicalSymbolsA::MathematicalFallingDiagonal => "mathematical falling diagonal",
            MiscellaneousMathematicalSymbolsA::SquaredLogicalAnd => "squared logical and",
            MiscellaneousMathematicalSymbolsA::SquaredLogicalOr => "squared logical or",
            MiscellaneousMathematicalSymbolsA::WhiteDiamondWithCentredDot => "white diamond with centred dot",
            MiscellaneousMathematicalSymbolsA::AndWithDot => "and with dot",
            MiscellaneousMathematicalSymbolsA::ElementOfOpeningUpwards => "element of opening upwards",
            MiscellaneousMathematicalSymbolsA::LowerRightCornerWithDot => "lower right corner with dot",
            MiscellaneousMathematicalSymbolsA::UpperLeftCornerWithDot => "upper left corner with dot",
            MiscellaneousMathematicalSymbolsA::LeftOuterJoin => "left outer join",
            MiscellaneousMathematicalSymbolsA::RightOuterJoin => "right outer join",
            MiscellaneousMathematicalSymbolsA::FullOuterJoin => "full outer join",
            MiscellaneousMathematicalSymbolsA::LargeUpTack => "large up tack",
            MiscellaneousMathematicalSymbolsA::LargeDownTack => "large down tack",
            MiscellaneousMathematicalSymbolsA::LeftAndRightDoubleTurnstile => "left and right double turnstile",
            MiscellaneousMathematicalSymbolsA::LeftAndRightTack => "left and right tack",
            MiscellaneousMathematicalSymbolsA::LeftMultimap => "left multimap",
            MiscellaneousMathematicalSymbolsA::LongRightTack => "long right tack",
            MiscellaneousMathematicalSymbolsA::LongLeftTack => "long left tack",
            MiscellaneousMathematicalSymbolsA::UpTackWithCircleAbove => "up tack with circle above",
            MiscellaneousMathematicalSymbolsA::LozengeDividedByHorizontalRule => "lozenge divided by horizontal rule",
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamond => "white concave-sided diamond",
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithLeftwardsTick => "white concave-sided diamond with leftwards tick",
            MiscellaneousMathematicalSymbolsA::WhiteConcaveDashSidedDiamondWithRightwardsTick => "white concave-sided diamond with rightwards tick",
            MiscellaneousMathematicalSymbolsA::WhiteSquareWithLeftwardsTick => "white square with leftwards tick",
            MiscellaneousMathematicalSymbolsA::WhiteSquareWithRightwardsTick => "white square with rightwards tick",
            MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteSquareBracket => "mathematical left white square bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteSquareBracket => "mathematical right white square bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalLeftAngleBracket => "mathematical left angle bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalRightAngleBracket => "mathematical right angle bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalLeftDoubleAngleBracket => "mathematical left double angle bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalRightDoubleAngleBracket => "mathematical right double angle bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalLeftWhiteTortoiseShellBracket => "mathematical left white tortoise shell bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalRightWhiteTortoiseShellBracket => "mathematical right white tortoise shell bracket",
            MiscellaneousMathematicalSymbolsA::MathematicalLeftFlattenedParenthesis => "mathematical left flattened parenthesis",
        }
    }
}
