
/// An enum to represent all characters in the GeometricShapesExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GeometricShapesExtended {
    /// \u{1f780}: '🞀'
    BlackLeftDashPointingIsoscelesRightTriangle,
    /// \u{1f781}: '🞁'
    BlackUpDashPointingIsoscelesRightTriangle,
    /// \u{1f782}: '🞂'
    BlackRightDashPointingIsoscelesRightTriangle,
    /// \u{1f783}: '🞃'
    BlackDownDashPointingIsoscelesRightTriangle,
    /// \u{1f784}: '🞄'
    BlackSlightlySmallCircle,
    /// \u{1f785}: '🞅'
    MediumBoldWhiteCircle,
    /// \u{1f786}: '🞆'
    BoldWhiteCircle,
    /// \u{1f787}: '🞇'
    HeavyWhiteCircle,
    /// \u{1f788}: '🞈'
    VeryHeavyWhiteCircle,
    /// \u{1f789}: '🞉'
    ExtremelyHeavyWhiteCircle,
    /// \u{1f78a}: '🞊'
    WhiteCircleContainingBlackSmallCircle,
    /// \u{1f78b}: '🞋'
    RoundTarget,
    /// \u{1f78c}: '🞌'
    BlackTinySquare,
    /// \u{1f78d}: '🞍'
    BlackSlightlySmallSquare,
    /// \u{1f78e}: '🞎'
    LightWhiteSquare,
    /// \u{1f78f}: '🞏'
    MediumWhiteSquare,
    /// \u{1f790}: '🞐'
    BoldWhiteSquare,
    /// \u{1f791}: '🞑'
    HeavyWhiteSquare,
    /// \u{1f792}: '🞒'
    VeryHeavyWhiteSquare,
    /// \u{1f793}: '🞓'
    ExtremelyHeavyWhiteSquare,
    /// \u{1f794}: '🞔'
    WhiteSquareContainingBlackVerySmallSquare,
    /// \u{1f795}: '🞕'
    WhiteSquareContainingBlackMediumSquare,
    /// \u{1f796}: '🞖'
    SquareTarget,
    /// \u{1f797}: '🞗'
    BlackTinyDiamond,
    /// \u{1f798}: '🞘'
    BlackVerySmallDiamond,
    /// \u{1f799}: '🞙'
    BlackMediumSmallDiamond,
    /// \u{1f79a}: '🞚'
    WhiteDiamondContainingBlackVerySmallDiamond,
    /// \u{1f79b}: '🞛'
    WhiteDiamondContainingBlackMediumDiamond,
    /// \u{1f79c}: '🞜'
    DiamondTarget,
    /// \u{1f79d}: '🞝'
    BlackTinyLozenge,
    /// \u{1f79e}: '🞞'
    BlackVerySmallLozenge,
    /// \u{1f79f}: '🞟'
    BlackMediumSmallLozenge,
    /// \u{1f7a0}: '🞠'
    WhiteLozengeContainingBlackSmallLozenge,
    /// \u{1f7a1}: '🞡'
    ThinGreekCross,
    /// \u{1f7a2}: '🞢'
    LightGreekCross,
    /// \u{1f7a3}: '🞣'
    MediumGreekCross,
    /// \u{1f7a4}: '🞤'
    BoldGreekCross,
    /// \u{1f7a5}: '🞥'
    VeryBoldGreekCross,
    /// \u{1f7a6}: '🞦'
    VeryHeavyGreekCross,
    /// \u{1f7a7}: '🞧'
    ExtremelyHeavyGreekCross,
    /// \u{1f7a8}: '🞨'
    ThinSaltire,
    /// \u{1f7a9}: '🞩'
    LightSaltire,
    /// \u{1f7aa}: '🞪'
    MediumSaltire,
    /// \u{1f7ab}: '🞫'
    BoldSaltire,
    /// \u{1f7ac}: '🞬'
    HeavySaltire,
    /// \u{1f7ad}: '🞭'
    VeryHeavySaltire,
    /// \u{1f7ae}: '🞮'
    ExtremelyHeavySaltire,
    /// \u{1f7af}: '🞯'
    LightFiveSpokedAsterisk,
    /// \u{1f7b0}: '🞰'
    MediumFiveSpokedAsterisk,
    /// \u{1f7b1}: '🞱'
    BoldFiveSpokedAsterisk,
    /// \u{1f7b2}: '🞲'
    HeavyFiveSpokedAsterisk,
    /// \u{1f7b3}: '🞳'
    VeryHeavyFiveSpokedAsterisk,
    /// \u{1f7b4}: '🞴'
    ExtremelyHeavyFiveSpokedAsterisk,
    /// \u{1f7b5}: '🞵'
    LightSixSpokedAsterisk,
    /// \u{1f7b6}: '🞶'
    MediumSixSpokedAsterisk,
    /// \u{1f7b7}: '🞷'
    BoldSixSpokedAsterisk,
    /// \u{1f7b8}: '🞸'
    HeavySixSpokedAsterisk,
    /// \u{1f7b9}: '🞹'
    VeryHeavySixSpokedAsterisk,
    /// \u{1f7ba}: '🞺'
    ExtremelyHeavySixSpokedAsterisk,
    /// \u{1f7bb}: '🞻'
    LightEightSpokedAsterisk,
    /// \u{1f7bc}: '🞼'
    MediumEightSpokedAsterisk,
    /// \u{1f7bd}: '🞽'
    BoldEightSpokedAsterisk,
    /// \u{1f7be}: '🞾'
    HeavyEightSpokedAsterisk,
    /// \u{1f7bf}: '🞿'
    VeryHeavyEightSpokedAsterisk,
    /// \u{1f7c0}: '🟀'
    LightThreePointedBlackStar,
    /// \u{1f7c1}: '🟁'
    MediumThreePointedBlackStar,
    /// \u{1f7c2}: '🟂'
    ThreePointedBlackStar,
    /// \u{1f7c3}: '🟃'
    MediumThreePointedPinwheelStar,
    /// \u{1f7c4}: '🟄'
    LightFourPointedBlackStar,
    /// \u{1f7c5}: '🟅'
    MediumFourPointedBlackStar,
    /// \u{1f7c6}: '🟆'
    FourPointedBlackStar,
    /// \u{1f7c7}: '🟇'
    MediumFourPointedPinwheelStar,
    /// \u{1f7c8}: '🟈'
    ReverseLightFourPointedPinwheelStar,
    /// \u{1f7c9}: '🟉'
    LightFivePointedBlackStar,
    /// \u{1f7ca}: '🟊'
    HeavyFivePointedBlackStar,
    /// \u{1f7cb}: '🟋'
    MediumSixPointedBlackStar,
    /// \u{1f7cc}: '🟌'
    HeavySixPointedBlackStar,
    /// \u{1f7cd}: '🟍'
    SixPointedPinwheelStar,
    /// \u{1f7ce}: '🟎'
    MediumEightPointedBlackStar,
    /// \u{1f7cf}: '🟏'
    HeavyEightPointedBlackStar,
    /// \u{1f7d0}: '🟐'
    VeryHeavyEightPointedBlackStar,
    /// \u{1f7d1}: '🟑'
    HeavyEightPointedPinwheelStar,
    /// \u{1f7d2}: '🟒'
    LightTwelvePointedBlackStar,
    /// \u{1f7d3}: '🟓'
    HeavyTwelvePointedBlackStar,
    /// \u{1f7d4}: '🟔'
    HeavyTwelvePointedPinwheelStar,
    /// \u{1f7d5}: '🟕'
    CircledTriangle,
    /// \u{1f7d6}: '🟖'
    NegativeCircledTriangle,
    /// \u{1f7d7}: '🟗'
    CircledSquare,
    /// \u{1f7d8}: '🟘'
    NegativeCircledSquare,
    /// \u{1f7e0}: '🟠'
    LargeOrangeCircle,
    /// \u{1f7e1}: '🟡'
    LargeYellowCircle,
    /// \u{1f7e2}: '🟢'
    LargeGreenCircle,
    /// \u{1f7e3}: '🟣'
    LargePurpleCircle,
    /// \u{1f7e4}: '🟤'
    LargeBrownCircle,
    /// \u{1f7e5}: '🟥'
    LargeRedSquare,
    /// \u{1f7e6}: '🟦'
    LargeBlueSquare,
    /// \u{1f7e7}: '🟧'
    LargeOrangeSquare,
    /// \u{1f7e8}: '🟨'
    LargeYellowSquare,
    /// \u{1f7e9}: '🟩'
    LargeGreenSquare,
    /// \u{1f7ea}: '🟪'
    LargePurpleSquare,
    /// \u{1f7eb}: '🟫'
    LargeBrownSquare,
}

