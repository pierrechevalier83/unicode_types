
/// An enum to represent all characters in the MiscellaneousSymbolsandArrows block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MiscellaneousSymbolsandArrows {
    /// \u{2b00}: '⬀'
    NorthEastWhiteArrow,
    /// \u{2b01}: '⬁'
    NorthWestWhiteArrow,
    /// \u{2b02}: '⬂'
    SouthEastWhiteArrow,
    /// \u{2b03}: '⬃'
    SouthWestWhiteArrow,
    /// \u{2b04}: '⬄'
    LeftRightWhiteArrow,
    /// \u{2b05}: '⬅'
    LeftwardsBlackArrow,
    /// \u{2b06}: '⬆'
    UpwardsBlackArrow,
    /// \u{2b07}: '⬇'
    DownwardsBlackArrow,
    /// \u{2b08}: '⬈'
    NorthEastBlackArrow,
    /// \u{2b09}: '⬉'
    NorthWestBlackArrow,
    /// \u{2b0a}: '⬊'
    SouthEastBlackArrow,
    /// \u{2b0b}: '⬋'
    SouthWestBlackArrow,
    /// \u{2b0c}: '⬌'
    LeftRightBlackArrow,
    /// \u{2b0d}: '⬍'
    UpDownBlackArrow,
    /// \u{2b0e}: '⬎'
    RightwardsArrowWithTipDownwards,
    /// \u{2b0f}: '⬏'
    RightwardsArrowWithTipUpwards,
    /// \u{2b10}: '⬐'
    LeftwardsArrowWithTipDownwards,
    /// \u{2b11}: '⬑'
    LeftwardsArrowWithTipUpwards,
    /// \u{2b12}: '⬒'
    SquareWithTopHalfBlack,
    /// \u{2b13}: '⬓'
    SquareWithBottomHalfBlack,
    /// \u{2b14}: '⬔'
    SquareWithUpperRightDiagonalHalfBlack,
    /// \u{2b15}: '⬕'
    SquareWithLowerLeftDiagonalHalfBlack,
    /// \u{2b16}: '⬖'
    DiamondWithLeftHalfBlack,
    /// \u{2b17}: '⬗'
    DiamondWithRightHalfBlack,
    /// \u{2b18}: '⬘'
    DiamondWithTopHalfBlack,
    /// \u{2b19}: '⬙'
    DiamondWithBottomHalfBlack,
    /// \u{2b1a}: '⬚'
    DottedSquare,
    /// \u{2b1b}: '⬛'
    BlackLargeSquare,
    /// \u{2b1c}: '⬜'
    WhiteLargeSquare,
    /// \u{2b1d}: '⬝'
    BlackVerySmallSquare,
    /// \u{2b1e}: '⬞'
    WhiteVerySmallSquare,
    /// \u{2b1f}: '⬟'
    BlackPentagon,
    /// \u{2b20}: '⬠'
    WhitePentagon,
    /// \u{2b21}: '⬡'
    WhiteHexagon,
    /// \u{2b22}: '⬢'
    BlackHexagon,
    /// \u{2b23}: '⬣'
    HorizontalBlackHexagon,
    /// \u{2b24}: '⬤'
    BlackLargeCircle,
    /// \u{2b25}: '⬥'
    BlackMediumDiamond,
    /// \u{2b26}: '⬦'
    WhiteMediumDiamond,
    /// \u{2b27}: '⬧'
    BlackMediumLozenge,
    /// \u{2b28}: '⬨'
    WhiteMediumLozenge,
    /// \u{2b29}: '⬩'
    BlackSmallDiamond,
    /// \u{2b2a}: '⬪'
    BlackSmallLozenge,
    /// \u{2b2b}: '⬫'
    WhiteSmallLozenge,
    /// \u{2b2c}: '⬬'
    BlackHorizontalEllipse,
    /// \u{2b2d}: '⬭'
    WhiteHorizontalEllipse,
    /// \u{2b2e}: '⬮'
    BlackVerticalEllipse,
    /// \u{2b2f}: '⬯'
    WhiteVerticalEllipse,
    /// \u{2b30}: '⬰'
    LeftArrowWithSmallCircle,
    /// \u{2b31}: '⬱'
    ThreeLeftwardsArrows,
    /// \u{2b32}: '⬲'
    LeftArrowWithCircledPlus,
    /// \u{2b33}: '⬳'
    LongLeftwardsSquiggleArrow,
    /// \u{2b34}: '⬴'
    LeftwardsTwoDashHeadedArrowWithVerticalStroke,
    /// \u{2b35}: '⬵'
    LeftwardsTwoDashHeadedArrowWithDoubleVerticalStroke,
    /// \u{2b36}: '⬶'
    LeftwardsTwoDashHeadedArrowFromBar,
    /// \u{2b37}: '⬷'
    LeftwardsTwoDashHeadedTripleDashArrow,
    /// \u{2b38}: '⬸'
    LeftwardsArrowWithDottedStem,
    /// \u{2b39}: '⬹'
    LeftwardsArrowWithTailWithVerticalStroke,
    /// \u{2b3a}: '⬺'
    LeftwardsArrowWithTailWithDoubleVerticalStroke,
    /// \u{2b3b}: '⬻'
    LeftwardsTwoDashHeadedArrowWithTail,
    /// \u{2b3c}: '⬼'
    LeftwardsTwoDashHeadedArrowWithTailWithVerticalStroke,
    /// \u{2b3d}: '⬽'
    LeftwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke,
    /// \u{2b3e}: '⬾'
    LeftwardsArrowThroughX,
    /// \u{2b3f}: '⬿'
    WaveArrowPointingDirectlyLeft,
    /// \u{2b40}: '⭀'
    EqualsSignAboveLeftwardsArrow,
    /// \u{2b41}: '⭁'
    ReverseTildeOperatorAboveLeftwardsArrow,
    /// \u{2b42}: '⭂'
    LeftwardsArrowAboveReverseAlmostEqualTo,
    /// \u{2b43}: '⭃'
    RightwardsArrowThroughGreaterDashThan,
    /// \u{2b44}: '⭄'
    RightwardsArrowThroughSuperset,
    /// \u{2b45}: '⭅'
    LeftwardsQuadrupleArrow,
    /// \u{2b46}: '⭆'
    RightwardsQuadrupleArrow,
    /// \u{2b47}: '⭇'
    ReverseTildeOperatorAboveRightwardsArrow,
    /// \u{2b48}: '⭈'
    RightwardsArrowAboveReverseAlmostEqualTo,
    /// \u{2b49}: '⭉'
    TildeOperatorAboveLeftwardsArrow,
    /// \u{2b4a}: '⭊'
    LeftwardsArrowAboveAlmostEqualTo,
    /// \u{2b4b}: '⭋'
    LeftwardsArrowAboveReverseTildeOperator,
    /// \u{2b4c}: '⭌'
    RightwardsArrowAboveReverseTildeOperator,
    /// \u{2b4d}: '⭍'
    DownwardsTriangleDashHeadedZigzagArrow,
    /// \u{2b4e}: '⭎'
    ShortSlantedNorthArrow,
    /// \u{2b4f}: '⭏'
    ShortBackslantedSouthArrow,
    /// \u{2b50}: '⭐'
    WhiteMediumStar,
    /// \u{2b51}: '⭑'
    BlackSmallStar,
    /// \u{2b52}: '⭒'
    WhiteSmallStar,
    /// \u{2b53}: '⭓'
    BlackRightDashPointingPentagon,
    /// \u{2b54}: '⭔'
    WhiteRightDashPointingPentagon,
    /// \u{2b55}: '⭕'
    HeavyLargeCircle,
    /// \u{2b56}: '⭖'
    HeavyOvalWithOvalInside,
    /// \u{2b57}: '⭗'
    HeavyCircleWithCircleInside,
    /// \u{2b58}: '⭘'
    HeavyCircle,
    /// \u{2b59}: '⭙'
    HeavyCircledSaltire,
    /// \u{2b5a}: '⭚'
    SlantedNorthArrowWithHookedHead,
    /// \u{2b5b}: '⭛'
    BackslantedSouthArrowWithHookedTail,
    /// \u{2b5c}: '⭜'
    SlantedNorthArrowWithHorizontalTail,
    /// \u{2b5d}: '⭝'
    BackslantedSouthArrowWithHorizontalTail,
    /// \u{2b5e}: '⭞'
    BentArrowPointingDownwardsThenNorthEast,
    /// \u{2b5f}: '⭟'
    ShortBentArrowPointingDownwardsThenNorthEast,
    /// \u{2b60}: '⭠'
    LeftwardsTriangleDashHeadedArrow,
    /// \u{2b61}: '⭡'
    UpwardsTriangleDashHeadedArrow,
    /// \u{2b62}: '⭢'
    RightwardsTriangleDashHeadedArrow,
    /// \u{2b63}: '⭣'
    DownwardsTriangleDashHeadedArrow,
    /// \u{2b64}: '⭤'
    LeftRightTriangleDashHeadedArrow,
    /// \u{2b65}: '⭥'
    UpDownTriangleDashHeadedArrow,
    /// \u{2b66}: '⭦'
    NorthWestTriangleDashHeadedArrow,
    /// \u{2b67}: '⭧'
    NorthEastTriangleDashHeadedArrow,
    /// \u{2b68}: '⭨'
    SouthEastTriangleDashHeadedArrow,
    /// \u{2b69}: '⭩'
    SouthWestTriangleDashHeadedArrow,
    /// \u{2b6a}: '⭪'
    LeftwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6b}: '⭫'
    UpwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6c}: '⭬'
    RightwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6d}: '⭭'
    DownwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6e}: '⭮'
    ClockwiseTriangleDashHeadedOpenCircleArrow,
    /// \u{2b6f}: '⭯'
    AnticlockwiseTriangleDashHeadedOpenCircleArrow,
    /// \u{2b70}: '⭰'
    LeftwardsTriangleDashHeadedArrowToBar,
    /// \u{2b71}: '⭱'
    UpwardsTriangleDashHeadedArrowToBar,
    /// \u{2b72}: '⭲'
    RightwardsTriangleDashHeadedArrowToBar,
    /// \u{2b73}: '⭳'
    DownwardsTriangleDashHeadedArrowToBar,
    /// \u{2b76}: '⭶'
    NorthWestTriangleDashHeadedArrowToBar,
    /// \u{2b77}: '⭷'
    NorthEastTriangleDashHeadedArrowToBar,
    /// \u{2b78}: '⭸'
    SouthEastTriangleDashHeadedArrowToBar,
    /// \u{2b79}: '⭹'
    SouthWestTriangleDashHeadedArrowToBar,
    /// \u{2b7a}: '⭺'
    LeftwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7b}: '⭻'
    UpwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7c}: '⭼'
    RightwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7d}: '⭽'
    DownwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7e}: '⭾'
    HorizontalTabKey,
    /// \u{2b7f}: '⭿'
    VerticalTabKey,
    /// \u{2b80}: '⮀'
    LeftwardsTriangleDashHeadedArrowOverRightwardsTriangleDashHeadedArrow,
    /// \u{2b81}: '⮁'
    UpwardsTriangleDashHeadedArrowLeftwardsOfDownwardsTriangleDashHeadedArrow,
    /// \u{2b82}: '⮂'
    RightwardsTriangleDashHeadedArrowOverLeftwardsTriangleDashHeadedArrow,
    /// \u{2b83}: '⮃'
    DownwardsTriangleDashHeadedArrowLeftwardsOfUpwardsTriangleDashHeadedArrow,
    /// \u{2b84}: '⮄'
    LeftwardsTriangleDashHeadedPairedArrows,
    /// \u{2b85}: '⮅'
    UpwardsTriangleDashHeadedPairedArrows,
    /// \u{2b86}: '⮆'
    RightwardsTriangleDashHeadedPairedArrows,
    /// \u{2b87}: '⮇'
    DownwardsTriangleDashHeadedPairedArrows,
    /// \u{2b88}: '⮈'
    LeftwardsBlackCircledWhiteArrow,
    /// \u{2b89}: '⮉'
    UpwardsBlackCircledWhiteArrow,
    /// \u{2b8a}: '⮊'
    RightwardsBlackCircledWhiteArrow,
    /// \u{2b8b}: '⮋'
    DownwardsBlackCircledWhiteArrow,
    /// \u{2b8c}: '⮌'
    AnticlockwiseTriangleDashHeadedRightUDashShapedArrow,
    /// \u{2b8d}: '⮍'
    AnticlockwiseTriangleDashHeadedBottomUDashShapedArrow,
    /// \u{2b8e}: '⮎'
    AnticlockwiseTriangleDashHeadedLeftUDashShapedArrow,
    /// \u{2b8f}: '⮏'
    AnticlockwiseTriangleDashHeadedTopUDashShapedArrow,
    /// \u{2b90}: '⮐'
    ReturnLeft,
    /// \u{2b91}: '⮑'
    ReturnRight,
    /// \u{2b92}: '⮒'
    NewlineLeft,
    /// \u{2b93}: '⮓'
    NewlineRight,
    /// \u{2b94}: '⮔'
    FourCornerArrowsCirclingAnticlockwise,
    /// \u{2b95}: '⮕'
    RightwardsBlackArrow,
    /// \u{2b98}: '⮘'
    ThreeDashDTopDashLightedLeftwardsEquilateralArrowhead,
    /// \u{2b99}: '⮙'
    ThreeDashDRightDashLightedUpwardsEquilateralArrowhead,
    /// \u{2b9a}: '⮚'
    ThreeDashDTopDashLightedRightwardsEquilateralArrowhead,
    /// \u{2b9b}: '⮛'
    ThreeDashDLeftDashLightedDownwardsEquilateralArrowhead,
    /// \u{2b9c}: '⮜'
    BlackLeftwardsEquilateralArrowhead,
    /// \u{2b9d}: '⮝'
    BlackUpwardsEquilateralArrowhead,
    /// \u{2b9e}: '⮞'
    BlackRightwardsEquilateralArrowhead,
    /// \u{2b9f}: '⮟'
    BlackDownwardsEquilateralArrowhead,
    /// \u{2ba0}: '⮠'
    DownwardsTriangleDashHeadedArrowWithLongTipLeftwards,
    /// \u{2ba1}: '⮡'
    DownwardsTriangleDashHeadedArrowWithLongTipRightwards,
    /// \u{2ba2}: '⮢'
    UpwardsTriangleDashHeadedArrowWithLongTipLeftwards,
    /// \u{2ba3}: '⮣'
    UpwardsTriangleDashHeadedArrowWithLongTipRightwards,
    /// \u{2ba4}: '⮤'
    LeftwardsTriangleDashHeadedArrowWithLongTipUpwards,
    /// \u{2ba5}: '⮥'
    RightwardsTriangleDashHeadedArrowWithLongTipUpwards,
    /// \u{2ba6}: '⮦'
    LeftwardsTriangleDashHeadedArrowWithLongTipDownwards,
    /// \u{2ba7}: '⮧'
    RightwardsTriangleDashHeadedArrowWithLongTipDownwards,
    /// \u{2ba8}: '⮨'
    BlackCurvedDownwardsAndLeftwardsArrow,
    /// \u{2ba9}: '⮩'
    BlackCurvedDownwardsAndRightwardsArrow,
    /// \u{2baa}: '⮪'
    BlackCurvedUpwardsAndLeftwardsArrow,
    /// \u{2bab}: '⮫'
    BlackCurvedUpwardsAndRightwardsArrow,
    /// \u{2bac}: '⮬'
    BlackCurvedLeftwardsAndUpwardsArrow,
    /// \u{2bad}: '⮭'
    BlackCurvedRightwardsAndUpwardsArrow,
    /// \u{2bae}: '⮮'
    BlackCurvedLeftwardsAndDownwardsArrow,
    /// \u{2baf}: '⮯'
    BlackCurvedRightwardsAndDownwardsArrow,
    /// \u{2bb0}: '⮰'
    RibbonArrowDownLeft,
    /// \u{2bb1}: '⮱'
    RibbonArrowDownRight,
    /// \u{2bb2}: '⮲'
    RibbonArrowUpLeft,
    /// \u{2bb3}: '⮳'
    RibbonArrowUpRight,
    /// \u{2bb4}: '⮴'
    RibbonArrowLeftUp,
    /// \u{2bb5}: '⮵'
    RibbonArrowRightUp,
    /// \u{2bb6}: '⮶'
    RibbonArrowLeftDown,
    /// \u{2bb7}: '⮷'
    RibbonArrowRightDown,
    /// \u{2bb8}: '⮸'
    UpwardsWhiteArrowFromBarWithHorizontalBar,
    /// \u{2bb9}: '⮹'
    UpArrowheadInARectangleBox,
    /// \u{2bba}: '⮺'
    OverlappingWhiteSquares,
    /// \u{2bbb}: '⮻'
    OverlappingWhiteAndBlackSquares,
    /// \u{2bbc}: '⮼'
    OverlappingBlackSquares,
    /// \u{2bbd}: '⮽'
    BallotBoxWithLightX,
    /// \u{2bbe}: '⮾'
    CircledX,
    /// \u{2bbf}: '⮿'
    CircledBoldX,
    /// \u{2bc0}: '⯀'
    BlackSquareCentred,
    /// \u{2bc1}: '⯁'
    BlackDiamondCentred,
    /// \u{2bc2}: '⯂'
    TurnedBlackPentagon,
    /// \u{2bc3}: '⯃'
    HorizontalBlackOctagon,
    /// \u{2bc4}: '⯄'
    BlackOctagon,
    /// \u{2bc5}: '⯅'
    BlackMediumUpDashPointingTriangleCentred,
    /// \u{2bc6}: '⯆'
    BlackMediumDownDashPointingTriangleCentred,
    /// \u{2bc7}: '⯇'
    BlackMediumLeftDashPointingTriangleCentred,
    /// \u{2bc8}: '⯈'
    BlackMediumRightDashPointingTriangleCentred,
    /// \u{2bc9}: '⯉'
    NeptuneFormTwo,
    /// \u{2bca}: '⯊'
    TopHalfBlackCircle,
    /// \u{2bcb}: '⯋'
    BottomHalfBlackCircle,
    /// \u{2bcc}: '⯌'
    LightFourPointedBlackCusp,
    /// \u{2bcd}: '⯍'
    RotatedLightFourPointedBlackCusp,
    /// \u{2bce}: '⯎'
    WhiteFourPointedCusp,
    /// \u{2bcf}: '⯏'
    RotatedWhiteFourPointedCusp,
    /// \u{2bd0}: '⯐'
    SquarePositionIndicator,
    /// \u{2bd1}: '⯑'
    UncertaintySign,
    /// \u{2bd2}: '⯒'
    GroupMark,
    /// \u{2bd3}: '⯓'
    PlutoFormTwo,
    /// \u{2bd4}: '⯔'
    PlutoFormThree,
    /// \u{2bd5}: '⯕'
    PlutoFormFour,
    /// \u{2bd6}: '⯖'
    PlutoFormFive,
    /// \u{2bd7}: '⯗'
    Transpluto,
    /// \u{2bd8}: '⯘'
    Proserpina,
    /// \u{2bd9}: '⯙'
    Astraea,
    /// \u{2bda}: '⯚'
    Hygiea,
    /// \u{2bdb}: '⯛'
    Pholus,
    /// \u{2bdc}: '⯜'
    Nessus,
    /// \u{2bdd}: '⯝'
    WhiteMoonSelena,
    /// \u{2bde}: '⯞'
    BlackDiamondOnCross,
    /// \u{2bdf}: '⯟'
    TrueLightMoonArta,
    /// \u{2be0}: '⯠'
    Cupido,
    /// \u{2be1}: '⯡'
    Hades,
    /// \u{2be2}: '⯢'
    Zeus,
    /// \u{2be3}: '⯣'
    Kronos,
    /// \u{2be4}: '⯤'
    Apollon,
    /// \u{2be5}: '⯥'
    Admetos,
    /// \u{2be6}: '⯦'
    Vulcanus,
    /// \u{2be7}: '⯧'
    Poseidon,
    /// \u{2be8}: '⯨'
    LeftHalfBlackStar,
    /// \u{2be9}: '⯩'
    RightHalfBlackStar,
    /// \u{2bea}: '⯪'
    StarWithLeftHalfBlack,
    /// \u{2beb}: '⯫'
    StarWithRightHalfBlack,
    /// \u{2bec}: '⯬'
    LeftwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bed}: '⯭'
    UpwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bee}: '⯮'
    RightwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bef}: '⯯'
    DownwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bf0}: '⯰'
    ErisFormOne,
    /// \u{2bf1}: '⯱'
    ErisFormTwo,
    /// \u{2bf2}: '⯲'
    Sedna,
    /// \u{2bf3}: '⯳'
    RussianAstrologicalSymbolVigintile,
    /// \u{2bf4}: '⯴'
    RussianAstrologicalSymbolNovile,
    /// \u{2bf5}: '⯵'
    RussianAstrologicalSymbolQuintile,
    /// \u{2bf6}: '⯶'
    RussianAstrologicalSymbolBinovile,
    /// \u{2bf7}: '⯷'
    RussianAstrologicalSymbolSentagon,
    /// \u{2bf8}: '⯸'
    RussianAstrologicalSymbolTredecile,
    /// \u{2bf9}: '⯹'
    EqualsSignWithInfinityBelow,
    /// \u{2bfa}: '⯺'
    UnitedSymbol,
    /// \u{2bfb}: '⯻'
    SeparatedSymbol,
    /// \u{2bfc}: '⯼'
    DoubledSymbol,
    /// \u{2bfd}: '⯽'
    PassedSymbol,
    /// \u{2bfe}: '⯾'
    ReversedRightAngle,
}

