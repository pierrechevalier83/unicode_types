
/// An enum to represent all characters in the GeometricShapes block.
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
        match self {
            GeometricShapes::BlackSquare => '■',
            GeometricShapes::WhiteSquare => '□',
            GeometricShapes::WhiteSquareWithRoundedCorners => '▢',
            GeometricShapes::WhiteSquareContainingBlackSmallSquare => '▣',
            GeometricShapes::SquareWithHorizontalFill => '▤',
            GeometricShapes::SquareWithVerticalFill => '▥',
            GeometricShapes::SquareWithOrthogonalCrosshatchFill => '▦',
            GeometricShapes::SquareWithUpperLeftToLowerRightFill => '▧',
            GeometricShapes::SquareWithUpperRightToLowerLeftFill => '▨',
            GeometricShapes::SquareWithDiagonalCrosshatchFill => '▩',
            GeometricShapes::BlackSmallSquare => '▪',
            GeometricShapes::WhiteSmallSquare => '▫',
            GeometricShapes::BlackRectangle => '▬',
            GeometricShapes::WhiteRectangle => '▭',
            GeometricShapes::BlackVerticalRectangle => '▮',
            GeometricShapes::WhiteVerticalRectangle => '▯',
            GeometricShapes::BlackParallelogram => '▰',
            GeometricShapes::WhiteParallelogram => '▱',
            GeometricShapes::BlackUpDashPointingTriangle => '▲',
            GeometricShapes::WhiteUpDashPointingTriangle => '△',
            GeometricShapes::BlackUpDashPointingSmallTriangle => '▴',
            GeometricShapes::WhiteUpDashPointingSmallTriangle => '▵',
            GeometricShapes::BlackRightDashPointingTriangle => '▶',
            GeometricShapes::WhiteRightDashPointingTriangle => '▷',
            GeometricShapes::BlackRightDashPointingSmallTriangle => '▸',
            GeometricShapes::WhiteRightDashPointingSmallTriangle => '▹',
            GeometricShapes::BlackRightDashPointingPointer => '►',
            GeometricShapes::WhiteRightDashPointingPointer => '▻',
            GeometricShapes::BlackDownDashPointingTriangle => '▼',
            GeometricShapes::WhiteDownDashPointingTriangle => '▽',
            GeometricShapes::BlackDownDashPointingSmallTriangle => '▾',
            GeometricShapes::WhiteDownDashPointingSmallTriangle => '▿',
            GeometricShapes::BlackLeftDashPointingTriangle => '◀',
            GeometricShapes::WhiteLeftDashPointingTriangle => '◁',
            GeometricShapes::BlackLeftDashPointingSmallTriangle => '◂',
            GeometricShapes::WhiteLeftDashPointingSmallTriangle => '◃',
            GeometricShapes::BlackLeftDashPointingPointer => '◄',
            GeometricShapes::WhiteLeftDashPointingPointer => '◅',
            GeometricShapes::BlackDiamond => '◆',
            GeometricShapes::WhiteDiamond => '◇',
            GeometricShapes::WhiteDiamondContainingBlackSmallDiamond => '◈',
            GeometricShapes::Fisheye => '◉',
            GeometricShapes::Lozenge => '◊',
            GeometricShapes::WhiteCircle => '○',
            GeometricShapes::DottedCircle => '◌',
            GeometricShapes::CircleWithVerticalFill => '◍',
            GeometricShapes::Bullseye => '◎',
            GeometricShapes::BlackCircle => '●',
            GeometricShapes::CircleWithLeftHalfBlack => '◐',
            GeometricShapes::CircleWithRightHalfBlack => '◑',
            GeometricShapes::CircleWithLowerHalfBlack => '◒',
            GeometricShapes::CircleWithUpperHalfBlack => '◓',
            GeometricShapes::CircleWithUpperRightQuadrantBlack => '◔',
            GeometricShapes::CircleWithAllButUpperLeftQuadrantBlack => '◕',
            GeometricShapes::LeftHalfBlackCircle => '◖',
            GeometricShapes::RightHalfBlackCircle => '◗',
            GeometricShapes::InverseBullet => '◘',
            GeometricShapes::InverseWhiteCircle => '◙',
            GeometricShapes::UpperHalfInverseWhiteCircle => '◚',
            GeometricShapes::LowerHalfInverseWhiteCircle => '◛',
            GeometricShapes::UpperLeftQuadrantCircularArc => '◜',
            GeometricShapes::UpperRightQuadrantCircularArc => '◝',
            GeometricShapes::LowerRightQuadrantCircularArc => '◞',
            GeometricShapes::LowerLeftQuadrantCircularArc => '◟',
            GeometricShapes::UpperHalfCircle => '◠',
            GeometricShapes::LowerHalfCircle => '◡',
            GeometricShapes::BlackLowerRightTriangle => '◢',
            GeometricShapes::BlackLowerLeftTriangle => '◣',
            GeometricShapes::BlackUpperLeftTriangle => '◤',
            GeometricShapes::BlackUpperRightTriangle => '◥',
            GeometricShapes::WhiteBullet => '◦',
            GeometricShapes::SquareWithLeftHalfBlack => '◧',
            GeometricShapes::SquareWithRightHalfBlack => '◨',
            GeometricShapes::SquareWithUpperLeftDiagonalHalfBlack => '◩',
            GeometricShapes::SquareWithLowerRightDiagonalHalfBlack => '◪',
            GeometricShapes::WhiteSquareWithVerticalBisectingLine => '◫',
            GeometricShapes::WhiteUpDashPointingTriangleWithDot => '◬',
            GeometricShapes::UpDashPointingTriangleWithLeftHalfBlack => '◭',
            GeometricShapes::UpDashPointingTriangleWithRightHalfBlack => '◮',
            GeometricShapes::LargeCircle => '◯',
            GeometricShapes::WhiteSquareWithUpperLeftQuadrant => '◰',
            GeometricShapes::WhiteSquareWithLowerLeftQuadrant => '◱',
            GeometricShapes::WhiteSquareWithLowerRightQuadrant => '◲',
            GeometricShapes::WhiteSquareWithUpperRightQuadrant => '◳',
            GeometricShapes::WhiteCircleWithUpperLeftQuadrant => '◴',
            GeometricShapes::WhiteCircleWithLowerLeftQuadrant => '◵',
            GeometricShapes::WhiteCircleWithLowerRightQuadrant => '◶',
            GeometricShapes::WhiteCircleWithUpperRightQuadrant => '◷',
            GeometricShapes::UpperLeftTriangle => '◸',
            GeometricShapes::UpperRightTriangle => '◹',
            GeometricShapes::LowerLeftTriangle => '◺',
            GeometricShapes::WhiteMediumSquare => '◻',
            GeometricShapes::BlackMediumSquare => '◼',
            GeometricShapes::WhiteMediumSmallSquare => '◽',
            GeometricShapes::BlackMediumSmallSquare => '◾',
        }
    }
}

