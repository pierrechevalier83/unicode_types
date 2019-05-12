/// \u{25a0} → \u{25ff}\
///\
/// ■ □ ▢ ▣ ▤ ▥ ▦ ▧ ▨ ▩ ▪ ▫ ▬ ▭ ▮ ▯
/// ▰ ▱ ▲ △ ▴ ▵ ▶ ▷ ▸ ▹ ► ▻ ▼ ▽ ▾ ▿
/// ◀ ◁ ◂ ◃ ◄ ◅ ◆ ◇ ◈ ◉ ◊ ○ ◌ ◍ ◎ ●
/// ◐ ◑ ◒ ◓ ◔ ◕ ◖ ◗ ◘ ◙ ◚ ◛ ◜ ◝ ◞ ◟
/// ◠ ◡ ◢ ◣ ◤ ◥ ◦ ◧ ◨ ◩ ◪ ◫ ◬ ◭ ◮ ◯
/// ◰ ◱ ◲ ◳ ◴ ◵ ◶ ◷ ◸ ◹ ◺ ◻ ◼ ◽ ◾
pub mod constants {
    /// \u{25a0}: '■'
    pub const BLACK_SQUARE: char = '■';
    /// \u{25a1}: '□'
    pub const WHITE_SQUARE: char = '□';
    /// \u{25a2}: '▢'
    pub const WHITE_SQUARE_WITH_ROUNDED_CORNERS: char = '▢';
    /// \u{25a3}: '▣'
    pub const WHITE_SQUARE_CONTAINING_BLACK_SMALL_SQUARE: char = '▣';
    /// \u{25a4}: '▤'
    pub const SQUARE_WITH_HORIZONTAL_FILL: char = '▤';
    /// \u{25a5}: '▥'
    pub const SQUARE_WITH_VERTICAL_FILL: char = '▥';
    /// \u{25a6}: '▦'
    pub const SQUARE_WITH_ORTHOGONAL_CROSSHATCH_FILL: char = '▦';
    /// \u{25a7}: '▧'
    pub const SQUARE_WITH_UPPER_LEFT_TO_LOWER_RIGHT_FILL: char = '▧';
    /// \u{25a8}: '▨'
    pub const SQUARE_WITH_UPPER_RIGHT_TO_LOWER_LEFT_FILL: char = '▨';
    /// \u{25a9}: '▩'
    pub const SQUARE_WITH_DIAGONAL_CROSSHATCH_FILL: char = '▩';
    /// \u{25aa}: '▪'
    pub const BLACK_SMALL_SQUARE: char = '▪';
    /// \u{25ab}: '▫'
    pub const WHITE_SMALL_SQUARE: char = '▫';
    /// \u{25ac}: '▬'
    pub const BLACK_RECTANGLE: char = '▬';
    /// \u{25ad}: '▭'
    pub const WHITE_RECTANGLE: char = '▭';
    /// \u{25ae}: '▮'
    pub const BLACK_VERTICAL_RECTANGLE: char = '▮';
    /// \u{25af}: '▯'
    pub const WHITE_VERTICAL_RECTANGLE: char = '▯';
    /// \u{25b0}: '▰'
    pub const BLACK_PARALLELOGRAM: char = '▰';
    /// \u{25b1}: '▱'
    pub const WHITE_PARALLELOGRAM: char = '▱';
    /// \u{25b2}: '▲'
    pub const BLACK_UP_DASH_POINTING_TRIANGLE: char = '▲';
    /// \u{25b3}: '△'
    pub const WHITE_UP_DASH_POINTING_TRIANGLE: char = '△';
    /// \u{25b4}: '▴'
    pub const BLACK_UP_DASH_POINTING_SMALL_TRIANGLE: char = '▴';
    /// \u{25b5}: '▵'
    pub const WHITE_UP_DASH_POINTING_SMALL_TRIANGLE: char = '▵';
    /// \u{25b6}: '▶'
    pub const BLACK_RIGHT_DASH_POINTING_TRIANGLE: char = '▶';
    /// \u{25b7}: '▷'
    pub const WHITE_RIGHT_DASH_POINTING_TRIANGLE: char = '▷';
    /// \u{25b8}: '▸'
    pub const BLACK_RIGHT_DASH_POINTING_SMALL_TRIANGLE: char = '▸';
    /// \u{25b9}: '▹'
    pub const WHITE_RIGHT_DASH_POINTING_SMALL_TRIANGLE: char = '▹';
    /// \u{25ba}: '►'
    pub const BLACK_RIGHT_DASH_POINTING_POINTER: char = '►';
    /// \u{25bb}: '▻'
    pub const WHITE_RIGHT_DASH_POINTING_POINTER: char = '▻';
    /// \u{25bc}: '▼'
    pub const BLACK_DOWN_DASH_POINTING_TRIANGLE: char = '▼';
    /// \u{25bd}: '▽'
    pub const WHITE_DOWN_DASH_POINTING_TRIANGLE: char = '▽';
    /// \u{25be}: '▾'
    pub const BLACK_DOWN_DASH_POINTING_SMALL_TRIANGLE: char = '▾';
    /// \u{25bf}: '▿'
    pub const WHITE_DOWN_DASH_POINTING_SMALL_TRIANGLE: char = '▿';
    /// \u{25c0}: '◀'
    pub const BLACK_LEFT_DASH_POINTING_TRIANGLE: char = '◀';
    /// \u{25c1}: '◁'
    pub const WHITE_LEFT_DASH_POINTING_TRIANGLE: char = '◁';
    /// \u{25c2}: '◂'
    pub const BLACK_LEFT_DASH_POINTING_SMALL_TRIANGLE: char = '◂';
    /// \u{25c3}: '◃'
    pub const WHITE_LEFT_DASH_POINTING_SMALL_TRIANGLE: char = '◃';
    /// \u{25c4}: '◄'
    pub const BLACK_LEFT_DASH_POINTING_POINTER: char = '◄';
    /// \u{25c5}: '◅'
    pub const WHITE_LEFT_DASH_POINTING_POINTER: char = '◅';
    /// \u{25c6}: '◆'
    pub const BLACK_DIAMOND: char = '◆';
    /// \u{25c7}: '◇'
    pub const WHITE_DIAMOND: char = '◇';
    /// \u{25c8}: '◈'
    pub const WHITE_DIAMOND_CONTAINING_BLACK_SMALL_DIAMOND: char = '◈';
    /// \u{25c9}: '◉'
    pub const FISHEYE: char = '◉';
    /// \u{25ca}: '◊'
    pub const LOZENGE: char = '◊';
    /// \u{25cb}: '○'
    pub const WHITE_CIRCLE: char = '○';
    /// \u{25cc}: '◌'
    pub const DOTTED_CIRCLE: char = '◌';
    /// \u{25cd}: '◍'
    pub const CIRCLE_WITH_VERTICAL_FILL: char = '◍';
    /// \u{25ce}: '◎'
    pub const BULLSEYE: char = '◎';
    /// \u{25cf}: '●'
    pub const BLACK_CIRCLE: char = '●';
    /// \u{25d0}: '◐'
    pub const CIRCLE_WITH_LEFT_HALF_BLACK: char = '◐';
    /// \u{25d1}: '◑'
    pub const CIRCLE_WITH_RIGHT_HALF_BLACK: char = '◑';
    /// \u{25d2}: '◒'
    pub const CIRCLE_WITH_LOWER_HALF_BLACK: char = '◒';
    /// \u{25d3}: '◓'
    pub const CIRCLE_WITH_UPPER_HALF_BLACK: char = '◓';
    /// \u{25d4}: '◔'
    pub const CIRCLE_WITH_UPPER_RIGHT_QUADRANT_BLACK: char = '◔';
    /// \u{25d5}: '◕'
    pub const CIRCLE_WITH_ALL_BUT_UPPER_LEFT_QUADRANT_BLACK: char = '◕';
    /// \u{25d6}: '◖'
    pub const LEFT_HALF_BLACK_CIRCLE: char = '◖';
    /// \u{25d7}: '◗'
    pub const RIGHT_HALF_BLACK_CIRCLE: char = '◗';
    /// \u{25d8}: '◘'
    pub const INVERSE_BULLET: char = '◘';
    /// \u{25d9}: '◙'
    pub const INVERSE_WHITE_CIRCLE: char = '◙';
    /// \u{25da}: '◚'
    pub const UPPER_HALF_INVERSE_WHITE_CIRCLE: char = '◚';
    /// \u{25db}: '◛'
    pub const LOWER_HALF_INVERSE_WHITE_CIRCLE: char = '◛';
    /// \u{25dc}: '◜'
    pub const UPPER_LEFT_QUADRANT_CIRCULAR_ARC: char = '◜';
    /// \u{25dd}: '◝'
    pub const UPPER_RIGHT_QUADRANT_CIRCULAR_ARC: char = '◝';
    /// \u{25de}: '◞'
    pub const LOWER_RIGHT_QUADRANT_CIRCULAR_ARC: char = '◞';
    /// \u{25df}: '◟'
    pub const LOWER_LEFT_QUADRANT_CIRCULAR_ARC: char = '◟';
    /// \u{25e0}: '◠'
    pub const UPPER_HALF_CIRCLE: char = '◠';
    /// \u{25e1}: '◡'
    pub const LOWER_HALF_CIRCLE: char = '◡';
    /// \u{25e2}: '◢'
    pub const BLACK_LOWER_RIGHT_TRIANGLE: char = '◢';
    /// \u{25e3}: '◣'
    pub const BLACK_LOWER_LEFT_TRIANGLE: char = '◣';
    /// \u{25e4}: '◤'
    pub const BLACK_UPPER_LEFT_TRIANGLE: char = '◤';
    /// \u{25e5}: '◥'
    pub const BLACK_UPPER_RIGHT_TRIANGLE: char = '◥';
    /// \u{25e6}: '◦'
    pub const WHITE_BULLET: char = '◦';
    /// \u{25e7}: '◧'
    pub const SQUARE_WITH_LEFT_HALF_BLACK: char = '◧';
    /// \u{25e8}: '◨'
    pub const SQUARE_WITH_RIGHT_HALF_BLACK: char = '◨';
    /// \u{25e9}: '◩'
    pub const SQUARE_WITH_UPPER_LEFT_DIAGONAL_HALF_BLACK: char = '◩';
    /// \u{25ea}: '◪'
    pub const SQUARE_WITH_LOWER_RIGHT_DIAGONAL_HALF_BLACK: char = '◪';
    /// \u{25eb}: '◫'
    pub const WHITE_SQUARE_WITH_VERTICAL_BISECTING_LINE: char = '◫';
    /// \u{25ec}: '◬'
    pub const WHITE_UP_DASH_POINTING_TRIANGLE_WITH_DOT: char = '◬';
    /// \u{25ed}: '◭'
    pub const UP_DASH_POINTING_TRIANGLE_WITH_LEFT_HALF_BLACK: char = '◭';
    /// \u{25ee}: '◮'
    pub const UP_DASH_POINTING_TRIANGLE_WITH_RIGHT_HALF_BLACK: char = '◮';
    /// \u{25ef}: '◯'
    pub const LARGE_CIRCLE: char = '◯';
    /// \u{25f0}: '◰'
    pub const WHITE_SQUARE_WITH_UPPER_LEFT_QUADRANT: char = '◰';
    /// \u{25f1}: '◱'
    pub const WHITE_SQUARE_WITH_LOWER_LEFT_QUADRANT: char = '◱';
    /// \u{25f2}: '◲'
    pub const WHITE_SQUARE_WITH_LOWER_RIGHT_QUADRANT: char = '◲';
    /// \u{25f3}: '◳'
    pub const WHITE_SQUARE_WITH_UPPER_RIGHT_QUADRANT: char = '◳';
    /// \u{25f4}: '◴'
    pub const WHITE_CIRCLE_WITH_UPPER_LEFT_QUADRANT: char = '◴';
    /// \u{25f5}: '◵'
    pub const WHITE_CIRCLE_WITH_LOWER_LEFT_QUADRANT: char = '◵';
    /// \u{25f6}: '◶'
    pub const WHITE_CIRCLE_WITH_LOWER_RIGHT_QUADRANT: char = '◶';
    /// \u{25f7}: '◷'
    pub const WHITE_CIRCLE_WITH_UPPER_RIGHT_QUADRANT: char = '◷';
    /// \u{25f8}: '◸'
    pub const UPPER_LEFT_TRIANGLE: char = '◸';
    /// \u{25f9}: '◹'
    pub const UPPER_RIGHT_TRIANGLE: char = '◹';
    /// \u{25fa}: '◺'
    pub const LOWER_LEFT_TRIANGLE: char = '◺';
    /// \u{25fb}: '◻'
    pub const WHITE_MEDIUM_SQUARE: char = '◻';
    /// \u{25fc}: '◼'
    pub const BLACK_MEDIUM_SQUARE: char = '◼';
    /// \u{25fd}: '◽'
    pub const WHITE_MEDIUM_SMALL_SQUARE: char = '◽';
    /// \u{25fe}: '◾'
    pub const BLACK_MEDIUM_SMALL_SQUARE: char = '◾';
}

