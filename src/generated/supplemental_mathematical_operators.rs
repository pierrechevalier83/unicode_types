
/// An enum to represent all characters in the SupplementalMathematicalOperators block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SupplementalMathematicalOperators {
    /// \u{2a00}: '⨀'
    NDashAryCircledDotOperator,
    /// \u{2a01}: '⨁'
    NDashAryCircledPlusOperator,
    /// \u{2a02}: '⨂'
    NDashAryCircledTimesOperator,
    /// \u{2a03}: '⨃'
    NDashAryUnionOperatorWithDot,
    /// \u{2a04}: '⨄'
    NDashAryUnionOperatorWithPlus,
    /// \u{2a05}: '⨅'
    NDashArySquareIntersectionOperator,
    /// \u{2a06}: '⨆'
    NDashArySquareUnionOperator,
    /// \u{2a07}: '⨇'
    TwoLogicalAndOperator,
    /// \u{2a08}: '⨈'
    TwoLogicalOrOperator,
    /// \u{2a09}: '⨉'
    NDashAryTimesOperator,
    /// \u{2a0a}: '⨊'
    ModuloTwoSum,
    /// \u{2a0b}: '⨋'
    SummationWithIntegral,
    /// \u{2a0c}: '⨌'
    QuadrupleIntegralOperator,
    /// \u{2a0d}: '⨍'
    FinitePartIntegral,
    /// \u{2a0e}: '⨎'
    IntegralWithDoubleStroke,
    /// \u{2a0f}: '⨏'
    IntegralAverageWithSlash,
    /// \u{2a10}: '⨐'
    CirculationFunction,
    /// \u{2a11}: '⨑'
    AnticlockwiseIntegration,
    /// \u{2a12}: '⨒'
    LineIntegrationWithRectangularPathAroundPole,
    /// \u{2a13}: '⨓'
    LineIntegrationWithSemicircularPathAroundPole,
    /// \u{2a14}: '⨔'
    LineIntegrationNotIncludingThePole,
    /// \u{2a15}: '⨕'
    IntegralAroundAPointOperator,
    /// \u{2a16}: '⨖'
    QuaternionIntegralOperator,
    /// \u{2a17}: '⨗'
    IntegralWithLeftwardsArrowWithHook,
    /// \u{2a18}: '⨘'
    IntegralWithTimesSign,
    /// \u{2a19}: '⨙'
    IntegralWithIntersection,
    /// \u{2a1a}: '⨚'
    IntegralWithUnion,
    /// \u{2a1b}: '⨛'
    IntegralWithOverbar,
    /// \u{2a1c}: '⨜'
    IntegralWithUnderbar,
    /// \u{2a1d}: '⨝'
    Join,
    /// \u{2a1e}: '⨞'
    LargeLeftTriangleOperator,
    /// \u{2a1f}: '⨟'
    ZNotationSchemaComposition,
    /// \u{2a20}: '⨠'
    ZNotationSchemaPiping,
    /// \u{2a21}: '⨡'
    ZNotationSchemaProjection,
    /// \u{2a22}: '⨢'
    PlusSignWithSmallCircleAbove,
    /// \u{2a23}: '⨣'
    PlusSignWithCircumflexAccentAbove,
    /// \u{2a24}: '⨤'
    PlusSignWithTildeAbove,
    /// \u{2a25}: '⨥'
    PlusSignWithDotBelow,
    /// \u{2a26}: '⨦'
    PlusSignWithTildeBelow,
    /// \u{2a27}: '⨧'
    PlusSignWithSubscriptTwo,
    /// \u{2a28}: '⨨'
    PlusSignWithBlackTriangle,
    /// \u{2a29}: '⨩'
    MinusSignWithCommaAbove,
    /// \u{2a2a}: '⨪'
    MinusSignWithDotBelow,
    /// \u{2a2b}: '⨫'
    MinusSignWithFallingDots,
    /// \u{2a2c}: '⨬'
    MinusSignWithRisingDots,
    /// \u{2a2d}: '⨭'
    PlusSignInLeftHalfCircle,
    /// \u{2a2e}: '⨮'
    PlusSignInRightHalfCircle,
    /// \u{2a2f}: '⨯'
    VectorOrCrossProduct,
    /// \u{2a30}: '⨰'
    MultiplicationSignWithDotAbove,
    /// \u{2a31}: '⨱'
    MultiplicationSignWithUnderbar,
    /// \u{2a32}: '⨲'
    SemidirectProductWithBottomClosed,
    /// \u{2a33}: '⨳'
    SmashProduct,
    /// \u{2a34}: '⨴'
    MultiplicationSignInLeftHalfCircle,
    /// \u{2a35}: '⨵'
    MultiplicationSignInRightHalfCircle,
    /// \u{2a36}: '⨶'
    CircledMultiplicationSignWithCircumflexAccent,
    /// \u{2a37}: '⨷'
    MultiplicationSignInDoubleCircle,
    /// \u{2a38}: '⨸'
    CircledDivisionSign,
    /// \u{2a39}: '⨹'
    PlusSignInTriangle,
    /// \u{2a3a}: '⨺'
    MinusSignInTriangle,
    /// \u{2a3b}: '⨻'
    MultiplicationSignInTriangle,
    /// \u{2a3c}: '⨼'
    InteriorProduct,
    /// \u{2a3d}: '⨽'
    RighthandInteriorProduct,
    /// \u{2a3e}: '⨾'
    ZNotationRelationalComposition,
    /// \u{2a3f}: '⨿'
    AmalgamationOrCoproduct,
    /// \u{2a40}: '⩀'
    IntersectionWithDot,
    /// \u{2a41}: '⩁'
    UnionWithMinusSign,
    /// \u{2a42}: '⩂'
    UnionWithOverbar,
    /// \u{2a43}: '⩃'
    IntersectionWithOverbar,
    /// \u{2a44}: '⩄'
    IntersectionWithLogicalAnd,
    /// \u{2a45}: '⩅'
    UnionWithLogicalOr,
    /// \u{2a46}: '⩆'
    UnionAboveIntersection,
    /// \u{2a47}: '⩇'
    IntersectionAboveUnion,
    /// \u{2a48}: '⩈'
    UnionAboveBarAboveIntersection,
    /// \u{2a49}: '⩉'
    IntersectionAboveBarAboveUnion,
    /// \u{2a4a}: '⩊'
    UnionBesideAndJoinedWithUnion,
    /// \u{2a4b}: '⩋'
    IntersectionBesideAndJoinedWithIntersection,
    /// \u{2a4c}: '⩌'
    ClosedUnionWithSerifs,
    /// \u{2a4d}: '⩍'
    ClosedIntersectionWithSerifs,
    /// \u{2a4e}: '⩎'
    DoubleSquareIntersection,
    /// \u{2a4f}: '⩏'
    DoubleSquareUnion,
    /// \u{2a50}: '⩐'
    ClosedUnionWithSerifsAndSmashProduct,
    /// \u{2a51}: '⩑'
    LogicalAndWithDotAbove,
    /// \u{2a52}: '⩒'
    LogicalOrWithDotAbove,
    /// \u{2a53}: '⩓'
    DoubleLogicalAnd,
    /// \u{2a54}: '⩔'
    DoubleLogicalOr,
    /// \u{2a55}: '⩕'
    TwoIntersectingLogicalAnd,
    /// \u{2a56}: '⩖'
    TwoIntersectingLogicalOr,
    /// \u{2a57}: '⩗'
    SlopingLargeOr,
    /// \u{2a58}: '⩘'
    SlopingLargeAnd,
    /// \u{2a59}: '⩙'
    LogicalOrOverlappingLogicalAnd,
    /// \u{2a5a}: '⩚'
    LogicalAndWithMiddleStem,
    /// \u{2a5b}: '⩛'
    LogicalOrWithMiddleStem,
    /// \u{2a5c}: '⩜'
    LogicalAndWithHorizontalDash,
    /// \u{2a5d}: '⩝'
    LogicalOrWithHorizontalDash,
    /// \u{2a5e}: '⩞'
    LogicalAndWithDoubleOverbar,
    /// \u{2a5f}: '⩟'
    LogicalAndWithUnderbar,
    /// \u{2a60}: '⩠'
    LogicalAndWithDoubleUnderbar,
    /// \u{2a61}: '⩡'
    SmallVeeWithUnderbar,
    /// \u{2a62}: '⩢'
    LogicalOrWithDoubleOverbar,
    /// \u{2a63}: '⩣'
    LogicalOrWithDoubleUnderbar,
    /// \u{2a64}: '⩤'
    ZNotationDomainAntirestriction,
    /// \u{2a65}: '⩥'
    ZNotationRangeAntirestriction,
    /// \u{2a66}: '⩦'
    EqualsSignWithDotBelow,
    /// \u{2a67}: '⩧'
    IdenticalWithDotAbove,
    /// \u{2a68}: '⩨'
    TripleHorizontalBarWithDoubleVerticalStroke,
    /// \u{2a69}: '⩩'
    TripleHorizontalBarWithTripleVerticalStroke,
    /// \u{2a6a}: '⩪'
    TildeOperatorWithDotAbove,
    /// \u{2a6b}: '⩫'
    TildeOperatorWithRisingDots,
    /// \u{2a6c}: '⩬'
    SimilarMinusSimilar,
    /// \u{2a6d}: '⩭'
    CongruentWithDotAbove,
    /// \u{2a6e}: '⩮'
    EqualsWithAsterisk,
    /// \u{2a6f}: '⩯'
    AlmostEqualToWithCircumflexAccent,
    /// \u{2a70}: '⩰'
    ApproximatelyEqualOrEqualTo,
    /// \u{2a71}: '⩱'
    EqualsSignAbovePlusSign,
    /// \u{2a72}: '⩲'
    PlusSignAboveEqualsSign,
    /// \u{2a73}: '⩳'
    EqualsSignAboveTildeOperator,
    /// \u{2a74}: '⩴'
    DoubleColonEqual,
    /// \u{2a75}: '⩵'
    TwoConsecutiveEqualsSigns,
    /// \u{2a76}: '⩶'
    ThreeConsecutiveEqualsSigns,
    /// \u{2a77}: '⩷'
    EqualsSignWithTwoDotsAboveAndTwoDotsBelow,
    /// \u{2a78}: '⩸'
    EquivalentWithFourDotsAbove,
    /// \u{2a79}: '⩹'
    LessDashThanWithCircleInside,
    /// \u{2a7a}: '⩺'
    GreaterDashThanWithCircleInside,
    /// \u{2a7b}: '⩻'
    LessDashThanWithQuestionMarkAbove,
    /// \u{2a7c}: '⩼'
    GreaterDashThanWithQuestionMarkAbove,
    /// \u{2a7d}: '⩽'
    LessDashThanOrSlantedEqualTo,
    /// \u{2a7e}: '⩾'
    GreaterDashThanOrSlantedEqualTo,
    /// \u{2a7f}: '⩿'
    LessDashThanOrSlantedEqualToWithDotInside,
    /// \u{2a80}: '⪀'
    GreaterDashThanOrSlantedEqualToWithDotInside,
    /// \u{2a81}: '⪁'
    LessDashThanOrSlantedEqualToWithDotAbove,
    /// \u{2a82}: '⪂'
    GreaterDashThanOrSlantedEqualToWithDotAbove,
    /// \u{2a83}: '⪃'
    LessDashThanOrSlantedEqualToWithDotAboveRight,
    /// \u{2a84}: '⪄'
    GreaterDashThanOrSlantedEqualToWithDotAboveLeft,
    /// \u{2a85}: '⪅'
    LessDashThanOrApproximate,
    /// \u{2a86}: '⪆'
    GreaterDashThanOrApproximate,
    /// \u{2a87}: '⪇'
    LessDashThanAndSingleDashLineNotEqualTo,
    /// \u{2a88}: '⪈'
    GreaterDashThanAndSingleDashLineNotEqualTo,
    /// \u{2a89}: '⪉'
    LessDashThanAndNotApproximate,
    /// \u{2a8a}: '⪊'
    GreaterDashThanAndNotApproximate,
    /// \u{2a8b}: '⪋'
    LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan,
    /// \u{2a8c}: '⪌'
    GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan,
    /// \u{2a8d}: '⪍'
    LessDashThanAboveSimilarOrEqual,
    /// \u{2a8e}: '⪎'
    GreaterDashThanAboveSimilarOrEqual,
    /// \u{2a8f}: '⪏'
    LessDashThanAboveSimilarAboveGreaterDashThan,
    /// \u{2a90}: '⪐'
    GreaterDashThanAboveSimilarAboveLessDashThan,
    /// \u{2a91}: '⪑'
    LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual,
    /// \u{2a92}: '⪒'
    GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual,
    /// \u{2a93}: '⪓'
    LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual,
    /// \u{2a94}: '⪔'
    GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual,
    /// \u{2a95}: '⪕'
    SlantedEqualToOrLessDashThan,
    /// \u{2a96}: '⪖'
    SlantedEqualToOrGreaterDashThan,
    /// \u{2a97}: '⪗'
    SlantedEqualToOrLessDashThanWithDotInside,
    /// \u{2a98}: '⪘'
    SlantedEqualToOrGreaterDashThanWithDotInside,
    /// \u{2a99}: '⪙'
    DoubleDashLineEqualToOrLessDashThan,
    /// \u{2a9a}: '⪚'
    DoubleDashLineEqualToOrGreaterDashThan,
    /// \u{2a9b}: '⪛'
    DoubleDashLineSlantedEqualToOrLessDashThan,
    /// \u{2a9c}: '⪜'
    DoubleDashLineSlantedEqualToOrGreaterDashThan,
    /// \u{2a9d}: '⪝'
    SimilarOrLessDashThan,
    /// \u{2a9e}: '⪞'
    SimilarOrGreaterDashThan,
    /// \u{2a9f}: '⪟'
    SimilarAboveLessDashThanAboveEqualsSign,
    /// \u{2aa0}: '⪠'
    SimilarAboveGreaterDashThanAboveEqualsSign,
    /// \u{2aa1}: '⪡'
    DoubleNestedLessDashThan,
    /// \u{2aa2}: '⪢'
    DoubleNestedGreaterDashThan,
    /// \u{2aa3}: '⪣'
    DoubleNestedLessDashThanWithUnderbar,
    /// \u{2aa4}: '⪤'
    GreaterDashThanOverlappingLessDashThan,
    /// \u{2aa5}: '⪥'
    GreaterDashThanBesideLessDashThan,
    /// \u{2aa6}: '⪦'
    LessDashThanClosedByCurve,
    /// \u{2aa7}: '⪧'
    GreaterDashThanClosedByCurve,
    /// \u{2aa8}: '⪨'
    LessDashThanClosedByCurveAboveSlantedEqual,
    /// \u{2aa9}: '⪩'
    GreaterDashThanClosedByCurveAboveSlantedEqual,
    /// \u{2aaa}: '⪪'
    SmallerThan,
    /// \u{2aab}: '⪫'
    LargerThan,
    /// \u{2aac}: '⪬'
    SmallerThanOrEqualTo,
    /// \u{2aad}: '⪭'
    LargerThanOrEqualTo,
    /// \u{2aae}: '⪮'
    EqualsSignWithBumpyAbove,
    /// \u{2aaf}: '⪯'
    PrecedesAboveSingleDashLineEqualsSign,
    /// \u{2ab0}: '⪰'
    SucceedsAboveSingleDashLineEqualsSign,
    /// \u{2ab1}: '⪱'
    PrecedesAboveSingleDashLineNotEqualTo,
    /// \u{2ab2}: '⪲'
    SucceedsAboveSingleDashLineNotEqualTo,
    /// \u{2ab3}: '⪳'
    PrecedesAboveEqualsSign,
    /// \u{2ab4}: '⪴'
    SucceedsAboveEqualsSign,
    /// \u{2ab5}: '⪵'
    PrecedesAboveNotEqualTo,
    /// \u{2ab6}: '⪶'
    SucceedsAboveNotEqualTo,
    /// \u{2ab7}: '⪷'
    PrecedesAboveAlmostEqualTo,
    /// \u{2ab8}: '⪸'
    SucceedsAboveAlmostEqualTo,
    /// \u{2ab9}: '⪹'
    PrecedesAboveNotAlmostEqualTo,
    /// \u{2aba}: '⪺'
    SucceedsAboveNotAlmostEqualTo,
    /// \u{2abb}: '⪻'
    DoublePrecedes,
    /// \u{2abc}: '⪼'
    DoubleSucceeds,
    /// \u{2abd}: '⪽'
    SubsetWithDot,
    /// \u{2abe}: '⪾'
    SupersetWithDot,
    /// \u{2abf}: '⪿'
    SubsetWithPlusSignBelow,
    /// \u{2ac0}: '⫀'
    SupersetWithPlusSignBelow,
    /// \u{2ac1}: '⫁'
    SubsetWithMultiplicationSignBelow,
    /// \u{2ac2}: '⫂'
    SupersetWithMultiplicationSignBelow,
    /// \u{2ac3}: '⫃'
    SubsetOfOrEqualToWithDotAbove,
    /// \u{2ac4}: '⫄'
    SupersetOfOrEqualToWithDotAbove,
    /// \u{2ac5}: '⫅'
    SubsetOfAboveEqualsSign,
    /// \u{2ac6}: '⫆'
    SupersetOfAboveEqualsSign,
    /// \u{2ac7}: '⫇'
    SubsetOfAboveTildeOperator,
    /// \u{2ac8}: '⫈'
    SupersetOfAboveTildeOperator,
    /// \u{2ac9}: '⫉'
    SubsetOfAboveAlmostEqualTo,
    /// \u{2aca}: '⫊'
    SupersetOfAboveAlmostEqualTo,
    /// \u{2acb}: '⫋'
    SubsetOfAboveNotEqualTo,
    /// \u{2acc}: '⫌'
    SupersetOfAboveNotEqualTo,
    /// \u{2acd}: '⫍'
    SquareLeftOpenBoxOperator,
    /// \u{2ace}: '⫎'
    SquareRightOpenBoxOperator,
    /// \u{2acf}: '⫏'
    ClosedSubset,
    /// \u{2ad0}: '⫐'
    ClosedSuperset,
    /// \u{2ad1}: '⫑'
    ClosedSubsetOrEqualTo,
    /// \u{2ad2}: '⫒'
    ClosedSupersetOrEqualTo,
    /// \u{2ad3}: '⫓'
    SubsetAboveSuperset,
    /// \u{2ad4}: '⫔'
    SupersetAboveSubset,
    /// \u{2ad5}: '⫕'
    SubsetAboveSubset,
    /// \u{2ad6}: '⫖'
    SupersetAboveSuperset,
    /// \u{2ad7}: '⫗'
    SupersetBesideSubset,
    /// \u{2ad8}: '⫘'
    SupersetBesideAndJoinedByDashWithSubset,
    /// \u{2ad9}: '⫙'
    ElementOfOpeningDownwards,
    /// \u{2ada}: '⫚'
    PitchforkWithTeeTop,
    /// \u{2adb}: '⫛'
    TransversalIntersection,
    /// \u{2adc}: '⫝̸'
    Forking,
    /// \u{2add}: '⫝'
    Nonforking,
    /// \u{2ade}: '⫞'
    ShortLeftTack,
    /// \u{2adf}: '⫟'
    ShortDownTack,
    /// \u{2ae0}: '⫠'
    ShortUpTack,
    /// \u{2ae1}: '⫡'
    PerpendicularWithS,
    /// \u{2ae2}: '⫢'
    VerticalBarTripleRightTurnstile,
    /// \u{2ae3}: '⫣'
    DoubleVerticalBarLeftTurnstile,
    /// \u{2ae4}: '⫤'
    VerticalBarDoubleLeftTurnstile,
    /// \u{2ae5}: '⫥'
    DoubleVerticalBarDoubleLeftTurnstile,
    /// \u{2ae6}: '⫦'
    LongDashFromLeftMemberOfDoubleVertical,
    /// \u{2ae7}: '⫧'
    ShortDownTackWithOverbar,
    /// \u{2ae8}: '⫨'
    ShortUpTackWithUnderbar,
    /// \u{2ae9}: '⫩'
    ShortUpTackAboveShortDownTack,
    /// \u{2aea}: '⫪'
    DoubleDownTack,
    /// \u{2aeb}: '⫫'
    DoubleUpTack,
    /// \u{2aec}: '⫬'
    DoubleStrokeNotSign,
    /// \u{2aed}: '⫭'
    ReversedDoubleStrokeNotSign,
    /// \u{2aee}: '⫮'
    DoesNotDivideWithReversedNegationSlash,
    /// \u{2aef}: '⫯'
    VerticalLineWithCircleAbove,
    /// \u{2af0}: '⫰'
    VerticalLineWithCircleBelow,
    /// \u{2af1}: '⫱'
    DownTackWithCircleBelow,
    /// \u{2af2}: '⫲'
    ParallelWithHorizontalStroke,
    /// \u{2af3}: '⫳'
    ParallelWithTildeOperator,
    /// \u{2af4}: '⫴'
    TripleVerticalBarBinaryRelation,
    /// \u{2af5}: '⫵'
    TripleVerticalBarWithHorizontalStroke,
    /// \u{2af6}: '⫶'
    TripleColonOperator,
    /// \u{2af7}: '⫷'
    TripleNestedLessDashThan,
    /// \u{2af8}: '⫸'
    TripleNestedGreaterDashThan,
    /// \u{2af9}: '⫹'
    DoubleDashLineSlantedLessDashThanOrEqualTo,
    /// \u{2afa}: '⫺'
    DoubleDashLineSlantedGreaterDashThanOrEqualTo,
    /// \u{2afb}: '⫻'
    TripleSolidusBinaryRelation,
    /// \u{2afc}: '⫼'
    LargeTripleVerticalBarOperator,
    /// \u{2afd}: '⫽'
    DoubleSolidusOperator,
    /// \u{2afe}: '⫾'
    WhiteVerticalBar,
}

