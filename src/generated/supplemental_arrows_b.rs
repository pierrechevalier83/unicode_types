
/// An enum to represent all characters in the SupplementalArrowsB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SupplementalArrowsB {
    /// \u{2900}: '⤀'
    RightwardsTwoDashHeadedArrowWithVerticalStroke,
    /// \u{2901}: '⤁'
    RightwardsTwoDashHeadedArrowWithDoubleVerticalStroke,
    /// \u{2902}: '⤂'
    LeftwardsDoubleArrowWithVerticalStroke,
    /// \u{2903}: '⤃'
    RightwardsDoubleArrowWithVerticalStroke,
    /// \u{2904}: '⤄'
    LeftRightDoubleArrowWithVerticalStroke,
    /// \u{2905}: '⤅'
    RightwardsTwoDashHeadedArrowFromBar,
    /// \u{2906}: '⤆'
    LeftwardsDoubleArrowFromBar,
    /// \u{2907}: '⤇'
    RightwardsDoubleArrowFromBar,
    /// \u{2908}: '⤈'
    DownwardsArrowWithHorizontalStroke,
    /// \u{2909}: '⤉'
    UpwardsArrowWithHorizontalStroke,
    /// \u{290a}: '⤊'
    UpwardsTripleArrow,
    /// \u{290b}: '⤋'
    DownwardsTripleArrow,
    /// \u{290c}: '⤌'
    LeftwardsDoubleDashArrow,
    /// \u{290d}: '⤍'
    RightwardsDoubleDashArrow,
    /// \u{290e}: '⤎'
    LeftwardsTripleDashArrow,
    /// \u{290f}: '⤏'
    RightwardsTripleDashArrow,
    /// \u{2910}: '⤐'
    RightwardsTwoDashHeadedTripleDashArrow,
    /// \u{2911}: '⤑'
    RightwardsArrowWithDottedStem,
    /// \u{2912}: '⤒'
    UpwardsArrowToBar,
    /// \u{2913}: '⤓'
    DownwardsArrowToBar,
    /// \u{2914}: '⤔'
    RightwardsArrowWithTailWithVerticalStroke,
    /// \u{2915}: '⤕'
    RightwardsArrowWithTailWithDoubleVerticalStroke,
    /// \u{2916}: '⤖'
    RightwardsTwoDashHeadedArrowWithTail,
    /// \u{2917}: '⤗'
    RightwardsTwoDashHeadedArrowWithTailWithVerticalStroke,
    /// \u{2918}: '⤘'
    RightwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke,
    /// \u{2919}: '⤙'
    LeftwardsArrowDashTail,
    /// \u{291a}: '⤚'
    RightwardsArrowDashTail,
    /// \u{291b}: '⤛'
    LeftwardsDoubleArrowDashTail,
    /// \u{291c}: '⤜'
    RightwardsDoubleArrowDashTail,
    /// \u{291d}: '⤝'
    LeftwardsArrowToBlackDiamond,
    /// \u{291e}: '⤞'
    RightwardsArrowToBlackDiamond,
    /// \u{291f}: '⤟'
    LeftwardsArrowFromBarToBlackDiamond,
    /// \u{2920}: '⤠'
    RightwardsArrowFromBarToBlackDiamond,
    /// \u{2921}: '⤡'
    NorthWestAndSouthEastArrow,
    /// \u{2922}: '⤢'
    NorthEastAndSouthWestArrow,
    /// \u{2923}: '⤣'
    NorthWestArrowWithHook,
    /// \u{2924}: '⤤'
    NorthEastArrowWithHook,
    /// \u{2925}: '⤥'
    SouthEastArrowWithHook,
    /// \u{2926}: '⤦'
    SouthWestArrowWithHook,
    /// \u{2927}: '⤧'
    NorthWestArrowAndNorthEastArrow,
    /// \u{2928}: '⤨'
    NorthEastArrowAndSouthEastArrow,
    /// \u{2929}: '⤩'
    SouthEastArrowAndSouthWestArrow,
    /// \u{292a}: '⤪'
    SouthWestArrowAndNorthWestArrow,
    /// \u{292b}: '⤫'
    RisingDiagonalCrossingFallingDiagonal,
    /// \u{292c}: '⤬'
    FallingDiagonalCrossingRisingDiagonal,
    /// \u{292d}: '⤭'
    SouthEastArrowCrossingNorthEastArrow,
    /// \u{292e}: '⤮'
    NorthEastArrowCrossingSouthEastArrow,
    /// \u{292f}: '⤯'
    FallingDiagonalCrossingNorthEastArrow,
    /// \u{2930}: '⤰'
    RisingDiagonalCrossingSouthEastArrow,
    /// \u{2931}: '⤱'
    NorthEastArrowCrossingNorthWestArrow,
    /// \u{2932}: '⤲'
    NorthWestArrowCrossingNorthEastArrow,
    /// \u{2933}: '⤳'
    WaveArrowPointingDirectlyRight,
    /// \u{2934}: '⤴'
    ArrowPointingRightwardsThenCurvingUpwards,
    /// \u{2935}: '⤵'
    ArrowPointingRightwardsThenCurvingDownwards,
    /// \u{2936}: '⤶'
    ArrowPointingDownwardsThenCurvingLeftwards,
    /// \u{2937}: '⤷'
    ArrowPointingDownwardsThenCurvingRightwards,
    /// \u{2938}: '⤸'
    RightDashSideArcClockwiseArrow,
    /// \u{2939}: '⤹'
    LeftDashSideArcAnticlockwiseArrow,
    /// \u{293a}: '⤺'
    TopArcAnticlockwiseArrow,
    /// \u{293b}: '⤻'
    BottomArcAnticlockwiseArrow,
    /// \u{293c}: '⤼'
    TopArcClockwiseArrowWithMinus,
    /// \u{293d}: '⤽'
    TopArcAnticlockwiseArrowWithPlus,
    /// \u{293e}: '⤾'
    LowerRightSemicircularClockwiseArrow,
    /// \u{293f}: '⤿'
    LowerLeftSemicircularAnticlockwiseArrow,
    /// \u{2940}: '⥀'
    AnticlockwiseClosedCircleArrow,
    /// \u{2941}: '⥁'
    ClockwiseClosedCircleArrow,
    /// \u{2942}: '⥂'
    RightwardsArrowAboveShortLeftwardsArrow,
    /// \u{2943}: '⥃'
    LeftwardsArrowAboveShortRightwardsArrow,
    /// \u{2944}: '⥄'
    ShortRightwardsArrowAboveLeftwardsArrow,
    /// \u{2945}: '⥅'
    RightwardsArrowWithPlusBelow,
    /// \u{2946}: '⥆'
    LeftwardsArrowWithPlusBelow,
    /// \u{2947}: '⥇'
    RightwardsArrowThroughX,
    /// \u{2948}: '⥈'
    LeftRightArrowThroughSmallCircle,
    /// \u{2949}: '⥉'
    UpwardsTwoDashHeadedArrowFromSmallCircle,
    /// \u{294a}: '⥊'
    LeftBarbUpRightBarbDownHarpoon,
    /// \u{294b}: '⥋'
    LeftBarbDownRightBarbUpHarpoon,
    /// \u{294c}: '⥌'
    UpBarbRightDownBarbLeftHarpoon,
    /// \u{294d}: '⥍'
    UpBarbLeftDownBarbRightHarpoon,
    /// \u{294e}: '⥎'
    LeftBarbUpRightBarbUpHarpoon,
    /// \u{294f}: '⥏'
    UpBarbRightDownBarbRightHarpoon,
    /// \u{2950}: '⥐'
    LeftBarbDownRightBarbDownHarpoon,
    /// \u{2951}: '⥑'
    UpBarbLeftDownBarbLeftHarpoon,
    /// \u{2952}: '⥒'
    LeftwardsHarpoonWithBarbUpToBar,
    /// \u{2953}: '⥓'
    RightwardsHarpoonWithBarbUpToBar,
    /// \u{2954}: '⥔'
    UpwardsHarpoonWithBarbRightToBar,
    /// \u{2955}: '⥕'
    DownwardsHarpoonWithBarbRightToBar,
    /// \u{2956}: '⥖'
    LeftwardsHarpoonWithBarbDownToBar,
    /// \u{2957}: '⥗'
    RightwardsHarpoonWithBarbDownToBar,
    /// \u{2958}: '⥘'
    UpwardsHarpoonWithBarbLeftToBar,
    /// \u{2959}: '⥙'
    DownwardsHarpoonWithBarbLeftToBar,
    /// \u{295a}: '⥚'
    LeftwardsHarpoonWithBarbUpFromBar,
    /// \u{295b}: '⥛'
    RightwardsHarpoonWithBarbUpFromBar,
    /// \u{295c}: '⥜'
    UpwardsHarpoonWithBarbRightFromBar,
    /// \u{295d}: '⥝'
    DownwardsHarpoonWithBarbRightFromBar,
    /// \u{295e}: '⥞'
    LeftwardsHarpoonWithBarbDownFromBar,
    /// \u{295f}: '⥟'
    RightwardsHarpoonWithBarbDownFromBar,
    /// \u{2960}: '⥠'
    UpwardsHarpoonWithBarbLeftFromBar,
    /// \u{2961}: '⥡'
    DownwardsHarpoonWithBarbLeftFromBar,
    /// \u{2962}: '⥢'
    LeftwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbDown,
    /// \u{2963}: '⥣'
    UpwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight,
    /// \u{2964}: '⥤'
    RightwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbDown,
    /// \u{2965}: '⥥'
    DownwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight,
    /// \u{2966}: '⥦'
    LeftwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbUp,
    /// \u{2967}: '⥧'
    LeftwardsHarpoonWithBarbDownAboveRightwardsHarpoonWithBarbDown,
    /// \u{2968}: '⥨'
    RightwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbUp,
    /// \u{2969}: '⥩'
    RightwardsHarpoonWithBarbDownAboveLeftwardsHarpoonWithBarbDown,
    /// \u{296a}: '⥪'
    LeftwardsHarpoonWithBarbUpAboveLongDash,
    /// \u{296b}: '⥫'
    LeftwardsHarpoonWithBarbDownBelowLongDash,
    /// \u{296c}: '⥬'
    RightwardsHarpoonWithBarbUpAboveLongDash,
    /// \u{296d}: '⥭'
    RightwardsHarpoonWithBarbDownBelowLongDash,
    /// \u{296e}: '⥮'
    UpwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight,
    /// \u{296f}: '⥯'
    DownwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight,
    /// \u{2970}: '⥰'
    RightDoubleArrowWithRoundedHead,
    /// \u{2971}: '⥱'
    EqualsSignAboveRightwardsArrow,
    /// \u{2972}: '⥲'
    TildeOperatorAboveRightwardsArrow,
    /// \u{2973}: '⥳'
    LeftwardsArrowAboveTildeOperator,
    /// \u{2974}: '⥴'
    RightwardsArrowAboveTildeOperator,
    /// \u{2975}: '⥵'
    RightwardsArrowAboveAlmostEqualTo,
    /// \u{2976}: '⥶'
    LessDashThanAboveLeftwardsArrow,
    /// \u{2977}: '⥷'
    LeftwardsArrowThroughLessDashThan,
    /// \u{2978}: '⥸'
    GreaterDashThanAboveRightwardsArrow,
    /// \u{2979}: '⥹'
    SubsetAboveRightwardsArrow,
    /// \u{297a}: '⥺'
    LeftwardsArrowThroughSubset,
    /// \u{297b}: '⥻'
    SupersetAboveLeftwardsArrow,
    /// \u{297c}: '⥼'
    LeftFishTail,
    /// \u{297d}: '⥽'
    RightFishTail,
    /// \u{297e}: '⥾'
    UpFishTail,
}

