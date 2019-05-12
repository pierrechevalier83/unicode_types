
/// An enum to represent all characters in the MiscellaneousTechnical block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MiscellaneousTechnical {
    /// \u{2300}: '⌀'
    DiameterSign,
    /// \u{2301}: '⌁'
    ElectricArrow,
    /// \u{2302}: '⌂'
    House,
    /// \u{2303}: '⌃'
    UpArrowhead,
    /// \u{2304}: '⌄'
    DownArrowhead,
    /// \u{2305}: '⌅'
    Projective,
    /// \u{2306}: '⌆'
    Perspective,
    /// \u{2307}: '⌇'
    WavyLine,
    /// \u{2308}: '⌈'
    LeftCeiling,
    /// \u{2309}: '⌉'
    RightCeiling,
    /// \u{230a}: '⌊'
    LeftFloor,
    /// \u{230b}: '⌋'
    RightFloor,
    /// \u{230c}: '⌌'
    BottomRightCrop,
    /// \u{230d}: '⌍'
    BottomLeftCrop,
    /// \u{230e}: '⌎'
    TopRightCrop,
    /// \u{230f}: '⌏'
    TopLeftCrop,
    /// \u{2310}: '⌐'
    ReversedNotSign,
    /// \u{2311}: '⌑'
    SquareLozenge,
    /// \u{2312}: '⌒'
    Arc,
    /// \u{2313}: '⌓'
    Segment,
    /// \u{2314}: '⌔'
    Sector,
    /// \u{2315}: '⌕'
    TelephoneRecorder,
    /// \u{2316}: '⌖'
    PositionIndicator,
    /// \u{2317}: '⌗'
    ViewdataSquare,
    /// \u{2318}: '⌘'
    PlaceOfInterestSign,
    /// \u{2319}: '⌙'
    TurnedNotSign,
    /// \u{231a}: '⌚'
    Watch,
    /// \u{231b}: '⌛'
    Hourglass,
    /// \u{231c}: '⌜'
    TopLeftCorner,
    /// \u{231d}: '⌝'
    TopRightCorner,
    /// \u{231e}: '⌞'
    BottomLeftCorner,
    /// \u{231f}: '⌟'
    BottomRightCorner,
    /// \u{2320}: '⌠'
    TopHalfIntegral,
    /// \u{2321}: '⌡'
    BottomHalfIntegral,
    /// \u{2322}: '⌢'
    Frown,
    /// \u{2323}: '⌣'
    Smile,
    /// \u{2324}: '⌤'
    UpArrowheadBetweenTwoHorizontalBars,
    /// \u{2325}: '⌥'
    OptionKey,
    /// \u{2326}: '⌦'
    EraseToTheRight,
    /// \u{2327}: '⌧'
    XInARectangleBox,
    /// \u{2328}: '⌨'
    Keyboard,
    /// \u{2329}: '〈'
    LeftDashPointingAngleBracket,
    /// \u{232a}: '〉'
    RightDashPointingAngleBracket,
    /// \u{232b}: '⌫'
    EraseToTheLeft,
    /// \u{232c}: '⌬'
    BenzeneRing,
    /// \u{232d}: '⌭'
    Cylindricity,
    /// \u{232e}: '⌮'
    AllAroundDashProfile,
    /// \u{232f}: '⌯'
    Symmetry,
    /// \u{2330}: '⌰'
    TotalRunout,
    /// \u{2331}: '⌱'
    DimensionOrigin,
    /// \u{2332}: '⌲'
    ConicalTaper,
    /// \u{2333}: '⌳'
    Slope,
    /// \u{2334}: '⌴'
    Counterbore,
    /// \u{2335}: '⌵'
    Countersink,
    /// \u{2336}: '⌶'
    AplFunctionalSymbolIDashBeam,
    /// \u{2337}: '⌷'
    AplFunctionalSymbolSquishQuad,
    /// \u{2338}: '⌸'
    AplFunctionalSymbolQuadEqual,
    /// \u{2339}: '⌹'
    AplFunctionalSymbolQuadDivide,
    /// \u{233a}: '⌺'
    AplFunctionalSymbolQuadDiamond,
    /// \u{233b}: '⌻'
    AplFunctionalSymbolQuadJot,
    /// \u{233c}: '⌼'
    AplFunctionalSymbolQuadCircle,
    /// \u{233d}: '⌽'
    AplFunctionalSymbolCircleStile,
    /// \u{233e}: '⌾'
    AplFunctionalSymbolCircleJot,
    /// \u{233f}: '⌿'
    AplFunctionalSymbolSlashBar,
    /// \u{2340}: '⍀'
    AplFunctionalSymbolBackslashBar,
    /// \u{2341}: '⍁'
    AplFunctionalSymbolQuadSlash,
    /// \u{2342}: '⍂'
    AplFunctionalSymbolQuadBackslash,
    /// \u{2343}: '⍃'
    AplFunctionalSymbolQuadLessDashThan,
    /// \u{2344}: '⍄'
    AplFunctionalSymbolQuadGreaterDashThan,
    /// \u{2345}: '⍅'
    AplFunctionalSymbolLeftwardsVane,
    /// \u{2346}: '⍆'
    AplFunctionalSymbolRightwardsVane,
    /// \u{2347}: '⍇'
    AplFunctionalSymbolQuadLeftwardsArrow,
    /// \u{2348}: '⍈'
    AplFunctionalSymbolQuadRightwardsArrow,
    /// \u{2349}: '⍉'
    AplFunctionalSymbolCircleBackslash,
    /// \u{234a}: '⍊'
    AplFunctionalSymbolDownTackUnderbar,
    /// \u{234b}: '⍋'
    AplFunctionalSymbolDeltaStile,
    /// \u{234c}: '⍌'
    AplFunctionalSymbolQuadDownCaret,
    /// \u{234d}: '⍍'
    AplFunctionalSymbolQuadDelta,
    /// \u{234e}: '⍎'
    AplFunctionalSymbolDownTackJot,
    /// \u{234f}: '⍏'
    AplFunctionalSymbolUpwardsVane,
    /// \u{2350}: '⍐'
    AplFunctionalSymbolQuadUpwardsArrow,
    /// \u{2351}: '⍑'
    AplFunctionalSymbolUpTackOverbar,
    /// \u{2352}: '⍒'
    AplFunctionalSymbolDelStile,
    /// \u{2353}: '⍓'
    AplFunctionalSymbolQuadUpCaret,
    /// \u{2354}: '⍔'
    AplFunctionalSymbolQuadDel,
    /// \u{2355}: '⍕'
    AplFunctionalSymbolUpTackJot,
    /// \u{2356}: '⍖'
    AplFunctionalSymbolDownwardsVane,
    /// \u{2357}: '⍗'
    AplFunctionalSymbolQuadDownwardsArrow,
    /// \u{2358}: '⍘'
    AplFunctionalSymbolQuoteUnderbar,
    /// \u{2359}: '⍙'
    AplFunctionalSymbolDeltaUnderbar,
    /// \u{235a}: '⍚'
    AplFunctionalSymbolDiamondUnderbar,
    /// \u{235b}: '⍛'
    AplFunctionalSymbolJotUnderbar,
    /// \u{235c}: '⍜'
    AplFunctionalSymbolCircleUnderbar,
    /// \u{235d}: '⍝'
    AplFunctionalSymbolUpShoeJot,
    /// \u{235e}: '⍞'
    AplFunctionalSymbolQuoteQuad,
    /// \u{235f}: '⍟'
    AplFunctionalSymbolCircleStar,
    /// \u{2360}: '⍠'
    AplFunctionalSymbolQuadColon,
    /// \u{2361}: '⍡'
    AplFunctionalSymbolUpTackDiaeresis,
    /// \u{2362}: '⍢'
    AplFunctionalSymbolDelDiaeresis,
    /// \u{2363}: '⍣'
    AplFunctionalSymbolStarDiaeresis,
    /// \u{2364}: '⍤'
    AplFunctionalSymbolJotDiaeresis,
    /// \u{2365}: '⍥'
    AplFunctionalSymbolCircleDiaeresis,
    /// \u{2366}: '⍦'
    AplFunctionalSymbolDownShoeStile,
    /// \u{2367}: '⍧'
    AplFunctionalSymbolLeftShoeStile,
    /// \u{2368}: '⍨'
    AplFunctionalSymbolTildeDiaeresis,
    /// \u{2369}: '⍩'
    AplFunctionalSymbolGreaterDashThanDiaeresis,
    /// \u{236a}: '⍪'
    AplFunctionalSymbolCommaBar,
    /// \u{236b}: '⍫'
    AplFunctionalSymbolDelTilde,
    /// \u{236c}: '⍬'
    AplFunctionalSymbolZilde,
    /// \u{236d}: '⍭'
    AplFunctionalSymbolStileTilde,
    /// \u{236e}: '⍮'
    AplFunctionalSymbolSemicolonUnderbar,
    /// \u{236f}: '⍯'
    AplFunctionalSymbolQuadNotEqual,
    /// \u{2370}: '⍰'
    AplFunctionalSymbolQuadQuestion,
    /// \u{2371}: '⍱'
    AplFunctionalSymbolDownCaretTilde,
    /// \u{2372}: '⍲'
    AplFunctionalSymbolUpCaretTilde,
    /// \u{2373}: '⍳'
    AplFunctionalSymbolIota,
    /// \u{2374}: '⍴'
    AplFunctionalSymbolRho,
    /// \u{2375}: '⍵'
    AplFunctionalSymbolOmega,
    /// \u{2376}: '⍶'
    AplFunctionalSymbolAlphaUnderbar,
    /// \u{2377}: '⍷'
    AplFunctionalSymbolEpsilonUnderbar,
    /// \u{2378}: '⍸'
    AplFunctionalSymbolIotaUnderbar,
    /// \u{2379}: '⍹'
    AplFunctionalSymbolOmegaUnderbar,
    /// \u{237a}: '⍺'
    AplFunctionalSymbolAlpha,
    /// \u{237b}: '⍻'
    NotCheckMark,
    /// \u{237c}: '⍼'
    RightAngleWithDownwardsZigzagArrow,
    /// \u{237d}: '⍽'
    ShoulderedOpenBox,
    /// \u{237e}: '⍾'
    BellSymbol,
    /// \u{237f}: '⍿'
    VerticalLineWithMiddleDot,
    /// \u{2380}: '⎀'
    InsertionSymbol,
    /// \u{2381}: '⎁'
    ContinuousUnderlineSymbol,
    /// \u{2382}: '⎂'
    DiscontinuousUnderlineSymbol,
    /// \u{2383}: '⎃'
    EmphasisSymbol,
    /// \u{2384}: '⎄'
    CompositionSymbol,
    /// \u{2385}: '⎅'
    WhiteSquareWithCentreVerticalLine,
    /// \u{2386}: '⎆'
    EnterSymbol,
    /// \u{2387}: '⎇'
    AlternativeKeySymbol,
    /// \u{2388}: '⎈'
    HelmSymbol,
    /// \u{2389}: '⎉'
    CircledHorizontalBarWithNotch,
    /// \u{238a}: '⎊'
    CircledTriangleDown,
    /// \u{238b}: '⎋'
    BrokenCircleWithNorthwestArrow,
    /// \u{238c}: '⎌'
    UndoSymbol,
    /// \u{238d}: '⎍'
    MonostableSymbol,
    /// \u{238e}: '⎎'
    HysteresisSymbol,
    /// \u{238f}: '⎏'
    OpenDashCircuitDashOutputHDashTypeSymbol,
    /// \u{2390}: '⎐'
    OpenDashCircuitDashOutputLDashTypeSymbol,
    /// \u{2391}: '⎑'
    PassiveDashPullDashDownDashOutputSymbol,
    /// \u{2392}: '⎒'
    PassiveDashPullDashUpDashOutputSymbol,
    /// \u{2393}: '⎓'
    DirectCurrentSymbolFormTwo,
    /// \u{2394}: '⎔'
    SoftwareDashFunctionSymbol,
    /// \u{2395}: '⎕'
    AplFunctionalSymbolQuad,
    /// \u{2396}: '⎖'
    DecimalSeparatorKeySymbol,
    /// \u{2397}: '⎗'
    PreviousPage,
    /// \u{2398}: '⎘'
    NextPage,
    /// \u{2399}: '⎙'
    PrintScreenSymbol,
    /// \u{239a}: '⎚'
    ClearScreenSymbol,
    /// \u{239b}: '⎛'
    LeftParenthesisUpperHook,
    /// \u{239c}: '⎜'
    LeftParenthesisExtension,
    /// \u{239d}: '⎝'
    LeftParenthesisLowerHook,
    /// \u{239e}: '⎞'
    RightParenthesisUpperHook,
    /// \u{239f}: '⎟'
    RightParenthesisExtension,
    /// \u{23a0}: '⎠'
    RightParenthesisLowerHook,
    /// \u{23a1}: '⎡'
    LeftSquareBracketUpperCorner,
    /// \u{23a2}: '⎢'
    LeftSquareBracketExtension,
    /// \u{23a3}: '⎣'
    LeftSquareBracketLowerCorner,
    /// \u{23a4}: '⎤'
    RightSquareBracketUpperCorner,
    /// \u{23a5}: '⎥'
    RightSquareBracketExtension,
    /// \u{23a6}: '⎦'
    RightSquareBracketLowerCorner,
    /// \u{23a7}: '⎧'
    LeftCurlyBracketUpperHook,
    /// \u{23a8}: '⎨'
    LeftCurlyBracketMiddlePiece,
    /// \u{23a9}: '⎩'
    LeftCurlyBracketLowerHook,
    /// \u{23aa}: '⎪'
    CurlyBracketExtension,
    /// \u{23ab}: '⎫'
    RightCurlyBracketUpperHook,
    /// \u{23ac}: '⎬'
    RightCurlyBracketMiddlePiece,
    /// \u{23ad}: '⎭'
    RightCurlyBracketLowerHook,
    /// \u{23ae}: '⎮'
    IntegralExtension,
    /// \u{23af}: '⎯'
    HorizontalLineExtension,
    /// \u{23b0}: '⎰'
    UpperLeftOrLowerRightCurlyBracketSection,
    /// \u{23b1}: '⎱'
    UpperRightOrLowerLeftCurlyBracketSection,
    /// \u{23b2}: '⎲'
    SummationTop,
    /// \u{23b3}: '⎳'
    SummationBottom,
    /// \u{23b4}: '⎴'
    TopSquareBracket,
    /// \u{23b5}: '⎵'
    BottomSquareBracket,
    /// \u{23b6}: '⎶'
    BottomSquareBracketOverTopSquareBracket,
    /// \u{23b7}: '⎷'
    RadicalSymbolBottom,
    /// \u{23b8}: '⎸'
    LeftVerticalBoxLine,
    /// \u{23b9}: '⎹'
    RightVerticalBoxLine,
    /// \u{23ba}: '⎺'
    HorizontalScanLineDash1,
    /// \u{23bb}: '⎻'
    HorizontalScanLineDash3,
    /// \u{23bc}: '⎼'
    HorizontalScanLineDash7,
    /// \u{23bd}: '⎽'
    HorizontalScanLineDash9,
    /// \u{23be}: '⎾'
    DentistrySymbolLightVerticalAndTopRight,
    /// \u{23bf}: '⎿'
    DentistrySymbolLightVerticalAndBottomRight,
    /// \u{23c0}: '⏀'
    DentistrySymbolLightVerticalWithCircle,
    /// \u{23c1}: '⏁'
    DentistrySymbolLightDownAndHorizontalWithCircle,
    /// \u{23c2}: '⏂'
    DentistrySymbolLightUpAndHorizontalWithCircle,
    /// \u{23c3}: '⏃'
    DentistrySymbolLightVerticalWithTriangle,
    /// \u{23c4}: '⏄'
    DentistrySymbolLightDownAndHorizontalWithTriangle,
    /// \u{23c5}: '⏅'
    DentistrySymbolLightUpAndHorizontalWithTriangle,
    /// \u{23c6}: '⏆'
    DentistrySymbolLightVerticalAndWave,
    /// \u{23c7}: '⏇'
    DentistrySymbolLightDownAndHorizontalWithWave,
    /// \u{23c8}: '⏈'
    DentistrySymbolLightUpAndHorizontalWithWave,
    /// \u{23c9}: '⏉'
    DentistrySymbolLightDownAndHorizontal,
    /// \u{23ca}: '⏊'
    DentistrySymbolLightUpAndHorizontal,
    /// \u{23cb}: '⏋'
    DentistrySymbolLightVerticalAndTopLeft,
    /// \u{23cc}: '⏌'
    DentistrySymbolLightVerticalAndBottomLeft,
    /// \u{23cd}: '⏍'
    SquareFoot,
    /// \u{23ce}: '⏎'
    ReturnSymbol,
    /// \u{23cf}: '⏏'
    EjectSymbol,
    /// \u{23d0}: '⏐'
    VerticalLineExtension,
    /// \u{23d1}: '⏑'
    MetricalBreve,
    /// \u{23d2}: '⏒'
    MetricalLongOverShort,
    /// \u{23d3}: '⏓'
    MetricalShortOverLong,
    /// \u{23d4}: '⏔'
    MetricalLongOverTwoShorts,
    /// \u{23d5}: '⏕'
    MetricalTwoShortsOverLong,
    /// \u{23d6}: '⏖'
    MetricalTwoShortsJoined,
    /// \u{23d7}: '⏗'
    MetricalTriseme,
    /// \u{23d8}: '⏘'
    MetricalTetraseme,
    /// \u{23d9}: '⏙'
    MetricalPentaseme,
    /// \u{23da}: '⏚'
    EarthGround,
    /// \u{23db}: '⏛'
    Fuse,
    /// \u{23dc}: '⏜'
    TopParenthesis,
    /// \u{23dd}: '⏝'
    BottomParenthesis,
    /// \u{23de}: '⏞'
    TopCurlyBracket,
    /// \u{23df}: '⏟'
    BottomCurlyBracket,
    /// \u{23e0}: '⏠'
    TopTortoiseShellBracket,
    /// \u{23e1}: '⏡'
    BottomTortoiseShellBracket,
    /// \u{23e2}: '⏢'
    WhiteTrapezium,
    /// \u{23e3}: '⏣'
    BenzeneRingWithCircle,
    /// \u{23e4}: '⏤'
    Straightness,
    /// \u{23e5}: '⏥'
    Flatness,
    /// \u{23e6}: '⏦'
    AcCurrent,
    /// \u{23e7}: '⏧'
    ElectricalIntersection,
    /// \u{23e8}: '⏨'
    DecimalExponentSymbol,
    /// \u{23e9}: '⏩'
    BlackRightDashPointingDoubleTriangle,
    /// \u{23ea}: '⏪'
    BlackLeftDashPointingDoubleTriangle,
    /// \u{23eb}: '⏫'
    BlackUpDashPointingDoubleTriangle,
    /// \u{23ec}: '⏬'
    BlackDownDashPointingDoubleTriangle,
    /// \u{23ed}: '⏭'
    BlackRightDashPointingDoubleTriangleWithVerticalBar,
    /// \u{23ee}: '⏮'
    BlackLeftDashPointingDoubleTriangleWithVerticalBar,
    /// \u{23ef}: '⏯'
    BlackRightDashPointingTriangleWithDoubleVerticalBar,
    /// \u{23f0}: '⏰'
    AlarmClock,
    /// \u{23f1}: '⏱'
    Stopwatch,
    /// \u{23f2}: '⏲'
    TimerClock,
    /// \u{23f3}: '⏳'
    HourglassWithFlowingSand,
    /// \u{23f4}: '⏴'
    BlackMediumLeftDashPointingTriangle,
    /// \u{23f5}: '⏵'
    BlackMediumRightDashPointingTriangle,
    /// \u{23f6}: '⏶'
    BlackMediumUpDashPointingTriangle,
    /// \u{23f7}: '⏷'
    BlackMediumDownDashPointingTriangle,
    /// \u{23f8}: '⏸'
    DoubleVerticalBar,
    /// \u{23f9}: '⏹'
    BlackSquareForStop,
    /// \u{23fa}: '⏺'
    BlackCircleForRecord,
    /// \u{23fb}: '⏻'
    PowerSymbol,
    /// \u{23fc}: '⏼'
    PowerOnDashOffSymbol,
    /// \u{23fd}: '⏽'
    PowerOnSymbol,
    /// \u{23fe}: '⏾'
    PowerSleepSymbol,
}