/// \u{25a0} → \u{25ff}\
///\
/// ■ □ ▢ ▣ ▤ ▥ ▦ ▧ ▨ ▩ ▪ ▫ ▬ ▭ ▮ ▯
/// ▰ ▱ ▲ △ ▴ ▵ ▶ ▷ ▸ ▹ ► ▻ ▼ ▽ ▾ ▿
/// ◀ ◁ ◂ ◃ ◄ ◅ ◆ ◇ ◈ ◉ ◊ ○ ◌ ◍ ◎ ●
/// ◐ ◑ ◒ ◓ ◔ ◕ ◖ ◗ ◘ ◙ ◚ ◛ ◜ ◝ ◞ ◟
/// ◠ ◡ ◢ ◣ ◤ ◥ ◦ ◧ ◨ ◩ ◪ ◫ ◬ ◭ ◮ ◯
/// ◰ ◱ ◲ ◳ ◴ ◵ ◶ ◷ ◸ ◹ ◺ ◻ ◼ ◽ ◾
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GeometricShapes {
    /// \u{25a0}: '■'
    BlackSquare,
    /// \u{25a1}: '□'
    WhiteSquare,
    /// \u{25a2}: '▢'
    WhiteSquareWithRoundedCorners,
    /// \u{25a3}: '▣'
    WhiteSquareContainingBlackSmallSquare,
    /// \u{25a4}: '▤'
    SquareWithHorizontalFill,
    /// \u{25a5}: '▥'
    SquareWithVerticalFill,
    /// \u{25a6}: '▦'
    SquareWithOrthogonalCrosshatchFill,
    /// \u{25a7}: '▧'
    SquareWithUpperLeftToLowerRightFill,
    /// \u{25a8}: '▨'
    SquareWithUpperRightToLowerLeftFill,
    /// \u{25a9}: '▩'
    SquareWithDiagonalCrosshatchFill,
    /// \u{25aa}: '▪'
    BlackSmallSquare,
    /// \u{25ab}: '▫'
    WhiteSmallSquare,
    /// \u{25ac}: '▬'
    BlackRectangle,
    /// \u{25ad}: '▭'
    WhiteRectangle,
    /// \u{25ae}: '▮'
    BlackVerticalRectangle,
    /// \u{25af}: '▯'
    WhiteVerticalRectangle,
    /// \u{25b0}: '▰'
    BlackParallelogram,
    /// \u{25b1}: '▱'
    WhiteParallelogram,
    /// \u{25b2}: '▲'
    BlackUpDashPointingTriangle,
    /// \u{25b3}: '△'
    WhiteUpDashPointingTriangle,
    /// \u{25b4}: '▴'
    BlackUpDashPointingSmallTriangle,
    /// \u{25b5}: '▵'
    WhiteUpDashPointingSmallTriangle,
    /// \u{25b6}: '▶'
    BlackRightDashPointingTriangle,
    /// \u{25b7}: '▷'
    WhiteRightDashPointingTriangle,
    /// \u{25b8}: '▸'
    BlackRightDashPointingSmallTriangle,
    /// \u{25b9}: '▹'
    WhiteRightDashPointingSmallTriangle,
    /// \u{25ba}: '►'
    BlackRightDashPointingPointer,
    /// \u{25bb}: '▻'
    WhiteRightDashPointingPointer,
    /// \u{25bc}: '▼'
    BlackDownDashPointingTriangle,
    /// \u{25bd}: '▽'
    WhiteDownDashPointingTriangle,
    /// \u{25be}: '▾'
    BlackDownDashPointingSmallTriangle,
    /// \u{25bf}: '▿'
    WhiteDownDashPointingSmallTriangle,
    /// \u{25c0}: '◀'
    BlackLeftDashPointingTriangle,
    /// \u{25c1}: '◁'
    WhiteLeftDashPointingTriangle,
    /// \u{25c2}: '◂'
    BlackLeftDashPointingSmallTriangle,
    /// \u{25c3}: '◃'
    WhiteLeftDashPointingSmallTriangle,
    /// \u{25c4}: '◄'
    BlackLeftDashPointingPointer,
    /// \u{25c5}: '◅'
    WhiteLeftDashPointingPointer,
    /// \u{25c6}: '◆'
    BlackDiamond,
    /// \u{25c7}: '◇'
    WhiteDiamond,
    /// \u{25c8}: '◈'
    WhiteDiamondContainingBlackSmallDiamond,
    /// \u{25c9}: '◉'
    Fisheye,
    /// \u{25ca}: '◊'
    Lozenge,
    /// \u{25cb}: '○'
    WhiteCircle,
    /// \u{25cc}: '◌'
    DottedCircle,
    /// \u{25cd}: '◍'
    CircleWithVerticalFill,
    /// \u{25ce}: '◎'
    Bullseye,
    /// \u{25cf}: '●'
    BlackCircle,
    /// \u{25d0}: '◐'
    CircleWithLeftHalfBlack,
    /// \u{25d1}: '◑'
    CircleWithRightHalfBlack,
    /// \u{25d2}: '◒'
    CircleWithLowerHalfBlack,
    /// \u{25d3}: '◓'
    CircleWithUpperHalfBlack,
    /// \u{25d4}: '◔'
    CircleWithUpperRightQuadrantBlack,
    /// \u{25d5}: '◕'
    CircleWithAllButUpperLeftQuadrantBlack,
    /// \u{25d6}: '◖'
    LeftHalfBlackCircle,
    /// \u{25d7}: '◗'
    RightHalfBlackCircle,
    /// \u{25d8}: '◘'
    InverseBullet,
    /// \u{25d9}: '◙'
    InverseWhiteCircle,
    /// \u{25da}: '◚'
    UpperHalfInverseWhiteCircle,
    /// \u{25db}: '◛'
    LowerHalfInverseWhiteCircle,
    /// \u{25dc}: '◜'
    UpperLeftQuadrantCircularArc,
    /// \u{25dd}: '◝'
    UpperRightQuadrantCircularArc,
    /// \u{25de}: '◞'
    LowerRightQuadrantCircularArc,
    /// \u{25df}: '◟'
    LowerLeftQuadrantCircularArc,
    /// \u{25e0}: '◠'
    UpperHalfCircle,
    /// \u{25e1}: '◡'
    LowerHalfCircle,
    /// \u{25e2}: '◢'
    BlackLowerRightTriangle,
    /// \u{25e3}: '◣'
    BlackLowerLeftTriangle,
    /// \u{25e4}: '◤'
    BlackUpperLeftTriangle,
    /// \u{25e5}: '◥'
    BlackUpperRightTriangle,
    /// \u{25e6}: '◦'
    WhiteBullet,
    /// \u{25e7}: '◧'
    SquareWithLeftHalfBlack,
    /// \u{25e8}: '◨'
    SquareWithRightHalfBlack,
    /// \u{25e9}: '◩'
    SquareWithUpperLeftDiagonalHalfBlack,
    /// \u{25ea}: '◪'
    SquareWithLowerRightDiagonalHalfBlack,
    /// \u{25eb}: '◫'
    WhiteSquareWithVerticalBisectingLine,
    /// \u{25ec}: '◬'
    WhiteUpDashPointingTriangleWithDot,
    /// \u{25ed}: '◭'
    UpDashPointingTriangleWithLeftHalfBlack,
    /// \u{25ee}: '◮'
    UpDashPointingTriangleWithRightHalfBlack,
    /// \u{25ef}: '◯'
    LargeCircle,
    /// \u{25f0}: '◰'
    WhiteSquareWithUpperLeftQuadrant,
    /// \u{25f1}: '◱'
    WhiteSquareWithLowerLeftQuadrant,
    /// \u{25f2}: '◲'
    WhiteSquareWithLowerRightQuadrant,
    /// \u{25f3}: '◳'
    WhiteSquareWithUpperRightQuadrant,
    /// \u{25f4}: '◴'
    WhiteCircleWithUpperLeftQuadrant,
    /// \u{25f5}: '◵'
    WhiteCircleWithLowerLeftQuadrant,
    /// \u{25f6}: '◶'
    WhiteCircleWithLowerRightQuadrant,
    /// \u{25f7}: '◷'
    WhiteCircleWithUpperRightQuadrant,
    /// \u{25f8}: '◸'
    UpperLeftTriangle,
    /// \u{25f9}: '◹'
    UpperRightTriangle,
    /// \u{25fa}: '◺'
    LowerLeftTriangle,
    /// \u{25fb}: '◻'
    WhiteMediumSquare,
    /// \u{25fc}: '◼'
    BlackMediumSquare,
    /// \u{25fd}: '◽'
    WhiteMediumSmallSquare,
    /// \u{25fe}: '◾'
    BlackMediumSmallSquare,
}