impl Into<char> for SupplementalMathematicalOperators {
    fn into(self) -> char {
        match self {
            SupplementalMathematicalOperators::NDashAryCircledDotOperator => '⨀',
            SupplementalMathematicalOperators::NDashAryCircledPlusOperator => '⨁',
            SupplementalMathematicalOperators::NDashAryCircledTimesOperator => '⨂',
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithDot => '⨃',
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithPlus => '⨄',
            SupplementalMathematicalOperators::NDashArySquareIntersectionOperator => '⨅',
            SupplementalMathematicalOperators::NDashArySquareUnionOperator => '⨆',
            SupplementalMathematicalOperators::TwoLogicalAndOperator => '⨇',
            SupplementalMathematicalOperators::TwoLogicalOrOperator => '⨈',
            SupplementalMathematicalOperators::NDashAryTimesOperator => '⨉',
            SupplementalMathematicalOperators::ModuloTwoSum => '⨊',
            SupplementalMathematicalOperators::SummationWithIntegral => '⨋',
            SupplementalMathematicalOperators::QuadrupleIntegralOperator => '⨌',
            SupplementalMathematicalOperators::FinitePartIntegral => '⨍',
            SupplementalMathematicalOperators::IntegralWithDoubleStroke => '⨎',
            SupplementalMathematicalOperators::IntegralAverageWithSlash => '⨏',
            SupplementalMathematicalOperators::CirculationFunction => '⨐',
            SupplementalMathematicalOperators::AnticlockwiseIntegration => '⨑',
            SupplementalMathematicalOperators::LineIntegrationWithRectangularPathAroundPole => '⨒',
            SupplementalMathematicalOperators::LineIntegrationWithSemicircularPathAroundPole => '⨓',
            SupplementalMathematicalOperators::LineIntegrationNotIncludingThePole => '⨔',
            SupplementalMathematicalOperators::IntegralAroundAPointOperator => '⨕',
            SupplementalMathematicalOperators::QuaternionIntegralOperator => '⨖',
            SupplementalMathematicalOperators::IntegralWithLeftwardsArrowWithHook => '⨗',
            SupplementalMathematicalOperators::IntegralWithTimesSign => '⨘',
            SupplementalMathematicalOperators::IntegralWithIntersection => '⨙',
            SupplementalMathematicalOperators::IntegralWithUnion => '⨚',
            SupplementalMathematicalOperators::IntegralWithOverbar => '⨛',
            SupplementalMathematicalOperators::IntegralWithUnderbar => '⨜',
            SupplementalMathematicalOperators::Join => '⨝',
            SupplementalMathematicalOperators::LargeLeftTriangleOperator => '⨞',
            SupplementalMathematicalOperators::ZNotationSchemaComposition => '⨟',
            SupplementalMathematicalOperators::ZNotationSchemaPiping => '⨠',
            SupplementalMathematicalOperators::ZNotationSchemaProjection => '⨡',
            SupplementalMathematicalOperators::PlusSignWithSmallCircleAbove => '⨢',
            SupplementalMathematicalOperators::PlusSignWithCircumflexAccentAbove => '⨣',
            SupplementalMathematicalOperators::PlusSignWithTildeAbove => '⨤',
            SupplementalMathematicalOperators::PlusSignWithDotBelow => '⨥',
            SupplementalMathematicalOperators::PlusSignWithTildeBelow => '⨦',
            SupplementalMathematicalOperators::PlusSignWithSubscriptTwo => '⨧',
            SupplementalMathematicalOperators::PlusSignWithBlackTriangle => '⨨',
            SupplementalMathematicalOperators::MinusSignWithCommaAbove => '⨩',
            SupplementalMathematicalOperators::MinusSignWithDotBelow => '⨪',
            SupplementalMathematicalOperators::MinusSignWithFallingDots => '⨫',
            SupplementalMathematicalOperators::MinusSignWithRisingDots => '⨬',
            SupplementalMathematicalOperators::PlusSignInLeftHalfCircle => '⨭',
            SupplementalMathematicalOperators::PlusSignInRightHalfCircle => '⨮',
            SupplementalMathematicalOperators::VectorOrCrossProduct => '⨯',
            SupplementalMathematicalOperators::MultiplicationSignWithDotAbove => '⨰',
            SupplementalMathematicalOperators::MultiplicationSignWithUnderbar => '⨱',
            SupplementalMathematicalOperators::SemidirectProductWithBottomClosed => '⨲',
            SupplementalMathematicalOperators::SmashProduct => '⨳',
            SupplementalMathematicalOperators::MultiplicationSignInLeftHalfCircle => '⨴',
            SupplementalMathematicalOperators::MultiplicationSignInRightHalfCircle => '⨵',
            SupplementalMathematicalOperators::CircledMultiplicationSignWithCircumflexAccent => '⨶',
            SupplementalMathematicalOperators::MultiplicationSignInDoubleCircle => '⨷',
            SupplementalMathematicalOperators::CircledDivisionSign => '⨸',
            SupplementalMathematicalOperators::PlusSignInTriangle => '⨹',
            SupplementalMathematicalOperators::MinusSignInTriangle => '⨺',
            SupplementalMathematicalOperators::MultiplicationSignInTriangle => '⨻',
            SupplementalMathematicalOperators::InteriorProduct => '⨼',
            SupplementalMathematicalOperators::RighthandInteriorProduct => '⨽',
            SupplementalMathematicalOperators::ZNotationRelationalComposition => '⨾',
            SupplementalMathematicalOperators::AmalgamationOrCoproduct => '⨿',
            SupplementalMathematicalOperators::IntersectionWithDot => '⩀',
            SupplementalMathematicalOperators::UnionWithMinusSign => '⩁',
            SupplementalMathematicalOperators::UnionWithOverbar => '⩂',
            SupplementalMathematicalOperators::IntersectionWithOverbar => '⩃',
            SupplementalMathematicalOperators::IntersectionWithLogicalAnd => '⩄',
            SupplementalMathematicalOperators::UnionWithLogicalOr => '⩅',
            SupplementalMathematicalOperators::UnionAboveIntersection => '⩆',
            SupplementalMathematicalOperators::IntersectionAboveUnion => '⩇',
            SupplementalMathematicalOperators::UnionAboveBarAboveIntersection => '⩈',
            SupplementalMathematicalOperators::IntersectionAboveBarAboveUnion => '⩉',
            SupplementalMathematicalOperators::UnionBesideAndJoinedWithUnion => '⩊',
            SupplementalMathematicalOperators::IntersectionBesideAndJoinedWithIntersection => '⩋',
            SupplementalMathematicalOperators::ClosedUnionWithSerifs => '⩌',
            SupplementalMathematicalOperators::ClosedIntersectionWithSerifs => '⩍',
            SupplementalMathematicalOperators::DoubleSquareIntersection => '⩎',
            SupplementalMathematicalOperators::DoubleSquareUnion => '⩏',
            SupplementalMathematicalOperators::ClosedUnionWithSerifsAndSmashProduct => '⩐',
            SupplementalMathematicalOperators::LogicalAndWithDotAbove => '⩑',
            SupplementalMathematicalOperators::LogicalOrWithDotAbove => '⩒',
            SupplementalMathematicalOperators::DoubleLogicalAnd => '⩓',
            SupplementalMathematicalOperators::DoubleLogicalOr => '⩔',
            SupplementalMathematicalOperators::TwoIntersectingLogicalAnd => '⩕',
            SupplementalMathematicalOperators::TwoIntersectingLogicalOr => '⩖',
            SupplementalMathematicalOperators::SlopingLargeOr => '⩗',
            SupplementalMathematicalOperators::SlopingLargeAnd => '⩘',
            SupplementalMathematicalOperators::LogicalOrOverlappingLogicalAnd => '⩙',
            SupplementalMathematicalOperators::LogicalAndWithMiddleStem => '⩚',
            SupplementalMathematicalOperators::LogicalOrWithMiddleStem => '⩛',
            SupplementalMathematicalOperators::LogicalAndWithHorizontalDash => '⩜',
            SupplementalMathematicalOperators::LogicalOrWithHorizontalDash => '⩝',
            SupplementalMathematicalOperators::LogicalAndWithDoubleOverbar => '⩞',
            SupplementalMathematicalOperators::LogicalAndWithUnderbar => '⩟',
            SupplementalMathematicalOperators::LogicalAndWithDoubleUnderbar => '⩠',
            SupplementalMathematicalOperators::SmallVeeWithUnderbar => '⩡',
            SupplementalMathematicalOperators::LogicalOrWithDoubleOverbar => '⩢',
            SupplementalMathematicalOperators::LogicalOrWithDoubleUnderbar => '⩣',
            SupplementalMathematicalOperators::ZNotationDomainAntirestriction => '⩤',
            SupplementalMathematicalOperators::ZNotationRangeAntirestriction => '⩥',
            SupplementalMathematicalOperators::EqualsSignWithDotBelow => '⩦',
            SupplementalMathematicalOperators::IdenticalWithDotAbove => '⩧',
            SupplementalMathematicalOperators::TripleHorizontalBarWithDoubleVerticalStroke => '⩨',
            SupplementalMathematicalOperators::TripleHorizontalBarWithTripleVerticalStroke => '⩩',
            SupplementalMathematicalOperators::TildeOperatorWithDotAbove => '⩪',
            SupplementalMathematicalOperators::TildeOperatorWithRisingDots => '⩫',
            SupplementalMathematicalOperators::SimilarMinusSimilar => '⩬',
            SupplementalMathematicalOperators::CongruentWithDotAbove => '⩭',
            SupplementalMathematicalOperators::EqualsWithAsterisk => '⩮',
            SupplementalMathematicalOperators::AlmostEqualToWithCircumflexAccent => '⩯',
            SupplementalMathematicalOperators::ApproximatelyEqualOrEqualTo => '⩰',
            SupplementalMathematicalOperators::EqualsSignAbovePlusSign => '⩱',
            SupplementalMathematicalOperators::PlusSignAboveEqualsSign => '⩲',
            SupplementalMathematicalOperators::EqualsSignAboveTildeOperator => '⩳',
            SupplementalMathematicalOperators::DoubleColonEqual => '⩴',
            SupplementalMathematicalOperators::TwoConsecutiveEqualsSigns => '⩵',
            SupplementalMathematicalOperators::ThreeConsecutiveEqualsSigns => '⩶',
            SupplementalMathematicalOperators::EqualsSignWithTwoDotsAboveAndTwoDotsBelow => '⩷',
            SupplementalMathematicalOperators::EquivalentWithFourDotsAbove => '⩸',
            SupplementalMathematicalOperators::LessDashThanWithCircleInside => '⩹',
            SupplementalMathematicalOperators::GreaterDashThanWithCircleInside => '⩺',
            SupplementalMathematicalOperators::LessDashThanWithQuestionMarkAbove => '⩻',
            SupplementalMathematicalOperators::GreaterDashThanWithQuestionMarkAbove => '⩼',
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualTo => '⩽',
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualTo => '⩾',
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotInside => '⩿',
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotInside => '⪀',
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAbove => '⪁',
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAbove => '⪂',
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAboveRight => '⪃',
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAboveLeft => '⪄',
            SupplementalMathematicalOperators::LessDashThanOrApproximate => '⪅',
            SupplementalMathematicalOperators::GreaterDashThanOrApproximate => '⪆',
            SupplementalMathematicalOperators::LessDashThanAndSingleDashLineNotEqualTo => '⪇',
            SupplementalMathematicalOperators::GreaterDashThanAndSingleDashLineNotEqualTo => '⪈',
            SupplementalMathematicalOperators::LessDashThanAndNotApproximate => '⪉',
            SupplementalMathematicalOperators::GreaterDashThanAndNotApproximate => '⪊',
            SupplementalMathematicalOperators::LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan => '⪋',
            SupplementalMathematicalOperators::GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan => '⪌',
            SupplementalMathematicalOperators::LessDashThanAboveSimilarOrEqual => '⪍',
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarOrEqual => '⪎',
            SupplementalMathematicalOperators::LessDashThanAboveSimilarAboveGreaterDashThan => '⪏',
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarAboveLessDashThan => '⪐',
            SupplementalMathematicalOperators::LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual => '⪑',
            SupplementalMathematicalOperators::GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual => '⪒',
            SupplementalMathematicalOperators::LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual => '⪓',
            SupplementalMathematicalOperators::GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual => '⪔',
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThan => '⪕',
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThan => '⪖',
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThanWithDotInside => '⪗',
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThanWithDotInside => '⪘',
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrLessDashThan => '⪙',
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrGreaterDashThan => '⪚',
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrLessDashThan => '⪛',
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrGreaterDashThan => '⪜',
            SupplementalMathematicalOperators::SimilarOrLessDashThan => '⪝',
            SupplementalMathematicalOperators::SimilarOrGreaterDashThan => '⪞',
            SupplementalMathematicalOperators::SimilarAboveLessDashThanAboveEqualsSign => '⪟',
            SupplementalMathematicalOperators::SimilarAboveGreaterDashThanAboveEqualsSign => '⪠',
            SupplementalMathematicalOperators::DoubleNestedLessDashThan => '⪡',
            SupplementalMathematicalOperators::DoubleNestedGreaterDashThan => '⪢',
            SupplementalMathematicalOperators::DoubleNestedLessDashThanWithUnderbar => '⪣',
            SupplementalMathematicalOperators::GreaterDashThanOverlappingLessDashThan => '⪤',
            SupplementalMathematicalOperators::GreaterDashThanBesideLessDashThan => '⪥',
            SupplementalMathematicalOperators::LessDashThanClosedByCurve => '⪦',
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurve => '⪧',
            SupplementalMathematicalOperators::LessDashThanClosedByCurveAboveSlantedEqual => '⪨',
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurveAboveSlantedEqual => '⪩',
            SupplementalMathematicalOperators::SmallerThan => '⪪',
            SupplementalMathematicalOperators::LargerThan => '⪫',
            SupplementalMathematicalOperators::SmallerThanOrEqualTo => '⪬',
            SupplementalMathematicalOperators::LargerThanOrEqualTo => '⪭',
            SupplementalMathematicalOperators::EqualsSignWithBumpyAbove => '⪮',
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineEqualsSign => '⪯',
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineEqualsSign => '⪰',
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineNotEqualTo => '⪱',
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineNotEqualTo => '⪲',
            SupplementalMathematicalOperators::PrecedesAboveEqualsSign => '⪳',
            SupplementalMathematicalOperators::SucceedsAboveEqualsSign => '⪴',
            SupplementalMathematicalOperators::PrecedesAboveNotEqualTo => '⪵',
            SupplementalMathematicalOperators::SucceedsAboveNotEqualTo => '⪶',
            SupplementalMathematicalOperators::PrecedesAboveAlmostEqualTo => '⪷',
            SupplementalMathematicalOperators::SucceedsAboveAlmostEqualTo => '⪸',
            SupplementalMathematicalOperators::PrecedesAboveNotAlmostEqualTo => '⪹',
            SupplementalMathematicalOperators::SucceedsAboveNotAlmostEqualTo => '⪺',
            SupplementalMathematicalOperators::DoublePrecedes => '⪻',
            SupplementalMathematicalOperators::DoubleSucceeds => '⪼',
            SupplementalMathematicalOperators::SubsetWithDot => '⪽',
            SupplementalMathematicalOperators::SupersetWithDot => '⪾',
            SupplementalMathematicalOperators::SubsetWithPlusSignBelow => '⪿',
            SupplementalMathematicalOperators::SupersetWithPlusSignBelow => '⫀',
            SupplementalMathematicalOperators::SubsetWithMultiplicationSignBelow => '⫁',
            SupplementalMathematicalOperators::SupersetWithMultiplicationSignBelow => '⫂',
            SupplementalMathematicalOperators::SubsetOfOrEqualToWithDotAbove => '⫃',
            SupplementalMathematicalOperators::SupersetOfOrEqualToWithDotAbove => '⫄',
            SupplementalMathematicalOperators::SubsetOfAboveEqualsSign => '⫅',
            SupplementalMathematicalOperators::SupersetOfAboveEqualsSign => '⫆',
            SupplementalMathematicalOperators::SubsetOfAboveTildeOperator => '⫇',
            SupplementalMathematicalOperators::SupersetOfAboveTildeOperator => '⫈',
            SupplementalMathematicalOperators::SubsetOfAboveAlmostEqualTo => '⫉',
            SupplementalMathematicalOperators::SupersetOfAboveAlmostEqualTo => '⫊',
            SupplementalMathematicalOperators::SubsetOfAboveNotEqualTo => '⫋',
            SupplementalMathematicalOperators::SupersetOfAboveNotEqualTo => '⫌',
            SupplementalMathematicalOperators::SquareLeftOpenBoxOperator => '⫍',
            SupplementalMathematicalOperators::SquareRightOpenBoxOperator => '⫎',
            SupplementalMathematicalOperators::ClosedSubset => '⫏',
            SupplementalMathematicalOperators::ClosedSuperset => '⫐',
            SupplementalMathematicalOperators::ClosedSubsetOrEqualTo => '⫑',
            SupplementalMathematicalOperators::ClosedSupersetOrEqualTo => '⫒',
            SupplementalMathematicalOperators::SubsetAboveSuperset => '⫓',
            SupplementalMathematicalOperators::SupersetAboveSubset => '⫔',
            SupplementalMathematicalOperators::SubsetAboveSubset => '⫕',
            SupplementalMathematicalOperators::SupersetAboveSuperset => '⫖',
            SupplementalMathematicalOperators::SupersetBesideSubset => '⫗',
            SupplementalMathematicalOperators::SupersetBesideAndJoinedByDashWithSubset => '⫘',
            SupplementalMathematicalOperators::ElementOfOpeningDownwards => '⫙',
            SupplementalMathematicalOperators::PitchforkWithTeeTop => '⫚',
            SupplementalMathematicalOperators::TransversalIntersection => '⫛',
            SupplementalMathematicalOperators::Forking => '⫝̸',
            SupplementalMathematicalOperators::Nonforking => '⫝',
            SupplementalMathematicalOperators::ShortLeftTack => '⫞',
            SupplementalMathematicalOperators::ShortDownTack => '⫟',
            SupplementalMathematicalOperators::ShortUpTack => '⫠',
            SupplementalMathematicalOperators::PerpendicularWithS => '⫡',
            SupplementalMathematicalOperators::VerticalBarTripleRightTurnstile => '⫢',
            SupplementalMathematicalOperators::DoubleVerticalBarLeftTurnstile => '⫣',
            SupplementalMathematicalOperators::VerticalBarDoubleLeftTurnstile => '⫤',
            SupplementalMathematicalOperators::DoubleVerticalBarDoubleLeftTurnstile => '⫥',
            SupplementalMathematicalOperators::LongDashFromLeftMemberOfDoubleVertical => '⫦',
            SupplementalMathematicalOperators::ShortDownTackWithOverbar => '⫧',
            SupplementalMathematicalOperators::ShortUpTackWithUnderbar => '⫨',
            SupplementalMathematicalOperators::ShortUpTackAboveShortDownTack => '⫩',
            SupplementalMathematicalOperators::DoubleDownTack => '⫪',
            SupplementalMathematicalOperators::DoubleUpTack => '⫫',
            SupplementalMathematicalOperators::DoubleStrokeNotSign => '⫬',
            SupplementalMathematicalOperators::ReversedDoubleStrokeNotSign => '⫭',
            SupplementalMathematicalOperators::DoesNotDivideWithReversedNegationSlash => '⫮',
            SupplementalMathematicalOperators::VerticalLineWithCircleAbove => '⫯',
            SupplementalMathematicalOperators::VerticalLineWithCircleBelow => '⫰',
            SupplementalMathematicalOperators::DownTackWithCircleBelow => '⫱',
            SupplementalMathematicalOperators::ParallelWithHorizontalStroke => '⫲',
            SupplementalMathematicalOperators::ParallelWithTildeOperator => '⫳',
            SupplementalMathematicalOperators::TripleVerticalBarBinaryRelation => '⫴',
            SupplementalMathematicalOperators::TripleVerticalBarWithHorizontalStroke => '⫵',
            SupplementalMathematicalOperators::TripleColonOperator => '⫶',
            SupplementalMathematicalOperators::TripleNestedLessDashThan => '⫷',
            SupplementalMathematicalOperators::TripleNestedGreaterDashThan => '⫸',
            SupplementalMathematicalOperators::DoubleDashLineSlantedLessDashThanOrEqualTo => '⫹',
            SupplementalMathematicalOperators::DoubleDashLineSlantedGreaterDashThanOrEqualTo => '⫺',
            SupplementalMathematicalOperators::TripleSolidusBinaryRelation => '⫻',
            SupplementalMathematicalOperators::LargeTripleVerticalBarOperator => '⫼',
            SupplementalMathematicalOperators::DoubleSolidusOperator => '⫽',
            SupplementalMathematicalOperators::WhiteVerticalBar => '⫾',
        }
    }
}