impl Into<char> for MiscellaneousTechnical {
    fn into(self) -> char {
        match self {
            MiscellaneousTechnical::DiameterSign => '⌀',
            MiscellaneousTechnical::ElectricArrow => '⌁',
            MiscellaneousTechnical::House => '⌂',
            MiscellaneousTechnical::UpArrowhead => '⌃',
            MiscellaneousTechnical::DownArrowhead => '⌄',
            MiscellaneousTechnical::Projective => '⌅',
            MiscellaneousTechnical::Perspective => '⌆',
            MiscellaneousTechnical::WavyLine => '⌇',
            MiscellaneousTechnical::LeftCeiling => '⌈',
            MiscellaneousTechnical::RightCeiling => '⌉',
            MiscellaneousTechnical::LeftFloor => '⌊',
            MiscellaneousTechnical::RightFloor => '⌋',
            MiscellaneousTechnical::BottomRightCrop => '⌌',
            MiscellaneousTechnical::BottomLeftCrop => '⌍',
            MiscellaneousTechnical::TopRightCrop => '⌎',
            MiscellaneousTechnical::TopLeftCrop => '⌏',
            MiscellaneousTechnical::ReversedNotSign => '⌐',
            MiscellaneousTechnical::SquareLozenge => '⌑',
            MiscellaneousTechnical::Arc => '⌒',
            MiscellaneousTechnical::Segment => '⌓',
            MiscellaneousTechnical::Sector => '⌔',
            MiscellaneousTechnical::TelephoneRecorder => '⌕',
            MiscellaneousTechnical::PositionIndicator => '⌖',
            MiscellaneousTechnical::ViewdataSquare => '⌗',
            MiscellaneousTechnical::PlaceOfInterestSign => '⌘',
            MiscellaneousTechnical::TurnedNotSign => '⌙',
            MiscellaneousTechnical::Watch => '⌚',
            MiscellaneousTechnical::Hourglass => '⌛',
            MiscellaneousTechnical::TopLeftCorner => '⌜',
            MiscellaneousTechnical::TopRightCorner => '⌝',
            MiscellaneousTechnical::BottomLeftCorner => '⌞',
            MiscellaneousTechnical::BottomRightCorner => '⌟',
            MiscellaneousTechnical::TopHalfIntegral => '⌠',
            MiscellaneousTechnical::BottomHalfIntegral => '⌡',
            MiscellaneousTechnical::Frown => '⌢',
            MiscellaneousTechnical::Smile => '⌣',
            MiscellaneousTechnical::UpArrowheadBetweenTwoHorizontalBars => '⌤',
            MiscellaneousTechnical::OptionKey => '⌥',
            MiscellaneousTechnical::EraseToTheRight => '⌦',
            MiscellaneousTechnical::XInARectangleBox => '⌧',
            MiscellaneousTechnical::Keyboard => '⌨',
            MiscellaneousTechnical::LeftDashPointingAngleBracket => '〈',
            MiscellaneousTechnical::RightDashPointingAngleBracket => '〉',
            MiscellaneousTechnical::EraseToTheLeft => '⌫',
            MiscellaneousTechnical::BenzeneRing => '⌬',
            MiscellaneousTechnical::Cylindricity => '⌭',
            MiscellaneousTechnical::AllAroundDashProfile => '⌮',
            MiscellaneousTechnical::Symmetry => '⌯',
            MiscellaneousTechnical::TotalRunout => '⌰',
            MiscellaneousTechnical::DimensionOrigin => '⌱',
            MiscellaneousTechnical::ConicalTaper => '⌲',
            MiscellaneousTechnical::Slope => '⌳',
            MiscellaneousTechnical::Counterbore => '⌴',
            MiscellaneousTechnical::Countersink => '⌵',
            MiscellaneousTechnical::AplFunctionalSymbolIDashBeam => '⌶',
            MiscellaneousTechnical::AplFunctionalSymbolSquishQuad => '⌷',
            MiscellaneousTechnical::AplFunctionalSymbolQuadEqual => '⌸',
            MiscellaneousTechnical::AplFunctionalSymbolQuadDivide => '⌹',
            MiscellaneousTechnical::AplFunctionalSymbolQuadDiamond => '⌺',
            MiscellaneousTechnical::AplFunctionalSymbolQuadJot => '⌻',
            MiscellaneousTechnical::AplFunctionalSymbolQuadCircle => '⌼',
            MiscellaneousTechnical::AplFunctionalSymbolCircleStile => '⌽',
            MiscellaneousTechnical::AplFunctionalSymbolCircleJot => '⌾',
            MiscellaneousTechnical::AplFunctionalSymbolSlashBar => '⌿',
            MiscellaneousTechnical::AplFunctionalSymbolBackslashBar => '⍀',
            MiscellaneousTechnical::AplFunctionalSymbolQuadSlash => '⍁',
            MiscellaneousTechnical::AplFunctionalSymbolQuadBackslash => '⍂',
            MiscellaneousTechnical::AplFunctionalSymbolQuadLessDashThan => '⍃',
            MiscellaneousTechnical::AplFunctionalSymbolQuadGreaterDashThan => '⍄',
            MiscellaneousTechnical::AplFunctionalSymbolLeftwardsVane => '⍅',
            MiscellaneousTechnical::AplFunctionalSymbolRightwardsVane => '⍆',
            MiscellaneousTechnical::AplFunctionalSymbolQuadLeftwardsArrow => '⍇',
            MiscellaneousTechnical::AplFunctionalSymbolQuadRightwardsArrow => '⍈',
            MiscellaneousTechnical::AplFunctionalSymbolCircleBackslash => '⍉',
            MiscellaneousTechnical::AplFunctionalSymbolDownTackUnderbar => '⍊',
            MiscellaneousTechnical::AplFunctionalSymbolDeltaStile => '⍋',
            MiscellaneousTechnical::AplFunctionalSymbolQuadDownCaret => '⍌',
            MiscellaneousTechnical::AplFunctionalSymbolQuadDelta => '⍍',
            MiscellaneousTechnical::AplFunctionalSymbolDownTackJot => '⍎',
            MiscellaneousTechnical::AplFunctionalSymbolUpwardsVane => '⍏',
            MiscellaneousTechnical::AplFunctionalSymbolQuadUpwardsArrow => '⍐',
            MiscellaneousTechnical::AplFunctionalSymbolUpTackOverbar => '⍑',
            MiscellaneousTechnical::AplFunctionalSymbolDelStile => '⍒',
            MiscellaneousTechnical::AplFunctionalSymbolQuadUpCaret => '⍓',
            MiscellaneousTechnical::AplFunctionalSymbolQuadDel => '⍔',
            MiscellaneousTechnical::AplFunctionalSymbolUpTackJot => '⍕',
            MiscellaneousTechnical::AplFunctionalSymbolDownwardsVane => '⍖',
            MiscellaneousTechnical::AplFunctionalSymbolQuadDownwardsArrow => '⍗',
            MiscellaneousTechnical::AplFunctionalSymbolQuoteUnderbar => '⍘',
            MiscellaneousTechnical::AplFunctionalSymbolDeltaUnderbar => '⍙',
            MiscellaneousTechnical::AplFunctionalSymbolDiamondUnderbar => '⍚',
            MiscellaneousTechnical::AplFunctionalSymbolJotUnderbar => '⍛',
            MiscellaneousTechnical::AplFunctionalSymbolCircleUnderbar => '⍜',
            MiscellaneousTechnical::AplFunctionalSymbolUpShoeJot => '⍝',
            MiscellaneousTechnical::AplFunctionalSymbolQuoteQuad => '⍞',
            MiscellaneousTechnical::AplFunctionalSymbolCircleStar => '⍟',
            MiscellaneousTechnical::AplFunctionalSymbolQuadColon => '⍠',
            MiscellaneousTechnical::AplFunctionalSymbolUpTackDiaeresis => '⍡',
            MiscellaneousTechnical::AplFunctionalSymbolDelDiaeresis => '⍢',
            MiscellaneousTechnical::AplFunctionalSymbolStarDiaeresis => '⍣',
            MiscellaneousTechnical::AplFunctionalSymbolJotDiaeresis => '⍤',
            MiscellaneousTechnical::AplFunctionalSymbolCircleDiaeresis => '⍥',
            MiscellaneousTechnical::AplFunctionalSymbolDownShoeStile => '⍦',
            MiscellaneousTechnical::AplFunctionalSymbolLeftShoeStile => '⍧',
            MiscellaneousTechnical::AplFunctionalSymbolTildeDiaeresis => '⍨',
            MiscellaneousTechnical::AplFunctionalSymbolGreaterDashThanDiaeresis => '⍩',
            MiscellaneousTechnical::AplFunctionalSymbolCommaBar => '⍪',
            MiscellaneousTechnical::AplFunctionalSymbolDelTilde => '⍫',
            MiscellaneousTechnical::AplFunctionalSymbolZilde => '⍬',
            MiscellaneousTechnical::AplFunctionalSymbolStileTilde => '⍭',
            MiscellaneousTechnical::AplFunctionalSymbolSemicolonUnderbar => '⍮',
            MiscellaneousTechnical::AplFunctionalSymbolQuadNotEqual => '⍯',
            MiscellaneousTechnical::AplFunctionalSymbolQuadQuestion => '⍰',
            MiscellaneousTechnical::AplFunctionalSymbolDownCaretTilde => '⍱',
            MiscellaneousTechnical::AplFunctionalSymbolUpCaretTilde => '⍲',
            MiscellaneousTechnical::AplFunctionalSymbolIota => '⍳',
            MiscellaneousTechnical::AplFunctionalSymbolRho => '⍴',
            MiscellaneousTechnical::AplFunctionalSymbolOmega => '⍵',
            MiscellaneousTechnical::AplFunctionalSymbolAlphaUnderbar => '⍶',
            MiscellaneousTechnical::AplFunctionalSymbolEpsilonUnderbar => '⍷',
            MiscellaneousTechnical::AplFunctionalSymbolIotaUnderbar => '⍸',
            MiscellaneousTechnical::AplFunctionalSymbolOmegaUnderbar => '⍹',
            MiscellaneousTechnical::AplFunctionalSymbolAlpha => '⍺',
            MiscellaneousTechnical::NotCheckMark => '⍻',
            MiscellaneousTechnical::RightAngleWithDownwardsZigzagArrow => '⍼',
            MiscellaneousTechnical::ShoulderedOpenBox => '⍽',
            MiscellaneousTechnical::BellSymbol => '⍾',
            MiscellaneousTechnical::VerticalLineWithMiddleDot => '⍿',
            MiscellaneousTechnical::InsertionSymbol => '⎀',
            MiscellaneousTechnical::ContinuousUnderlineSymbol => '⎁',
            MiscellaneousTechnical::DiscontinuousUnderlineSymbol => '⎂',
            MiscellaneousTechnical::EmphasisSymbol => '⎃',
            MiscellaneousTechnical::CompositionSymbol => '⎄',
            MiscellaneousTechnical::WhiteSquareWithCentreVerticalLine => '⎅',
            MiscellaneousTechnical::EnterSymbol => '⎆',
            MiscellaneousTechnical::AlternativeKeySymbol => '⎇',
            MiscellaneousTechnical::HelmSymbol => '⎈',
            MiscellaneousTechnical::CircledHorizontalBarWithNotch => '⎉',
            MiscellaneousTechnical::CircledTriangleDown => '⎊',
            MiscellaneousTechnical::BrokenCircleWithNorthwestArrow => '⎋',
            MiscellaneousTechnical::UndoSymbol => '⎌',
            MiscellaneousTechnical::MonostableSymbol => '⎍',
            MiscellaneousTechnical::HysteresisSymbol => '⎎',
            MiscellaneousTechnical::OpenDashCircuitDashOutputHDashTypeSymbol => '⎏',
            MiscellaneousTechnical::OpenDashCircuitDashOutputLDashTypeSymbol => '⎐',
            MiscellaneousTechnical::PassiveDashPullDashDownDashOutputSymbol => '⎑',
            MiscellaneousTechnical::PassiveDashPullDashUpDashOutputSymbol => '⎒',
            MiscellaneousTechnical::DirectCurrentSymbolFormTwo => '⎓',
            MiscellaneousTechnical::SoftwareDashFunctionSymbol => '⎔',
            MiscellaneousTechnical::AplFunctionalSymbolQuad => '⎕',
            MiscellaneousTechnical::DecimalSeparatorKeySymbol => '⎖',
            MiscellaneousTechnical::PreviousPage => '⎗',
            MiscellaneousTechnical::NextPage => '⎘',
            MiscellaneousTechnical::PrintScreenSymbol => '⎙',
            MiscellaneousTechnical::ClearScreenSymbol => '⎚',
            MiscellaneousTechnical::LeftParenthesisUpperHook => '⎛',
            MiscellaneousTechnical::LeftParenthesisExtension => '⎜',
            MiscellaneousTechnical::LeftParenthesisLowerHook => '⎝',
            MiscellaneousTechnical::RightParenthesisUpperHook => '⎞',
            MiscellaneousTechnical::RightParenthesisExtension => '⎟',
            MiscellaneousTechnical::RightParenthesisLowerHook => '⎠',
            MiscellaneousTechnical::LeftSquareBracketUpperCorner => '⎡',
            MiscellaneousTechnical::LeftSquareBracketExtension => '⎢',
            MiscellaneousTechnical::LeftSquareBracketLowerCorner => '⎣',
            MiscellaneousTechnical::RightSquareBracketUpperCorner => '⎤',
            MiscellaneousTechnical::RightSquareBracketExtension => '⎥',
            MiscellaneousTechnical::RightSquareBracketLowerCorner => '⎦',
            MiscellaneousTechnical::LeftCurlyBracketUpperHook => '⎧',
            MiscellaneousTechnical::LeftCurlyBracketMiddlePiece => '⎨',
            MiscellaneousTechnical::LeftCurlyBracketLowerHook => '⎩',
            MiscellaneousTechnical::CurlyBracketExtension => '⎪',
            MiscellaneousTechnical::RightCurlyBracketUpperHook => '⎫',
            MiscellaneousTechnical::RightCurlyBracketMiddlePiece => '⎬',
            MiscellaneousTechnical::RightCurlyBracketLowerHook => '⎭',
            MiscellaneousTechnical::IntegralExtension => '⎮',
            MiscellaneousTechnical::HorizontalLineExtension => '⎯',
            MiscellaneousTechnical::UpperLeftOrLowerRightCurlyBracketSection => '⎰',
            MiscellaneousTechnical::UpperRightOrLowerLeftCurlyBracketSection => '⎱',
            MiscellaneousTechnical::SummationTop => '⎲',
            MiscellaneousTechnical::SummationBottom => '⎳',
            MiscellaneousTechnical::TopSquareBracket => '⎴',
            MiscellaneousTechnical::BottomSquareBracket => '⎵',
            MiscellaneousTechnical::BottomSquareBracketOverTopSquareBracket => '⎶',
            MiscellaneousTechnical::RadicalSymbolBottom => '⎷',
            MiscellaneousTechnical::LeftVerticalBoxLine => '⎸',
            MiscellaneousTechnical::RightVerticalBoxLine => '⎹',
            MiscellaneousTechnical::HorizontalScanLineDash1 => '⎺',
            MiscellaneousTechnical::HorizontalScanLineDash3 => '⎻',
            MiscellaneousTechnical::HorizontalScanLineDash7 => '⎼',
            MiscellaneousTechnical::HorizontalScanLineDash9 => '⎽',
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndTopRight => '⎾',
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndBottomRight => '⎿',
            MiscellaneousTechnical::DentistrySymbolLightVerticalWithCircle => '⏀',
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithCircle => '⏁',
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithCircle => '⏂',
            MiscellaneousTechnical::DentistrySymbolLightVerticalWithTriangle => '⏃',
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithTriangle => '⏄',
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithTriangle => '⏅',
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndWave => '⏆',
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithWave => '⏇',
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithWave => '⏈',
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontal => '⏉',
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontal => '⏊',
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndTopLeft => '⏋',
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndBottomLeft => '⏌',
            MiscellaneousTechnical::SquareFoot => '⏍',
            MiscellaneousTechnical::ReturnSymbol => '⏎',
            MiscellaneousTechnical::EjectSymbol => '⏏',
            MiscellaneousTechnical::VerticalLineExtension => '⏐',
            MiscellaneousTechnical::MetricalBreve => '⏑',
            MiscellaneousTechnical::MetricalLongOverShort => '⏒',
            MiscellaneousTechnical::MetricalShortOverLong => '⏓',
            MiscellaneousTechnical::MetricalLongOverTwoShorts => '⏔',
            MiscellaneousTechnical::MetricalTwoShortsOverLong => '⏕',
            MiscellaneousTechnical::MetricalTwoShortsJoined => '⏖',
            MiscellaneousTechnical::MetricalTriseme => '⏗',
            MiscellaneousTechnical::MetricalTetraseme => '⏘',
            MiscellaneousTechnical::MetricalPentaseme => '⏙',
            MiscellaneousTechnical::EarthGround => '⏚',
            MiscellaneousTechnical::Fuse => '⏛',
            MiscellaneousTechnical::TopParenthesis => '⏜',
            MiscellaneousTechnical::BottomParenthesis => '⏝',
            MiscellaneousTechnical::TopCurlyBracket => '⏞',
            MiscellaneousTechnical::BottomCurlyBracket => '⏟',
            MiscellaneousTechnical::TopTortoiseShellBracket => '⏠',
            MiscellaneousTechnical::BottomTortoiseShellBracket => '⏡',
            MiscellaneousTechnical::WhiteTrapezium => '⏢',
            MiscellaneousTechnical::BenzeneRingWithCircle => '⏣',
            MiscellaneousTechnical::Straightness => '⏤',
            MiscellaneousTechnical::Flatness => '⏥',
            MiscellaneousTechnical::AcCurrent => '⏦',
            MiscellaneousTechnical::ElectricalIntersection => '⏧',
            MiscellaneousTechnical::DecimalExponentSymbol => '⏨',
            MiscellaneousTechnical::BlackRightDashPointingDoubleTriangle => '⏩',
            MiscellaneousTechnical::BlackLeftDashPointingDoubleTriangle => '⏪',
            MiscellaneousTechnical::BlackUpDashPointingDoubleTriangle => '⏫',
            MiscellaneousTechnical::BlackDownDashPointingDoubleTriangle => '⏬',
            MiscellaneousTechnical::BlackRightDashPointingDoubleTriangleWithVerticalBar => '⏭',
            MiscellaneousTechnical::BlackLeftDashPointingDoubleTriangleWithVerticalBar => '⏮',
            MiscellaneousTechnical::BlackRightDashPointingTriangleWithDoubleVerticalBar => '⏯',
            MiscellaneousTechnical::AlarmClock => '⏰',
            MiscellaneousTechnical::Stopwatch => '⏱',
            MiscellaneousTechnical::TimerClock => '⏲',
            MiscellaneousTechnical::HourglassWithFlowingSand => '⏳',
            MiscellaneousTechnical::BlackMediumLeftDashPointingTriangle => '⏴',
            MiscellaneousTechnical::BlackMediumRightDashPointingTriangle => '⏵',
            MiscellaneousTechnical::BlackMediumUpDashPointingTriangle => '⏶',
            MiscellaneousTechnical::BlackMediumDownDashPointingTriangle => '⏷',
            MiscellaneousTechnical::DoubleVerticalBar => '⏸',
            MiscellaneousTechnical::BlackSquareForStop => '⏹',
            MiscellaneousTechnical::BlackCircleForRecord => '⏺',
            MiscellaneousTechnical::PowerSymbol => '⏻',
            MiscellaneousTechnical::PowerOnDashOffSymbol => '⏼',
            MiscellaneousTechnical::PowerOnSymbol => '⏽',
            MiscellaneousTechnical::PowerSleepSymbol => '⏾',
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousTechnical {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⌀' => Ok(MiscellaneousTechnical::DiameterSign),
            '⌁' => Ok(MiscellaneousTechnical::ElectricArrow),
            '⌂' => Ok(MiscellaneousTechnical::House),
            '⌃' => Ok(MiscellaneousTechnical::UpArrowhead),
            '⌄' => Ok(MiscellaneousTechnical::DownArrowhead),
            '⌅' => Ok(MiscellaneousTechnical::Projective),
            '⌆' => Ok(MiscellaneousTechnical::Perspective),
            '⌇' => Ok(MiscellaneousTechnical::WavyLine),
            '⌈' => Ok(MiscellaneousTechnical::LeftCeiling),
            '⌉' => Ok(MiscellaneousTechnical::RightCeiling),
            '⌊' => Ok(MiscellaneousTechnical::LeftFloor),
            '⌋' => Ok(MiscellaneousTechnical::RightFloor),
            '⌌' => Ok(MiscellaneousTechnical::BottomRightCrop),
            '⌍' => Ok(MiscellaneousTechnical::BottomLeftCrop),
            '⌎' => Ok(MiscellaneousTechnical::TopRightCrop),
            '⌏' => Ok(MiscellaneousTechnical::TopLeftCrop),
            '⌐' => Ok(MiscellaneousTechnical::ReversedNotSign),
            '⌑' => Ok(MiscellaneousTechnical::SquareLozenge),
            '⌒' => Ok(MiscellaneousTechnical::Arc),
            '⌓' => Ok(MiscellaneousTechnical::Segment),
            '⌔' => Ok(MiscellaneousTechnical::Sector),
            '⌕' => Ok(MiscellaneousTechnical::TelephoneRecorder),
            '⌖' => Ok(MiscellaneousTechnical::PositionIndicator),
            '⌗' => Ok(MiscellaneousTechnical::ViewdataSquare),
            '⌘' => Ok(MiscellaneousTechnical::PlaceOfInterestSign),
            '⌙' => Ok(MiscellaneousTechnical::TurnedNotSign),
            '⌚' => Ok(MiscellaneousTechnical::Watch),
            '⌛' => Ok(MiscellaneousTechnical::Hourglass),
            '⌜' => Ok(MiscellaneousTechnical::TopLeftCorner),
            '⌝' => Ok(MiscellaneousTechnical::TopRightCorner),
            '⌞' => Ok(MiscellaneousTechnical::BottomLeftCorner),
            '⌟' => Ok(MiscellaneousTechnical::BottomRightCorner),
            '⌠' => Ok(MiscellaneousTechnical::TopHalfIntegral),
            '⌡' => Ok(MiscellaneousTechnical::BottomHalfIntegral),
            '⌢' => Ok(MiscellaneousTechnical::Frown),
            '⌣' => Ok(MiscellaneousTechnical::Smile),
            '⌤' => Ok(MiscellaneousTechnical::UpArrowheadBetweenTwoHorizontalBars),
            '⌥' => Ok(MiscellaneousTechnical::OptionKey),
            '⌦' => Ok(MiscellaneousTechnical::EraseToTheRight),
            '⌧' => Ok(MiscellaneousTechnical::XInARectangleBox),
            '⌨' => Ok(MiscellaneousTechnical::Keyboard),
            '〈' => Ok(MiscellaneousTechnical::LeftDashPointingAngleBracket),
            '〉' => Ok(MiscellaneousTechnical::RightDashPointingAngleBracket),
            '⌫' => Ok(MiscellaneousTechnical::EraseToTheLeft),
            '⌬' => Ok(MiscellaneousTechnical::BenzeneRing),
            '⌭' => Ok(MiscellaneousTechnical::Cylindricity),
            '⌮' => Ok(MiscellaneousTechnical::AllAroundDashProfile),
            '⌯' => Ok(MiscellaneousTechnical::Symmetry),
            '⌰' => Ok(MiscellaneousTechnical::TotalRunout),
            '⌱' => Ok(MiscellaneousTechnical::DimensionOrigin),
            '⌲' => Ok(MiscellaneousTechnical::ConicalTaper),
            '⌳' => Ok(MiscellaneousTechnical::Slope),
            '⌴' => Ok(MiscellaneousTechnical::Counterbore),
            '⌵' => Ok(MiscellaneousTechnical::Countersink),
            '⌶' => Ok(MiscellaneousTechnical::AplFunctionalSymbolIDashBeam),
            '⌷' => Ok(MiscellaneousTechnical::AplFunctionalSymbolSquishQuad),
            '⌸' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadEqual),
            '⌹' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadDivide),
            '⌺' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadDiamond),
            '⌻' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadJot),
            '⌼' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadCircle),
            '⌽' => Ok(MiscellaneousTechnical::AplFunctionalSymbolCircleStile),
            '⌾' => Ok(MiscellaneousTechnical::AplFunctionalSymbolCircleJot),
            '⌿' => Ok(MiscellaneousTechnical::AplFunctionalSymbolSlashBar),
            '⍀' => Ok(MiscellaneousTechnical::AplFunctionalSymbolBackslashBar),
            '⍁' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadSlash),
            '⍂' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadBackslash),
            '⍃' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadLessDashThan),
            '⍄' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadGreaterDashThan),
            '⍅' => Ok(MiscellaneousTechnical::AplFunctionalSymbolLeftwardsVane),
            '⍆' => Ok(MiscellaneousTechnical::AplFunctionalSymbolRightwardsVane),
            '⍇' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadLeftwardsArrow),
            '⍈' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadRightwardsArrow),
            '⍉' => Ok(MiscellaneousTechnical::AplFunctionalSymbolCircleBackslash),
            '⍊' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDownTackUnderbar),
            '⍋' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDeltaStile),
            '⍌' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadDownCaret),
            '⍍' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadDelta),
            '⍎' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDownTackJot),
            '⍏' => Ok(MiscellaneousTechnical::AplFunctionalSymbolUpwardsVane),
            '⍐' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadUpwardsArrow),
            '⍑' => Ok(MiscellaneousTechnical::AplFunctionalSymbolUpTackOverbar),
            '⍒' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDelStile),
            '⍓' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadUpCaret),
            '⍔' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadDel),
            '⍕' => Ok(MiscellaneousTechnical::AplFunctionalSymbolUpTackJot),
            '⍖' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDownwardsVane),
            '⍗' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadDownwardsArrow),
            '⍘' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuoteUnderbar),
            '⍙' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDeltaUnderbar),
            '⍚' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDiamondUnderbar),
            '⍛' => Ok(MiscellaneousTechnical::AplFunctionalSymbolJotUnderbar),
            '⍜' => Ok(MiscellaneousTechnical::AplFunctionalSymbolCircleUnderbar),
            '⍝' => Ok(MiscellaneousTechnical::AplFunctionalSymbolUpShoeJot),
            '⍞' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuoteQuad),
            '⍟' => Ok(MiscellaneousTechnical::AplFunctionalSymbolCircleStar),
            '⍠' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadColon),
            '⍡' => Ok(MiscellaneousTechnical::AplFunctionalSymbolUpTackDiaeresis),
            '⍢' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDelDiaeresis),
            '⍣' => Ok(MiscellaneousTechnical::AplFunctionalSymbolStarDiaeresis),
            '⍤' => Ok(MiscellaneousTechnical::AplFunctionalSymbolJotDiaeresis),
            '⍥' => Ok(MiscellaneousTechnical::AplFunctionalSymbolCircleDiaeresis),
            '⍦' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDownShoeStile),
            '⍧' => Ok(MiscellaneousTechnical::AplFunctionalSymbolLeftShoeStile),
            '⍨' => Ok(MiscellaneousTechnical::AplFunctionalSymbolTildeDiaeresis),
            '⍩' => Ok(MiscellaneousTechnical::AplFunctionalSymbolGreaterDashThanDiaeresis),
            '⍪' => Ok(MiscellaneousTechnical::AplFunctionalSymbolCommaBar),
            '⍫' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDelTilde),
            '⍬' => Ok(MiscellaneousTechnical::AplFunctionalSymbolZilde),
            '⍭' => Ok(MiscellaneousTechnical::AplFunctionalSymbolStileTilde),
            '⍮' => Ok(MiscellaneousTechnical::AplFunctionalSymbolSemicolonUnderbar),
            '⍯' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadNotEqual),
            '⍰' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuadQuestion),
            '⍱' => Ok(MiscellaneousTechnical::AplFunctionalSymbolDownCaretTilde),
            '⍲' => Ok(MiscellaneousTechnical::AplFunctionalSymbolUpCaretTilde),
            '⍳' => Ok(MiscellaneousTechnical::AplFunctionalSymbolIota),
            '⍴' => Ok(MiscellaneousTechnical::AplFunctionalSymbolRho),
            '⍵' => Ok(MiscellaneousTechnical::AplFunctionalSymbolOmega),
            '⍶' => Ok(MiscellaneousTechnical::AplFunctionalSymbolAlphaUnderbar),
            '⍷' => Ok(MiscellaneousTechnical::AplFunctionalSymbolEpsilonUnderbar),
            '⍸' => Ok(MiscellaneousTechnical::AplFunctionalSymbolIotaUnderbar),
            '⍹' => Ok(MiscellaneousTechnical::AplFunctionalSymbolOmegaUnderbar),
            '⍺' => Ok(MiscellaneousTechnical::AplFunctionalSymbolAlpha),
            '⍻' => Ok(MiscellaneousTechnical::NotCheckMark),
            '⍼' => Ok(MiscellaneousTechnical::RightAngleWithDownwardsZigzagArrow),
            '⍽' => Ok(MiscellaneousTechnical::ShoulderedOpenBox),
            '⍾' => Ok(MiscellaneousTechnical::BellSymbol),
            '⍿' => Ok(MiscellaneousTechnical::VerticalLineWithMiddleDot),
            '⎀' => Ok(MiscellaneousTechnical::InsertionSymbol),
            '⎁' => Ok(MiscellaneousTechnical::ContinuousUnderlineSymbol),
            '⎂' => Ok(MiscellaneousTechnical::DiscontinuousUnderlineSymbol),
            '⎃' => Ok(MiscellaneousTechnical::EmphasisSymbol),
            '⎄' => Ok(MiscellaneousTechnical::CompositionSymbol),
            '⎅' => Ok(MiscellaneousTechnical::WhiteSquareWithCentreVerticalLine),
            '⎆' => Ok(MiscellaneousTechnical::EnterSymbol),
            '⎇' => Ok(MiscellaneousTechnical::AlternativeKeySymbol),
            '⎈' => Ok(MiscellaneousTechnical::HelmSymbol),
            '⎉' => Ok(MiscellaneousTechnical::CircledHorizontalBarWithNotch),
            '⎊' => Ok(MiscellaneousTechnical::CircledTriangleDown),
            '⎋' => Ok(MiscellaneousTechnical::BrokenCircleWithNorthwestArrow),
            '⎌' => Ok(MiscellaneousTechnical::UndoSymbol),
            '⎍' => Ok(MiscellaneousTechnical::MonostableSymbol),
            '⎎' => Ok(MiscellaneousTechnical::HysteresisSymbol),
            '⎏' => Ok(MiscellaneousTechnical::OpenDashCircuitDashOutputHDashTypeSymbol),
            '⎐' => Ok(MiscellaneousTechnical::OpenDashCircuitDashOutputLDashTypeSymbol),
            '⎑' => Ok(MiscellaneousTechnical::PassiveDashPullDashDownDashOutputSymbol),
            '⎒' => Ok(MiscellaneousTechnical::PassiveDashPullDashUpDashOutputSymbol),
            '⎓' => Ok(MiscellaneousTechnical::DirectCurrentSymbolFormTwo),
            '⎔' => Ok(MiscellaneousTechnical::SoftwareDashFunctionSymbol),
            '⎕' => Ok(MiscellaneousTechnical::AplFunctionalSymbolQuad),
            '⎖' => Ok(MiscellaneousTechnical::DecimalSeparatorKeySymbol),
            '⎗' => Ok(MiscellaneousTechnical::PreviousPage),
            '⎘' => Ok(MiscellaneousTechnical::NextPage),
            '⎙' => Ok(MiscellaneousTechnical::PrintScreenSymbol),
            '⎚' => Ok(MiscellaneousTechnical::ClearScreenSymbol),
            '⎛' => Ok(MiscellaneousTechnical::LeftParenthesisUpperHook),
            '⎜' => Ok(MiscellaneousTechnical::LeftParenthesisExtension),
            '⎝' => Ok(MiscellaneousTechnical::LeftParenthesisLowerHook),
            '⎞' => Ok(MiscellaneousTechnical::RightParenthesisUpperHook),
            '⎟' => Ok(MiscellaneousTechnical::RightParenthesisExtension),
            '⎠' => Ok(MiscellaneousTechnical::RightParenthesisLowerHook),
            '⎡' => Ok(MiscellaneousTechnical::LeftSquareBracketUpperCorner),
            '⎢' => Ok(MiscellaneousTechnical::LeftSquareBracketExtension),
            '⎣' => Ok(MiscellaneousTechnical::LeftSquareBracketLowerCorner),
            '⎤' => Ok(MiscellaneousTechnical::RightSquareBracketUpperCorner),
            '⎥' => Ok(MiscellaneousTechnical::RightSquareBracketExtension),
            '⎦' => Ok(MiscellaneousTechnical::RightSquareBracketLowerCorner),
            '⎧' => Ok(MiscellaneousTechnical::LeftCurlyBracketUpperHook),
            '⎨' => Ok(MiscellaneousTechnical::LeftCurlyBracketMiddlePiece),
            '⎩' => Ok(MiscellaneousTechnical::LeftCurlyBracketLowerHook),
            '⎪' => Ok(MiscellaneousTechnical::CurlyBracketExtension),
            '⎫' => Ok(MiscellaneousTechnical::RightCurlyBracketUpperHook),
            '⎬' => Ok(MiscellaneousTechnical::RightCurlyBracketMiddlePiece),
            '⎭' => Ok(MiscellaneousTechnical::RightCurlyBracketLowerHook),
            '⎮' => Ok(MiscellaneousTechnical::IntegralExtension),
            '⎯' => Ok(MiscellaneousTechnical::HorizontalLineExtension),
            '⎰' => Ok(MiscellaneousTechnical::UpperLeftOrLowerRightCurlyBracketSection),
            '⎱' => Ok(MiscellaneousTechnical::UpperRightOrLowerLeftCurlyBracketSection),
            '⎲' => Ok(MiscellaneousTechnical::SummationTop),
            '⎳' => Ok(MiscellaneousTechnical::SummationBottom),
            '⎴' => Ok(MiscellaneousTechnical::TopSquareBracket),
            '⎵' => Ok(MiscellaneousTechnical::BottomSquareBracket),
            '⎶' => Ok(MiscellaneousTechnical::BottomSquareBracketOverTopSquareBracket),
            '⎷' => Ok(MiscellaneousTechnical::RadicalSymbolBottom),
            '⎸' => Ok(MiscellaneousTechnical::LeftVerticalBoxLine),
            '⎹' => Ok(MiscellaneousTechnical::RightVerticalBoxLine),
            '⎺' => Ok(MiscellaneousTechnical::HorizontalScanLineDash1),
            '⎻' => Ok(MiscellaneousTechnical::HorizontalScanLineDash3),
            '⎼' => Ok(MiscellaneousTechnical::HorizontalScanLineDash7),
            '⎽' => Ok(MiscellaneousTechnical::HorizontalScanLineDash9),
            '⎾' => Ok(MiscellaneousTechnical::DentistrySymbolLightVerticalAndTopRight),
            '⎿' => Ok(MiscellaneousTechnical::DentistrySymbolLightVerticalAndBottomRight),
            '⏀' => Ok(MiscellaneousTechnical::DentistrySymbolLightVerticalWithCircle),
            '⏁' => Ok(MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithCircle),
            '⏂' => Ok(MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithCircle),
            '⏃' => Ok(MiscellaneousTechnical::DentistrySymbolLightVerticalWithTriangle),
            '⏄' => Ok(MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithTriangle),
            '⏅' => Ok(MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithTriangle),
            '⏆' => Ok(MiscellaneousTechnical::DentistrySymbolLightVerticalAndWave),
            '⏇' => Ok(MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithWave),
            '⏈' => Ok(MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithWave),
            '⏉' => Ok(MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontal),
            '⏊' => Ok(MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontal),
            '⏋' => Ok(MiscellaneousTechnical::DentistrySymbolLightVerticalAndTopLeft),
            '⏌' => Ok(MiscellaneousTechnical::DentistrySymbolLightVerticalAndBottomLeft),
            '⏍' => Ok(MiscellaneousTechnical::SquareFoot),
            '⏎' => Ok(MiscellaneousTechnical::ReturnSymbol),
            '⏏' => Ok(MiscellaneousTechnical::EjectSymbol),
            '⏐' => Ok(MiscellaneousTechnical::VerticalLineExtension),
            '⏑' => Ok(MiscellaneousTechnical::MetricalBreve),
            '⏒' => Ok(MiscellaneousTechnical::MetricalLongOverShort),
            '⏓' => Ok(MiscellaneousTechnical::MetricalShortOverLong),
            '⏔' => Ok(MiscellaneousTechnical::MetricalLongOverTwoShorts),
            '⏕' => Ok(MiscellaneousTechnical::MetricalTwoShortsOverLong),
            '⏖' => Ok(MiscellaneousTechnical::MetricalTwoShortsJoined),
            '⏗' => Ok(MiscellaneousTechnical::MetricalTriseme),
            '⏘' => Ok(MiscellaneousTechnical::MetricalTetraseme),
            '⏙' => Ok(MiscellaneousTechnical::MetricalPentaseme),
            '⏚' => Ok(MiscellaneousTechnical::EarthGround),
            '⏛' => Ok(MiscellaneousTechnical::Fuse),
            '⏜' => Ok(MiscellaneousTechnical::TopParenthesis),
            '⏝' => Ok(MiscellaneousTechnical::BottomParenthesis),
            '⏞' => Ok(MiscellaneousTechnical::TopCurlyBracket),
            '⏟' => Ok(MiscellaneousTechnical::BottomCurlyBracket),
            '⏠' => Ok(MiscellaneousTechnical::TopTortoiseShellBracket),
            '⏡' => Ok(MiscellaneousTechnical::BottomTortoiseShellBracket),
            '⏢' => Ok(MiscellaneousTechnical::WhiteTrapezium),
            '⏣' => Ok(MiscellaneousTechnical::BenzeneRingWithCircle),
            '⏤' => Ok(MiscellaneousTechnical::Straightness),
            '⏥' => Ok(MiscellaneousTechnical::Flatness),
            '⏦' => Ok(MiscellaneousTechnical::AcCurrent),
            '⏧' => Ok(MiscellaneousTechnical::ElectricalIntersection),
            '⏨' => Ok(MiscellaneousTechnical::DecimalExponentSymbol),
            '⏩' => Ok(MiscellaneousTechnical::BlackRightDashPointingDoubleTriangle),
            '⏪' => Ok(MiscellaneousTechnical::BlackLeftDashPointingDoubleTriangle),
            '⏫' => Ok(MiscellaneousTechnical::BlackUpDashPointingDoubleTriangle),
            '⏬' => Ok(MiscellaneousTechnical::BlackDownDashPointingDoubleTriangle),
            '⏭' => Ok(MiscellaneousTechnical::BlackRightDashPointingDoubleTriangleWithVerticalBar),
            '⏮' => Ok(MiscellaneousTechnical::BlackLeftDashPointingDoubleTriangleWithVerticalBar),
            '⏯' => Ok(MiscellaneousTechnical::BlackRightDashPointingTriangleWithDoubleVerticalBar),
            '⏰' => Ok(MiscellaneousTechnical::AlarmClock),
            '⏱' => Ok(MiscellaneousTechnical::Stopwatch),
            '⏲' => Ok(MiscellaneousTechnical::TimerClock),
            '⏳' => Ok(MiscellaneousTechnical::HourglassWithFlowingSand),
            '⏴' => Ok(MiscellaneousTechnical::BlackMediumLeftDashPointingTriangle),
            '⏵' => Ok(MiscellaneousTechnical::BlackMediumRightDashPointingTriangle),
            '⏶' => Ok(MiscellaneousTechnical::BlackMediumUpDashPointingTriangle),
            '⏷' => Ok(MiscellaneousTechnical::BlackMediumDownDashPointingTriangle),
            '⏸' => Ok(MiscellaneousTechnical::DoubleVerticalBar),
            '⏹' => Ok(MiscellaneousTechnical::BlackSquareForStop),
            '⏺' => Ok(MiscellaneousTechnical::BlackCircleForRecord),
            '⏻' => Ok(MiscellaneousTechnical::PowerSymbol),
            '⏼' => Ok(MiscellaneousTechnical::PowerOnDashOffSymbol),
            '⏽' => Ok(MiscellaneousTechnical::PowerOnSymbol),
            '⏾' => Ok(MiscellaneousTechnical::PowerSleepSymbol),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MiscellaneousTechnical {
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

impl std::convert::TryFrom<u32> for MiscellaneousTechnical {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MiscellaneousTechnical {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MiscellaneousTechnical {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MiscellaneousTechnical::DiameterSign
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MiscellaneousTechnical::DiameterSign => "diameter sign",
            MiscellaneousTechnical::ElectricArrow => "electric arrow",
            MiscellaneousTechnical::House => "house",
            MiscellaneousTechnical::UpArrowhead => "up arrowhead",
            MiscellaneousTechnical::DownArrowhead => "down arrowhead",
            MiscellaneousTechnical::Projective => "projective",
            MiscellaneousTechnical::Perspective => "perspective",
            MiscellaneousTechnical::WavyLine => "wavy line",
            MiscellaneousTechnical::LeftCeiling => "left ceiling",
            MiscellaneousTechnical::RightCeiling => "right ceiling",
            MiscellaneousTechnical::LeftFloor => "left floor",
            MiscellaneousTechnical::RightFloor => "right floor",
            MiscellaneousTechnical::BottomRightCrop => "bottom right crop",
            MiscellaneousTechnical::BottomLeftCrop => "bottom left crop",
            MiscellaneousTechnical::TopRightCrop => "top right crop",
            MiscellaneousTechnical::TopLeftCrop => "top left crop",
            MiscellaneousTechnical::ReversedNotSign => "reversed not sign",
            MiscellaneousTechnical::SquareLozenge => "square lozenge",
            MiscellaneousTechnical::Arc => "arc",
            MiscellaneousTechnical::Segment => "segment",
            MiscellaneousTechnical::Sector => "sector",
            MiscellaneousTechnical::TelephoneRecorder => "telephone recorder",
            MiscellaneousTechnical::PositionIndicator => "position indicator",
            MiscellaneousTechnical::ViewdataSquare => "viewdata square",
            MiscellaneousTechnical::PlaceOfInterestSign => "place of interest sign",
            MiscellaneousTechnical::TurnedNotSign => "turned not sign",
            MiscellaneousTechnical::Watch => "watch",
            MiscellaneousTechnical::Hourglass => "hourglass",
            MiscellaneousTechnical::TopLeftCorner => "top left corner",
            MiscellaneousTechnical::TopRightCorner => "top right corner",
            MiscellaneousTechnical::BottomLeftCorner => "bottom left corner",
            MiscellaneousTechnical::BottomRightCorner => "bottom right corner",
            MiscellaneousTechnical::TopHalfIntegral => "top half integral",
            MiscellaneousTechnical::BottomHalfIntegral => "bottom half integral",
            MiscellaneousTechnical::Frown => "frown",
            MiscellaneousTechnical::Smile => "smile",
            MiscellaneousTechnical::UpArrowheadBetweenTwoHorizontalBars => "up arrowhead between two horizontal bars",
            MiscellaneousTechnical::OptionKey => "option key",
            MiscellaneousTechnical::EraseToTheRight => "erase to the right",
            MiscellaneousTechnical::XInARectangleBox => "x in a rectangle box",
            MiscellaneousTechnical::Keyboard => "keyboard",
            MiscellaneousTechnical::LeftDashPointingAngleBracket => "left-pointing angle bracket",
            MiscellaneousTechnical::RightDashPointingAngleBracket => "right-pointing angle bracket",
            MiscellaneousTechnical::EraseToTheLeft => "erase to the left",
            MiscellaneousTechnical::BenzeneRing => "benzene ring",
            MiscellaneousTechnical::Cylindricity => "cylindricity",
            MiscellaneousTechnical::AllAroundDashProfile => "all around-profile",
            MiscellaneousTechnical::Symmetry => "symmetry",
            MiscellaneousTechnical::TotalRunout => "total runout",
            MiscellaneousTechnical::DimensionOrigin => "dimension origin",
            MiscellaneousTechnical::ConicalTaper => "conical taper",
            MiscellaneousTechnical::Slope => "slope",
            MiscellaneousTechnical::Counterbore => "counterbore",
            MiscellaneousTechnical::Countersink => "countersink",
            MiscellaneousTechnical::AplFunctionalSymbolIDashBeam => "apl functional symbol i-beam",
            MiscellaneousTechnical::AplFunctionalSymbolSquishQuad => "apl functional symbol squish quad",
            MiscellaneousTechnical::AplFunctionalSymbolQuadEqual => "apl functional symbol quad equal",
            MiscellaneousTechnical::AplFunctionalSymbolQuadDivide => "apl functional symbol quad divide",
            MiscellaneousTechnical::AplFunctionalSymbolQuadDiamond => "apl functional symbol quad diamond",
            MiscellaneousTechnical::AplFunctionalSymbolQuadJot => "apl functional symbol quad jot",
            MiscellaneousTechnical::AplFunctionalSymbolQuadCircle => "apl functional symbol quad circle",
            MiscellaneousTechnical::AplFunctionalSymbolCircleStile => "apl functional symbol circle stile",
            MiscellaneousTechnical::AplFunctionalSymbolCircleJot => "apl functional symbol circle jot",
            MiscellaneousTechnical::AplFunctionalSymbolSlashBar => "apl functional symbol slash bar",
            MiscellaneousTechnical::AplFunctionalSymbolBackslashBar => "apl functional symbol backslash bar",
            MiscellaneousTechnical::AplFunctionalSymbolQuadSlash => "apl functional symbol quad slash",
            MiscellaneousTechnical::AplFunctionalSymbolQuadBackslash => "apl functional symbol quad backslash",
            MiscellaneousTechnical::AplFunctionalSymbolQuadLessDashThan => "apl functional symbol quad less-than",
            MiscellaneousTechnical::AplFunctionalSymbolQuadGreaterDashThan => "apl functional symbol quad greater-than",
            MiscellaneousTechnical::AplFunctionalSymbolLeftwardsVane => "apl functional symbol leftwards vane",
            MiscellaneousTechnical::AplFunctionalSymbolRightwardsVane => "apl functional symbol rightwards vane",
            MiscellaneousTechnical::AplFunctionalSymbolQuadLeftwardsArrow => "apl functional symbol quad leftwards arrow",
            MiscellaneousTechnical::AplFunctionalSymbolQuadRightwardsArrow => "apl functional symbol quad rightwards arrow",
            MiscellaneousTechnical::AplFunctionalSymbolCircleBackslash => "apl functional symbol circle backslash",
            MiscellaneousTechnical::AplFunctionalSymbolDownTackUnderbar => "apl functional symbol down tack underbar",
            MiscellaneousTechnical::AplFunctionalSymbolDeltaStile => "apl functional symbol delta stile",
            MiscellaneousTechnical::AplFunctionalSymbolQuadDownCaret => "apl functional symbol quad down caret",
            MiscellaneousTechnical::AplFunctionalSymbolQuadDelta => "apl functional symbol quad delta",
            MiscellaneousTechnical::AplFunctionalSymbolDownTackJot => "apl functional symbol down tack jot",
            MiscellaneousTechnical::AplFunctionalSymbolUpwardsVane => "apl functional symbol upwards vane",
            MiscellaneousTechnical::AplFunctionalSymbolQuadUpwardsArrow => "apl functional symbol quad upwards arrow",
            MiscellaneousTechnical::AplFunctionalSymbolUpTackOverbar => "apl functional symbol up tack overbar",
            MiscellaneousTechnical::AplFunctionalSymbolDelStile => "apl functional symbol del stile",
            MiscellaneousTechnical::AplFunctionalSymbolQuadUpCaret => "apl functional symbol quad up caret",
            MiscellaneousTechnical::AplFunctionalSymbolQuadDel => "apl functional symbol quad del",
            MiscellaneousTechnical::AplFunctionalSymbolUpTackJot => "apl functional symbol up tack jot",
            MiscellaneousTechnical::AplFunctionalSymbolDownwardsVane => "apl functional symbol downwards vane",
            MiscellaneousTechnical::AplFunctionalSymbolQuadDownwardsArrow => "apl functional symbol quad downwards arrow",
            MiscellaneousTechnical::AplFunctionalSymbolQuoteUnderbar => "apl functional symbol quote underbar",
            MiscellaneousTechnical::AplFunctionalSymbolDeltaUnderbar => "apl functional symbol delta underbar",
            MiscellaneousTechnical::AplFunctionalSymbolDiamondUnderbar => "apl functional symbol diamond underbar",
            MiscellaneousTechnical::AplFunctionalSymbolJotUnderbar => "apl functional symbol jot underbar",
            MiscellaneousTechnical::AplFunctionalSymbolCircleUnderbar => "apl functional symbol circle underbar",
            MiscellaneousTechnical::AplFunctionalSymbolUpShoeJot => "apl functional symbol up shoe jot",
            MiscellaneousTechnical::AplFunctionalSymbolQuoteQuad => "apl functional symbol quote quad",
            MiscellaneousTechnical::AplFunctionalSymbolCircleStar => "apl functional symbol circle star",
            MiscellaneousTechnical::AplFunctionalSymbolQuadColon => "apl functional symbol quad colon",
            MiscellaneousTechnical::AplFunctionalSymbolUpTackDiaeresis => "apl functional symbol up tack diaeresis",
            MiscellaneousTechnical::AplFunctionalSymbolDelDiaeresis => "apl functional symbol del diaeresis",
            MiscellaneousTechnical::AplFunctionalSymbolStarDiaeresis => "apl functional symbol star diaeresis",
            MiscellaneousTechnical::AplFunctionalSymbolJotDiaeresis => "apl functional symbol jot diaeresis",
            MiscellaneousTechnical::AplFunctionalSymbolCircleDiaeresis => "apl functional symbol circle diaeresis",
            MiscellaneousTechnical::AplFunctionalSymbolDownShoeStile => "apl functional symbol down shoe stile",
            MiscellaneousTechnical::AplFunctionalSymbolLeftShoeStile => "apl functional symbol left shoe stile",
            MiscellaneousTechnical::AplFunctionalSymbolTildeDiaeresis => "apl functional symbol tilde diaeresis",
            MiscellaneousTechnical::AplFunctionalSymbolGreaterDashThanDiaeresis => "apl functional symbol greater-than diaeresis",
            MiscellaneousTechnical::AplFunctionalSymbolCommaBar => "apl functional symbol comma bar",
            MiscellaneousTechnical::AplFunctionalSymbolDelTilde => "apl functional symbol del tilde",
            MiscellaneousTechnical::AplFunctionalSymbolZilde => "apl functional symbol zilde",
            MiscellaneousTechnical::AplFunctionalSymbolStileTilde => "apl functional symbol stile tilde",
            MiscellaneousTechnical::AplFunctionalSymbolSemicolonUnderbar => "apl functional symbol semicolon underbar",
            MiscellaneousTechnical::AplFunctionalSymbolQuadNotEqual => "apl functional symbol quad not equal",
            MiscellaneousTechnical::AplFunctionalSymbolQuadQuestion => "apl functional symbol quad question",
            MiscellaneousTechnical::AplFunctionalSymbolDownCaretTilde => "apl functional symbol down caret tilde",
            MiscellaneousTechnical::AplFunctionalSymbolUpCaretTilde => "apl functional symbol up caret tilde",
            MiscellaneousTechnical::AplFunctionalSymbolIota => "apl functional symbol iota",
            MiscellaneousTechnical::AplFunctionalSymbolRho => "apl functional symbol rho",
            MiscellaneousTechnical::AplFunctionalSymbolOmega => "apl functional symbol omega",
            MiscellaneousTechnical::AplFunctionalSymbolAlphaUnderbar => "apl functional symbol alpha underbar",
            MiscellaneousTechnical::AplFunctionalSymbolEpsilonUnderbar => "apl functional symbol epsilon underbar",
            MiscellaneousTechnical::AplFunctionalSymbolIotaUnderbar => "apl functional symbol iota underbar",
            MiscellaneousTechnical::AplFunctionalSymbolOmegaUnderbar => "apl functional symbol omega underbar",
            MiscellaneousTechnical::AplFunctionalSymbolAlpha => "apl functional symbol alpha",
            MiscellaneousTechnical::NotCheckMark => "not check mark",
            MiscellaneousTechnical::RightAngleWithDownwardsZigzagArrow => "right angle with downwards zigzag arrow",
            MiscellaneousTechnical::ShoulderedOpenBox => "shouldered open box",
            MiscellaneousTechnical::BellSymbol => "bell symbol",
            MiscellaneousTechnical::VerticalLineWithMiddleDot => "vertical line with middle dot",
            MiscellaneousTechnical::InsertionSymbol => "insertion symbol",
            MiscellaneousTechnical::ContinuousUnderlineSymbol => "continuous underline symbol",
            MiscellaneousTechnical::DiscontinuousUnderlineSymbol => "discontinuous underline symbol",
            MiscellaneousTechnical::EmphasisSymbol => "emphasis symbol",
            MiscellaneousTechnical::CompositionSymbol => "composition symbol",
            MiscellaneousTechnical::WhiteSquareWithCentreVerticalLine => "white square with centre vertical line",
            MiscellaneousTechnical::EnterSymbol => "enter symbol",
            MiscellaneousTechnical::AlternativeKeySymbol => "alternative key symbol",
            MiscellaneousTechnical::HelmSymbol => "helm symbol",
            MiscellaneousTechnical::CircledHorizontalBarWithNotch => "circled horizontal bar with notch",
            MiscellaneousTechnical::CircledTriangleDown => "circled triangle down",
            MiscellaneousTechnical::BrokenCircleWithNorthwestArrow => "broken circle with northwest arrow",
            MiscellaneousTechnical::UndoSymbol => "undo symbol",
            MiscellaneousTechnical::MonostableSymbol => "monostable symbol",
            MiscellaneousTechnical::HysteresisSymbol => "hysteresis symbol",
            MiscellaneousTechnical::OpenDashCircuitDashOutputHDashTypeSymbol => "open-circuit-output h-type symbol",
            MiscellaneousTechnical::OpenDashCircuitDashOutputLDashTypeSymbol => "open-circuit-output l-type symbol",
            MiscellaneousTechnical::PassiveDashPullDashDownDashOutputSymbol => "passive-pull-down-output symbol",
            MiscellaneousTechnical::PassiveDashPullDashUpDashOutputSymbol => "passive-pull-up-output symbol",
            MiscellaneousTechnical::DirectCurrentSymbolFormTwo => "direct current symbol form two",
            MiscellaneousTechnical::SoftwareDashFunctionSymbol => "software-function symbol",
            MiscellaneousTechnical::AplFunctionalSymbolQuad => "apl functional symbol quad",
            MiscellaneousTechnical::DecimalSeparatorKeySymbol => "decimal separator key symbol",
            MiscellaneousTechnical::PreviousPage => "previous page",
            MiscellaneousTechnical::NextPage => "next page",
            MiscellaneousTechnical::PrintScreenSymbol => "print screen symbol",
            MiscellaneousTechnical::ClearScreenSymbol => "clear screen symbol",
            MiscellaneousTechnical::LeftParenthesisUpperHook => "left parenthesis upper hook",
            MiscellaneousTechnical::LeftParenthesisExtension => "left parenthesis extension",
            MiscellaneousTechnical::LeftParenthesisLowerHook => "left parenthesis lower hook",
            MiscellaneousTechnical::RightParenthesisUpperHook => "right parenthesis upper hook",
            MiscellaneousTechnical::RightParenthesisExtension => "right parenthesis extension",
            MiscellaneousTechnical::RightParenthesisLowerHook => "right parenthesis lower hook",
            MiscellaneousTechnical::LeftSquareBracketUpperCorner => "left square bracket upper corner",
            MiscellaneousTechnical::LeftSquareBracketExtension => "left square bracket extension",
            MiscellaneousTechnical::LeftSquareBracketLowerCorner => "left square bracket lower corner",
            MiscellaneousTechnical::RightSquareBracketUpperCorner => "right square bracket upper corner",
            MiscellaneousTechnical::RightSquareBracketExtension => "right square bracket extension",
            MiscellaneousTechnical::RightSquareBracketLowerCorner => "right square bracket lower corner",
            MiscellaneousTechnical::LeftCurlyBracketUpperHook => "left curly bracket upper hook",
            MiscellaneousTechnical::LeftCurlyBracketMiddlePiece => "left curly bracket middle piece",
            MiscellaneousTechnical::LeftCurlyBracketLowerHook => "left curly bracket lower hook",
            MiscellaneousTechnical::CurlyBracketExtension => "curly bracket extension",
            MiscellaneousTechnical::RightCurlyBracketUpperHook => "right curly bracket upper hook",
            MiscellaneousTechnical::RightCurlyBracketMiddlePiece => "right curly bracket middle piece",
            MiscellaneousTechnical::RightCurlyBracketLowerHook => "right curly bracket lower hook",
            MiscellaneousTechnical::IntegralExtension => "integral extension",
            MiscellaneousTechnical::HorizontalLineExtension => "horizontal line extension",
            MiscellaneousTechnical::UpperLeftOrLowerRightCurlyBracketSection => "upper left or lower right curly bracket section",
            MiscellaneousTechnical::UpperRightOrLowerLeftCurlyBracketSection => "upper right or lower left curly bracket section",
            MiscellaneousTechnical::SummationTop => "summation top",
            MiscellaneousTechnical::SummationBottom => "summation bottom",
            MiscellaneousTechnical::TopSquareBracket => "top square bracket",
            MiscellaneousTechnical::BottomSquareBracket => "bottom square bracket",
            MiscellaneousTechnical::BottomSquareBracketOverTopSquareBracket => "bottom square bracket over top square bracket",
            MiscellaneousTechnical::RadicalSymbolBottom => "radical symbol bottom",
            MiscellaneousTechnical::LeftVerticalBoxLine => "left vertical box line",
            MiscellaneousTechnical::RightVerticalBoxLine => "right vertical box line",
            MiscellaneousTechnical::HorizontalScanLineDash1 => "horizontal scan line-1",
            MiscellaneousTechnical::HorizontalScanLineDash3 => "horizontal scan line-3",
            MiscellaneousTechnical::HorizontalScanLineDash7 => "horizontal scan line-7",
            MiscellaneousTechnical::HorizontalScanLineDash9 => "horizontal scan line-9",
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndTopRight => "dentistry symbol light vertical and top right",
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndBottomRight => "dentistry symbol light vertical and bottom right",
            MiscellaneousTechnical::DentistrySymbolLightVerticalWithCircle => "dentistry symbol light vertical with circle",
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithCircle => "dentistry symbol light down and horizontal with circle",
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithCircle => "dentistry symbol light up and horizontal with circle",
            MiscellaneousTechnical::DentistrySymbolLightVerticalWithTriangle => "dentistry symbol light vertical with triangle",
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithTriangle => "dentistry symbol light down and horizontal with triangle",
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithTriangle => "dentistry symbol light up and horizontal with triangle",
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndWave => "dentistry symbol light vertical and wave",
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontalWithWave => "dentistry symbol light down and horizontal with wave",
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontalWithWave => "dentistry symbol light up and horizontal with wave",
            MiscellaneousTechnical::DentistrySymbolLightDownAndHorizontal => "dentistry symbol light down and horizontal",
            MiscellaneousTechnical::DentistrySymbolLightUpAndHorizontal => "dentistry symbol light up and horizontal",
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndTopLeft => "dentistry symbol light vertical and top left",
            MiscellaneousTechnical::DentistrySymbolLightVerticalAndBottomLeft => "dentistry symbol light vertical and bottom left",
            MiscellaneousTechnical::SquareFoot => "square foot",
            MiscellaneousTechnical::ReturnSymbol => "return symbol",
            MiscellaneousTechnical::EjectSymbol => "eject symbol",
            MiscellaneousTechnical::VerticalLineExtension => "vertical line extension",
            MiscellaneousTechnical::MetricalBreve => "metrical breve",
            MiscellaneousTechnical::MetricalLongOverShort => "metrical long over short",
            MiscellaneousTechnical::MetricalShortOverLong => "metrical short over long",
            MiscellaneousTechnical::MetricalLongOverTwoShorts => "metrical long over two shorts",
            MiscellaneousTechnical::MetricalTwoShortsOverLong => "metrical two shorts over long",
            MiscellaneousTechnical::MetricalTwoShortsJoined => "metrical two shorts joined",
            MiscellaneousTechnical::MetricalTriseme => "metrical triseme",
            MiscellaneousTechnical::MetricalTetraseme => "metrical tetraseme",
            MiscellaneousTechnical::MetricalPentaseme => "metrical pentaseme",
            MiscellaneousTechnical::EarthGround => "earth ground",
            MiscellaneousTechnical::Fuse => "fuse",
            MiscellaneousTechnical::TopParenthesis => "top parenthesis",
            MiscellaneousTechnical::BottomParenthesis => "bottom parenthesis",
            MiscellaneousTechnical::TopCurlyBracket => "top curly bracket",
            MiscellaneousTechnical::BottomCurlyBracket => "bottom curly bracket",
            MiscellaneousTechnical::TopTortoiseShellBracket => "top tortoise shell bracket",
            MiscellaneousTechnical::BottomTortoiseShellBracket => "bottom tortoise shell bracket",
            MiscellaneousTechnical::WhiteTrapezium => "white trapezium",
            MiscellaneousTechnical::BenzeneRingWithCircle => "benzene ring with circle",
            MiscellaneousTechnical::Straightness => "straightness",
            MiscellaneousTechnical::Flatness => "flatness",
            MiscellaneousTechnical::AcCurrent => "ac current",
            MiscellaneousTechnical::ElectricalIntersection => "electrical intersection",
            MiscellaneousTechnical::DecimalExponentSymbol => "decimal exponent symbol",
            MiscellaneousTechnical::BlackRightDashPointingDoubleTriangle => "black right-pointing double triangle",
            MiscellaneousTechnical::BlackLeftDashPointingDoubleTriangle => "black left-pointing double triangle",
            MiscellaneousTechnical::BlackUpDashPointingDoubleTriangle => "black up-pointing double triangle",
            MiscellaneousTechnical::BlackDownDashPointingDoubleTriangle => "black down-pointing double triangle",
            MiscellaneousTechnical::BlackRightDashPointingDoubleTriangleWithVerticalBar => "black right-pointing double triangle with vertical bar",
            MiscellaneousTechnical::BlackLeftDashPointingDoubleTriangleWithVerticalBar => "black left-pointing double triangle with vertical bar",
            MiscellaneousTechnical::BlackRightDashPointingTriangleWithDoubleVerticalBar => "black right-pointing triangle with double vertical bar",
            MiscellaneousTechnical::AlarmClock => "alarm clock",
            MiscellaneousTechnical::Stopwatch => "stopwatch",
            MiscellaneousTechnical::TimerClock => "timer clock",
            MiscellaneousTechnical::HourglassWithFlowingSand => "hourglass with flowing sand",
            MiscellaneousTechnical::BlackMediumLeftDashPointingTriangle => "black medium left-pointing triangle",
            MiscellaneousTechnical::BlackMediumRightDashPointingTriangle => "black medium right-pointing triangle",
            MiscellaneousTechnical::BlackMediumUpDashPointingTriangle => "black medium up-pointing triangle",
            MiscellaneousTechnical::BlackMediumDownDashPointingTriangle => "black medium down-pointing triangle",
            MiscellaneousTechnical::DoubleVerticalBar => "double vertical bar",
            MiscellaneousTechnical::BlackSquareForStop => "black square for stop",
            MiscellaneousTechnical::BlackCircleForRecord => "black circle for record",
            MiscellaneousTechnical::PowerSymbol => "power symbol",
            MiscellaneousTechnical::PowerOnDashOffSymbol => "power on-off symbol",
            MiscellaneousTechnical::PowerOnSymbol => "power on symbol",
            MiscellaneousTechnical::PowerSleepSymbol => "power sleep symbol",
        }
    }
}