impl Into<char> for GeometricShapes {
    fn into(self) -> char {
        use constants::*;
        match self {
            GeometricShapes::BlackSquare => BLACK_SQUARE,
            GeometricShapes::WhiteSquare => WHITE_SQUARE,
            GeometricShapes::WhiteSquareWithRoundedCorners => WHITE_SQUARE_WITH_ROUNDED_CORNERS,
            GeometricShapes::WhiteSquareContainingBlackSmallSquare => WHITE_SQUARE_CONTAINING_BLACK_SMALL_SQUARE,
            GeometricShapes::SquareWithHorizontalFill => SQUARE_WITH_HORIZONTAL_FILL,
            GeometricShapes::SquareWithVerticalFill => SQUARE_WITH_VERTICAL_FILL,
            GeometricShapes::SquareWithOrthogonalCrosshatchFill => SQUARE_WITH_ORTHOGONAL_CROSSHATCH_FILL,
            GeometricShapes::SquareWithUpperLeftToLowerRightFill => SQUARE_WITH_UPPER_LEFT_TO_LOWER_RIGHT_FILL,
            GeometricShapes::SquareWithUpperRightToLowerLeftFill => SQUARE_WITH_UPPER_RIGHT_TO_LOWER_LEFT_FILL,
            GeometricShapes::SquareWithDiagonalCrosshatchFill => SQUARE_WITH_DIAGONAL_CROSSHATCH_FILL,
            GeometricShapes::BlackSmallSquare => BLACK_SMALL_SQUARE,
            GeometricShapes::WhiteSmallSquare => WHITE_SMALL_SQUARE,
            GeometricShapes::BlackRectangle => BLACK_RECTANGLE,
            GeometricShapes::WhiteRectangle => WHITE_RECTANGLE,
            GeometricShapes::BlackVerticalRectangle => BLACK_VERTICAL_RECTANGLE,
            GeometricShapes::WhiteVerticalRectangle => WHITE_VERTICAL_RECTANGLE,
            GeometricShapes::BlackParallelogram => BLACK_PARALLELOGRAM,
            GeometricShapes::WhiteParallelogram => WHITE_PARALLELOGRAM,
            GeometricShapes::BlackUpDashPointingTriangle => BLACK_UP_DASH_POINTING_TRIANGLE,
            GeometricShapes::WhiteUpDashPointingTriangle => WHITE_UP_DASH_POINTING_TRIANGLE,
            GeometricShapes::BlackUpDashPointingSmallTriangle => BLACK_UP_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::WhiteUpDashPointingSmallTriangle => WHITE_UP_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::BlackRightDashPointingTriangle => BLACK_RIGHT_DASH_POINTING_TRIANGLE,
            GeometricShapes::WhiteRightDashPointingTriangle => WHITE_RIGHT_DASH_POINTING_TRIANGLE,
            GeometricShapes::BlackRightDashPointingSmallTriangle => BLACK_RIGHT_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::WhiteRightDashPointingSmallTriangle => WHITE_RIGHT_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::BlackRightDashPointingPointer => BLACK_RIGHT_DASH_POINTING_POINTER,
            GeometricShapes::WhiteRightDashPointingPointer => WHITE_RIGHT_DASH_POINTING_POINTER,
            GeometricShapes::BlackDownDashPointingTriangle => BLACK_DOWN_DASH_POINTING_TRIANGLE,
            GeometricShapes::WhiteDownDashPointingTriangle => WHITE_DOWN_DASH_POINTING_TRIANGLE,
            GeometricShapes::BlackDownDashPointingSmallTriangle => BLACK_DOWN_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::WhiteDownDashPointingSmallTriangle => WHITE_DOWN_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::BlackLeftDashPointingTriangle => BLACK_LEFT_DASH_POINTING_TRIANGLE,
            GeometricShapes::WhiteLeftDashPointingTriangle => WHITE_LEFT_DASH_POINTING_TRIANGLE,
            GeometricShapes::BlackLeftDashPointingSmallTriangle => BLACK_LEFT_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::WhiteLeftDashPointingSmallTriangle => WHITE_LEFT_DASH_POINTING_SMALL_TRIANGLE,
            GeometricShapes::BlackLeftDashPointingPointer => BLACK_LEFT_DASH_POINTING_POINTER,
            GeometricShapes::WhiteLeftDashPointingPointer => WHITE_LEFT_DASH_POINTING_POINTER,
            GeometricShapes::BlackDiamond => BLACK_DIAMOND,
            GeometricShapes::WhiteDiamond => WHITE_DIAMOND,
            GeometricShapes::WhiteDiamondContainingBlackSmallDiamond => WHITE_DIAMOND_CONTAINING_BLACK_SMALL_DIAMOND,
            GeometricShapes::Fisheye => FISHEYE,
            GeometricShapes::Lozenge => LOZENGE,
            GeometricShapes::WhiteCircle => WHITE_CIRCLE,
            GeometricShapes::DottedCircle => DOTTED_CIRCLE,
            GeometricShapes::CircleWithVerticalFill => CIRCLE_WITH_VERTICAL_FILL,
            GeometricShapes::Bullseye => BULLSEYE,
            GeometricShapes::BlackCircle => BLACK_CIRCLE,
            GeometricShapes::CircleWithLeftHalfBlack => CIRCLE_WITH_LEFT_HALF_BLACK,
            GeometricShapes::CircleWithRightHalfBlack => CIRCLE_WITH_RIGHT_HALF_BLACK,
            GeometricShapes::CircleWithLowerHalfBlack => CIRCLE_WITH_LOWER_HALF_BLACK,
            GeometricShapes::CircleWithUpperHalfBlack => CIRCLE_WITH_UPPER_HALF_BLACK,
            GeometricShapes::CircleWithUpperRightQuadrantBlack => CIRCLE_WITH_UPPER_RIGHT_QUADRANT_BLACK,
            GeometricShapes::CircleWithAllButUpperLeftQuadrantBlack => CIRCLE_WITH_ALL_BUT_UPPER_LEFT_QUADRANT_BLACK,
            GeometricShapes::LeftHalfBlackCircle => LEFT_HALF_BLACK_CIRCLE,
            GeometricShapes::RightHalfBlackCircle => RIGHT_HALF_BLACK_CIRCLE,
            GeometricShapes::InverseBullet => INVERSE_BULLET,
            GeometricShapes::InverseWhiteCircle => INVERSE_WHITE_CIRCLE,
            GeometricShapes::UpperHalfInverseWhiteCircle => UPPER_HALF_INVERSE_WHITE_CIRCLE,
            GeometricShapes::LowerHalfInverseWhiteCircle => LOWER_HALF_INVERSE_WHITE_CIRCLE,
            GeometricShapes::UpperLeftQuadrantCircularArc => UPPER_LEFT_QUADRANT_CIRCULAR_ARC,
            GeometricShapes::UpperRightQuadrantCircularArc => UPPER_RIGHT_QUADRANT_CIRCULAR_ARC,
            GeometricShapes::LowerRightQuadrantCircularArc => LOWER_RIGHT_QUADRANT_CIRCULAR_ARC,
            GeometricShapes::LowerLeftQuadrantCircularArc => LOWER_LEFT_QUADRANT_CIRCULAR_ARC,
            GeometricShapes::UpperHalfCircle => UPPER_HALF_CIRCLE,
            GeometricShapes::LowerHalfCircle => LOWER_HALF_CIRCLE,
            GeometricShapes::BlackLowerRightTriangle => BLACK_LOWER_RIGHT_TRIANGLE,
            GeometricShapes::BlackLowerLeftTriangle => BLACK_LOWER_LEFT_TRIANGLE,
            GeometricShapes::BlackUpperLeftTriangle => BLACK_UPPER_LEFT_TRIANGLE,
            GeometricShapes::BlackUpperRightTriangle => BLACK_UPPER_RIGHT_TRIANGLE,
            GeometricShapes::WhiteBullet => WHITE_BULLET,
            GeometricShapes::SquareWithLeftHalfBlack => SQUARE_WITH_LEFT_HALF_BLACK,
            GeometricShapes::SquareWithRightHalfBlack => SQUARE_WITH_RIGHT_HALF_BLACK,
            GeometricShapes::SquareWithUpperLeftDiagonalHalfBlack => SQUARE_WITH_UPPER_LEFT_DIAGONAL_HALF_BLACK,
            GeometricShapes::SquareWithLowerRightDiagonalHalfBlack => SQUARE_WITH_LOWER_RIGHT_DIAGONAL_HALF_BLACK,
            GeometricShapes::WhiteSquareWithVerticalBisectingLine => WHITE_SQUARE_WITH_VERTICAL_BISECTING_LINE,
            GeometricShapes::WhiteUpDashPointingTriangleWithDot => WHITE_UP_DASH_POINTING_TRIANGLE_WITH_DOT,
            GeometricShapes::UpDashPointingTriangleWithLeftHalfBlack => UP_DASH_POINTING_TRIANGLE_WITH_LEFT_HALF_BLACK,
            GeometricShapes::UpDashPointingTriangleWithRightHalfBlack => UP_DASH_POINTING_TRIANGLE_WITH_RIGHT_HALF_BLACK,
            GeometricShapes::LargeCircle => LARGE_CIRCLE,
            GeometricShapes::WhiteSquareWithUpperLeftQuadrant => WHITE_SQUARE_WITH_UPPER_LEFT_QUADRANT,
            GeometricShapes::WhiteSquareWithLowerLeftQuadrant => WHITE_SQUARE_WITH_LOWER_LEFT_QUADRANT,
            GeometricShapes::WhiteSquareWithLowerRightQuadrant => WHITE_SQUARE_WITH_LOWER_RIGHT_QUADRANT,
            GeometricShapes::WhiteSquareWithUpperRightQuadrant => WHITE_SQUARE_WITH_UPPER_RIGHT_QUADRANT,
            GeometricShapes::WhiteCircleWithUpperLeftQuadrant => WHITE_CIRCLE_WITH_UPPER_LEFT_QUADRANT,
            GeometricShapes::WhiteCircleWithLowerLeftQuadrant => WHITE_CIRCLE_WITH_LOWER_LEFT_QUADRANT,
            GeometricShapes::WhiteCircleWithLowerRightQuadrant => WHITE_CIRCLE_WITH_LOWER_RIGHT_QUADRANT,
            GeometricShapes::WhiteCircleWithUpperRightQuadrant => WHITE_CIRCLE_WITH_UPPER_RIGHT_QUADRANT,
            GeometricShapes::UpperLeftTriangle => UPPER_LEFT_TRIANGLE,
            GeometricShapes::UpperRightTriangle => UPPER_RIGHT_TRIANGLE,
            GeometricShapes::LowerLeftTriangle => LOWER_LEFT_TRIANGLE,
            GeometricShapes::WhiteMediumSquare => WHITE_MEDIUM_SQUARE,
            GeometricShapes::BlackMediumSquare => BLACK_MEDIUM_SQUARE,
            GeometricShapes::WhiteMediumSmallSquare => WHITE_MEDIUM_SMALL_SQUARE,
            GeometricShapes::BlackMediumSmallSquare => BLACK_MEDIUM_SMALL_SQUARE,
        }
    }
}