impl Into<char> for GeometricShapesExtended {
    fn into(self) -> char {
        match self {
            GeometricShapesExtended::BlackLeftDashPointingIsoscelesRightTriangle => '🞀',
            GeometricShapesExtended::BlackUpDashPointingIsoscelesRightTriangle => '🞁',
            GeometricShapesExtended::BlackRightDashPointingIsoscelesRightTriangle => '🞂',
            GeometricShapesExtended::BlackDownDashPointingIsoscelesRightTriangle => '🞃',
            GeometricShapesExtended::BlackSlightlySmallCircle => '🞄',
            GeometricShapesExtended::MediumBoldWhiteCircle => '🞅',
            GeometricShapesExtended::BoldWhiteCircle => '🞆',
            GeometricShapesExtended::HeavyWhiteCircle => '🞇',
            GeometricShapesExtended::VeryHeavyWhiteCircle => '🞈',
            GeometricShapesExtended::ExtremelyHeavyWhiteCircle => '🞉',
            GeometricShapesExtended::WhiteCircleContainingBlackSmallCircle => '🞊',
            GeometricShapesExtended::RoundTarget => '🞋',
            GeometricShapesExtended::BlackTinySquare => '🞌',
            GeometricShapesExtended::BlackSlightlySmallSquare => '🞍',
            GeometricShapesExtended::LightWhiteSquare => '🞎',
            GeometricShapesExtended::MediumWhiteSquare => '🞏',
            GeometricShapesExtended::BoldWhiteSquare => '🞐',
            GeometricShapesExtended::HeavyWhiteSquare => '🞑',
            GeometricShapesExtended::VeryHeavyWhiteSquare => '🞒',
            GeometricShapesExtended::ExtremelyHeavyWhiteSquare => '🞓',
            GeometricShapesExtended::WhiteSquareContainingBlackVerySmallSquare => '🞔',
            GeometricShapesExtended::WhiteSquareContainingBlackMediumSquare => '🞕',
            GeometricShapesExtended::SquareTarget => '🞖',
            GeometricShapesExtended::BlackTinyDiamond => '🞗',
            GeometricShapesExtended::BlackVerySmallDiamond => '🞘',
            GeometricShapesExtended::BlackMediumSmallDiamond => '🞙',
            GeometricShapesExtended::WhiteDiamondContainingBlackVerySmallDiamond => '🞚',
            GeometricShapesExtended::WhiteDiamondContainingBlackMediumDiamond => '🞛',
            GeometricShapesExtended::DiamondTarget => '🞜',
            GeometricShapesExtended::BlackTinyLozenge => '🞝',
            GeometricShapesExtended::BlackVerySmallLozenge => '🞞',
            GeometricShapesExtended::BlackMediumSmallLozenge => '🞟',
            GeometricShapesExtended::WhiteLozengeContainingBlackSmallLozenge => '🞠',
            GeometricShapesExtended::ThinGreekCross => '🞡',
            GeometricShapesExtended::LightGreekCross => '🞢',
            GeometricShapesExtended::MediumGreekCross => '🞣',
            GeometricShapesExtended::BoldGreekCross => '🞤',
            GeometricShapesExtended::VeryBoldGreekCross => '🞥',
            GeometricShapesExtended::VeryHeavyGreekCross => '🞦',
            GeometricShapesExtended::ExtremelyHeavyGreekCross => '🞧',
            GeometricShapesExtended::ThinSaltire => '🞨',
            GeometricShapesExtended::LightSaltire => '🞩',
            GeometricShapesExtended::MediumSaltire => '🞪',
            GeometricShapesExtended::BoldSaltire => '🞫',
            GeometricShapesExtended::HeavySaltire => '🞬',
            GeometricShapesExtended::VeryHeavySaltire => '🞭',
            GeometricShapesExtended::ExtremelyHeavySaltire => '🞮',
            GeometricShapesExtended::LightFiveSpokedAsterisk => '🞯',
            GeometricShapesExtended::MediumFiveSpokedAsterisk => '🞰',
            GeometricShapesExtended::BoldFiveSpokedAsterisk => '🞱',
            GeometricShapesExtended::HeavyFiveSpokedAsterisk => '🞲',
            GeometricShapesExtended::VeryHeavyFiveSpokedAsterisk => '🞳',
            GeometricShapesExtended::ExtremelyHeavyFiveSpokedAsterisk => '🞴',
            GeometricShapesExtended::LightSixSpokedAsterisk => '🞵',
            GeometricShapesExtended::MediumSixSpokedAsterisk => '🞶',
            GeometricShapesExtended::BoldSixSpokedAsterisk => '🞷',
            GeometricShapesExtended::HeavySixSpokedAsterisk => '🞸',
            GeometricShapesExtended::VeryHeavySixSpokedAsterisk => '🞹',
            GeometricShapesExtended::ExtremelyHeavySixSpokedAsterisk => '🞺',
            GeometricShapesExtended::LightEightSpokedAsterisk => '🞻',
            GeometricShapesExtended::MediumEightSpokedAsterisk => '🞼',
            GeometricShapesExtended::BoldEightSpokedAsterisk => '🞽',
            GeometricShapesExtended::HeavyEightSpokedAsterisk => '🞾',
            GeometricShapesExtended::VeryHeavyEightSpokedAsterisk => '🞿',
            GeometricShapesExtended::LightThreePointedBlackStar => '🟀',
            GeometricShapesExtended::MediumThreePointedBlackStar => '🟁',
            GeometricShapesExtended::ThreePointedBlackStar => '🟂',
            GeometricShapesExtended::MediumThreePointedPinwheelStar => '🟃',
            GeometricShapesExtended::LightFourPointedBlackStar => '🟄',
            GeometricShapesExtended::MediumFourPointedBlackStar => '🟅',
            GeometricShapesExtended::FourPointedBlackStar => '🟆',
            GeometricShapesExtended::MediumFourPointedPinwheelStar => '🟇',
            GeometricShapesExtended::ReverseLightFourPointedPinwheelStar => '🟈',
            GeometricShapesExtended::LightFivePointedBlackStar => '🟉',
            GeometricShapesExtended::HeavyFivePointedBlackStar => '🟊',
            GeometricShapesExtended::MediumSixPointedBlackStar => '🟋',
            GeometricShapesExtended::HeavySixPointedBlackStar => '🟌',
            GeometricShapesExtended::SixPointedPinwheelStar => '🟍',
            GeometricShapesExtended::MediumEightPointedBlackStar => '🟎',
            GeometricShapesExtended::HeavyEightPointedBlackStar => '🟏',
            GeometricShapesExtended::VeryHeavyEightPointedBlackStar => '🟐',
            GeometricShapesExtended::HeavyEightPointedPinwheelStar => '🟑',
            GeometricShapesExtended::LightTwelvePointedBlackStar => '🟒',
            GeometricShapesExtended::HeavyTwelvePointedBlackStar => '🟓',
            GeometricShapesExtended::HeavyTwelvePointedPinwheelStar => '🟔',
            GeometricShapesExtended::CircledTriangle => '🟕',
            GeometricShapesExtended::NegativeCircledTriangle => '🟖',
            GeometricShapesExtended::CircledSquare => '🟗',
            GeometricShapesExtended::NegativeCircledSquare => '🟘',
            GeometricShapesExtended::LargeOrangeCircle => '🟠',
            GeometricShapesExtended::LargeYellowCircle => '🟡',
            GeometricShapesExtended::LargeGreenCircle => '🟢',
            GeometricShapesExtended::LargePurpleCircle => '🟣',
            GeometricShapesExtended::LargeBrownCircle => '🟤',
            GeometricShapesExtended::LargeRedSquare => '🟥',
            GeometricShapesExtended::LargeBlueSquare => '🟦',
            GeometricShapesExtended::LargeOrangeSquare => '🟧',
            GeometricShapesExtended::LargeYellowSquare => '🟨',
            GeometricShapesExtended::LargeGreenSquare => '🟩',
            GeometricShapesExtended::LargePurpleSquare => '🟪',
            GeometricShapesExtended::LargeBrownSquare => '🟫',
        }
    }
}