impl Into<char> for SupplementalArrowsB {
    fn into(self) -> char {
        match self {
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithVerticalStroke => '⤀',
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithDoubleVerticalStroke => '⤁',
            SupplementalArrowsB::LeftwardsDoubleArrowWithVerticalStroke => '⤂',
            SupplementalArrowsB::RightwardsDoubleArrowWithVerticalStroke => '⤃',
            SupplementalArrowsB::LeftRightDoubleArrowWithVerticalStroke => '⤄',
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowFromBar => '⤅',
            SupplementalArrowsB::LeftwardsDoubleArrowFromBar => '⤆',
            SupplementalArrowsB::RightwardsDoubleArrowFromBar => '⤇',
            SupplementalArrowsB::DownwardsArrowWithHorizontalStroke => '⤈',
            SupplementalArrowsB::UpwardsArrowWithHorizontalStroke => '⤉',
            SupplementalArrowsB::UpwardsTripleArrow => '⤊',
            SupplementalArrowsB::DownwardsTripleArrow => '⤋',
            SupplementalArrowsB::LeftwardsDoubleDashArrow => '⤌',
            SupplementalArrowsB::RightwardsDoubleDashArrow => '⤍',
            SupplementalArrowsB::LeftwardsTripleDashArrow => '⤎',
            SupplementalArrowsB::RightwardsTripleDashArrow => '⤏',
            SupplementalArrowsB::RightwardsTwoDashHeadedTripleDashArrow => '⤐',
            SupplementalArrowsB::RightwardsArrowWithDottedStem => '⤑',
            SupplementalArrowsB::UpwardsArrowToBar => '⤒',
            SupplementalArrowsB::DownwardsArrowToBar => '⤓',
            SupplementalArrowsB::RightwardsArrowWithTailWithVerticalStroke => '⤔',
            SupplementalArrowsB::RightwardsArrowWithTailWithDoubleVerticalStroke => '⤕',
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTail => '⤖',
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTailWithVerticalStroke => '⤗',
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke => '⤘',
            SupplementalArrowsB::LeftwardsArrowDashTail => '⤙',
            SupplementalArrowsB::RightwardsArrowDashTail => '⤚',
            SupplementalArrowsB::LeftwardsDoubleArrowDashTail => '⤛',
            SupplementalArrowsB::RightwardsDoubleArrowDashTail => '⤜',
            SupplementalArrowsB::LeftwardsArrowToBlackDiamond => '⤝',
            SupplementalArrowsB::RightwardsArrowToBlackDiamond => '⤞',
            SupplementalArrowsB::LeftwardsArrowFromBarToBlackDiamond => '⤟',
            SupplementalArrowsB::RightwardsArrowFromBarToBlackDiamond => '⤠',
            SupplementalArrowsB::NorthWestAndSouthEastArrow => '⤡',
            SupplementalArrowsB::NorthEastAndSouthWestArrow => '⤢',
            SupplementalArrowsB::NorthWestArrowWithHook => '⤣',
            SupplementalArrowsB::NorthEastArrowWithHook => '⤤',
            SupplementalArrowsB::SouthEastArrowWithHook => '⤥',
            SupplementalArrowsB::SouthWestArrowWithHook => '⤦',
            SupplementalArrowsB::NorthWestArrowAndNorthEastArrow => '⤧',
            SupplementalArrowsB::NorthEastArrowAndSouthEastArrow => '⤨',
            SupplementalArrowsB::SouthEastArrowAndSouthWestArrow => '⤩',
            SupplementalArrowsB::SouthWestArrowAndNorthWestArrow => '⤪',
            SupplementalArrowsB::RisingDiagonalCrossingFallingDiagonal => '⤫',
            SupplementalArrowsB::FallingDiagonalCrossingRisingDiagonal => '⤬',
            SupplementalArrowsB::SouthEastArrowCrossingNorthEastArrow => '⤭',
            SupplementalArrowsB::NorthEastArrowCrossingSouthEastArrow => '⤮',
            SupplementalArrowsB::FallingDiagonalCrossingNorthEastArrow => '⤯',
            SupplementalArrowsB::RisingDiagonalCrossingSouthEastArrow => '⤰',
            SupplementalArrowsB::NorthEastArrowCrossingNorthWestArrow => '⤱',
            SupplementalArrowsB::NorthWestArrowCrossingNorthEastArrow => '⤲',
            SupplementalArrowsB::WaveArrowPointingDirectlyRight => '⤳',
            SupplementalArrowsB::ArrowPointingRightwardsThenCurvingUpwards => '⤴',
            SupplementalArrowsB::ArrowPointingRightwardsThenCurvingDownwards => '⤵',
            SupplementalArrowsB::ArrowPointingDownwardsThenCurvingLeftwards => '⤶',
            SupplementalArrowsB::ArrowPointingDownwardsThenCurvingRightwards => '⤷',
            SupplementalArrowsB::RightDashSideArcClockwiseArrow => '⤸',
            SupplementalArrowsB::LeftDashSideArcAnticlockwiseArrow => '⤹',
            SupplementalArrowsB::TopArcAnticlockwiseArrow => '⤺',
            SupplementalArrowsB::BottomArcAnticlockwiseArrow => '⤻',
            SupplementalArrowsB::TopArcClockwiseArrowWithMinus => '⤼',
            SupplementalArrowsB::TopArcAnticlockwiseArrowWithPlus => '⤽',
            SupplementalArrowsB::LowerRightSemicircularClockwiseArrow => '⤾',
            SupplementalArrowsB::LowerLeftSemicircularAnticlockwiseArrow => '⤿',
            SupplementalArrowsB::AnticlockwiseClosedCircleArrow => '⥀',
            SupplementalArrowsB::ClockwiseClosedCircleArrow => '⥁',
            SupplementalArrowsB::RightwardsArrowAboveShortLeftwardsArrow => '⥂',
            SupplementalArrowsB::LeftwardsArrowAboveShortRightwardsArrow => '⥃',
            SupplementalArrowsB::ShortRightwardsArrowAboveLeftwardsArrow => '⥄',
            SupplementalArrowsB::RightwardsArrowWithPlusBelow => '⥅',
            SupplementalArrowsB::LeftwardsArrowWithPlusBelow => '⥆',
            SupplementalArrowsB::RightwardsArrowThroughX => '⥇',
            SupplementalArrowsB::LeftRightArrowThroughSmallCircle => '⥈',
            SupplementalArrowsB::UpwardsTwoDashHeadedArrowFromSmallCircle => '⥉',
            SupplementalArrowsB::LeftBarbUpRightBarbDownHarpoon => '⥊',
            SupplementalArrowsB::LeftBarbDownRightBarbUpHarpoon => '⥋',
            SupplementalArrowsB::UpBarbRightDownBarbLeftHarpoon => '⥌',
            SupplementalArrowsB::UpBarbLeftDownBarbRightHarpoon => '⥍',
            SupplementalArrowsB::LeftBarbUpRightBarbUpHarpoon => '⥎',
            SupplementalArrowsB::UpBarbRightDownBarbRightHarpoon => '⥏',
            SupplementalArrowsB::LeftBarbDownRightBarbDownHarpoon => '⥐',
            SupplementalArrowsB::UpBarbLeftDownBarbLeftHarpoon => '⥑',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpToBar => '⥒',
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpToBar => '⥓',
            SupplementalArrowsB::UpwardsHarpoonWithBarbRightToBar => '⥔',
            SupplementalArrowsB::DownwardsHarpoonWithBarbRightToBar => '⥕',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownToBar => '⥖',
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownToBar => '⥗',
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftToBar => '⥘',
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftToBar => '⥙',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpFromBar => '⥚',
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpFromBar => '⥛',
            SupplementalArrowsB::UpwardsHarpoonWithBarbRightFromBar => '⥜',
            SupplementalArrowsB::DownwardsHarpoonWithBarbRightFromBar => '⥝',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownFromBar => '⥞',
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownFromBar => '⥟',
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftFromBar => '⥠',
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftFromBar => '⥡',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbDown => '⥢',
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight => '⥣',
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbDown => '⥤',
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight => '⥥',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbUp => '⥦',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownAboveRightwardsHarpoonWithBarbDown => '⥧',
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbUp => '⥨',
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownAboveLeftwardsHarpoonWithBarbDown => '⥩',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveLongDash => '⥪',
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownBelowLongDash => '⥫',
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveLongDash => '⥬',
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownBelowLongDash => '⥭',
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight => '⥮',
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight => '⥯',
            SupplementalArrowsB::RightDoubleArrowWithRoundedHead => '⥰',
            SupplementalArrowsB::EqualsSignAboveRightwardsArrow => '⥱',
            SupplementalArrowsB::TildeOperatorAboveRightwardsArrow => '⥲',
            SupplementalArrowsB::LeftwardsArrowAboveTildeOperator => '⥳',
            SupplementalArrowsB::RightwardsArrowAboveTildeOperator => '⥴',
            SupplementalArrowsB::RightwardsArrowAboveAlmostEqualTo => '⥵',
            SupplementalArrowsB::LessDashThanAboveLeftwardsArrow => '⥶',
            SupplementalArrowsB::LeftwardsArrowThroughLessDashThan => '⥷',
            SupplementalArrowsB::GreaterDashThanAboveRightwardsArrow => '⥸',
            SupplementalArrowsB::SubsetAboveRightwardsArrow => '⥹',
            SupplementalArrowsB::LeftwardsArrowThroughSubset => '⥺',
            SupplementalArrowsB::SupersetAboveLeftwardsArrow => '⥻',
            SupplementalArrowsB::LeftFishTail => '⥼',
            SupplementalArrowsB::RightFishTail => '⥽',
            SupplementalArrowsB::UpFishTail => '⥾',
        }
    }
}