impl Into<char> for MiscellaneousSymbolsandArrows {
    fn into(self) -> char {
        match self {
            MiscellaneousSymbolsandArrows::NorthEastWhiteArrow => '⬀',
            MiscellaneousSymbolsandArrows::NorthWestWhiteArrow => '⬁',
            MiscellaneousSymbolsandArrows::SouthEastWhiteArrow => '⬂',
            MiscellaneousSymbolsandArrows::SouthWestWhiteArrow => '⬃',
            MiscellaneousSymbolsandArrows::LeftRightWhiteArrow => '⬄',
            MiscellaneousSymbolsandArrows::LeftwardsBlackArrow => '⬅',
            MiscellaneousSymbolsandArrows::UpwardsBlackArrow => '⬆',
            MiscellaneousSymbolsandArrows::DownwardsBlackArrow => '⬇',
            MiscellaneousSymbolsandArrows::NorthEastBlackArrow => '⬈',
            MiscellaneousSymbolsandArrows::NorthWestBlackArrow => '⬉',
            MiscellaneousSymbolsandArrows::SouthEastBlackArrow => '⬊',
            MiscellaneousSymbolsandArrows::SouthWestBlackArrow => '⬋',
            MiscellaneousSymbolsandArrows::LeftRightBlackArrow => '⬌',
            MiscellaneousSymbolsandArrows::UpDownBlackArrow => '⬍',
            MiscellaneousSymbolsandArrows::RightwardsArrowWithTipDownwards => '⬎',
            MiscellaneousSymbolsandArrows::RightwardsArrowWithTipUpwards => '⬏',
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipDownwards => '⬐',
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipUpwards => '⬑',
            MiscellaneousSymbolsandArrows::SquareWithTopHalfBlack => '⬒',
            MiscellaneousSymbolsandArrows::SquareWithBottomHalfBlack => '⬓',
            MiscellaneousSymbolsandArrows::SquareWithUpperRightDiagonalHalfBlack => '⬔',
            MiscellaneousSymbolsandArrows::SquareWithLowerLeftDiagonalHalfBlack => '⬕',
            MiscellaneousSymbolsandArrows::DiamondWithLeftHalfBlack => '⬖',
            MiscellaneousSymbolsandArrows::DiamondWithRightHalfBlack => '⬗',
            MiscellaneousSymbolsandArrows::DiamondWithTopHalfBlack => '⬘',
            MiscellaneousSymbolsandArrows::DiamondWithBottomHalfBlack => '⬙',
            MiscellaneousSymbolsandArrows::DottedSquare => '⬚',
            MiscellaneousSymbolsandArrows::BlackLargeSquare => '⬛',
            MiscellaneousSymbolsandArrows::WhiteLargeSquare => '⬜',
            MiscellaneousSymbolsandArrows::BlackVerySmallSquare => '⬝',
            MiscellaneousSymbolsandArrows::WhiteVerySmallSquare => '⬞',
            MiscellaneousSymbolsandArrows::BlackPentagon => '⬟',
            MiscellaneousSymbolsandArrows::WhitePentagon => '⬠',
            MiscellaneousSymbolsandArrows::WhiteHexagon => '⬡',
            MiscellaneousSymbolsandArrows::BlackHexagon => '⬢',
            MiscellaneousSymbolsandArrows::HorizontalBlackHexagon => '⬣',
            MiscellaneousSymbolsandArrows::BlackLargeCircle => '⬤',
            MiscellaneousSymbolsandArrows::BlackMediumDiamond => '⬥',
            MiscellaneousSymbolsandArrows::WhiteMediumDiamond => '⬦',
            MiscellaneousSymbolsandArrows::BlackMediumLozenge => '⬧',
            MiscellaneousSymbolsandArrows::WhiteMediumLozenge => '⬨',
            MiscellaneousSymbolsandArrows::BlackSmallDiamond => '⬩',
            MiscellaneousSymbolsandArrows::BlackSmallLozenge => '⬪',
            MiscellaneousSymbolsandArrows::WhiteSmallLozenge => '⬫',
            MiscellaneousSymbolsandArrows::BlackHorizontalEllipse => '⬬',
            MiscellaneousSymbolsandArrows::WhiteHorizontalEllipse => '⬭',
            MiscellaneousSymbolsandArrows::BlackVerticalEllipse => '⬮',
            MiscellaneousSymbolsandArrows::WhiteVerticalEllipse => '⬯',
            MiscellaneousSymbolsandArrows::LeftArrowWithSmallCircle => '⬰',
            MiscellaneousSymbolsandArrows::ThreeLeftwardsArrows => '⬱',
            MiscellaneousSymbolsandArrows::LeftArrowWithCircledPlus => '⬲',
            MiscellaneousSymbolsandArrows::LongLeftwardsSquiggleArrow => '⬳',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithVerticalStroke => '⬴',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithDoubleVerticalStroke => '⬵',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowFromBar => '⬶',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedTripleDashArrow => '⬷',
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithDottedStem => '⬸',
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithVerticalStroke => '⬹',
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithDoubleVerticalStroke => '⬺',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTail => '⬻',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithVerticalStroke => '⬼',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke => '⬽',
            MiscellaneousSymbolsandArrows::LeftwardsArrowThroughX => '⬾',
            MiscellaneousSymbolsandArrows::WaveArrowPointingDirectlyLeft => '⬿',
            MiscellaneousSymbolsandArrows::EqualsSignAboveLeftwardsArrow => '⭀',
            MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveLeftwardsArrow => '⭁',
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseAlmostEqualTo => '⭂',
            MiscellaneousSymbolsandArrows::RightwardsArrowThroughGreaterDashThan => '⭃',
            MiscellaneousSymbolsandArrows::RightwardsArrowThroughSuperset => '⭄',
            MiscellaneousSymbolsandArrows::LeftwardsQuadrupleArrow => '⭅',
            MiscellaneousSymbolsandArrows::RightwardsQuadrupleArrow => '⭆',
            MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveRightwardsArrow => '⭇',
            MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseAlmostEqualTo => '⭈',
            MiscellaneousSymbolsandArrows::TildeOperatorAboveLeftwardsArrow => '⭉',
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveAlmostEqualTo => '⭊',
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseTildeOperator => '⭋',
            MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseTildeOperator => '⭌',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedZigzagArrow => '⭍',
            MiscellaneousSymbolsandArrows::ShortSlantedNorthArrow => '⭎',
            MiscellaneousSymbolsandArrows::ShortBackslantedSouthArrow => '⭏',
            MiscellaneousSymbolsandArrows::WhiteMediumStar => '⭐',
            MiscellaneousSymbolsandArrows::BlackSmallStar => '⭑',
            MiscellaneousSymbolsandArrows::WhiteSmallStar => '⭒',
            MiscellaneousSymbolsandArrows::BlackRightDashPointingPentagon => '⭓',
            MiscellaneousSymbolsandArrows::WhiteRightDashPointingPentagon => '⭔',
            MiscellaneousSymbolsandArrows::HeavyLargeCircle => '⭕',
            MiscellaneousSymbolsandArrows::HeavyOvalWithOvalInside => '⭖',
            MiscellaneousSymbolsandArrows::HeavyCircleWithCircleInside => '⭗',
            MiscellaneousSymbolsandArrows::HeavyCircle => '⭘',
            MiscellaneousSymbolsandArrows::HeavyCircledSaltire => '⭙',
            MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHookedHead => '⭚',
            MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHookedTail => '⭛',
            MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHorizontalTail => '⭜',
            MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHorizontalTail => '⭝',
            MiscellaneousSymbolsandArrows::BentArrowPointingDownwardsThenNorthEast => '⭞',
            MiscellaneousSymbolsandArrows::ShortBentArrowPointingDownwardsThenNorthEast => '⭟',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrow => '⭠',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrow => '⭡',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrow => '⭢',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrow => '⭣',
            MiscellaneousSymbolsandArrows::LeftRightTriangleDashHeadedArrow => '⭤',
            MiscellaneousSymbolsandArrows::UpDownTriangleDashHeadedArrow => '⭥',
            MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrow => '⭦',
            MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrow => '⭧',
            MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrow => '⭨',
            MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrow => '⭩',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedDashedArrow => '⭪',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedDashedArrow => '⭫',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedDashedArrow => '⭬',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedDashedArrow => '⭭',
            MiscellaneousSymbolsandArrows::ClockwiseTriangleDashHeadedOpenCircleArrow => '⭮',
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedOpenCircleArrow => '⭯',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowToBar => '⭰',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowToBar => '⭱',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowToBar => '⭲',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowToBar => '⭳',
            MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrowToBar => '⭶',
            MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrowToBar => '⭷',
            MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrowToBar => '⭸',
            MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrowToBar => '⭹',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => '⭺',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => '⭻',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => '⭼',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => '⭽',
            MiscellaneousSymbolsandArrows::HorizontalTabKey => '⭾',
            MiscellaneousSymbolsandArrows::VerticalTabKey => '⭿',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowOverRightwardsTriangleDashHeadedArrow => '⮀',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowLeftwardsOfDownwardsTriangleDashHeadedArrow => '⮁',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowOverLeftwardsTriangleDashHeadedArrow => '⮂',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowLeftwardsOfUpwardsTriangleDashHeadedArrow => '⮃',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedPairedArrows => '⮄',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedPairedArrows => '⮅',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedPairedArrows => '⮆',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedPairedArrows => '⮇',
            MiscellaneousSymbolsandArrows::LeftwardsBlackCircledWhiteArrow => '⮈',
            MiscellaneousSymbolsandArrows::UpwardsBlackCircledWhiteArrow => '⮉',
            MiscellaneousSymbolsandArrows::RightwardsBlackCircledWhiteArrow => '⮊',
            MiscellaneousSymbolsandArrows::DownwardsBlackCircledWhiteArrow => '⮋',
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedRightUDashShapedArrow => '⮌',
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedBottomUDashShapedArrow => '⮍',
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedLeftUDashShapedArrow => '⮎',
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedTopUDashShapedArrow => '⮏',
            MiscellaneousSymbolsandArrows::ReturnLeft => '⮐',
            MiscellaneousSymbolsandArrows::ReturnRight => '⮑',
            MiscellaneousSymbolsandArrows::NewlineLeft => '⮒',
            MiscellaneousSymbolsandArrows::NewlineRight => '⮓',
            MiscellaneousSymbolsandArrows::FourCornerArrowsCirclingAnticlockwise => '⮔',
            MiscellaneousSymbolsandArrows::RightwardsBlackArrow => '⮕',
            MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedLeftwardsEquilateralArrowhead => '⮘',
            MiscellaneousSymbolsandArrows::ThreeDashDRightDashLightedUpwardsEquilateralArrowhead => '⮙',
            MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedRightwardsEquilateralArrowhead => '⮚',
            MiscellaneousSymbolsandArrows::ThreeDashDLeftDashLightedDownwardsEquilateralArrowhead => '⮛',
            MiscellaneousSymbolsandArrows::BlackLeftwardsEquilateralArrowhead => '⮜',
            MiscellaneousSymbolsandArrows::BlackUpwardsEquilateralArrowhead => '⮝',
            MiscellaneousSymbolsandArrows::BlackRightwardsEquilateralArrowhead => '⮞',
            MiscellaneousSymbolsandArrows::BlackDownwardsEquilateralArrowhead => '⮟',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipLeftwards => '⮠',
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipRightwards => '⮡',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipLeftwards => '⮢',
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipRightwards => '⮣',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipUpwards => '⮤',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipUpwards => '⮥',
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipDownwards => '⮦',
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipDownwards => '⮧',
            MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndLeftwardsArrow => '⮨',
            MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndRightwardsArrow => '⮩',
            MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndLeftwardsArrow => '⮪',
            MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndRightwardsArrow => '⮫',
            MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndUpwardsArrow => '⮬',
            MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndUpwardsArrow => '⮭',
            MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndDownwardsArrow => '⮮',
            MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndDownwardsArrow => '⮯',
            MiscellaneousSymbolsandArrows::RibbonArrowDownLeft => '⮰',
            MiscellaneousSymbolsandArrows::RibbonArrowDownRight => '⮱',
            MiscellaneousSymbolsandArrows::RibbonArrowUpLeft => '⮲',
            MiscellaneousSymbolsandArrows::RibbonArrowUpRight => '⮳',
            MiscellaneousSymbolsandArrows::RibbonArrowLeftUp => '⮴',
            MiscellaneousSymbolsandArrows::RibbonArrowRightUp => '⮵',
            MiscellaneousSymbolsandArrows::RibbonArrowLeftDown => '⮶',
            MiscellaneousSymbolsandArrows::RibbonArrowRightDown => '⮷',
            MiscellaneousSymbolsandArrows::UpwardsWhiteArrowFromBarWithHorizontalBar => '⮸',
            MiscellaneousSymbolsandArrows::UpArrowheadInARectangleBox => '⮹',
            MiscellaneousSymbolsandArrows::OverlappingWhiteSquares => '⮺',
            MiscellaneousSymbolsandArrows::OverlappingWhiteAndBlackSquares => '⮻',
            MiscellaneousSymbolsandArrows::OverlappingBlackSquares => '⮼',
            MiscellaneousSymbolsandArrows::BallotBoxWithLightX => '⮽',
            MiscellaneousSymbolsandArrows::CircledX => '⮾',
            MiscellaneousSymbolsandArrows::CircledBoldX => '⮿',
            MiscellaneousSymbolsandArrows::BlackSquareCentred => '⯀',
            MiscellaneousSymbolsandArrows::BlackDiamondCentred => '⯁',
            MiscellaneousSymbolsandArrows::TurnedBlackPentagon => '⯂',
            MiscellaneousSymbolsandArrows::HorizontalBlackOctagon => '⯃',
            MiscellaneousSymbolsandArrows::BlackOctagon => '⯄',
            MiscellaneousSymbolsandArrows::BlackMediumUpDashPointingTriangleCentred => '⯅',
            MiscellaneousSymbolsandArrows::BlackMediumDownDashPointingTriangleCentred => '⯆',
            MiscellaneousSymbolsandArrows::BlackMediumLeftDashPointingTriangleCentred => '⯇',
            MiscellaneousSymbolsandArrows::BlackMediumRightDashPointingTriangleCentred => '⯈',
            MiscellaneousSymbolsandArrows::NeptuneFormTwo => '⯉',
            MiscellaneousSymbolsandArrows::TopHalfBlackCircle => '⯊',
            MiscellaneousSymbolsandArrows::BottomHalfBlackCircle => '⯋',
            MiscellaneousSymbolsandArrows::LightFourPointedBlackCusp => '⯌',
            MiscellaneousSymbolsandArrows::RotatedLightFourPointedBlackCusp => '⯍',
            MiscellaneousSymbolsandArrows::WhiteFourPointedCusp => '⯎',
            MiscellaneousSymbolsandArrows::RotatedWhiteFourPointedCusp => '⯏',
            MiscellaneousSymbolsandArrows::SquarePositionIndicator => '⯐',
            MiscellaneousSymbolsandArrows::UncertaintySign => '⯑',
            MiscellaneousSymbolsandArrows::GroupMark => '⯒',
            MiscellaneousSymbolsandArrows::PlutoFormTwo => '⯓',
            MiscellaneousSymbolsandArrows::PlutoFormThree => '⯔',
            MiscellaneousSymbolsandArrows::PlutoFormFour => '⯕',
            MiscellaneousSymbolsandArrows::PlutoFormFive => '⯖',
            MiscellaneousSymbolsandArrows::Transpluto => '⯗',
            MiscellaneousSymbolsandArrows::Proserpina => '⯘',
            MiscellaneousSymbolsandArrows::Astraea => '⯙',
            MiscellaneousSymbolsandArrows::Hygiea => '⯚',
            MiscellaneousSymbolsandArrows::Pholus => '⯛',
            MiscellaneousSymbolsandArrows::Nessus => '⯜',
            MiscellaneousSymbolsandArrows::WhiteMoonSelena => '⯝',
            MiscellaneousSymbolsandArrows::BlackDiamondOnCross => '⯞',
            MiscellaneousSymbolsandArrows::TrueLightMoonArta => '⯟',
            MiscellaneousSymbolsandArrows::Cupido => '⯠',
            MiscellaneousSymbolsandArrows::Hades => '⯡',
            MiscellaneousSymbolsandArrows::Zeus => '⯢',
            MiscellaneousSymbolsandArrows::Kronos => '⯣',
            MiscellaneousSymbolsandArrows::Apollon => '⯤',
            MiscellaneousSymbolsandArrows::Admetos => '⯥',
            MiscellaneousSymbolsandArrows::Vulcanus => '⯦',
            MiscellaneousSymbolsandArrows::Poseidon => '⯧',
            MiscellaneousSymbolsandArrows::LeftHalfBlackStar => '⯨',
            MiscellaneousSymbolsandArrows::RightHalfBlackStar => '⯩',
            MiscellaneousSymbolsandArrows::StarWithLeftHalfBlack => '⯪',
            MiscellaneousSymbolsandArrows::StarWithRightHalfBlack => '⯫',
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTriangleArrowheads => '⯬',
            MiscellaneousSymbolsandArrows::UpwardsTwoDashHeadedArrowWithTriangleArrowheads => '⯭',
            MiscellaneousSymbolsandArrows::RightwardsTwoDashHeadedArrowWithTriangleArrowheads => '⯮',
            MiscellaneousSymbolsandArrows::DownwardsTwoDashHeadedArrowWithTriangleArrowheads => '⯯',
            MiscellaneousSymbolsandArrows::ErisFormOne => '⯰',
            MiscellaneousSymbolsandArrows::ErisFormTwo => '⯱',
            MiscellaneousSymbolsandArrows::Sedna => '⯲',
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolVigintile => '⯳',
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolNovile => '⯴',
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolQuintile => '⯵',
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolBinovile => '⯶',
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolSentagon => '⯷',
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolTredecile => '⯸',
            MiscellaneousSymbolsandArrows::EqualsSignWithInfinityBelow => '⯹',
            MiscellaneousSymbolsandArrows::UnitedSymbol => '⯺',
            MiscellaneousSymbolsandArrows::SeparatedSymbol => '⯻',
            MiscellaneousSymbolsandArrows::DoubledSymbol => '⯼',
            MiscellaneousSymbolsandArrows::PassedSymbol => '⯽',
            MiscellaneousSymbolsandArrows::ReversedRightAngle => '⯾',
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousSymbolsandArrows {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⬀' => Ok(MiscellaneousSymbolsandArrows::NorthEastWhiteArrow),
            '⬁' => Ok(MiscellaneousSymbolsandArrows::NorthWestWhiteArrow),
            '⬂' => Ok(MiscellaneousSymbolsandArrows::SouthEastWhiteArrow),
            '⬃' => Ok(MiscellaneousSymbolsandArrows::SouthWestWhiteArrow),
            '⬄' => Ok(MiscellaneousSymbolsandArrows::LeftRightWhiteArrow),
            '⬅' => Ok(MiscellaneousSymbolsandArrows::LeftwardsBlackArrow),
            '⬆' => Ok(MiscellaneousSymbolsandArrows::UpwardsBlackArrow),
            '⬇' => Ok(MiscellaneousSymbolsandArrows::DownwardsBlackArrow),
            '⬈' => Ok(MiscellaneousSymbolsandArrows::NorthEastBlackArrow),
            '⬉' => Ok(MiscellaneousSymbolsandArrows::NorthWestBlackArrow),
            '⬊' => Ok(MiscellaneousSymbolsandArrows::SouthEastBlackArrow),
            '⬋' => Ok(MiscellaneousSymbolsandArrows::SouthWestBlackArrow),
            '⬌' => Ok(MiscellaneousSymbolsandArrows::LeftRightBlackArrow),
            '⬍' => Ok(MiscellaneousSymbolsandArrows::UpDownBlackArrow),
            '⬎' => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowWithTipDownwards),
            '⬏' => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowWithTipUpwards),
            '⬐' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipDownwards),
            '⬑' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipUpwards),
            '⬒' => Ok(MiscellaneousSymbolsandArrows::SquareWithTopHalfBlack),
            '⬓' => Ok(MiscellaneousSymbolsandArrows::SquareWithBottomHalfBlack),
            '⬔' => Ok(MiscellaneousSymbolsandArrows::SquareWithUpperRightDiagonalHalfBlack),
            '⬕' => Ok(MiscellaneousSymbolsandArrows::SquareWithLowerLeftDiagonalHalfBlack),
            '⬖' => Ok(MiscellaneousSymbolsandArrows::DiamondWithLeftHalfBlack),
            '⬗' => Ok(MiscellaneousSymbolsandArrows::DiamondWithRightHalfBlack),
            '⬘' => Ok(MiscellaneousSymbolsandArrows::DiamondWithTopHalfBlack),
            '⬙' => Ok(MiscellaneousSymbolsandArrows::DiamondWithBottomHalfBlack),
            '⬚' => Ok(MiscellaneousSymbolsandArrows::DottedSquare),
            '⬛' => Ok(MiscellaneousSymbolsandArrows::BlackLargeSquare),
            '⬜' => Ok(MiscellaneousSymbolsandArrows::WhiteLargeSquare),
            '⬝' => Ok(MiscellaneousSymbolsandArrows::BlackVerySmallSquare),
            '⬞' => Ok(MiscellaneousSymbolsandArrows::WhiteVerySmallSquare),
            '⬟' => Ok(MiscellaneousSymbolsandArrows::BlackPentagon),
            '⬠' => Ok(MiscellaneousSymbolsandArrows::WhitePentagon),
            '⬡' => Ok(MiscellaneousSymbolsandArrows::WhiteHexagon),
            '⬢' => Ok(MiscellaneousSymbolsandArrows::BlackHexagon),
            '⬣' => Ok(MiscellaneousSymbolsandArrows::HorizontalBlackHexagon),
            '⬤' => Ok(MiscellaneousSymbolsandArrows::BlackLargeCircle),
            '⬥' => Ok(MiscellaneousSymbolsandArrows::BlackMediumDiamond),
            '⬦' => Ok(MiscellaneousSymbolsandArrows::WhiteMediumDiamond),
            '⬧' => Ok(MiscellaneousSymbolsandArrows::BlackMediumLozenge),
            '⬨' => Ok(MiscellaneousSymbolsandArrows::WhiteMediumLozenge),
            '⬩' => Ok(MiscellaneousSymbolsandArrows::BlackSmallDiamond),
            '⬪' => Ok(MiscellaneousSymbolsandArrows::BlackSmallLozenge),
            '⬫' => Ok(MiscellaneousSymbolsandArrows::WhiteSmallLozenge),
            '⬬' => Ok(MiscellaneousSymbolsandArrows::BlackHorizontalEllipse),
            '⬭' => Ok(MiscellaneousSymbolsandArrows::WhiteHorizontalEllipse),
            '⬮' => Ok(MiscellaneousSymbolsandArrows::BlackVerticalEllipse),
            '⬯' => Ok(MiscellaneousSymbolsandArrows::WhiteVerticalEllipse),
            '⬰' => Ok(MiscellaneousSymbolsandArrows::LeftArrowWithSmallCircle),
            '⬱' => Ok(MiscellaneousSymbolsandArrows::ThreeLeftwardsArrows),
            '⬲' => Ok(MiscellaneousSymbolsandArrows::LeftArrowWithCircledPlus),
            '⬳' => Ok(MiscellaneousSymbolsandArrows::LongLeftwardsSquiggleArrow),
            '⬴' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithVerticalStroke),
            '⬵' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithDoubleVerticalStroke),
            '⬶' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowFromBar),
            '⬷' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedTripleDashArrow),
            '⬸' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithDottedStem),
            '⬹' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithVerticalStroke),
            '⬺' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithDoubleVerticalStroke),
            '⬻' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTail),
            '⬼' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithVerticalStroke),
            '⬽' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke),
            '⬾' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowThroughX),
            '⬿' => Ok(MiscellaneousSymbolsandArrows::WaveArrowPointingDirectlyLeft),
            '⭀' => Ok(MiscellaneousSymbolsandArrows::EqualsSignAboveLeftwardsArrow),
            '⭁' => Ok(MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveLeftwardsArrow),
            '⭂' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseAlmostEqualTo),
            '⭃' => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowThroughGreaterDashThan),
            '⭄' => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowThroughSuperset),
            '⭅' => Ok(MiscellaneousSymbolsandArrows::LeftwardsQuadrupleArrow),
            '⭆' => Ok(MiscellaneousSymbolsandArrows::RightwardsQuadrupleArrow),
            '⭇' => Ok(MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveRightwardsArrow),
            '⭈' => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseAlmostEqualTo),
            '⭉' => Ok(MiscellaneousSymbolsandArrows::TildeOperatorAboveLeftwardsArrow),
            '⭊' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowAboveAlmostEqualTo),
            '⭋' => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseTildeOperator),
            '⭌' => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseTildeOperator),
            '⭍' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedZigzagArrow),
            '⭎' => Ok(MiscellaneousSymbolsandArrows::ShortSlantedNorthArrow),
            '⭏' => Ok(MiscellaneousSymbolsandArrows::ShortBackslantedSouthArrow),
            '⭐' => Ok(MiscellaneousSymbolsandArrows::WhiteMediumStar),
            '⭑' => Ok(MiscellaneousSymbolsandArrows::BlackSmallStar),
            '⭒' => Ok(MiscellaneousSymbolsandArrows::WhiteSmallStar),
            '⭓' => Ok(MiscellaneousSymbolsandArrows::BlackRightDashPointingPentagon),
            '⭔' => Ok(MiscellaneousSymbolsandArrows::WhiteRightDashPointingPentagon),
            '⭕' => Ok(MiscellaneousSymbolsandArrows::HeavyLargeCircle),
            '⭖' => Ok(MiscellaneousSymbolsandArrows::HeavyOvalWithOvalInside),
            '⭗' => Ok(MiscellaneousSymbolsandArrows::HeavyCircleWithCircleInside),
            '⭘' => Ok(MiscellaneousSymbolsandArrows::HeavyCircle),
            '⭙' => Ok(MiscellaneousSymbolsandArrows::HeavyCircledSaltire),
            '⭚' => Ok(MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHookedHead),
            '⭛' => Ok(MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHookedTail),
            '⭜' => Ok(MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHorizontalTail),
            '⭝' => Ok(MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHorizontalTail),
            '⭞' => Ok(MiscellaneousSymbolsandArrows::BentArrowPointingDownwardsThenNorthEast),
            '⭟' => Ok(MiscellaneousSymbolsandArrows::ShortBentArrowPointingDownwardsThenNorthEast),
            '⭠' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrow),
            '⭡' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrow),
            '⭢' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrow),
            '⭣' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrow),
            '⭤' => Ok(MiscellaneousSymbolsandArrows::LeftRightTriangleDashHeadedArrow),
            '⭥' => Ok(MiscellaneousSymbolsandArrows::UpDownTriangleDashHeadedArrow),
            '⭦' => Ok(MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrow),
            '⭧' => Ok(MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrow),
            '⭨' => Ok(MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrow),
            '⭩' => Ok(MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrow),
            '⭪' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedDashedArrow),
            '⭫' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedDashedArrow),
            '⭬' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedDashedArrow),
            '⭭' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedDashedArrow),
            '⭮' => Ok(MiscellaneousSymbolsandArrows::ClockwiseTriangleDashHeadedOpenCircleArrow),
            '⭯' => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedOpenCircleArrow),
            '⭰' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowToBar),
            '⭱' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowToBar),
            '⭲' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowToBar),
            '⭳' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowToBar),
            '⭶' => Ok(MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrowToBar),
            '⭷' => Ok(MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrowToBar),
            '⭸' => Ok(MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrowToBar),
            '⭹' => Ok(MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrowToBar),
            '⭺' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            '⭻' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            '⭼' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            '⭽' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            '⭾' => Ok(MiscellaneousSymbolsandArrows::HorizontalTabKey),
            '⭿' => Ok(MiscellaneousSymbolsandArrows::VerticalTabKey),
            '⮀' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowOverRightwardsTriangleDashHeadedArrow),
            '⮁' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowLeftwardsOfDownwardsTriangleDashHeadedArrow),
            '⮂' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowOverLeftwardsTriangleDashHeadedArrow),
            '⮃' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowLeftwardsOfUpwardsTriangleDashHeadedArrow),
            '⮄' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedPairedArrows),
            '⮅' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedPairedArrows),
            '⮆' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedPairedArrows),
            '⮇' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedPairedArrows),
            '⮈' => Ok(MiscellaneousSymbolsandArrows::LeftwardsBlackCircledWhiteArrow),
            '⮉' => Ok(MiscellaneousSymbolsandArrows::UpwardsBlackCircledWhiteArrow),
            '⮊' => Ok(MiscellaneousSymbolsandArrows::RightwardsBlackCircledWhiteArrow),
            '⮋' => Ok(MiscellaneousSymbolsandArrows::DownwardsBlackCircledWhiteArrow),
            '⮌' => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedRightUDashShapedArrow),
            '⮍' => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedBottomUDashShapedArrow),
            '⮎' => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedLeftUDashShapedArrow),
            '⮏' => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedTopUDashShapedArrow),
            '⮐' => Ok(MiscellaneousSymbolsandArrows::ReturnLeft),
            '⮑' => Ok(MiscellaneousSymbolsandArrows::ReturnRight),
            '⮒' => Ok(MiscellaneousSymbolsandArrows::NewlineLeft),
            '⮓' => Ok(MiscellaneousSymbolsandArrows::NewlineRight),
            '⮔' => Ok(MiscellaneousSymbolsandArrows::FourCornerArrowsCirclingAnticlockwise),
            '⮕' => Ok(MiscellaneousSymbolsandArrows::RightwardsBlackArrow),
            '⮘' => Ok(MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedLeftwardsEquilateralArrowhead),
            '⮙' => Ok(MiscellaneousSymbolsandArrows::ThreeDashDRightDashLightedUpwardsEquilateralArrowhead),
            '⮚' => Ok(MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedRightwardsEquilateralArrowhead),
            '⮛' => Ok(MiscellaneousSymbolsandArrows::ThreeDashDLeftDashLightedDownwardsEquilateralArrowhead),
            '⮜' => Ok(MiscellaneousSymbolsandArrows::BlackLeftwardsEquilateralArrowhead),
            '⮝' => Ok(MiscellaneousSymbolsandArrows::BlackUpwardsEquilateralArrowhead),
            '⮞' => Ok(MiscellaneousSymbolsandArrows::BlackRightwardsEquilateralArrowhead),
            '⮟' => Ok(MiscellaneousSymbolsandArrows::BlackDownwardsEquilateralArrowhead),
            '⮠' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipLeftwards),
            '⮡' => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipRightwards),
            '⮢' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipLeftwards),
            '⮣' => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipRightwards),
            '⮤' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipUpwards),
            '⮥' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipUpwards),
            '⮦' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipDownwards),
            '⮧' => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipDownwards),
            '⮨' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndLeftwardsArrow),
            '⮩' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndRightwardsArrow),
            '⮪' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndLeftwardsArrow),
            '⮫' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndRightwardsArrow),
            '⮬' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndUpwardsArrow),
            '⮭' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndUpwardsArrow),
            '⮮' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndDownwardsArrow),
            '⮯' => Ok(MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndDownwardsArrow),
            '⮰' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowDownLeft),
            '⮱' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowDownRight),
            '⮲' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowUpLeft),
            '⮳' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowUpRight),
            '⮴' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowLeftUp),
            '⮵' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowRightUp),
            '⮶' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowLeftDown),
            '⮷' => Ok(MiscellaneousSymbolsandArrows::RibbonArrowRightDown),
            '⮸' => Ok(MiscellaneousSymbolsandArrows::UpwardsWhiteArrowFromBarWithHorizontalBar),
            '⮹' => Ok(MiscellaneousSymbolsandArrows::UpArrowheadInARectangleBox),
            '⮺' => Ok(MiscellaneousSymbolsandArrows::OverlappingWhiteSquares),
            '⮻' => Ok(MiscellaneousSymbolsandArrows::OverlappingWhiteAndBlackSquares),
            '⮼' => Ok(MiscellaneousSymbolsandArrows::OverlappingBlackSquares),
            '⮽' => Ok(MiscellaneousSymbolsandArrows::BallotBoxWithLightX),
            '⮾' => Ok(MiscellaneousSymbolsandArrows::CircledX),
            '⮿' => Ok(MiscellaneousSymbolsandArrows::CircledBoldX),
            '⯀' => Ok(MiscellaneousSymbolsandArrows::BlackSquareCentred),
            '⯁' => Ok(MiscellaneousSymbolsandArrows::BlackDiamondCentred),
            '⯂' => Ok(MiscellaneousSymbolsandArrows::TurnedBlackPentagon),
            '⯃' => Ok(MiscellaneousSymbolsandArrows::HorizontalBlackOctagon),
            '⯄' => Ok(MiscellaneousSymbolsandArrows::BlackOctagon),
            '⯅' => Ok(MiscellaneousSymbolsandArrows::BlackMediumUpDashPointingTriangleCentred),
            '⯆' => Ok(MiscellaneousSymbolsandArrows::BlackMediumDownDashPointingTriangleCentred),
            '⯇' => Ok(MiscellaneousSymbolsandArrows::BlackMediumLeftDashPointingTriangleCentred),
            '⯈' => Ok(MiscellaneousSymbolsandArrows::BlackMediumRightDashPointingTriangleCentred),
            '⯉' => Ok(MiscellaneousSymbolsandArrows::NeptuneFormTwo),
            '⯊' => Ok(MiscellaneousSymbolsandArrows::TopHalfBlackCircle),
            '⯋' => Ok(MiscellaneousSymbolsandArrows::BottomHalfBlackCircle),
            '⯌' => Ok(MiscellaneousSymbolsandArrows::LightFourPointedBlackCusp),
            '⯍' => Ok(MiscellaneousSymbolsandArrows::RotatedLightFourPointedBlackCusp),
            '⯎' => Ok(MiscellaneousSymbolsandArrows::WhiteFourPointedCusp),
            '⯏' => Ok(MiscellaneousSymbolsandArrows::RotatedWhiteFourPointedCusp),
            '⯐' => Ok(MiscellaneousSymbolsandArrows::SquarePositionIndicator),
            '⯑' => Ok(MiscellaneousSymbolsandArrows::UncertaintySign),
            '⯒' => Ok(MiscellaneousSymbolsandArrows::GroupMark),
            '⯓' => Ok(MiscellaneousSymbolsandArrows::PlutoFormTwo),
            '⯔' => Ok(MiscellaneousSymbolsandArrows::PlutoFormThree),
            '⯕' => Ok(MiscellaneousSymbolsandArrows::PlutoFormFour),
            '⯖' => Ok(MiscellaneousSymbolsandArrows::PlutoFormFive),
            '⯗' => Ok(MiscellaneousSymbolsandArrows::Transpluto),
            '⯘' => Ok(MiscellaneousSymbolsandArrows::Proserpina),
            '⯙' => Ok(MiscellaneousSymbolsandArrows::Astraea),
            '⯚' => Ok(MiscellaneousSymbolsandArrows::Hygiea),
            '⯛' => Ok(MiscellaneousSymbolsandArrows::Pholus),
            '⯜' => Ok(MiscellaneousSymbolsandArrows::Nessus),
            '⯝' => Ok(MiscellaneousSymbolsandArrows::WhiteMoonSelena),
            '⯞' => Ok(MiscellaneousSymbolsandArrows::BlackDiamondOnCross),
            '⯟' => Ok(MiscellaneousSymbolsandArrows::TrueLightMoonArta),
            '⯠' => Ok(MiscellaneousSymbolsandArrows::Cupido),
            '⯡' => Ok(MiscellaneousSymbolsandArrows::Hades),
            '⯢' => Ok(MiscellaneousSymbolsandArrows::Zeus),
            '⯣' => Ok(MiscellaneousSymbolsandArrows::Kronos),
            '⯤' => Ok(MiscellaneousSymbolsandArrows::Apollon),
            '⯥' => Ok(MiscellaneousSymbolsandArrows::Admetos),
            '⯦' => Ok(MiscellaneousSymbolsandArrows::Vulcanus),
            '⯧' => Ok(MiscellaneousSymbolsandArrows::Poseidon),
            '⯨' => Ok(MiscellaneousSymbolsandArrows::LeftHalfBlackStar),
            '⯩' => Ok(MiscellaneousSymbolsandArrows::RightHalfBlackStar),
            '⯪' => Ok(MiscellaneousSymbolsandArrows::StarWithLeftHalfBlack),
            '⯫' => Ok(MiscellaneousSymbolsandArrows::StarWithRightHalfBlack),
            '⯬' => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTriangleArrowheads),
            '⯭' => Ok(MiscellaneousSymbolsandArrows::UpwardsTwoDashHeadedArrowWithTriangleArrowheads),
            '⯮' => Ok(MiscellaneousSymbolsandArrows::RightwardsTwoDashHeadedArrowWithTriangleArrowheads),
            '⯯' => Ok(MiscellaneousSymbolsandArrows::DownwardsTwoDashHeadedArrowWithTriangleArrowheads),
            '⯰' => Ok(MiscellaneousSymbolsandArrows::ErisFormOne),
            '⯱' => Ok(MiscellaneousSymbolsandArrows::ErisFormTwo),
            '⯲' => Ok(MiscellaneousSymbolsandArrows::Sedna),
            '⯳' => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolVigintile),
            '⯴' => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolNovile),
            '⯵' => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolQuintile),
            '⯶' => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolBinovile),
            '⯷' => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolSentagon),
            '⯸' => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolTredecile),
            '⯹' => Ok(MiscellaneousSymbolsandArrows::EqualsSignWithInfinityBelow),
            '⯺' => Ok(MiscellaneousSymbolsandArrows::UnitedSymbol),
            '⯻' => Ok(MiscellaneousSymbolsandArrows::SeparatedSymbol),
            '⯼' => Ok(MiscellaneousSymbolsandArrows::DoubledSymbol),
            '⯽' => Ok(MiscellaneousSymbolsandArrows::PassedSymbol),
            '⯾' => Ok(MiscellaneousSymbolsandArrows::ReversedRightAngle),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MiscellaneousSymbolsandArrows {
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

impl std::convert::TryFrom<u32> for MiscellaneousSymbolsandArrows {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MiscellaneousSymbolsandArrows {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MiscellaneousSymbolsandArrows {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MiscellaneousSymbolsandArrows::NorthEastWhiteArrow
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MiscellaneousSymbolsandArrows{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