impl std::convert::TryFrom<char> for GeometricShapesExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🞀' => Ok(GeometricShapesExtended::BlackLeftDashPointingIsoscelesRightTriangle),
            '🞁' => Ok(GeometricShapesExtended::BlackUpDashPointingIsoscelesRightTriangle),
            '🞂' => Ok(GeometricShapesExtended::BlackRightDashPointingIsoscelesRightTriangle),
            '🞃' => Ok(GeometricShapesExtended::BlackDownDashPointingIsoscelesRightTriangle),
            '🞄' => Ok(GeometricShapesExtended::BlackSlightlySmallCircle),
            '🞅' => Ok(GeometricShapesExtended::MediumBoldWhiteCircle),
            '🞆' => Ok(GeometricShapesExtended::BoldWhiteCircle),
            '🞇' => Ok(GeometricShapesExtended::HeavyWhiteCircle),
            '🞈' => Ok(GeometricShapesExtended::VeryHeavyWhiteCircle),
            '🞉' => Ok(GeometricShapesExtended::ExtremelyHeavyWhiteCircle),
            '🞊' => Ok(GeometricShapesExtended::WhiteCircleContainingBlackSmallCircle),
            '🞋' => Ok(GeometricShapesExtended::RoundTarget),
            '🞌' => Ok(GeometricShapesExtended::BlackTinySquare),
            '🞍' => Ok(GeometricShapesExtended::BlackSlightlySmallSquare),
            '🞎' => Ok(GeometricShapesExtended::LightWhiteSquare),
            '🞏' => Ok(GeometricShapesExtended::MediumWhiteSquare),
            '🞐' => Ok(GeometricShapesExtended::BoldWhiteSquare),
            '🞑' => Ok(GeometricShapesExtended::HeavyWhiteSquare),
            '🞒' => Ok(GeometricShapesExtended::VeryHeavyWhiteSquare),
            '🞓' => Ok(GeometricShapesExtended::ExtremelyHeavyWhiteSquare),
            '🞔' => Ok(GeometricShapesExtended::WhiteSquareContainingBlackVerySmallSquare),
            '🞕' => Ok(GeometricShapesExtended::WhiteSquareContainingBlackMediumSquare),
            '🞖' => Ok(GeometricShapesExtended::SquareTarget),
            '🞗' => Ok(GeometricShapesExtended::BlackTinyDiamond),
            '🞘' => Ok(GeometricShapesExtended::BlackVerySmallDiamond),
            '🞙' => Ok(GeometricShapesExtended::BlackMediumSmallDiamond),
            '🞚' => Ok(GeometricShapesExtended::WhiteDiamondContainingBlackVerySmallDiamond),
            '🞛' => Ok(GeometricShapesExtended::WhiteDiamondContainingBlackMediumDiamond),
            '🞜' => Ok(GeometricShapesExtended::DiamondTarget),
            '🞝' => Ok(GeometricShapesExtended::BlackTinyLozenge),
            '🞞' => Ok(GeometricShapesExtended::BlackVerySmallLozenge),
            '🞟' => Ok(GeometricShapesExtended::BlackMediumSmallLozenge),
            '🞠' => Ok(GeometricShapesExtended::WhiteLozengeContainingBlackSmallLozenge),
            '🞡' => Ok(GeometricShapesExtended::ThinGreekCross),
            '🞢' => Ok(GeometricShapesExtended::LightGreekCross),
            '🞣' => Ok(GeometricShapesExtended::MediumGreekCross),
            '🞤' => Ok(GeometricShapesExtended::BoldGreekCross),
            '🞥' => Ok(GeometricShapesExtended::VeryBoldGreekCross),
            '🞦' => Ok(GeometricShapesExtended::VeryHeavyGreekCross),
            '🞧' => Ok(GeometricShapesExtended::ExtremelyHeavyGreekCross),
            '🞨' => Ok(GeometricShapesExtended::ThinSaltire),
            '🞩' => Ok(GeometricShapesExtended::LightSaltire),
            '🞪' => Ok(GeometricShapesExtended::MediumSaltire),
            '🞫' => Ok(GeometricShapesExtended::BoldSaltire),
            '🞬' => Ok(GeometricShapesExtended::HeavySaltire),
            '🞭' => Ok(GeometricShapesExtended::VeryHeavySaltire),
            '🞮' => Ok(GeometricShapesExtended::ExtremelyHeavySaltire),
            '🞯' => Ok(GeometricShapesExtended::LightFiveSpokedAsterisk),
            '🞰' => Ok(GeometricShapesExtended::MediumFiveSpokedAsterisk),
            '🞱' => Ok(GeometricShapesExtended::BoldFiveSpokedAsterisk),
            '🞲' => Ok(GeometricShapesExtended::HeavyFiveSpokedAsterisk),
            '🞳' => Ok(GeometricShapesExtended::VeryHeavyFiveSpokedAsterisk),
            '🞴' => Ok(GeometricShapesExtended::ExtremelyHeavyFiveSpokedAsterisk),
            '🞵' => Ok(GeometricShapesExtended::LightSixSpokedAsterisk),
            '🞶' => Ok(GeometricShapesExtended::MediumSixSpokedAsterisk),
            '🞷' => Ok(GeometricShapesExtended::BoldSixSpokedAsterisk),
            '🞸' => Ok(GeometricShapesExtended::HeavySixSpokedAsterisk),
            '🞹' => Ok(GeometricShapesExtended::VeryHeavySixSpokedAsterisk),
            '🞺' => Ok(GeometricShapesExtended::ExtremelyHeavySixSpokedAsterisk),
            '🞻' => Ok(GeometricShapesExtended::LightEightSpokedAsterisk),
            '🞼' => Ok(GeometricShapesExtended::MediumEightSpokedAsterisk),
            '🞽' => Ok(GeometricShapesExtended::BoldEightSpokedAsterisk),
            '🞾' => Ok(GeometricShapesExtended::HeavyEightSpokedAsterisk),
            '🞿' => Ok(GeometricShapesExtended::VeryHeavyEightSpokedAsterisk),
            '🟀' => Ok(GeometricShapesExtended::LightThreePointedBlackStar),
            '🟁' => Ok(GeometricShapesExtended::MediumThreePointedBlackStar),
            '🟂' => Ok(GeometricShapesExtended::ThreePointedBlackStar),
            '🟃' => Ok(GeometricShapesExtended::MediumThreePointedPinwheelStar),
            '🟄' => Ok(GeometricShapesExtended::LightFourPointedBlackStar),
            '🟅' => Ok(GeometricShapesExtended::MediumFourPointedBlackStar),
            '🟆' => Ok(GeometricShapesExtended::FourPointedBlackStar),
            '🟇' => Ok(GeometricShapesExtended::MediumFourPointedPinwheelStar),
            '🟈' => Ok(GeometricShapesExtended::ReverseLightFourPointedPinwheelStar),
            '🟉' => Ok(GeometricShapesExtended::LightFivePointedBlackStar),
            '🟊' => Ok(GeometricShapesExtended::HeavyFivePointedBlackStar),
            '🟋' => Ok(GeometricShapesExtended::MediumSixPointedBlackStar),
            '🟌' => Ok(GeometricShapesExtended::HeavySixPointedBlackStar),
            '🟍' => Ok(GeometricShapesExtended::SixPointedPinwheelStar),
            '🟎' => Ok(GeometricShapesExtended::MediumEightPointedBlackStar),
            '🟏' => Ok(GeometricShapesExtended::HeavyEightPointedBlackStar),
            '🟐' => Ok(GeometricShapesExtended::VeryHeavyEightPointedBlackStar),
            '🟑' => Ok(GeometricShapesExtended::HeavyEightPointedPinwheelStar),
            '🟒' => Ok(GeometricShapesExtended::LightTwelvePointedBlackStar),
            '🟓' => Ok(GeometricShapesExtended::HeavyTwelvePointedBlackStar),
            '🟔' => Ok(GeometricShapesExtended::HeavyTwelvePointedPinwheelStar),
            '🟕' => Ok(GeometricShapesExtended::CircledTriangle),
            '🟖' => Ok(GeometricShapesExtended::NegativeCircledTriangle),
            '🟗' => Ok(GeometricShapesExtended::CircledSquare),
            '🟘' => Ok(GeometricShapesExtended::NegativeCircledSquare),
            '🟠' => Ok(GeometricShapesExtended::LargeOrangeCircle),
            '🟡' => Ok(GeometricShapesExtended::LargeYellowCircle),
            '🟢' => Ok(GeometricShapesExtended::LargeGreenCircle),
            '🟣' => Ok(GeometricShapesExtended::LargePurpleCircle),
            '🟤' => Ok(GeometricShapesExtended::LargeBrownCircle),
            '🟥' => Ok(GeometricShapesExtended::LargeRedSquare),
            '🟦' => Ok(GeometricShapesExtended::LargeBlueSquare),
            '🟧' => Ok(GeometricShapesExtended::LargeOrangeSquare),
            '🟨' => Ok(GeometricShapesExtended::LargeYellowSquare),
            '🟩' => Ok(GeometricShapesExtended::LargeGreenSquare),
            '🟪' => Ok(GeometricShapesExtended::LargePurpleSquare),
            '🟫' => Ok(GeometricShapesExtended::LargeBrownSquare),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GeometricShapesExtended {
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

impl std::convert::TryFrom<u32> for GeometricShapesExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GeometricShapesExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GeometricShapesExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GeometricShapesExtended::BlackLeftDashPointingIsoscelesRightTriangle
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GeometricShapesExtended::BlackLeftDashPointingIsoscelesRightTriangle => "black left-pointing isosceles right triangle",
            GeometricShapesExtended::BlackUpDashPointingIsoscelesRightTriangle => "black up-pointing isosceles right triangle",
            GeometricShapesExtended::BlackRightDashPointingIsoscelesRightTriangle => "black right-pointing isosceles right triangle",
            GeometricShapesExtended::BlackDownDashPointingIsoscelesRightTriangle => "black down-pointing isosceles right triangle",
            GeometricShapesExtended::BlackSlightlySmallCircle => "black slightly small circle",
            GeometricShapesExtended::MediumBoldWhiteCircle => "medium bold white circle",
            GeometricShapesExtended::BoldWhiteCircle => "bold white circle",
            GeometricShapesExtended::HeavyWhiteCircle => "heavy white circle",
            GeometricShapesExtended::VeryHeavyWhiteCircle => "very heavy white circle",
            GeometricShapesExtended::ExtremelyHeavyWhiteCircle => "extremely heavy white circle",
            GeometricShapesExtended::WhiteCircleContainingBlackSmallCircle => "white circle containing black small circle",
            GeometricShapesExtended::RoundTarget => "round target",
            GeometricShapesExtended::BlackTinySquare => "black tiny square",
            GeometricShapesExtended::BlackSlightlySmallSquare => "black slightly small square",
            GeometricShapesExtended::LightWhiteSquare => "light white square",
            GeometricShapesExtended::MediumWhiteSquare => "medium white square",
            GeometricShapesExtended::BoldWhiteSquare => "bold white square",
            GeometricShapesExtended::HeavyWhiteSquare => "heavy white square",
            GeometricShapesExtended::VeryHeavyWhiteSquare => "very heavy white square",
            GeometricShapesExtended::ExtremelyHeavyWhiteSquare => "extremely heavy white square",
            GeometricShapesExtended::WhiteSquareContainingBlackVerySmallSquare => "white square containing black very small square",
            GeometricShapesExtended::WhiteSquareContainingBlackMediumSquare => "white square containing black medium square",
            GeometricShapesExtended::SquareTarget => "square target",
            GeometricShapesExtended::BlackTinyDiamond => "black tiny diamond",
            GeometricShapesExtended::BlackVerySmallDiamond => "black very small diamond",
            GeometricShapesExtended::BlackMediumSmallDiamond => "black medium small diamond",
            GeometricShapesExtended::WhiteDiamondContainingBlackVerySmallDiamond => "white diamond containing black very small diamond",
            GeometricShapesExtended::WhiteDiamondContainingBlackMediumDiamond => "white diamond containing black medium diamond",
            GeometricShapesExtended::DiamondTarget => "diamond target",
            GeometricShapesExtended::BlackTinyLozenge => "black tiny lozenge",
            GeometricShapesExtended::BlackVerySmallLozenge => "black very small lozenge",
            GeometricShapesExtended::BlackMediumSmallLozenge => "black medium small lozenge",
            GeometricShapesExtended::WhiteLozengeContainingBlackSmallLozenge => "white lozenge containing black small lozenge",
            GeometricShapesExtended::ThinGreekCross => "thin greek cross",
            GeometricShapesExtended::LightGreekCross => "light greek cross",
            GeometricShapesExtended::MediumGreekCross => "medium greek cross",
            GeometricShapesExtended::BoldGreekCross => "bold greek cross",
            GeometricShapesExtended::VeryBoldGreekCross => "very bold greek cross",
            GeometricShapesExtended::VeryHeavyGreekCross => "very heavy greek cross",
            GeometricShapesExtended::ExtremelyHeavyGreekCross => "extremely heavy greek cross",
            GeometricShapesExtended::ThinSaltire => "thin saltire",
            GeometricShapesExtended::LightSaltire => "light saltire",
            GeometricShapesExtended::MediumSaltire => "medium saltire",
            GeometricShapesExtended::BoldSaltire => "bold saltire",
            GeometricShapesExtended::HeavySaltire => "heavy saltire",
            GeometricShapesExtended::VeryHeavySaltire => "very heavy saltire",
            GeometricShapesExtended::ExtremelyHeavySaltire => "extremely heavy saltire",
            GeometricShapesExtended::LightFiveSpokedAsterisk => "light five spoked asterisk",
            GeometricShapesExtended::MediumFiveSpokedAsterisk => "medium five spoked asterisk",
            GeometricShapesExtended::BoldFiveSpokedAsterisk => "bold five spoked asterisk",
            GeometricShapesExtended::HeavyFiveSpokedAsterisk => "heavy five spoked asterisk",
            GeometricShapesExtended::VeryHeavyFiveSpokedAsterisk => "very heavy five spoked asterisk",
            GeometricShapesExtended::ExtremelyHeavyFiveSpokedAsterisk => "extremely heavy five spoked asterisk",
            GeometricShapesExtended::LightSixSpokedAsterisk => "light six spoked asterisk",
            GeometricShapesExtended::MediumSixSpokedAsterisk => "medium six spoked asterisk",
            GeometricShapesExtended::BoldSixSpokedAsterisk => "bold six spoked asterisk",
            GeometricShapesExtended::HeavySixSpokedAsterisk => "heavy six spoked asterisk",
            GeometricShapesExtended::VeryHeavySixSpokedAsterisk => "very heavy six spoked asterisk",
            GeometricShapesExtended::ExtremelyHeavySixSpokedAsterisk => "extremely heavy six spoked asterisk",
            GeometricShapesExtended::LightEightSpokedAsterisk => "light eight spoked asterisk",
            GeometricShapesExtended::MediumEightSpokedAsterisk => "medium eight spoked asterisk",
            GeometricShapesExtended::BoldEightSpokedAsterisk => "bold eight spoked asterisk",
            GeometricShapesExtended::HeavyEightSpokedAsterisk => "heavy eight spoked asterisk",
            GeometricShapesExtended::VeryHeavyEightSpokedAsterisk => "very heavy eight spoked asterisk",
            GeometricShapesExtended::LightThreePointedBlackStar => "light three pointed black star",
            GeometricShapesExtended::MediumThreePointedBlackStar => "medium three pointed black star",
            GeometricShapesExtended::ThreePointedBlackStar => "three pointed black star",
            GeometricShapesExtended::MediumThreePointedPinwheelStar => "medium three pointed pinwheel star",
            GeometricShapesExtended::LightFourPointedBlackStar => "light four pointed black star",
            GeometricShapesExtended::MediumFourPointedBlackStar => "medium four pointed black star",
            GeometricShapesExtended::FourPointedBlackStar => "four pointed black star",
            GeometricShapesExtended::MediumFourPointedPinwheelStar => "medium four pointed pinwheel star",
            GeometricShapesExtended::ReverseLightFourPointedPinwheelStar => "reverse light four pointed pinwheel star",
            GeometricShapesExtended::LightFivePointedBlackStar => "light five pointed black star",
            GeometricShapesExtended::HeavyFivePointedBlackStar => "heavy five pointed black star",
            GeometricShapesExtended::MediumSixPointedBlackStar => "medium six pointed black star",
            GeometricShapesExtended::HeavySixPointedBlackStar => "heavy six pointed black star",
            GeometricShapesExtended::SixPointedPinwheelStar => "six pointed pinwheel star",
            GeometricShapesExtended::MediumEightPointedBlackStar => "medium eight pointed black star",
            GeometricShapesExtended::HeavyEightPointedBlackStar => "heavy eight pointed black star",
            GeometricShapesExtended::VeryHeavyEightPointedBlackStar => "very heavy eight pointed black star",
            GeometricShapesExtended::HeavyEightPointedPinwheelStar => "heavy eight pointed pinwheel star",
            GeometricShapesExtended::LightTwelvePointedBlackStar => "light twelve pointed black star",
            GeometricShapesExtended::HeavyTwelvePointedBlackStar => "heavy twelve pointed black star",
            GeometricShapesExtended::HeavyTwelvePointedPinwheelStar => "heavy twelve pointed pinwheel star",
            GeometricShapesExtended::CircledTriangle => "circled triangle",
            GeometricShapesExtended::NegativeCircledTriangle => "negative circled triangle",
            GeometricShapesExtended::CircledSquare => "circled square",
            GeometricShapesExtended::NegativeCircledSquare => "negative circled square",
            GeometricShapesExtended::LargeOrangeCircle => "large orange circle",
            GeometricShapesExtended::LargeYellowCircle => "large yellow circle",
            GeometricShapesExtended::LargeGreenCircle => "large green circle",
            GeometricShapesExtended::LargePurpleCircle => "large purple circle",
            GeometricShapesExtended::LargeBrownCircle => "large brown circle",
            GeometricShapesExtended::LargeRedSquare => "large red square",
            GeometricShapesExtended::LargeBlueSquare => "large blue square",
            GeometricShapesExtended::LargeOrangeSquare => "large orange square",
            GeometricShapesExtended::LargeYellowSquare => "large yellow square",
            GeometricShapesExtended::LargeGreenSquare => "large green square",
            GeometricShapesExtended::LargePurpleSquare => "large purple square",
            GeometricShapesExtended::LargeBrownSquare => "large brown square",
        }
    }
}