impl std::convert::TryFrom<char> for GeometricShapes {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BLACK_SQUARE => Ok(GeometricShapes::BlackSquare),
            WHITE_SQUARE => Ok(GeometricShapes::WhiteSquare),
            WHITE_SQUARE_WITH_ROUNDED_CORNERS => Ok(GeometricShapes::WhiteSquareWithRoundedCorners),
            WHITE_SQUARE_CONTAINING_BLACK_SMALL_SQUARE => Ok(GeometricShapes::WhiteSquareContainingBlackSmallSquare),
            SQUARE_WITH_HORIZONTAL_FILL => Ok(GeometricShapes::SquareWithHorizontalFill),
            SQUARE_WITH_VERTICAL_FILL => Ok(GeometricShapes::SquareWithVerticalFill),
            SQUARE_WITH_ORTHOGONAL_CROSSHATCH_FILL => Ok(GeometricShapes::SquareWithOrthogonalCrosshatchFill),
            SQUARE_WITH_UPPER_LEFT_TO_LOWER_RIGHT_FILL => Ok(GeometricShapes::SquareWithUpperLeftToLowerRightFill),
            SQUARE_WITH_UPPER_RIGHT_TO_LOWER_LEFT_FILL => Ok(GeometricShapes::SquareWithUpperRightToLowerLeftFill),
            SQUARE_WITH_DIAGONAL_CROSSHATCH_FILL => Ok(GeometricShapes::SquareWithDiagonalCrosshatchFill),
            BLACK_SMALL_SQUARE => Ok(GeometricShapes::BlackSmallSquare),
            WHITE_SMALL_SQUARE => Ok(GeometricShapes::WhiteSmallSquare),
            BLACK_RECTANGLE => Ok(GeometricShapes::BlackRectangle),
            WHITE_RECTANGLE => Ok(GeometricShapes::WhiteRectangle),
            BLACK_VERTICAL_RECTANGLE => Ok(GeometricShapes::BlackVerticalRectangle),
            WHITE_VERTICAL_RECTANGLE => Ok(GeometricShapes::WhiteVerticalRectangle),
            BLACK_PARALLELOGRAM => Ok(GeometricShapes::BlackParallelogram),
            WHITE_PARALLELOGRAM => Ok(GeometricShapes::WhiteParallelogram),
            BLACK_UP_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::BlackUpDashPointingTriangle),
            WHITE_UP_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::WhiteUpDashPointingTriangle),
            BLACK_UP_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::BlackUpDashPointingSmallTriangle),
            WHITE_UP_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::WhiteUpDashPointingSmallTriangle),
            BLACK_RIGHT_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::BlackRightDashPointingTriangle),
            WHITE_RIGHT_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::WhiteRightDashPointingTriangle),
            BLACK_RIGHT_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::BlackRightDashPointingSmallTriangle),
            WHITE_RIGHT_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::WhiteRightDashPointingSmallTriangle),
            BLACK_RIGHT_DASH_POINTING_POINTER => Ok(GeometricShapes::BlackRightDashPointingPointer),
            WHITE_RIGHT_DASH_POINTING_POINTER => Ok(GeometricShapes::WhiteRightDashPointingPointer),
            BLACK_DOWN_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::BlackDownDashPointingTriangle),
            WHITE_DOWN_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::WhiteDownDashPointingTriangle),
            BLACK_DOWN_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::BlackDownDashPointingSmallTriangle),
            WHITE_DOWN_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::WhiteDownDashPointingSmallTriangle),
            BLACK_LEFT_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::BlackLeftDashPointingTriangle),
            WHITE_LEFT_DASH_POINTING_TRIANGLE => Ok(GeometricShapes::WhiteLeftDashPointingTriangle),
            BLACK_LEFT_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::BlackLeftDashPointingSmallTriangle),
            WHITE_LEFT_DASH_POINTING_SMALL_TRIANGLE => Ok(GeometricShapes::WhiteLeftDashPointingSmallTriangle),
            BLACK_LEFT_DASH_POINTING_POINTER => Ok(GeometricShapes::BlackLeftDashPointingPointer),
            WHITE_LEFT_DASH_POINTING_POINTER => Ok(GeometricShapes::WhiteLeftDashPointingPointer),
            BLACK_DIAMOND => Ok(GeometricShapes::BlackDiamond),
            WHITE_DIAMOND => Ok(GeometricShapes::WhiteDiamond),
            WHITE_DIAMOND_CONTAINING_BLACK_SMALL_DIAMOND => Ok(GeometricShapes::WhiteDiamondContainingBlackSmallDiamond),
            FISHEYE => Ok(GeometricShapes::Fisheye),
            LOZENGE => Ok(GeometricShapes::Lozenge),
            WHITE_CIRCLE => Ok(GeometricShapes::WhiteCircle),
            DOTTED_CIRCLE => Ok(GeometricShapes::DottedCircle),
            CIRCLE_WITH_VERTICAL_FILL => Ok(GeometricShapes::CircleWithVerticalFill),
            BULLSEYE => Ok(GeometricShapes::Bullseye),
            BLACK_CIRCLE => Ok(GeometricShapes::BlackCircle),
            CIRCLE_WITH_LEFT_HALF_BLACK => Ok(GeometricShapes::CircleWithLeftHalfBlack),
            CIRCLE_WITH_RIGHT_HALF_BLACK => Ok(GeometricShapes::CircleWithRightHalfBlack),
            CIRCLE_WITH_LOWER_HALF_BLACK => Ok(GeometricShapes::CircleWithLowerHalfBlack),
            CIRCLE_WITH_UPPER_HALF_BLACK => Ok(GeometricShapes::CircleWithUpperHalfBlack),
            CIRCLE_WITH_UPPER_RIGHT_QUADRANT_BLACK => Ok(GeometricShapes::CircleWithUpperRightQuadrantBlack),
            CIRCLE_WITH_ALL_BUT_UPPER_LEFT_QUADRANT_BLACK => Ok(GeometricShapes::CircleWithAllButUpperLeftQuadrantBlack),
            LEFT_HALF_BLACK_CIRCLE => Ok(GeometricShapes::LeftHalfBlackCircle),
            RIGHT_HALF_BLACK_CIRCLE => Ok(GeometricShapes::RightHalfBlackCircle),
            INVERSE_BULLET => Ok(GeometricShapes::InverseBullet),
            INVERSE_WHITE_CIRCLE => Ok(GeometricShapes::InverseWhiteCircle),
            UPPER_HALF_INVERSE_WHITE_CIRCLE => Ok(GeometricShapes::UpperHalfInverseWhiteCircle),
            LOWER_HALF_INVERSE_WHITE_CIRCLE => Ok(GeometricShapes::LowerHalfInverseWhiteCircle),
            UPPER_LEFT_QUADRANT_CIRCULAR_ARC => Ok(GeometricShapes::UpperLeftQuadrantCircularArc),
            UPPER_RIGHT_QUADRANT_CIRCULAR_ARC => Ok(GeometricShapes::UpperRightQuadrantCircularArc),
            LOWER_RIGHT_QUADRANT_CIRCULAR_ARC => Ok(GeometricShapes::LowerRightQuadrantCircularArc),
            LOWER_LEFT_QUADRANT_CIRCULAR_ARC => Ok(GeometricShapes::LowerLeftQuadrantCircularArc),
            UPPER_HALF_CIRCLE => Ok(GeometricShapes::UpperHalfCircle),
            LOWER_HALF_CIRCLE => Ok(GeometricShapes::LowerHalfCircle),
            BLACK_LOWER_RIGHT_TRIANGLE => Ok(GeometricShapes::BlackLowerRightTriangle),
            BLACK_LOWER_LEFT_TRIANGLE => Ok(GeometricShapes::BlackLowerLeftTriangle),
            BLACK_UPPER_LEFT_TRIANGLE => Ok(GeometricShapes::BlackUpperLeftTriangle),
            BLACK_UPPER_RIGHT_TRIANGLE => Ok(GeometricShapes::BlackUpperRightTriangle),
            WHITE_BULLET => Ok(GeometricShapes::WhiteBullet),
            SQUARE_WITH_LEFT_HALF_BLACK => Ok(GeometricShapes::SquareWithLeftHalfBlack),
            SQUARE_WITH_RIGHT_HALF_BLACK => Ok(GeometricShapes::SquareWithRightHalfBlack),
            SQUARE_WITH_UPPER_LEFT_DIAGONAL_HALF_BLACK => Ok(GeometricShapes::SquareWithUpperLeftDiagonalHalfBlack),
            SQUARE_WITH_LOWER_RIGHT_DIAGONAL_HALF_BLACK => Ok(GeometricShapes::SquareWithLowerRightDiagonalHalfBlack),
            WHITE_SQUARE_WITH_VERTICAL_BISECTING_LINE => Ok(GeometricShapes::WhiteSquareWithVerticalBisectingLine),
            WHITE_UP_DASH_POINTING_TRIANGLE_WITH_DOT => Ok(GeometricShapes::WhiteUpDashPointingTriangleWithDot),
            UP_DASH_POINTING_TRIANGLE_WITH_LEFT_HALF_BLACK => Ok(GeometricShapes::UpDashPointingTriangleWithLeftHalfBlack),
            UP_DASH_POINTING_TRIANGLE_WITH_RIGHT_HALF_BLACK => Ok(GeometricShapes::UpDashPointingTriangleWithRightHalfBlack),
            LARGE_CIRCLE => Ok(GeometricShapes::LargeCircle),
            WHITE_SQUARE_WITH_UPPER_LEFT_QUADRANT => Ok(GeometricShapes::WhiteSquareWithUpperLeftQuadrant),
            WHITE_SQUARE_WITH_LOWER_LEFT_QUADRANT => Ok(GeometricShapes::WhiteSquareWithLowerLeftQuadrant),
            WHITE_SQUARE_WITH_LOWER_RIGHT_QUADRANT => Ok(GeometricShapes::WhiteSquareWithLowerRightQuadrant),
            WHITE_SQUARE_WITH_UPPER_RIGHT_QUADRANT => Ok(GeometricShapes::WhiteSquareWithUpperRightQuadrant),
            WHITE_CIRCLE_WITH_UPPER_LEFT_QUADRANT => Ok(GeometricShapes::WhiteCircleWithUpperLeftQuadrant),
            WHITE_CIRCLE_WITH_LOWER_LEFT_QUADRANT => Ok(GeometricShapes::WhiteCircleWithLowerLeftQuadrant),
            WHITE_CIRCLE_WITH_LOWER_RIGHT_QUADRANT => Ok(GeometricShapes::WhiteCircleWithLowerRightQuadrant),
            WHITE_CIRCLE_WITH_UPPER_RIGHT_QUADRANT => Ok(GeometricShapes::WhiteCircleWithUpperRightQuadrant),
            UPPER_LEFT_TRIANGLE => Ok(GeometricShapes::UpperLeftTriangle),
            UPPER_RIGHT_TRIANGLE => Ok(GeometricShapes::UpperRightTriangle),
            LOWER_LEFT_TRIANGLE => Ok(GeometricShapes::LowerLeftTriangle),
            WHITE_MEDIUM_SQUARE => Ok(GeometricShapes::WhiteMediumSquare),
            BLACK_MEDIUM_SQUARE => Ok(GeometricShapes::BlackMediumSquare),
            WHITE_MEDIUM_SMALL_SQUARE => Ok(GeometricShapes::WhiteMediumSmallSquare),
            BLACK_MEDIUM_SMALL_SQUARE => Ok(GeometricShapes::BlackMediumSmallSquare),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GeometricShapes {
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

impl std::convert::TryFrom<u32> for GeometricShapes {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GeometricShapes {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GeometricShapes {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GeometricShapes::BlackSquare
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GeometricShapes::BlackSquare => "black square",
            GeometricShapes::WhiteSquare => "white square",
            GeometricShapes::WhiteSquareWithRoundedCorners => "white square with rounded corners",
            GeometricShapes::WhiteSquareContainingBlackSmallSquare => "white square containing black small square",
            GeometricShapes::SquareWithHorizontalFill => "square with horizontal fill",
            GeometricShapes::SquareWithVerticalFill => "square with vertical fill",
            GeometricShapes::SquareWithOrthogonalCrosshatchFill => "square with orthogonal crosshatch fill",
            GeometricShapes::SquareWithUpperLeftToLowerRightFill => "square with upper left to lower right fill",
            GeometricShapes::SquareWithUpperRightToLowerLeftFill => "square with upper right to lower left fill",
            GeometricShapes::SquareWithDiagonalCrosshatchFill => "square with diagonal crosshatch fill",
            GeometricShapes::BlackSmallSquare => "black small square",
            GeometricShapes::WhiteSmallSquare => "white small square",
            GeometricShapes::BlackRectangle => "black rectangle",
            GeometricShapes::WhiteRectangle => "white rectangle",
            GeometricShapes::BlackVerticalRectangle => "black vertical rectangle",
            GeometricShapes::WhiteVerticalRectangle => "white vertical rectangle",
            GeometricShapes::BlackParallelogram => "black parallelogram",
            GeometricShapes::WhiteParallelogram => "white parallelogram",
            GeometricShapes::BlackUpDashPointingTriangle => "black up-pointing triangle",
            GeometricShapes::WhiteUpDashPointingTriangle => "white up-pointing triangle",
            GeometricShapes::BlackUpDashPointingSmallTriangle => "black up-pointing small triangle",
            GeometricShapes::WhiteUpDashPointingSmallTriangle => "white up-pointing small triangle",
            GeometricShapes::BlackRightDashPointingTriangle => "black right-pointing triangle",
            GeometricShapes::WhiteRightDashPointingTriangle => "white right-pointing triangle",
            GeometricShapes::BlackRightDashPointingSmallTriangle => "black right-pointing small triangle",
            GeometricShapes::WhiteRightDashPointingSmallTriangle => "white right-pointing small triangle",
            GeometricShapes::BlackRightDashPointingPointer => "black right-pointing pointer",
            GeometricShapes::WhiteRightDashPointingPointer => "white right-pointing pointer",
            GeometricShapes::BlackDownDashPointingTriangle => "black down-pointing triangle",
            GeometricShapes::WhiteDownDashPointingTriangle => "white down-pointing triangle",
            GeometricShapes::BlackDownDashPointingSmallTriangle => "black down-pointing small triangle",
            GeometricShapes::WhiteDownDashPointingSmallTriangle => "white down-pointing small triangle",
            GeometricShapes::BlackLeftDashPointingTriangle => "black left-pointing triangle",
            GeometricShapes::WhiteLeftDashPointingTriangle => "white left-pointing triangle",
            GeometricShapes::BlackLeftDashPointingSmallTriangle => "black left-pointing small triangle",
            GeometricShapes::WhiteLeftDashPointingSmallTriangle => "white left-pointing small triangle",
            GeometricShapes::BlackLeftDashPointingPointer => "black left-pointing pointer",
            GeometricShapes::WhiteLeftDashPointingPointer => "white left-pointing pointer",
            GeometricShapes::BlackDiamond => "black diamond",
            GeometricShapes::WhiteDiamond => "white diamond",
            GeometricShapes::WhiteDiamondContainingBlackSmallDiamond => "white diamond containing black small diamond",
            GeometricShapes::Fisheye => "fisheye",
            GeometricShapes::Lozenge => "lozenge",
            GeometricShapes::WhiteCircle => "white circle",
            GeometricShapes::DottedCircle => "dotted circle",
            GeometricShapes::CircleWithVerticalFill => "circle with vertical fill",
            GeometricShapes::Bullseye => "bullseye",
            GeometricShapes::BlackCircle => "black circle",
            GeometricShapes::CircleWithLeftHalfBlack => "circle with left half black",
            GeometricShapes::CircleWithRightHalfBlack => "circle with right half black",
            GeometricShapes::CircleWithLowerHalfBlack => "circle with lower half black",
            GeometricShapes::CircleWithUpperHalfBlack => "circle with upper half black",
            GeometricShapes::CircleWithUpperRightQuadrantBlack => "circle with upper right quadrant black",
            GeometricShapes::CircleWithAllButUpperLeftQuadrantBlack => "circle with all but upper left quadrant black",
            GeometricShapes::LeftHalfBlackCircle => "left half black circle",
            GeometricShapes::RightHalfBlackCircle => "right half black circle",
            GeometricShapes::InverseBullet => "inverse bullet",
            GeometricShapes::InverseWhiteCircle => "inverse white circle",
            GeometricShapes::UpperHalfInverseWhiteCircle => "upper half inverse white circle",
            GeometricShapes::LowerHalfInverseWhiteCircle => "lower half inverse white circle",
            GeometricShapes::UpperLeftQuadrantCircularArc => "upper left quadrant circular arc",
            GeometricShapes::UpperRightQuadrantCircularArc => "upper right quadrant circular arc",
            GeometricShapes::LowerRightQuadrantCircularArc => "lower right quadrant circular arc",
            GeometricShapes::LowerLeftQuadrantCircularArc => "lower left quadrant circular arc",
            GeometricShapes::UpperHalfCircle => "upper half circle",
            GeometricShapes::LowerHalfCircle => "lower half circle",
            GeometricShapes::BlackLowerRightTriangle => "black lower right triangle",
            GeometricShapes::BlackLowerLeftTriangle => "black lower left triangle",
            GeometricShapes::BlackUpperLeftTriangle => "black upper left triangle",
            GeometricShapes::BlackUpperRightTriangle => "black upper right triangle",
            GeometricShapes::WhiteBullet => "white bullet",
            GeometricShapes::SquareWithLeftHalfBlack => "square with left half black",
            GeometricShapes::SquareWithRightHalfBlack => "square with right half black",
            GeometricShapes::SquareWithUpperLeftDiagonalHalfBlack => "square with upper left diagonal half black",
            GeometricShapes::SquareWithLowerRightDiagonalHalfBlack => "square with lower right diagonal half black",
            GeometricShapes::WhiteSquareWithVerticalBisectingLine => "white square with vertical bisecting line",
            GeometricShapes::WhiteUpDashPointingTriangleWithDot => "white up-pointing triangle with dot",
            GeometricShapes::UpDashPointingTriangleWithLeftHalfBlack => "up-pointing triangle with left half black",
            GeometricShapes::UpDashPointingTriangleWithRightHalfBlack => "up-pointing triangle with right half black",
            GeometricShapes::LargeCircle => "large circle",
            GeometricShapes::WhiteSquareWithUpperLeftQuadrant => "white square with upper left quadrant",
            GeometricShapes::WhiteSquareWithLowerLeftQuadrant => "white square with lower left quadrant",
            GeometricShapes::WhiteSquareWithLowerRightQuadrant => "white square with lower right quadrant",
            GeometricShapes::WhiteSquareWithUpperRightQuadrant => "white square with upper right quadrant",
            GeometricShapes::WhiteCircleWithUpperLeftQuadrant => "white circle with upper left quadrant",
            GeometricShapes::WhiteCircleWithLowerLeftQuadrant => "white circle with lower left quadrant",
            GeometricShapes::WhiteCircleWithLowerRightQuadrant => "white circle with lower right quadrant",
            GeometricShapes::WhiteCircleWithUpperRightQuadrant => "white circle with upper right quadrant",
            GeometricShapes::UpperLeftTriangle => "upper left triangle",
            GeometricShapes::UpperRightTriangle => "upper right triangle",
            GeometricShapes::LowerLeftTriangle => "lower left triangle",
            GeometricShapes::WhiteMediumSquare => "white medium square",
            GeometricShapes::BlackMediumSquare => "black medium square",
            GeometricShapes::WhiteMediumSmallSquare => "white medium small square",
            GeometricShapes::BlackMediumSmallSquare => "black medium small square",
        }
    }
}