impl std::convert::TryFrom<char> for SupplementalArrowsB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⤀' => Ok(SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithVerticalStroke),
            '⤁' => Ok(SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithDoubleVerticalStroke),
            '⤂' => Ok(SupplementalArrowsB::LeftwardsDoubleArrowWithVerticalStroke),
            '⤃' => Ok(SupplementalArrowsB::RightwardsDoubleArrowWithVerticalStroke),
            '⤄' => Ok(SupplementalArrowsB::LeftRightDoubleArrowWithVerticalStroke),
            '⤅' => Ok(SupplementalArrowsB::RightwardsTwoDashHeadedArrowFromBar),
            '⤆' => Ok(SupplementalArrowsB::LeftwardsDoubleArrowFromBar),
            '⤇' => Ok(SupplementalArrowsB::RightwardsDoubleArrowFromBar),
            '⤈' => Ok(SupplementalArrowsB::DownwardsArrowWithHorizontalStroke),
            '⤉' => Ok(SupplementalArrowsB::UpwardsArrowWithHorizontalStroke),
            '⤊' => Ok(SupplementalArrowsB::UpwardsTripleArrow),
            '⤋' => Ok(SupplementalArrowsB::DownwardsTripleArrow),
            '⤌' => Ok(SupplementalArrowsB::LeftwardsDoubleDashArrow),
            '⤍' => Ok(SupplementalArrowsB::RightwardsDoubleDashArrow),
            '⤎' => Ok(SupplementalArrowsB::LeftwardsTripleDashArrow),
            '⤏' => Ok(SupplementalArrowsB::RightwardsTripleDashArrow),
            '⤐' => Ok(SupplementalArrowsB::RightwardsTwoDashHeadedTripleDashArrow),
            '⤑' => Ok(SupplementalArrowsB::RightwardsArrowWithDottedStem),
            '⤒' => Ok(SupplementalArrowsB::UpwardsArrowToBar),
            '⤓' => Ok(SupplementalArrowsB::DownwardsArrowToBar),
            '⤔' => Ok(SupplementalArrowsB::RightwardsArrowWithTailWithVerticalStroke),
            '⤕' => Ok(SupplementalArrowsB::RightwardsArrowWithTailWithDoubleVerticalStroke),
            '⤖' => Ok(SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTail),
            '⤗' => Ok(SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTailWithVerticalStroke),
            '⤘' => Ok(SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke),
            '⤙' => Ok(SupplementalArrowsB::LeftwardsArrowDashTail),
            '⤚' => Ok(SupplementalArrowsB::RightwardsArrowDashTail),
            '⤛' => Ok(SupplementalArrowsB::LeftwardsDoubleArrowDashTail),
            '⤜' => Ok(SupplementalArrowsB::RightwardsDoubleArrowDashTail),
            '⤝' => Ok(SupplementalArrowsB::LeftwardsArrowToBlackDiamond),
            '⤞' => Ok(SupplementalArrowsB::RightwardsArrowToBlackDiamond),
            '⤟' => Ok(SupplementalArrowsB::LeftwardsArrowFromBarToBlackDiamond),
            '⤠' => Ok(SupplementalArrowsB::RightwardsArrowFromBarToBlackDiamond),
            '⤡' => Ok(SupplementalArrowsB::NorthWestAndSouthEastArrow),
            '⤢' => Ok(SupplementalArrowsB::NorthEastAndSouthWestArrow),
            '⤣' => Ok(SupplementalArrowsB::NorthWestArrowWithHook),
            '⤤' => Ok(SupplementalArrowsB::NorthEastArrowWithHook),
            '⤥' => Ok(SupplementalArrowsB::SouthEastArrowWithHook),
            '⤦' => Ok(SupplementalArrowsB::SouthWestArrowWithHook),
            '⤧' => Ok(SupplementalArrowsB::NorthWestArrowAndNorthEastArrow),
            '⤨' => Ok(SupplementalArrowsB::NorthEastArrowAndSouthEastArrow),
            '⤩' => Ok(SupplementalArrowsB::SouthEastArrowAndSouthWestArrow),
            '⤪' => Ok(SupplementalArrowsB::SouthWestArrowAndNorthWestArrow),
            '⤫' => Ok(SupplementalArrowsB::RisingDiagonalCrossingFallingDiagonal),
            '⤬' => Ok(SupplementalArrowsB::FallingDiagonalCrossingRisingDiagonal),
            '⤭' => Ok(SupplementalArrowsB::SouthEastArrowCrossingNorthEastArrow),
            '⤮' => Ok(SupplementalArrowsB::NorthEastArrowCrossingSouthEastArrow),
            '⤯' => Ok(SupplementalArrowsB::FallingDiagonalCrossingNorthEastArrow),
            '⤰' => Ok(SupplementalArrowsB::RisingDiagonalCrossingSouthEastArrow),
            '⤱' => Ok(SupplementalArrowsB::NorthEastArrowCrossingNorthWestArrow),
            '⤲' => Ok(SupplementalArrowsB::NorthWestArrowCrossingNorthEastArrow),
            '⤳' => Ok(SupplementalArrowsB::WaveArrowPointingDirectlyRight),
            '⤴' => Ok(SupplementalArrowsB::ArrowPointingRightwardsThenCurvingUpwards),
            '⤵' => Ok(SupplementalArrowsB::ArrowPointingRightwardsThenCurvingDownwards),
            '⤶' => Ok(SupplementalArrowsB::ArrowPointingDownwardsThenCurvingLeftwards),
            '⤷' => Ok(SupplementalArrowsB::ArrowPointingDownwardsThenCurvingRightwards),
            '⤸' => Ok(SupplementalArrowsB::RightDashSideArcClockwiseArrow),
            '⤹' => Ok(SupplementalArrowsB::LeftDashSideArcAnticlockwiseArrow),
            '⤺' => Ok(SupplementalArrowsB::TopArcAnticlockwiseArrow),
            '⤻' => Ok(SupplementalArrowsB::BottomArcAnticlockwiseArrow),
            '⤼' => Ok(SupplementalArrowsB::TopArcClockwiseArrowWithMinus),
            '⤽' => Ok(SupplementalArrowsB::TopArcAnticlockwiseArrowWithPlus),
            '⤾' => Ok(SupplementalArrowsB::LowerRightSemicircularClockwiseArrow),
            '⤿' => Ok(SupplementalArrowsB::LowerLeftSemicircularAnticlockwiseArrow),
            '⥀' => Ok(SupplementalArrowsB::AnticlockwiseClosedCircleArrow),
            '⥁' => Ok(SupplementalArrowsB::ClockwiseClosedCircleArrow),
            '⥂' => Ok(SupplementalArrowsB::RightwardsArrowAboveShortLeftwardsArrow),
            '⥃' => Ok(SupplementalArrowsB::LeftwardsArrowAboveShortRightwardsArrow),
            '⥄' => Ok(SupplementalArrowsB::ShortRightwardsArrowAboveLeftwardsArrow),
            '⥅' => Ok(SupplementalArrowsB::RightwardsArrowWithPlusBelow),
            '⥆' => Ok(SupplementalArrowsB::LeftwardsArrowWithPlusBelow),
            '⥇' => Ok(SupplementalArrowsB::RightwardsArrowThroughX),
            '⥈' => Ok(SupplementalArrowsB::LeftRightArrowThroughSmallCircle),
            '⥉' => Ok(SupplementalArrowsB::UpwardsTwoDashHeadedArrowFromSmallCircle),
            '⥊' => Ok(SupplementalArrowsB::LeftBarbUpRightBarbDownHarpoon),
            '⥋' => Ok(SupplementalArrowsB::LeftBarbDownRightBarbUpHarpoon),
            '⥌' => Ok(SupplementalArrowsB::UpBarbRightDownBarbLeftHarpoon),
            '⥍' => Ok(SupplementalArrowsB::UpBarbLeftDownBarbRightHarpoon),
            '⥎' => Ok(SupplementalArrowsB::LeftBarbUpRightBarbUpHarpoon),
            '⥏' => Ok(SupplementalArrowsB::UpBarbRightDownBarbRightHarpoon),
            '⥐' => Ok(SupplementalArrowsB::LeftBarbDownRightBarbDownHarpoon),
            '⥑' => Ok(SupplementalArrowsB::UpBarbLeftDownBarbLeftHarpoon),
            '⥒' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbUpToBar),
            '⥓' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbUpToBar),
            '⥔' => Ok(SupplementalArrowsB::UpwardsHarpoonWithBarbRightToBar),
            '⥕' => Ok(SupplementalArrowsB::DownwardsHarpoonWithBarbRightToBar),
            '⥖' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbDownToBar),
            '⥗' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbDownToBar),
            '⥘' => Ok(SupplementalArrowsB::UpwardsHarpoonWithBarbLeftToBar),
            '⥙' => Ok(SupplementalArrowsB::DownwardsHarpoonWithBarbLeftToBar),
            '⥚' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbUpFromBar),
            '⥛' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbUpFromBar),
            '⥜' => Ok(SupplementalArrowsB::UpwardsHarpoonWithBarbRightFromBar),
            '⥝' => Ok(SupplementalArrowsB::DownwardsHarpoonWithBarbRightFromBar),
            '⥞' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbDownFromBar),
            '⥟' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbDownFromBar),
            '⥠' => Ok(SupplementalArrowsB::UpwardsHarpoonWithBarbLeftFromBar),
            '⥡' => Ok(SupplementalArrowsB::DownwardsHarpoonWithBarbLeftFromBar),
            '⥢' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbDown),
            '⥣' => Ok(SupplementalArrowsB::UpwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight),
            '⥤' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbDown),
            '⥥' => Ok(SupplementalArrowsB::DownwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight),
            '⥦' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbUp),
            '⥧' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbDownAboveRightwardsHarpoonWithBarbDown),
            '⥨' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbUp),
            '⥩' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbDownAboveLeftwardsHarpoonWithBarbDown),
            '⥪' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveLongDash),
            '⥫' => Ok(SupplementalArrowsB::LeftwardsHarpoonWithBarbDownBelowLongDash),
            '⥬' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveLongDash),
            '⥭' => Ok(SupplementalArrowsB::RightwardsHarpoonWithBarbDownBelowLongDash),
            '⥮' => Ok(SupplementalArrowsB::UpwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight),
            '⥯' => Ok(SupplementalArrowsB::DownwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight),
            '⥰' => Ok(SupplementalArrowsB::RightDoubleArrowWithRoundedHead),
            '⥱' => Ok(SupplementalArrowsB::EqualsSignAboveRightwardsArrow),
            '⥲' => Ok(SupplementalArrowsB::TildeOperatorAboveRightwardsArrow),
            '⥳' => Ok(SupplementalArrowsB::LeftwardsArrowAboveTildeOperator),
            '⥴' => Ok(SupplementalArrowsB::RightwardsArrowAboveTildeOperator),
            '⥵' => Ok(SupplementalArrowsB::RightwardsArrowAboveAlmostEqualTo),
            '⥶' => Ok(SupplementalArrowsB::LessDashThanAboveLeftwardsArrow),
            '⥷' => Ok(SupplementalArrowsB::LeftwardsArrowThroughLessDashThan),
            '⥸' => Ok(SupplementalArrowsB::GreaterDashThanAboveRightwardsArrow),
            '⥹' => Ok(SupplementalArrowsB::SubsetAboveRightwardsArrow),
            '⥺' => Ok(SupplementalArrowsB::LeftwardsArrowThroughSubset),
            '⥻' => Ok(SupplementalArrowsB::SupersetAboveLeftwardsArrow),
            '⥼' => Ok(SupplementalArrowsB::LeftFishTail),
            '⥽' => Ok(SupplementalArrowsB::RightFishTail),
            '⥾' => Ok(SupplementalArrowsB::UpFishTail),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SupplementalArrowsB {
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

impl std::convert::TryFrom<u32> for SupplementalArrowsB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SupplementalArrowsB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SupplementalArrowsB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithVerticalStroke
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithVerticalStroke => "rightwards two-headed arrow with vertical stroke",
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithDoubleVerticalStroke => "rightwards two-headed arrow with double vertical stroke",
            SupplementalArrowsB::LeftwardsDoubleArrowWithVerticalStroke => "leftwards double arrow with vertical stroke",
            SupplementalArrowsB::RightwardsDoubleArrowWithVerticalStroke => "rightwards double arrow with vertical stroke",
            SupplementalArrowsB::LeftRightDoubleArrowWithVerticalStroke => "left right double arrow with vertical stroke",
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowFromBar => "rightwards two-headed arrow from bar",
            SupplementalArrowsB::LeftwardsDoubleArrowFromBar => "leftwards double arrow from bar",
            SupplementalArrowsB::RightwardsDoubleArrowFromBar => "rightwards double arrow from bar",
            SupplementalArrowsB::DownwardsArrowWithHorizontalStroke => "downwards arrow with horizontal stroke",
            SupplementalArrowsB::UpwardsArrowWithHorizontalStroke => "upwards arrow with horizontal stroke",
            SupplementalArrowsB::UpwardsTripleArrow => "upwards triple arrow",
            SupplementalArrowsB::DownwardsTripleArrow => "downwards triple arrow",
            SupplementalArrowsB::LeftwardsDoubleDashArrow => "leftwards double dash arrow",
            SupplementalArrowsB::RightwardsDoubleDashArrow => "rightwards double dash arrow",
            SupplementalArrowsB::LeftwardsTripleDashArrow => "leftwards triple dash arrow",
            SupplementalArrowsB::RightwardsTripleDashArrow => "rightwards triple dash arrow",
            SupplementalArrowsB::RightwardsTwoDashHeadedTripleDashArrow => "rightwards two-headed triple dash arrow",
            SupplementalArrowsB::RightwardsArrowWithDottedStem => "rightwards arrow with dotted stem",
            SupplementalArrowsB::UpwardsArrowToBar => "upwards arrow to bar",
            SupplementalArrowsB::DownwardsArrowToBar => "downwards arrow to bar",
            SupplementalArrowsB::RightwardsArrowWithTailWithVerticalStroke => "rightwards arrow with tail with vertical stroke",
            SupplementalArrowsB::RightwardsArrowWithTailWithDoubleVerticalStroke => "rightwards arrow with tail with double vertical stroke",
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTail => "rightwards two-headed arrow with tail",
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTailWithVerticalStroke => "rightwards two-headed arrow with tail with vertical stroke",
            SupplementalArrowsB::RightwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke => "rightwards two-headed arrow with tail with double vertical stroke",
            SupplementalArrowsB::LeftwardsArrowDashTail => "leftwards arrow-tail",
            SupplementalArrowsB::RightwardsArrowDashTail => "rightwards arrow-tail",
            SupplementalArrowsB::LeftwardsDoubleArrowDashTail => "leftwards double arrow-tail",
            SupplementalArrowsB::RightwardsDoubleArrowDashTail => "rightwards double arrow-tail",
            SupplementalArrowsB::LeftwardsArrowToBlackDiamond => "leftwards arrow to black diamond",
            SupplementalArrowsB::RightwardsArrowToBlackDiamond => "rightwards arrow to black diamond",
            SupplementalArrowsB::LeftwardsArrowFromBarToBlackDiamond => "leftwards arrow from bar to black diamond",
            SupplementalArrowsB::RightwardsArrowFromBarToBlackDiamond => "rightwards arrow from bar to black diamond",
            SupplementalArrowsB::NorthWestAndSouthEastArrow => "north west and south east arrow",
            SupplementalArrowsB::NorthEastAndSouthWestArrow => "north east and south west arrow",
            SupplementalArrowsB::NorthWestArrowWithHook => "north west arrow with hook",
            SupplementalArrowsB::NorthEastArrowWithHook => "north east arrow with hook",
            SupplementalArrowsB::SouthEastArrowWithHook => "south east arrow with hook",
            SupplementalArrowsB::SouthWestArrowWithHook => "south west arrow with hook",
            SupplementalArrowsB::NorthWestArrowAndNorthEastArrow => "north west arrow and north east arrow",
            SupplementalArrowsB::NorthEastArrowAndSouthEastArrow => "north east arrow and south east arrow",
            SupplementalArrowsB::SouthEastArrowAndSouthWestArrow => "south east arrow and south west arrow",
            SupplementalArrowsB::SouthWestArrowAndNorthWestArrow => "south west arrow and north west arrow",
            SupplementalArrowsB::RisingDiagonalCrossingFallingDiagonal => "rising diagonal crossing falling diagonal",
            SupplementalArrowsB::FallingDiagonalCrossingRisingDiagonal => "falling diagonal crossing rising diagonal",
            SupplementalArrowsB::SouthEastArrowCrossingNorthEastArrow => "south east arrow crossing north east arrow",
            SupplementalArrowsB::NorthEastArrowCrossingSouthEastArrow => "north east arrow crossing south east arrow",
            SupplementalArrowsB::FallingDiagonalCrossingNorthEastArrow => "falling diagonal crossing north east arrow",
            SupplementalArrowsB::RisingDiagonalCrossingSouthEastArrow => "rising diagonal crossing south east arrow",
            SupplementalArrowsB::NorthEastArrowCrossingNorthWestArrow => "north east arrow crossing north west arrow",
            SupplementalArrowsB::NorthWestArrowCrossingNorthEastArrow => "north west arrow crossing north east arrow",
            SupplementalArrowsB::WaveArrowPointingDirectlyRight => "wave arrow pointing directly right",
            SupplementalArrowsB::ArrowPointingRightwardsThenCurvingUpwards => "arrow pointing rightwards then curving upwards",
            SupplementalArrowsB::ArrowPointingRightwardsThenCurvingDownwards => "arrow pointing rightwards then curving downwards",
            SupplementalArrowsB::ArrowPointingDownwardsThenCurvingLeftwards => "arrow pointing downwards then curving leftwards",
            SupplementalArrowsB::ArrowPointingDownwardsThenCurvingRightwards => "arrow pointing downwards then curving rightwards",
            SupplementalArrowsB::RightDashSideArcClockwiseArrow => "right-side arc clockwise arrow",
            SupplementalArrowsB::LeftDashSideArcAnticlockwiseArrow => "left-side arc anticlockwise arrow",
            SupplementalArrowsB::TopArcAnticlockwiseArrow => "top arc anticlockwise arrow",
            SupplementalArrowsB::BottomArcAnticlockwiseArrow => "bottom arc anticlockwise arrow",
            SupplementalArrowsB::TopArcClockwiseArrowWithMinus => "top arc clockwise arrow with minus",
            SupplementalArrowsB::TopArcAnticlockwiseArrowWithPlus => "top arc anticlockwise arrow with plus",
            SupplementalArrowsB::LowerRightSemicircularClockwiseArrow => "lower right semicircular clockwise arrow",
            SupplementalArrowsB::LowerLeftSemicircularAnticlockwiseArrow => "lower left semicircular anticlockwise arrow",
            SupplementalArrowsB::AnticlockwiseClosedCircleArrow => "anticlockwise closed circle arrow",
            SupplementalArrowsB::ClockwiseClosedCircleArrow => "clockwise closed circle arrow",
            SupplementalArrowsB::RightwardsArrowAboveShortLeftwardsArrow => "rightwards arrow above short leftwards arrow",
            SupplementalArrowsB::LeftwardsArrowAboveShortRightwardsArrow => "leftwards arrow above short rightwards arrow",
            SupplementalArrowsB::ShortRightwardsArrowAboveLeftwardsArrow => "short rightwards arrow above leftwards arrow",
            SupplementalArrowsB::RightwardsArrowWithPlusBelow => "rightwards arrow with plus below",
            SupplementalArrowsB::LeftwardsArrowWithPlusBelow => "leftwards arrow with plus below",
            SupplementalArrowsB::RightwardsArrowThroughX => "rightwards arrow through x",
            SupplementalArrowsB::LeftRightArrowThroughSmallCircle => "left right arrow through small circle",
            SupplementalArrowsB::UpwardsTwoDashHeadedArrowFromSmallCircle => "upwards two-headed arrow from small circle",
            SupplementalArrowsB::LeftBarbUpRightBarbDownHarpoon => "left barb up right barb down harpoon",
            SupplementalArrowsB::LeftBarbDownRightBarbUpHarpoon => "left barb down right barb up harpoon",
            SupplementalArrowsB::UpBarbRightDownBarbLeftHarpoon => "up barb right down barb left harpoon",
            SupplementalArrowsB::UpBarbLeftDownBarbRightHarpoon => "up barb left down barb right harpoon",
            SupplementalArrowsB::LeftBarbUpRightBarbUpHarpoon => "left barb up right barb up harpoon",
            SupplementalArrowsB::UpBarbRightDownBarbRightHarpoon => "up barb right down barb right harpoon",
            SupplementalArrowsB::LeftBarbDownRightBarbDownHarpoon => "left barb down right barb down harpoon",
            SupplementalArrowsB::UpBarbLeftDownBarbLeftHarpoon => "up barb left down barb left harpoon",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpToBar => "leftwards harpoon with barb up to bar",
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpToBar => "rightwards harpoon with barb up to bar",
            SupplementalArrowsB::UpwardsHarpoonWithBarbRightToBar => "upwards harpoon with barb right to bar",
            SupplementalArrowsB::DownwardsHarpoonWithBarbRightToBar => "downwards harpoon with barb right to bar",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownToBar => "leftwards harpoon with barb down to bar",
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownToBar => "rightwards harpoon with barb down to bar",
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftToBar => "upwards harpoon with barb left to bar",
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftToBar => "downwards harpoon with barb left to bar",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpFromBar => "leftwards harpoon with barb up from bar",
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpFromBar => "rightwards harpoon with barb up from bar",
            SupplementalArrowsB::UpwardsHarpoonWithBarbRightFromBar => "upwards harpoon with barb right from bar",
            SupplementalArrowsB::DownwardsHarpoonWithBarbRightFromBar => "downwards harpoon with barb right from bar",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownFromBar => "leftwards harpoon with barb down from bar",
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownFromBar => "rightwards harpoon with barb down from bar",
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftFromBar => "upwards harpoon with barb left from bar",
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftFromBar => "downwards harpoon with barb left from bar",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbDown => "leftwards harpoon with barb up above leftwards harpoon with barb down",
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight => "upwards harpoon with barb left beside upwards harpoon with barb right",
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbDown => "rightwards harpoon with barb up above rightwards harpoon with barb down",
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight => "downwards harpoon with barb left beside downwards harpoon with barb right",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveRightwardsHarpoonWithBarbUp => "leftwards harpoon with barb up above rightwards harpoon with barb up",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownAboveRightwardsHarpoonWithBarbDown => "leftwards harpoon with barb down above rightwards harpoon with barb down",
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveLeftwardsHarpoonWithBarbUp => "rightwards harpoon with barb up above leftwards harpoon with barb up",
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownAboveLeftwardsHarpoonWithBarbDown => "rightwards harpoon with barb down above leftwards harpoon with barb down",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbUpAboveLongDash => "leftwards harpoon with barb up above long dash",
            SupplementalArrowsB::LeftwardsHarpoonWithBarbDownBelowLongDash => "leftwards harpoon with barb down below long dash",
            SupplementalArrowsB::RightwardsHarpoonWithBarbUpAboveLongDash => "rightwards harpoon with barb up above long dash",
            SupplementalArrowsB::RightwardsHarpoonWithBarbDownBelowLongDash => "rightwards harpoon with barb down below long dash",
            SupplementalArrowsB::UpwardsHarpoonWithBarbLeftBesideDownwardsHarpoonWithBarbRight => "upwards harpoon with barb left beside downwards harpoon with barb right",
            SupplementalArrowsB::DownwardsHarpoonWithBarbLeftBesideUpwardsHarpoonWithBarbRight => "downwards harpoon with barb left beside upwards harpoon with barb right",
            SupplementalArrowsB::RightDoubleArrowWithRoundedHead => "right double arrow with rounded head",
            SupplementalArrowsB::EqualsSignAboveRightwardsArrow => "equals sign above rightwards arrow",
            SupplementalArrowsB::TildeOperatorAboveRightwardsArrow => "tilde operator above rightwards arrow",
            SupplementalArrowsB::LeftwardsArrowAboveTildeOperator => "leftwards arrow above tilde operator",
            SupplementalArrowsB::RightwardsArrowAboveTildeOperator => "rightwards arrow above tilde operator",
            SupplementalArrowsB::RightwardsArrowAboveAlmostEqualTo => "rightwards arrow above almost equal to",
            SupplementalArrowsB::LessDashThanAboveLeftwardsArrow => "less-than above leftwards arrow",
            SupplementalArrowsB::LeftwardsArrowThroughLessDashThan => "leftwards arrow through less-than",
            SupplementalArrowsB::GreaterDashThanAboveRightwardsArrow => "greater-than above rightwards arrow",
            SupplementalArrowsB::SubsetAboveRightwardsArrow => "subset above rightwards arrow",
            SupplementalArrowsB::LeftwardsArrowThroughSubset => "leftwards arrow through subset",
            SupplementalArrowsB::SupersetAboveLeftwardsArrow => "superset above leftwards arrow",
            SupplementalArrowsB::LeftFishTail => "left fish tail",
            SupplementalArrowsB::RightFishTail => "right fish tail",
            SupplementalArrowsB::UpFishTail => "up fish tail",
        }
    }
}
