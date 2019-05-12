
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SupplementalMathematicalOperators::NDashAryCircledDotOperator => "n-ary circled dot operator",
            SupplementalMathematicalOperators::NDashAryCircledPlusOperator => "n-ary circled plus operator",
            SupplementalMathematicalOperators::NDashAryCircledTimesOperator => "n-ary circled times operator",
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithDot => "n-ary union operator with dot",
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithPlus => "n-ary union operator with plus",
            SupplementalMathematicalOperators::NDashArySquareIntersectionOperator => "n-ary square intersection operator",
            SupplementalMathematicalOperators::NDashArySquareUnionOperator => "n-ary square union operator",
            SupplementalMathematicalOperators::TwoLogicalAndOperator => "two logical and operator",
            SupplementalMathematicalOperators::TwoLogicalOrOperator => "two logical or operator",
            SupplementalMathematicalOperators::NDashAryTimesOperator => "n-ary times operator",
            SupplementalMathematicalOperators::ModuloTwoSum => "modulo two sum",
            SupplementalMathematicalOperators::SummationWithIntegral => "summation with integral",
            SupplementalMathematicalOperators::QuadrupleIntegralOperator => "quadruple integral operator",
            SupplementalMathematicalOperators::FinitePartIntegral => "finite part integral",
            SupplementalMathematicalOperators::IntegralWithDoubleStroke => "integral with double stroke",
            SupplementalMathematicalOperators::IntegralAverageWithSlash => "integral average with slash",
            SupplementalMathematicalOperators::CirculationFunction => "circulation function",
            SupplementalMathematicalOperators::AnticlockwiseIntegration => "anticlockwise integration",
            SupplementalMathematicalOperators::LineIntegrationWithRectangularPathAroundPole => "line integration with rectangular path around pole",
            SupplementalMathematicalOperators::LineIntegrationWithSemicircularPathAroundPole => "line integration with semicircular path around pole",
            SupplementalMathematicalOperators::LineIntegrationNotIncludingThePole => "line integration not including the pole",
            SupplementalMathematicalOperators::IntegralAroundAPointOperator => "integral around a point operator",
            SupplementalMathematicalOperators::QuaternionIntegralOperator => "quaternion integral operator",
            SupplementalMathematicalOperators::IntegralWithLeftwardsArrowWithHook => "integral with leftwards arrow with hook",
            SupplementalMathematicalOperators::IntegralWithTimesSign => "integral with times sign",
            SupplementalMathematicalOperators::IntegralWithIntersection => "integral with intersection",
            SupplementalMathematicalOperators::IntegralWithUnion => "integral with union",
            SupplementalMathematicalOperators::IntegralWithOverbar => "integral with overbar",
            SupplementalMathematicalOperators::IntegralWithUnderbar => "integral with underbar",
            SupplementalMathematicalOperators::Join => "join",
            SupplementalMathematicalOperators::LargeLeftTriangleOperator => "large left triangle operator",
            SupplementalMathematicalOperators::ZNotationSchemaComposition => "z notation schema composition",
            SupplementalMathematicalOperators::ZNotationSchemaPiping => "z notation schema piping",
            SupplementalMathematicalOperators::ZNotationSchemaProjection => "z notation schema projection",
            SupplementalMathematicalOperators::PlusSignWithSmallCircleAbove => "plus sign with small circle above",
            SupplementalMathematicalOperators::PlusSignWithCircumflexAccentAbove => "plus sign with circumflex accent above",
            SupplementalMathematicalOperators::PlusSignWithTildeAbove => "plus sign with tilde above",
            SupplementalMathematicalOperators::PlusSignWithDotBelow => "plus sign with dot below",
            SupplementalMathematicalOperators::PlusSignWithTildeBelow => "plus sign with tilde below",
            SupplementalMathematicalOperators::PlusSignWithSubscriptTwo => "plus sign with subscript two",
            SupplementalMathematicalOperators::PlusSignWithBlackTriangle => "plus sign with black triangle",
            SupplementalMathematicalOperators::MinusSignWithCommaAbove => "minus sign with comma above",
            SupplementalMathematicalOperators::MinusSignWithDotBelow => "minus sign with dot below",
            SupplementalMathematicalOperators::MinusSignWithFallingDots => "minus sign with falling dots",
            SupplementalMathematicalOperators::MinusSignWithRisingDots => "minus sign with rising dots",
            SupplementalMathematicalOperators::PlusSignInLeftHalfCircle => "plus sign in left half circle",
            SupplementalMathematicalOperators::PlusSignInRightHalfCircle => "plus sign in right half circle",
            SupplementalMathematicalOperators::VectorOrCrossProduct => "vector or cross product",
            SupplementalMathematicalOperators::MultiplicationSignWithDotAbove => "multiplication sign with dot above",
            SupplementalMathematicalOperators::MultiplicationSignWithUnderbar => "multiplication sign with underbar",
            SupplementalMathematicalOperators::SemidirectProductWithBottomClosed => "semidirect product with bottom closed",
            SupplementalMathematicalOperators::SmashProduct => "smash product",
            SupplementalMathematicalOperators::MultiplicationSignInLeftHalfCircle => "multiplication sign in left half circle",
            SupplementalMathematicalOperators::MultiplicationSignInRightHalfCircle => "multiplication sign in right half circle",
            SupplementalMathematicalOperators::CircledMultiplicationSignWithCircumflexAccent => "circled multiplication sign with circumflex accent",
            SupplementalMathematicalOperators::MultiplicationSignInDoubleCircle => "multiplication sign in double circle",
            SupplementalMathematicalOperators::CircledDivisionSign => "circled division sign",
            SupplementalMathematicalOperators::PlusSignInTriangle => "plus sign in triangle",
            SupplementalMathematicalOperators::MinusSignInTriangle => "minus sign in triangle",
            SupplementalMathematicalOperators::MultiplicationSignInTriangle => "multiplication sign in triangle",
            SupplementalMathematicalOperators::InteriorProduct => "interior product",
            SupplementalMathematicalOperators::RighthandInteriorProduct => "righthand interior product",
            SupplementalMathematicalOperators::ZNotationRelationalComposition => "z notation relational composition",
            SupplementalMathematicalOperators::AmalgamationOrCoproduct => "amalgamation or coproduct",
            SupplementalMathematicalOperators::IntersectionWithDot => "intersection with dot",
            SupplementalMathematicalOperators::UnionWithMinusSign => "union with minus sign",
            SupplementalMathematicalOperators::UnionWithOverbar => "union with overbar",
            SupplementalMathematicalOperators::IntersectionWithOverbar => "intersection with overbar",
            SupplementalMathematicalOperators::IntersectionWithLogicalAnd => "intersection with logical and",
            SupplementalMathematicalOperators::UnionWithLogicalOr => "union with logical or",
            SupplementalMathematicalOperators::UnionAboveIntersection => "union above intersection",
            SupplementalMathematicalOperators::IntersectionAboveUnion => "intersection above union",
            SupplementalMathematicalOperators::UnionAboveBarAboveIntersection => "union above bar above intersection",
            SupplementalMathematicalOperators::IntersectionAboveBarAboveUnion => "intersection above bar above union",
            SupplementalMathematicalOperators::UnionBesideAndJoinedWithUnion => "union beside and joined with union",
            SupplementalMathematicalOperators::IntersectionBesideAndJoinedWithIntersection => "intersection beside and joined with intersection",
            SupplementalMathematicalOperators::ClosedUnionWithSerifs => "closed union with serifs",
            SupplementalMathematicalOperators::ClosedIntersectionWithSerifs => "closed intersection with serifs",
            SupplementalMathematicalOperators::DoubleSquareIntersection => "double square intersection",
            SupplementalMathematicalOperators::DoubleSquareUnion => "double square union",
            SupplementalMathematicalOperators::ClosedUnionWithSerifsAndSmashProduct => "closed union with serifs and smash product",
            SupplementalMathematicalOperators::LogicalAndWithDotAbove => "logical and with dot above",
            SupplementalMathematicalOperators::LogicalOrWithDotAbove => "logical or with dot above",
            SupplementalMathematicalOperators::DoubleLogicalAnd => "double logical and",
            SupplementalMathematicalOperators::DoubleLogicalOr => "double logical or",
            SupplementalMathematicalOperators::TwoIntersectingLogicalAnd => "two intersecting logical and",
            SupplementalMathematicalOperators::TwoIntersectingLogicalOr => "two intersecting logical or",
            SupplementalMathematicalOperators::SlopingLargeOr => "sloping large or",
            SupplementalMathematicalOperators::SlopingLargeAnd => "sloping large and",
            SupplementalMathematicalOperators::LogicalOrOverlappingLogicalAnd => "logical or overlapping logical and",
            SupplementalMathematicalOperators::LogicalAndWithMiddleStem => "logical and with middle stem",
            SupplementalMathematicalOperators::LogicalOrWithMiddleStem => "logical or with middle stem",
            SupplementalMathematicalOperators::LogicalAndWithHorizontalDash => "logical and with horizontal dash",
            SupplementalMathematicalOperators::LogicalOrWithHorizontalDash => "logical or with horizontal dash",
            SupplementalMathematicalOperators::LogicalAndWithDoubleOverbar => "logical and with double overbar",
            SupplementalMathematicalOperators::LogicalAndWithUnderbar => "logical and with underbar",
            SupplementalMathematicalOperators::LogicalAndWithDoubleUnderbar => "logical and with double underbar",
            SupplementalMathematicalOperators::SmallVeeWithUnderbar => "small vee with underbar",
            SupplementalMathematicalOperators::LogicalOrWithDoubleOverbar => "logical or with double overbar",
            SupplementalMathematicalOperators::LogicalOrWithDoubleUnderbar => "logical or with double underbar",
            SupplementalMathematicalOperators::ZNotationDomainAntirestriction => "z notation domain antirestriction",
            SupplementalMathematicalOperators::ZNotationRangeAntirestriction => "z notation range antirestriction",
            SupplementalMathematicalOperators::EqualsSignWithDotBelow => "equals sign with dot below",
            SupplementalMathematicalOperators::IdenticalWithDotAbove => "identical with dot above",
            SupplementalMathematicalOperators::TripleHorizontalBarWithDoubleVerticalStroke => "triple horizontal bar with double vertical stroke",
            SupplementalMathematicalOperators::TripleHorizontalBarWithTripleVerticalStroke => "triple horizontal bar with triple vertical stroke",
            SupplementalMathematicalOperators::TildeOperatorWithDotAbove => "tilde operator with dot above",
            SupplementalMathematicalOperators::TildeOperatorWithRisingDots => "tilde operator with rising dots",
            SupplementalMathematicalOperators::SimilarMinusSimilar => "similar minus similar",
            SupplementalMathematicalOperators::CongruentWithDotAbove => "congruent with dot above",
            SupplementalMathematicalOperators::EqualsWithAsterisk => "equals with asterisk",
            SupplementalMathematicalOperators::AlmostEqualToWithCircumflexAccent => "almost equal to with circumflex accent",
            SupplementalMathematicalOperators::ApproximatelyEqualOrEqualTo => "approximately equal or equal to",
            SupplementalMathematicalOperators::EqualsSignAbovePlusSign => "equals sign above plus sign",
            SupplementalMathematicalOperators::PlusSignAboveEqualsSign => "plus sign above equals sign",
            SupplementalMathematicalOperators::EqualsSignAboveTildeOperator => "equals sign above tilde operator",
            SupplementalMathematicalOperators::DoubleColonEqual => "double colon equal",
            SupplementalMathematicalOperators::TwoConsecutiveEqualsSigns => "two consecutive equals signs",
            SupplementalMathematicalOperators::ThreeConsecutiveEqualsSigns => "three consecutive equals signs",
            SupplementalMathematicalOperators::EqualsSignWithTwoDotsAboveAndTwoDotsBelow => "equals sign with two dots above and two dots below",
            SupplementalMathematicalOperators::EquivalentWithFourDotsAbove => "equivalent with four dots above",
            SupplementalMathematicalOperators::LessDashThanWithCircleInside => "less-than with circle inside",
            SupplementalMathematicalOperators::GreaterDashThanWithCircleInside => "greater-than with circle inside",
            SupplementalMathematicalOperators::LessDashThanWithQuestionMarkAbove => "less-than with question mark above",
            SupplementalMathematicalOperators::GreaterDashThanWithQuestionMarkAbove => "greater-than with question mark above",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualTo => "less-than or slanted equal to",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualTo => "greater-than or slanted equal to",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotInside => "less-than or slanted equal to with dot inside",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotInside => "greater-than or slanted equal to with dot inside",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAbove => "less-than or slanted equal to with dot above",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAbove => "greater-than or slanted equal to with dot above",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAboveRight => "less-than or slanted equal to with dot above right",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAboveLeft => "greater-than or slanted equal to with dot above left",
            SupplementalMathematicalOperators::LessDashThanOrApproximate => "less-than or approximate",
            SupplementalMathematicalOperators::GreaterDashThanOrApproximate => "greater-than or approximate",
            SupplementalMathematicalOperators::LessDashThanAndSingleDashLineNotEqualTo => "less-than and single-line not equal to",
            SupplementalMathematicalOperators::GreaterDashThanAndSingleDashLineNotEqualTo => "greater-than and single-line not equal to",
            SupplementalMathematicalOperators::LessDashThanAndNotApproximate => "less-than and not approximate",
            SupplementalMathematicalOperators::GreaterDashThanAndNotApproximate => "greater-than and not approximate",
            SupplementalMathematicalOperators::LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan => "less-than above double-line equal above greater-than",
            SupplementalMathematicalOperators::GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan => "greater-than above double-line equal above less-than",
            SupplementalMathematicalOperators::LessDashThanAboveSimilarOrEqual => "less-than above similar or equal",
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarOrEqual => "greater-than above similar or equal",
            SupplementalMathematicalOperators::LessDashThanAboveSimilarAboveGreaterDashThan => "less-than above similar above greater-than",
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarAboveLessDashThan => "greater-than above similar above less-than",
            SupplementalMathematicalOperators::LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual => "less-than above greater-than above double-line equal",
            SupplementalMathematicalOperators::GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual => "greater-than above less-than above double-line equal",
            SupplementalMathematicalOperators::LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual => "less-than above slanted equal above greater-than above slanted equal",
            SupplementalMathematicalOperators::GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual => "greater-than above slanted equal above less-than above slanted equal",
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThan => "slanted equal to or less-than",
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThan => "slanted equal to or greater-than",
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThanWithDotInside => "slanted equal to or less-than with dot inside",
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThanWithDotInside => "slanted equal to or greater-than with dot inside",
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrLessDashThan => "double-line equal to or less-than",
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrGreaterDashThan => "double-line equal to or greater-than",
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrLessDashThan => "double-line slanted equal to or less-than",
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrGreaterDashThan => "double-line slanted equal to or greater-than",
            SupplementalMathematicalOperators::SimilarOrLessDashThan => "similar or less-than",
            SupplementalMathematicalOperators::SimilarOrGreaterDashThan => "similar or greater-than",
            SupplementalMathematicalOperators::SimilarAboveLessDashThanAboveEqualsSign => "similar above less-than above equals sign",
            SupplementalMathematicalOperators::SimilarAboveGreaterDashThanAboveEqualsSign => "similar above greater-than above equals sign",
            SupplementalMathematicalOperators::DoubleNestedLessDashThan => "double nested less-than",
            SupplementalMathematicalOperators::DoubleNestedGreaterDashThan => "double nested greater-than",
            SupplementalMathematicalOperators::DoubleNestedLessDashThanWithUnderbar => "double nested less-than with underbar",
            SupplementalMathematicalOperators::GreaterDashThanOverlappingLessDashThan => "greater-than overlapping less-than",
            SupplementalMathematicalOperators::GreaterDashThanBesideLessDashThan => "greater-than beside less-than",
            SupplementalMathematicalOperators::LessDashThanClosedByCurve => "less-than closed by curve",
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurve => "greater-than closed by curve",
            SupplementalMathematicalOperators::LessDashThanClosedByCurveAboveSlantedEqual => "less-than closed by curve above slanted equal",
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurveAboveSlantedEqual => "greater-than closed by curve above slanted equal",
            SupplementalMathematicalOperators::SmallerThan => "smaller than",
            SupplementalMathematicalOperators::LargerThan => "larger than",
            SupplementalMathematicalOperators::SmallerThanOrEqualTo => "smaller than or equal to",
            SupplementalMathematicalOperators::LargerThanOrEqualTo => "larger than or equal to",
            SupplementalMathematicalOperators::EqualsSignWithBumpyAbove => "equals sign with bumpy above",
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineEqualsSign => "precedes above single-line equals sign",
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineEqualsSign => "succeeds above single-line equals sign",
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineNotEqualTo => "precedes above single-line not equal to",
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineNotEqualTo => "succeeds above single-line not equal to",
            SupplementalMathematicalOperators::PrecedesAboveEqualsSign => "precedes above equals sign",
            SupplementalMathematicalOperators::SucceedsAboveEqualsSign => "succeeds above equals sign",
            SupplementalMathematicalOperators::PrecedesAboveNotEqualTo => "precedes above not equal to",
            SupplementalMathematicalOperators::SucceedsAboveNotEqualTo => "succeeds above not equal to",
            SupplementalMathematicalOperators::PrecedesAboveAlmostEqualTo => "precedes above almost equal to",
            SupplementalMathematicalOperators::SucceedsAboveAlmostEqualTo => "succeeds above almost equal to",
            SupplementalMathematicalOperators::PrecedesAboveNotAlmostEqualTo => "precedes above not almost equal to",
            SupplementalMathematicalOperators::SucceedsAboveNotAlmostEqualTo => "succeeds above not almost equal to",
            SupplementalMathematicalOperators::DoublePrecedes => "double precedes",
            SupplementalMathematicalOperators::DoubleSucceeds => "double succeeds",
            SupplementalMathematicalOperators::SubsetWithDot => "subset with dot",
            SupplementalMathematicalOperators::SupersetWithDot => "superset with dot",
            SupplementalMathematicalOperators::SubsetWithPlusSignBelow => "subset with plus sign below",
            SupplementalMathematicalOperators::SupersetWithPlusSignBelow => "superset with plus sign below",
            SupplementalMathematicalOperators::SubsetWithMultiplicationSignBelow => "subset with multiplication sign below",
            SupplementalMathematicalOperators::SupersetWithMultiplicationSignBelow => "superset with multiplication sign below",
            SupplementalMathematicalOperators::SubsetOfOrEqualToWithDotAbove => "subset of or equal to with dot above",
            SupplementalMathematicalOperators::SupersetOfOrEqualToWithDotAbove => "superset of or equal to with dot above",
            SupplementalMathematicalOperators::SubsetOfAboveEqualsSign => "subset of above equals sign",
            SupplementalMathematicalOperators::SupersetOfAboveEqualsSign => "superset of above equals sign",
            SupplementalMathematicalOperators::SubsetOfAboveTildeOperator => "subset of above tilde operator",
            SupplementalMathematicalOperators::SupersetOfAboveTildeOperator => "superset of above tilde operator",
            SupplementalMathematicalOperators::SubsetOfAboveAlmostEqualTo => "subset of above almost equal to",
            SupplementalMathematicalOperators::SupersetOfAboveAlmostEqualTo => "superset of above almost equal to",
            SupplementalMathematicalOperators::SubsetOfAboveNotEqualTo => "subset of above not equal to",
            SupplementalMathematicalOperators::SupersetOfAboveNotEqualTo => "superset of above not equal to",
            SupplementalMathematicalOperators::SquareLeftOpenBoxOperator => "square left open box operator",
            SupplementalMathematicalOperators::SquareRightOpenBoxOperator => "square right open box operator",
            SupplementalMathematicalOperators::ClosedSubset => "closed subset",
            SupplementalMathematicalOperators::ClosedSuperset => "closed superset",
            SupplementalMathematicalOperators::ClosedSubsetOrEqualTo => "closed subset or equal to",
            SupplementalMathematicalOperators::ClosedSupersetOrEqualTo => "closed superset or equal to",
            SupplementalMathematicalOperators::SubsetAboveSuperset => "subset above superset",
            SupplementalMathematicalOperators::SupersetAboveSubset => "superset above subset",
            SupplementalMathematicalOperators::SubsetAboveSubset => "subset above subset",
            SupplementalMathematicalOperators::SupersetAboveSuperset => "superset above superset",
            SupplementalMathematicalOperators::SupersetBesideSubset => "superset beside subset",
            SupplementalMathematicalOperators::SupersetBesideAndJoinedByDashWithSubset => "superset beside and joined by dash with subset",
            SupplementalMathematicalOperators::ElementOfOpeningDownwards => "element of opening downwards",
            SupplementalMathematicalOperators::PitchforkWithTeeTop => "pitchfork with tee top",
            SupplementalMathematicalOperators::TransversalIntersection => "transversal intersection",
            SupplementalMathematicalOperators::Forking => "forking",
            SupplementalMathematicalOperators::Nonforking => "nonforking",
            SupplementalMathematicalOperators::ShortLeftTack => "short left tack",
            SupplementalMathematicalOperators::ShortDownTack => "short down tack",
            SupplementalMathematicalOperators::ShortUpTack => "short up tack",
            SupplementalMathematicalOperators::PerpendicularWithS => "perpendicular with s",
            SupplementalMathematicalOperators::VerticalBarTripleRightTurnstile => "vertical bar triple right turnstile",
            SupplementalMathematicalOperators::DoubleVerticalBarLeftTurnstile => "double vertical bar left turnstile",
            SupplementalMathematicalOperators::VerticalBarDoubleLeftTurnstile => "vertical bar double left turnstile",
            SupplementalMathematicalOperators::DoubleVerticalBarDoubleLeftTurnstile => "double vertical bar double left turnstile",
            SupplementalMathematicalOperators::LongDashFromLeftMemberOfDoubleVertical => "long dash from left member of double vertical",
            SupplementalMathematicalOperators::ShortDownTackWithOverbar => "short down tack with overbar",
            SupplementalMathematicalOperators::ShortUpTackWithUnderbar => "short up tack with underbar",
            SupplementalMathematicalOperators::ShortUpTackAboveShortDownTack => "short up tack above short down tack",
            SupplementalMathematicalOperators::DoubleDownTack => "double down tack",
            SupplementalMathematicalOperators::DoubleUpTack => "double up tack",
            SupplementalMathematicalOperators::DoubleStrokeNotSign => "double stroke not sign",
            SupplementalMathematicalOperators::ReversedDoubleStrokeNotSign => "reversed double stroke not sign",
            SupplementalMathematicalOperators::DoesNotDivideWithReversedNegationSlash => "does not divide with reversed negation slash",
            SupplementalMathematicalOperators::VerticalLineWithCircleAbove => "vertical line with circle above",
            SupplementalMathematicalOperators::VerticalLineWithCircleBelow => "vertical line with circle below",
            SupplementalMathematicalOperators::DownTackWithCircleBelow => "down tack with circle below",
            SupplementalMathematicalOperators::ParallelWithHorizontalStroke => "parallel with horizontal stroke",
            SupplementalMathematicalOperators::ParallelWithTildeOperator => "parallel with tilde operator",
            SupplementalMathematicalOperators::TripleVerticalBarBinaryRelation => "triple vertical bar binary relation",
            SupplementalMathematicalOperators::TripleVerticalBarWithHorizontalStroke => "triple vertical bar with horizontal stroke",
            SupplementalMathematicalOperators::TripleColonOperator => "triple colon operator",
            SupplementalMathematicalOperators::TripleNestedLessDashThan => "triple nested less-than",
            SupplementalMathematicalOperators::TripleNestedGreaterDashThan => "triple nested greater-than",
            SupplementalMathematicalOperators::DoubleDashLineSlantedLessDashThanOrEqualTo => "double-line slanted less-than or equal to",
            SupplementalMathematicalOperators::DoubleDashLineSlantedGreaterDashThanOrEqualTo => "double-line slanted greater-than or equal to",
            SupplementalMathematicalOperators::TripleSolidusBinaryRelation => "triple solidus binary relation",
            SupplementalMathematicalOperators::LargeTripleVerticalBarOperator => "large triple vertical bar operator",
            SupplementalMathematicalOperators::DoubleSolidusOperator => "double solidus operator",
            SupplementalMathematicalOperators::WhiteVerticalBar => "white vertical bar",
        }
    }
}