impl std::convert::TryFrom<char> for SupplementalMathematicalOperators {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⨀' => Ok(SupplementalMathematicalOperators::NDashAryCircledDotOperator),
            '⨁' => Ok(SupplementalMathematicalOperators::NDashAryCircledPlusOperator),
            '⨂' => Ok(SupplementalMathematicalOperators::NDashAryCircledTimesOperator),
            '⨃' => Ok(SupplementalMathematicalOperators::NDashAryUnionOperatorWithDot),
            '⨄' => Ok(SupplementalMathematicalOperators::NDashAryUnionOperatorWithPlus),
            '⨅' => Ok(SupplementalMathematicalOperators::NDashArySquareIntersectionOperator),
            '⨆' => Ok(SupplementalMathematicalOperators::NDashArySquareUnionOperator),
            '⨇' => Ok(SupplementalMathematicalOperators::TwoLogicalAndOperator),
            '⨈' => Ok(SupplementalMathematicalOperators::TwoLogicalOrOperator),
            '⨉' => Ok(SupplementalMathematicalOperators::NDashAryTimesOperator),
            '⨊' => Ok(SupplementalMathematicalOperators::ModuloTwoSum),
            '⨋' => Ok(SupplementalMathematicalOperators::SummationWithIntegral),
            '⨌' => Ok(SupplementalMathematicalOperators::QuadrupleIntegralOperator),
            '⨍' => Ok(SupplementalMathematicalOperators::FinitePartIntegral),
            '⨎' => Ok(SupplementalMathematicalOperators::IntegralWithDoubleStroke),
            '⨏' => Ok(SupplementalMathematicalOperators::IntegralAverageWithSlash),
            '⨐' => Ok(SupplementalMathematicalOperators::CirculationFunction),
            '⨑' => Ok(SupplementalMathematicalOperators::AnticlockwiseIntegration),
            '⨒' => Ok(SupplementalMathematicalOperators::LineIntegrationWithRectangularPathAroundPole),
            '⨓' => Ok(SupplementalMathematicalOperators::LineIntegrationWithSemicircularPathAroundPole),
            '⨔' => Ok(SupplementalMathematicalOperators::LineIntegrationNotIncludingThePole),
            '⨕' => Ok(SupplementalMathematicalOperators::IntegralAroundAPointOperator),
            '⨖' => Ok(SupplementalMathematicalOperators::QuaternionIntegralOperator),
            '⨗' => Ok(SupplementalMathematicalOperators::IntegralWithLeftwardsArrowWithHook),
            '⨘' => Ok(SupplementalMathematicalOperators::IntegralWithTimesSign),
            '⨙' => Ok(SupplementalMathematicalOperators::IntegralWithIntersection),
            '⨚' => Ok(SupplementalMathematicalOperators::IntegralWithUnion),
            '⨛' => Ok(SupplementalMathematicalOperators::IntegralWithOverbar),
            '⨜' => Ok(SupplementalMathematicalOperators::IntegralWithUnderbar),
            '⨝' => Ok(SupplementalMathematicalOperators::Join),
            '⨞' => Ok(SupplementalMathematicalOperators::LargeLeftTriangleOperator),
            '⨟' => Ok(SupplementalMathematicalOperators::ZNotationSchemaComposition),
            '⨠' => Ok(SupplementalMathematicalOperators::ZNotationSchemaPiping),
            '⨡' => Ok(SupplementalMathematicalOperators::ZNotationSchemaProjection),
            '⨢' => Ok(SupplementalMathematicalOperators::PlusSignWithSmallCircleAbove),
            '⨣' => Ok(SupplementalMathematicalOperators::PlusSignWithCircumflexAccentAbove),
            '⨤' => Ok(SupplementalMathematicalOperators::PlusSignWithTildeAbove),
            '⨥' => Ok(SupplementalMathematicalOperators::PlusSignWithDotBelow),
            '⨦' => Ok(SupplementalMathematicalOperators::PlusSignWithTildeBelow),
            '⨧' => Ok(SupplementalMathematicalOperators::PlusSignWithSubscriptTwo),
            '⨨' => Ok(SupplementalMathematicalOperators::PlusSignWithBlackTriangle),
            '⨩' => Ok(SupplementalMathematicalOperators::MinusSignWithCommaAbove),
            '⨪' => Ok(SupplementalMathematicalOperators::MinusSignWithDotBelow),
            '⨫' => Ok(SupplementalMathematicalOperators::MinusSignWithFallingDots),
            '⨬' => Ok(SupplementalMathematicalOperators::MinusSignWithRisingDots),
            '⨭' => Ok(SupplementalMathematicalOperators::PlusSignInLeftHalfCircle),
            '⨮' => Ok(SupplementalMathematicalOperators::PlusSignInRightHalfCircle),
            '⨯' => Ok(SupplementalMathematicalOperators::VectorOrCrossProduct),
            '⨰' => Ok(SupplementalMathematicalOperators::MultiplicationSignWithDotAbove),
            '⨱' => Ok(SupplementalMathematicalOperators::MultiplicationSignWithUnderbar),
            '⨲' => Ok(SupplementalMathematicalOperators::SemidirectProductWithBottomClosed),
            '⨳' => Ok(SupplementalMathematicalOperators::SmashProduct),
            '⨴' => Ok(SupplementalMathematicalOperators::MultiplicationSignInLeftHalfCircle),
            '⨵' => Ok(SupplementalMathematicalOperators::MultiplicationSignInRightHalfCircle),
            '⨶' => Ok(SupplementalMathematicalOperators::CircledMultiplicationSignWithCircumflexAccent),
            '⨷' => Ok(SupplementalMathematicalOperators::MultiplicationSignInDoubleCircle),
            '⨸' => Ok(SupplementalMathematicalOperators::CircledDivisionSign),
            '⨹' => Ok(SupplementalMathematicalOperators::PlusSignInTriangle),
            '⨺' => Ok(SupplementalMathematicalOperators::MinusSignInTriangle),
            '⨻' => Ok(SupplementalMathematicalOperators::MultiplicationSignInTriangle),
            '⨼' => Ok(SupplementalMathematicalOperators::InteriorProduct),
            '⨽' => Ok(SupplementalMathematicalOperators::RighthandInteriorProduct),
            '⨾' => Ok(SupplementalMathematicalOperators::ZNotationRelationalComposition),
            '⨿' => Ok(SupplementalMathematicalOperators::AmalgamationOrCoproduct),
            '⩀' => Ok(SupplementalMathematicalOperators::IntersectionWithDot),
            '⩁' => Ok(SupplementalMathematicalOperators::UnionWithMinusSign),
            '⩂' => Ok(SupplementalMathematicalOperators::UnionWithOverbar),
            '⩃' => Ok(SupplementalMathematicalOperators::IntersectionWithOverbar),
            '⩄' => Ok(SupplementalMathematicalOperators::IntersectionWithLogicalAnd),
            '⩅' => Ok(SupplementalMathematicalOperators::UnionWithLogicalOr),
            '⩆' => Ok(SupplementalMathematicalOperators::UnionAboveIntersection),
            '⩇' => Ok(SupplementalMathematicalOperators::IntersectionAboveUnion),
            '⩈' => Ok(SupplementalMathematicalOperators::UnionAboveBarAboveIntersection),
            '⩉' => Ok(SupplementalMathematicalOperators::IntersectionAboveBarAboveUnion),
            '⩊' => Ok(SupplementalMathematicalOperators::UnionBesideAndJoinedWithUnion),
            '⩋' => Ok(SupplementalMathematicalOperators::IntersectionBesideAndJoinedWithIntersection),
            '⩌' => Ok(SupplementalMathematicalOperators::ClosedUnionWithSerifs),
            '⩍' => Ok(SupplementalMathematicalOperators::ClosedIntersectionWithSerifs),
            '⩎' => Ok(SupplementalMathematicalOperators::DoubleSquareIntersection),
            '⩏' => Ok(SupplementalMathematicalOperators::DoubleSquareUnion),
            '⩐' => Ok(SupplementalMathematicalOperators::ClosedUnionWithSerifsAndSmashProduct),
            '⩑' => Ok(SupplementalMathematicalOperators::LogicalAndWithDotAbove),
            '⩒' => Ok(SupplementalMathematicalOperators::LogicalOrWithDotAbove),
            '⩓' => Ok(SupplementalMathematicalOperators::DoubleLogicalAnd),
            '⩔' => Ok(SupplementalMathematicalOperators::DoubleLogicalOr),
            '⩕' => Ok(SupplementalMathematicalOperators::TwoIntersectingLogicalAnd),
            '⩖' => Ok(SupplementalMathematicalOperators::TwoIntersectingLogicalOr),
            '⩗' => Ok(SupplementalMathematicalOperators::SlopingLargeOr),
            '⩘' => Ok(SupplementalMathematicalOperators::SlopingLargeAnd),
            '⩙' => Ok(SupplementalMathematicalOperators::LogicalOrOverlappingLogicalAnd),
            '⩚' => Ok(SupplementalMathematicalOperators::LogicalAndWithMiddleStem),
            '⩛' => Ok(SupplementalMathematicalOperators::LogicalOrWithMiddleStem),
            '⩜' => Ok(SupplementalMathematicalOperators::LogicalAndWithHorizontalDash),
            '⩝' => Ok(SupplementalMathematicalOperators::LogicalOrWithHorizontalDash),
            '⩞' => Ok(SupplementalMathematicalOperators::LogicalAndWithDoubleOverbar),
            '⩟' => Ok(SupplementalMathematicalOperators::LogicalAndWithUnderbar),
            '⩠' => Ok(SupplementalMathematicalOperators::LogicalAndWithDoubleUnderbar),
            '⩡' => Ok(SupplementalMathematicalOperators::SmallVeeWithUnderbar),
            '⩢' => Ok(SupplementalMathematicalOperators::LogicalOrWithDoubleOverbar),
            '⩣' => Ok(SupplementalMathematicalOperators::LogicalOrWithDoubleUnderbar),
            '⩤' => Ok(SupplementalMathematicalOperators::ZNotationDomainAntirestriction),
            '⩥' => Ok(SupplementalMathematicalOperators::ZNotationRangeAntirestriction),
            '⩦' => Ok(SupplementalMathematicalOperators::EqualsSignWithDotBelow),
            '⩧' => Ok(SupplementalMathematicalOperators::IdenticalWithDotAbove),
            '⩨' => Ok(SupplementalMathematicalOperators::TripleHorizontalBarWithDoubleVerticalStroke),
            '⩩' => Ok(SupplementalMathematicalOperators::TripleHorizontalBarWithTripleVerticalStroke),
            '⩪' => Ok(SupplementalMathematicalOperators::TildeOperatorWithDotAbove),
            '⩫' => Ok(SupplementalMathematicalOperators::TildeOperatorWithRisingDots),
            '⩬' => Ok(SupplementalMathematicalOperators::SimilarMinusSimilar),
            '⩭' => Ok(SupplementalMathematicalOperators::CongruentWithDotAbove),
            '⩮' => Ok(SupplementalMathematicalOperators::EqualsWithAsterisk),
            '⩯' => Ok(SupplementalMathematicalOperators::AlmostEqualToWithCircumflexAccent),
            '⩰' => Ok(SupplementalMathematicalOperators::ApproximatelyEqualOrEqualTo),
            '⩱' => Ok(SupplementalMathematicalOperators::EqualsSignAbovePlusSign),
            '⩲' => Ok(SupplementalMathematicalOperators::PlusSignAboveEqualsSign),
            '⩳' => Ok(SupplementalMathematicalOperators::EqualsSignAboveTildeOperator),
            '⩴' => Ok(SupplementalMathematicalOperators::DoubleColonEqual),
            '⩵' => Ok(SupplementalMathematicalOperators::TwoConsecutiveEqualsSigns),
            '⩶' => Ok(SupplementalMathematicalOperators::ThreeConsecutiveEqualsSigns),
            '⩷' => Ok(SupplementalMathematicalOperators::EqualsSignWithTwoDotsAboveAndTwoDotsBelow),
            '⩸' => Ok(SupplementalMathematicalOperators::EquivalentWithFourDotsAbove),
            '⩹' => Ok(SupplementalMathematicalOperators::LessDashThanWithCircleInside),
            '⩺' => Ok(SupplementalMathematicalOperators::GreaterDashThanWithCircleInside),
            '⩻' => Ok(SupplementalMathematicalOperators::LessDashThanWithQuestionMarkAbove),
            '⩼' => Ok(SupplementalMathematicalOperators::GreaterDashThanWithQuestionMarkAbove),
            '⩽' => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualTo),
            '⩾' => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualTo),
            '⩿' => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotInside),
            '⪀' => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotInside),
            '⪁' => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAbove),
            '⪂' => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAbove),
            '⪃' => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAboveRight),
            '⪄' => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAboveLeft),
            '⪅' => Ok(SupplementalMathematicalOperators::LessDashThanOrApproximate),
            '⪆' => Ok(SupplementalMathematicalOperators::GreaterDashThanOrApproximate),
            '⪇' => Ok(SupplementalMathematicalOperators::LessDashThanAndSingleDashLineNotEqualTo),
            '⪈' => Ok(SupplementalMathematicalOperators::GreaterDashThanAndSingleDashLineNotEqualTo),
            '⪉' => Ok(SupplementalMathematicalOperators::LessDashThanAndNotApproximate),
            '⪊' => Ok(SupplementalMathematicalOperators::GreaterDashThanAndNotApproximate),
            '⪋' => Ok(SupplementalMathematicalOperators::LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan),
            '⪌' => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan),
            '⪍' => Ok(SupplementalMathematicalOperators::LessDashThanAboveSimilarOrEqual),
            '⪎' => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveSimilarOrEqual),
            '⪏' => Ok(SupplementalMathematicalOperators::LessDashThanAboveSimilarAboveGreaterDashThan),
            '⪐' => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveSimilarAboveLessDashThan),
            '⪑' => Ok(SupplementalMathematicalOperators::LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual),
            '⪒' => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual),
            '⪓' => Ok(SupplementalMathematicalOperators::LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual),
            '⪔' => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual),
            '⪕' => Ok(SupplementalMathematicalOperators::SlantedEqualToOrLessDashThan),
            '⪖' => Ok(SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThan),
            '⪗' => Ok(SupplementalMathematicalOperators::SlantedEqualToOrLessDashThanWithDotInside),
            '⪘' => Ok(SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThanWithDotInside),
            '⪙' => Ok(SupplementalMathematicalOperators::DoubleDashLineEqualToOrLessDashThan),
            '⪚' => Ok(SupplementalMathematicalOperators::DoubleDashLineEqualToOrGreaterDashThan),
            '⪛' => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrLessDashThan),
            '⪜' => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrGreaterDashThan),
            '⪝' => Ok(SupplementalMathematicalOperators::SimilarOrLessDashThan),
            '⪞' => Ok(SupplementalMathematicalOperators::SimilarOrGreaterDashThan),
            '⪟' => Ok(SupplementalMathematicalOperators::SimilarAboveLessDashThanAboveEqualsSign),
            '⪠' => Ok(SupplementalMathematicalOperators::SimilarAboveGreaterDashThanAboveEqualsSign),
            '⪡' => Ok(SupplementalMathematicalOperators::DoubleNestedLessDashThan),
            '⪢' => Ok(SupplementalMathematicalOperators::DoubleNestedGreaterDashThan),
            '⪣' => Ok(SupplementalMathematicalOperators::DoubleNestedLessDashThanWithUnderbar),
            '⪤' => Ok(SupplementalMathematicalOperators::GreaterDashThanOverlappingLessDashThan),
            '⪥' => Ok(SupplementalMathematicalOperators::GreaterDashThanBesideLessDashThan),
            '⪦' => Ok(SupplementalMathematicalOperators::LessDashThanClosedByCurve),
            '⪧' => Ok(SupplementalMathematicalOperators::GreaterDashThanClosedByCurve),
            '⪨' => Ok(SupplementalMathematicalOperators::LessDashThanClosedByCurveAboveSlantedEqual),
            '⪩' => Ok(SupplementalMathematicalOperators::GreaterDashThanClosedByCurveAboveSlantedEqual),
            '⪪' => Ok(SupplementalMathematicalOperators::SmallerThan),
            '⪫' => Ok(SupplementalMathematicalOperators::LargerThan),
            '⪬' => Ok(SupplementalMathematicalOperators::SmallerThanOrEqualTo),
            '⪭' => Ok(SupplementalMathematicalOperators::LargerThanOrEqualTo),
            '⪮' => Ok(SupplementalMathematicalOperators::EqualsSignWithBumpyAbove),
            '⪯' => Ok(SupplementalMathematicalOperators::PrecedesAboveSingleDashLineEqualsSign),
            '⪰' => Ok(SupplementalMathematicalOperators::SucceedsAboveSingleDashLineEqualsSign),
            '⪱' => Ok(SupplementalMathematicalOperators::PrecedesAboveSingleDashLineNotEqualTo),
            '⪲' => Ok(SupplementalMathematicalOperators::SucceedsAboveSingleDashLineNotEqualTo),
            '⪳' => Ok(SupplementalMathematicalOperators::PrecedesAboveEqualsSign),
            '⪴' => Ok(SupplementalMathematicalOperators::SucceedsAboveEqualsSign),
            '⪵' => Ok(SupplementalMathematicalOperators::PrecedesAboveNotEqualTo),
            '⪶' => Ok(SupplementalMathematicalOperators::SucceedsAboveNotEqualTo),
            '⪷' => Ok(SupplementalMathematicalOperators::PrecedesAboveAlmostEqualTo),
            '⪸' => Ok(SupplementalMathematicalOperators::SucceedsAboveAlmostEqualTo),
            '⪹' => Ok(SupplementalMathematicalOperators::PrecedesAboveNotAlmostEqualTo),
            '⪺' => Ok(SupplementalMathematicalOperators::SucceedsAboveNotAlmostEqualTo),
            '⪻' => Ok(SupplementalMathematicalOperators::DoublePrecedes),
            '⪼' => Ok(SupplementalMathematicalOperators::DoubleSucceeds),
            '⪽' => Ok(SupplementalMathematicalOperators::SubsetWithDot),
            '⪾' => Ok(SupplementalMathematicalOperators::SupersetWithDot),
            '⪿' => Ok(SupplementalMathematicalOperators::SubsetWithPlusSignBelow),
            '⫀' => Ok(SupplementalMathematicalOperators::SupersetWithPlusSignBelow),
            '⫁' => Ok(SupplementalMathematicalOperators::SubsetWithMultiplicationSignBelow),
            '⫂' => Ok(SupplementalMathematicalOperators::SupersetWithMultiplicationSignBelow),
            '⫃' => Ok(SupplementalMathematicalOperators::SubsetOfOrEqualToWithDotAbove),
            '⫄' => Ok(SupplementalMathematicalOperators::SupersetOfOrEqualToWithDotAbove),
            '⫅' => Ok(SupplementalMathematicalOperators::SubsetOfAboveEqualsSign),
            '⫆' => Ok(SupplementalMathematicalOperators::SupersetOfAboveEqualsSign),
            '⫇' => Ok(SupplementalMathematicalOperators::SubsetOfAboveTildeOperator),
            '⫈' => Ok(SupplementalMathematicalOperators::SupersetOfAboveTildeOperator),
            '⫉' => Ok(SupplementalMathematicalOperators::SubsetOfAboveAlmostEqualTo),
            '⫊' => Ok(SupplementalMathematicalOperators::SupersetOfAboveAlmostEqualTo),
            '⫋' => Ok(SupplementalMathematicalOperators::SubsetOfAboveNotEqualTo),
            '⫌' => Ok(SupplementalMathematicalOperators::SupersetOfAboveNotEqualTo),
            '⫍' => Ok(SupplementalMathematicalOperators::SquareLeftOpenBoxOperator),
            '⫎' => Ok(SupplementalMathematicalOperators::SquareRightOpenBoxOperator),
            '⫏' => Ok(SupplementalMathematicalOperators::ClosedSubset),
            '⫐' => Ok(SupplementalMathematicalOperators::ClosedSuperset),
            '⫑' => Ok(SupplementalMathematicalOperators::ClosedSubsetOrEqualTo),
            '⫒' => Ok(SupplementalMathematicalOperators::ClosedSupersetOrEqualTo),
            '⫓' => Ok(SupplementalMathematicalOperators::SubsetAboveSuperset),
            '⫔' => Ok(SupplementalMathematicalOperators::SupersetAboveSubset),
            '⫕' => Ok(SupplementalMathematicalOperators::SubsetAboveSubset),
            '⫖' => Ok(SupplementalMathematicalOperators::SupersetAboveSuperset),
            '⫗' => Ok(SupplementalMathematicalOperators::SupersetBesideSubset),
            '⫘' => Ok(SupplementalMathematicalOperators::SupersetBesideAndJoinedByDashWithSubset),
            '⫙' => Ok(SupplementalMathematicalOperators::ElementOfOpeningDownwards),
            '⫚' => Ok(SupplementalMathematicalOperators::PitchforkWithTeeTop),
            '⫛' => Ok(SupplementalMathematicalOperators::TransversalIntersection),
            '⫝̸' => Ok(SupplementalMathematicalOperators::Forking),
            '⫝' => Ok(SupplementalMathematicalOperators::Nonforking),
            '⫞' => Ok(SupplementalMathematicalOperators::ShortLeftTack),
            '⫟' => Ok(SupplementalMathematicalOperators::ShortDownTack),
            '⫠' => Ok(SupplementalMathematicalOperators::ShortUpTack),
            '⫡' => Ok(SupplementalMathematicalOperators::PerpendicularWithS),
            '⫢' => Ok(SupplementalMathematicalOperators::VerticalBarTripleRightTurnstile),
            '⫣' => Ok(SupplementalMathematicalOperators::DoubleVerticalBarLeftTurnstile),
            '⫤' => Ok(SupplementalMathematicalOperators::VerticalBarDoubleLeftTurnstile),
            '⫥' => Ok(SupplementalMathematicalOperators::DoubleVerticalBarDoubleLeftTurnstile),
            '⫦' => Ok(SupplementalMathematicalOperators::LongDashFromLeftMemberOfDoubleVertical),
            '⫧' => Ok(SupplementalMathematicalOperators::ShortDownTackWithOverbar),
            '⫨' => Ok(SupplementalMathematicalOperators::ShortUpTackWithUnderbar),
            '⫩' => Ok(SupplementalMathematicalOperators::ShortUpTackAboveShortDownTack),
            '⫪' => Ok(SupplementalMathematicalOperators::DoubleDownTack),
            '⫫' => Ok(SupplementalMathematicalOperators::DoubleUpTack),
            '⫬' => Ok(SupplementalMathematicalOperators::DoubleStrokeNotSign),
            '⫭' => Ok(SupplementalMathematicalOperators::ReversedDoubleStrokeNotSign),
            '⫮' => Ok(SupplementalMathematicalOperators::DoesNotDivideWithReversedNegationSlash),
            '⫯' => Ok(SupplementalMathematicalOperators::VerticalLineWithCircleAbove),
            '⫰' => Ok(SupplementalMathematicalOperators::VerticalLineWithCircleBelow),
            '⫱' => Ok(SupplementalMathematicalOperators::DownTackWithCircleBelow),
            '⫲' => Ok(SupplementalMathematicalOperators::ParallelWithHorizontalStroke),
            '⫳' => Ok(SupplementalMathematicalOperators::ParallelWithTildeOperator),
            '⫴' => Ok(SupplementalMathematicalOperators::TripleVerticalBarBinaryRelation),
            '⫵' => Ok(SupplementalMathematicalOperators::TripleVerticalBarWithHorizontalStroke),
            '⫶' => Ok(SupplementalMathematicalOperators::TripleColonOperator),
            '⫷' => Ok(SupplementalMathematicalOperators::TripleNestedLessDashThan),
            '⫸' => Ok(SupplementalMathematicalOperators::TripleNestedGreaterDashThan),
            '⫹' => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedLessDashThanOrEqualTo),
            '⫺' => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedGreaterDashThanOrEqualTo),
            '⫻' => Ok(SupplementalMathematicalOperators::TripleSolidusBinaryRelation),
            '⫼' => Ok(SupplementalMathematicalOperators::LargeTripleVerticalBarOperator),
            '⫽' => Ok(SupplementalMathematicalOperators::DoubleSolidusOperator),
            '⫾' => Ok(SupplementalMathematicalOperators::WhiteVerticalBar),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SupplementalMathematicalOperators {
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

impl std::convert::TryFrom<u32> for SupplementalMathematicalOperators {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SupplementalMathematicalOperators {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SupplementalMathematicalOperators {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SupplementalMathematicalOperators::NDashAryCircledDotOperator
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SupplementalMathematicalOperators{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