impl std::convert::TryFrom<char> for GeometricShapes {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '■' => Ok(GeometricShapes::BlackSquare),
            '□' => Ok(GeometricShapes::WhiteSquare),
            '▢' => Ok(GeometricShapes::WhiteSquareWithRoundedCorners),
            '▣' => Ok(GeometricShapes::WhiteSquareContainingBlackSmallSquare),
            '▤' => Ok(GeometricShapes::SquareWithHorizontalFill),
            '▥' => Ok(GeometricShapes::SquareWithVerticalFill),
            '▦' => Ok(GeometricShapes::SquareWithOrthogonalCrosshatchFill),
            '▧' => Ok(GeometricShapes::SquareWithUpperLeftToLowerRightFill),
            '▨' => Ok(GeometricShapes::SquareWithUpperRightToLowerLeftFill),
            '▩' => Ok(GeometricShapes::SquareWithDiagonalCrosshatchFill),
            '▪' => Ok(GeometricShapes::BlackSmallSquare),
            '▫' => Ok(GeometricShapes::WhiteSmallSquare),
            '▬' => Ok(GeometricShapes::BlackRectangle),
            '▭' => Ok(GeometricShapes::WhiteRectangle),
            '▮' => Ok(GeometricShapes::BlackVerticalRectangle),
            '▯' => Ok(GeometricShapes::WhiteVerticalRectangle),
            '▰' => Ok(GeometricShapes::BlackParallelogram),
            '▱' => Ok(GeometricShapes::WhiteParallelogram),
            '▲' => Ok(GeometricShapes::BlackUpDashPointingTriangle),
            '△' => Ok(GeometricShapes::WhiteUpDashPointingTriangle),
            '▴' => Ok(GeometricShapes::BlackUpDashPointingSmallTriangle),
            '▵' => Ok(GeometricShapes::WhiteUpDashPointingSmallTriangle),
            '▶' => Ok(GeometricShapes::BlackRightDashPointingTriangle),
            '▷' => Ok(GeometricShapes::WhiteRightDashPointingTriangle),
            '▸' => Ok(GeometricShapes::BlackRightDashPointingSmallTriangle),
            '▹' => Ok(GeometricShapes::WhiteRightDashPointingSmallTriangle),
            '►' => Ok(GeometricShapes::BlackRightDashPointingPointer),
            '▻' => Ok(GeometricShapes::WhiteRightDashPointingPointer),
            '▼' => Ok(GeometricShapes::BlackDownDashPointingTriangle),
            '▽' => Ok(GeometricShapes::WhiteDownDashPointingTriangle),
            '▾' => Ok(GeometricShapes::BlackDownDashPointingSmallTriangle),
            '▿' => Ok(GeometricShapes::WhiteDownDashPointingSmallTriangle),
            '◀' => Ok(GeometricShapes::BlackLeftDashPointingTriangle),
            '◁' => Ok(GeometricShapes::WhiteLeftDashPointingTriangle),
            '◂' => Ok(GeometricShapes::BlackLeftDashPointingSmallTriangle),
            '◃' => Ok(GeometricShapes::WhiteLeftDashPointingSmallTriangle),
            '◄' => Ok(GeometricShapes::BlackLeftDashPointingPointer),
            '◅' => Ok(GeometricShapes::WhiteLeftDashPointingPointer),
            '◆' => Ok(GeometricShapes::BlackDiamond),
            '◇' => Ok(GeometricShapes::WhiteDiamond),
            '◈' => Ok(GeometricShapes::WhiteDiamondContainingBlackSmallDiamond),
            '◉' => Ok(GeometricShapes::Fisheye),
            '◊' => Ok(GeometricShapes::Lozenge),
            '○' => Ok(GeometricShapes::WhiteCircle),
            '◌' => Ok(GeometricShapes::DottedCircle),
            '◍' => Ok(GeometricShapes::CircleWithVerticalFill),
            '◎' => Ok(GeometricShapes::Bullseye),
            '●' => Ok(GeometricShapes::BlackCircle),
            '◐' => Ok(GeometricShapes::CircleWithLeftHalfBlack),
            '◑' => Ok(GeometricShapes::CircleWithRightHalfBlack),
            '◒' => Ok(GeometricShapes::CircleWithLowerHalfBlack),
            '◓' => Ok(GeometricShapes::CircleWithUpperHalfBlack),
            '◔' => Ok(GeometricShapes::CircleWithUpperRightQuadrantBlack),
            '◕' => Ok(GeometricShapes::CircleWithAllButUpperLeftQuadrantBlack),
            '◖' => Ok(GeometricShapes::LeftHalfBlackCircle),
            '◗' => Ok(GeometricShapes::RightHalfBlackCircle),
            '◘' => Ok(GeometricShapes::InverseBullet),
            '◙' => Ok(GeometricShapes::InverseWhiteCircle),
            '◚' => Ok(GeometricShapes::UpperHalfInverseWhiteCircle),
            '◛' => Ok(GeometricShapes::LowerHalfInverseWhiteCircle),
            '◜' => Ok(GeometricShapes::UpperLeftQuadrantCircularArc),
            '◝' => Ok(GeometricShapes::UpperRightQuadrantCircularArc),
            '◞' => Ok(GeometricShapes::LowerRightQuadrantCircularArc),
            '◟' => Ok(GeometricShapes::LowerLeftQuadrantCircularArc),
            '◠' => Ok(GeometricShapes::UpperHalfCircle),
            '◡' => Ok(GeometricShapes::LowerHalfCircle),
            '◢' => Ok(GeometricShapes::BlackLowerRightTriangle),
            '◣' => Ok(GeometricShapes::BlackLowerLeftTriangle),
            '◤' => Ok(GeometricShapes::BlackUpperLeftTriangle),
            '◥' => Ok(GeometricShapes::BlackUpperRightTriangle),
            '◦' => Ok(GeometricShapes::WhiteBullet),
            '◧' => Ok(GeometricShapes::SquareWithLeftHalfBlack),
            '◨' => Ok(GeometricShapes::SquareWithRightHalfBlack),
            '◩' => Ok(GeometricShapes::SquareWithUpperLeftDiagonalHalfBlack),
            '◪' => Ok(GeometricShapes::SquareWithLowerRightDiagonalHalfBlack),
            '◫' => Ok(GeometricShapes::WhiteSquareWithVerticalBisectingLine),
            '◬' => Ok(GeometricShapes::WhiteUpDashPointingTriangleWithDot),
            '◭' => Ok(GeometricShapes::UpDashPointingTriangleWithLeftHalfBlack),
            '◮' => Ok(GeometricShapes::UpDashPointingTriangleWithRightHalfBlack),
            '◯' => Ok(GeometricShapes::LargeCircle),
            '◰' => Ok(GeometricShapes::WhiteSquareWithUpperLeftQuadrant),
            '◱' => Ok(GeometricShapes::WhiteSquareWithLowerLeftQuadrant),
            '◲' => Ok(GeometricShapes::WhiteSquareWithLowerRightQuadrant),
            '◳' => Ok(GeometricShapes::WhiteSquareWithUpperRightQuadrant),
            '◴' => Ok(GeometricShapes::WhiteCircleWithUpperLeftQuadrant),
            '◵' => Ok(GeometricShapes::WhiteCircleWithLowerLeftQuadrant),
            '◶' => Ok(GeometricShapes::WhiteCircleWithLowerRightQuadrant),
            '◷' => Ok(GeometricShapes::WhiteCircleWithUpperRightQuadrant),
            '◸' => Ok(GeometricShapes::UpperLeftTriangle),
            '◹' => Ok(GeometricShapes::UpperRightTriangle),
            '◺' => Ok(GeometricShapes::LowerLeftTriangle),
            '◻' => Ok(GeometricShapes::WhiteMediumSquare),
            '◼' => Ok(GeometricShapes::BlackMediumSquare),
            '◽' => Ok(GeometricShapes::WhiteMediumSmallSquare),
            '◾' => Ok(GeometricShapes::BlackMediumSmallSquare),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("GeometricShapes{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
