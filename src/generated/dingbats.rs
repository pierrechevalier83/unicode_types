
/// An enum to represent all characters in the Dingbats block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dingbats {
    /// \u{2700}: '✀'
    BlackSafetyScissors,
    /// \u{2701}: '✁'
    UpperBladeScissors,
    /// \u{2702}: '✂'
    BlackScissors,
    /// \u{2703}: '✃'
    LowerBladeScissors,
    /// \u{2704}: '✄'
    WhiteScissors,
    /// \u{2705}: '✅'
    WhiteHeavyCheckMark,
    /// \u{2706}: '✆'
    TelephoneLocationSign,
    /// \u{2707}: '✇'
    TapeDrive,
    /// \u{2708}: '✈'
    Airplane,
    /// \u{2709}: '✉'
    Envelope,
    /// \u{270a}: '✊'
    RaisedFist,
    /// \u{270b}: '✋'
    RaisedHand,
    /// \u{270c}: '✌'
    VictoryHand,
    /// \u{270d}: '✍'
    WritingHand,
    /// \u{270e}: '✎'
    LowerRightPencil,
    /// \u{270f}: '✏'
    Pencil,
    /// \u{2710}: '✐'
    UpperRightPencil,
    /// \u{2711}: '✑'
    WhiteNib,
    /// \u{2712}: '✒'
    BlackNib,
    /// \u{2713}: '✓'
    CheckMark,
    /// \u{2714}: '✔'
    HeavyCheckMark,
    /// \u{2715}: '✕'
    MultiplicationX,
    /// \u{2716}: '✖'
    HeavyMultiplicationX,
    /// \u{2717}: '✗'
    BallotX,
    /// \u{2718}: '✘'
    HeavyBallotX,
    /// \u{2719}: '✙'
    OutlinedGreekCross,
    /// \u{271a}: '✚'
    HeavyGreekCross,
    /// \u{271b}: '✛'
    OpenCentreCross,
    /// \u{271c}: '✜'
    HeavyOpenCentreCross,
    /// \u{271d}: '✝'
    LatinCross,
    /// \u{271e}: '✞'
    ShadowedWhiteLatinCross,
    /// \u{271f}: '✟'
    OutlinedLatinCross,
    /// \u{2720}: '✠'
    MalteseCross,
    /// \u{2721}: '✡'
    StarOfDavid,
    /// \u{2722}: '✢'
    FourTeardropDashSpokedAsterisk,
    /// \u{2723}: '✣'
    FourBalloonDashSpokedAsterisk,
    /// \u{2724}: '✤'
    HeavyFourBalloonDashSpokedAsterisk,
    /// \u{2725}: '✥'
    FourClubDashSpokedAsterisk,
    /// \u{2726}: '✦'
    BlackFourPointedStar,
    /// \u{2727}: '✧'
    WhiteFourPointedStar,
    /// \u{2728}: '✨'
    Sparkles,
    /// \u{2729}: '✩'
    StressOutlinedWhiteStar,
    /// \u{272a}: '✪'
    CircledWhiteStar,
    /// \u{272b}: '✫'
    OpenCentreBlackStar,
    /// \u{272c}: '✬'
    BlackCentreWhiteStar,
    /// \u{272d}: '✭'
    OutlinedBlackStar,
    /// \u{272e}: '✮'
    HeavyOutlinedBlackStar,
    /// \u{272f}: '✯'
    PinwheelStar,
    /// \u{2730}: '✰'
    ShadowedWhiteStar,
    /// \u{2731}: '✱'
    HeavyAsterisk,
    /// \u{2732}: '✲'
    OpenCentreAsterisk,
    /// \u{2733}: '✳'
    EightSpokedAsterisk,
    /// \u{2734}: '✴'
    EightPointedBlackStar,
    /// \u{2735}: '✵'
    EightPointedPinwheelStar,
    /// \u{2736}: '✶'
    SixPointedBlackStar,
    /// \u{2737}: '✷'
    EightPointedRectilinearBlackStar,
    /// \u{2738}: '✸'
    HeavyEightPointedRectilinearBlackStar,
    /// \u{2739}: '✹'
    TwelvePointedBlackStar,
    /// \u{273a}: '✺'
    SixteenPointedAsterisk,
    /// \u{273b}: '✻'
    TeardropDashSpokedAsterisk,
    /// \u{273c}: '✼'
    OpenCentreTeardropDashSpokedAsterisk,
    /// \u{273d}: '✽'
    HeavyTeardropDashSpokedAsterisk,
    /// \u{273e}: '✾'
    SixPetalledBlackAndWhiteFlorette,
    /// \u{273f}: '✿'
    BlackFlorette,
    /// \u{2740}: '❀'
    WhiteFlorette,
    /// \u{2741}: '❁'
    EightPetalledOutlinedBlackFlorette,
    /// \u{2742}: '❂'
    CircledOpenCentreEightPointedStar,
    /// \u{2743}: '❃'
    HeavyTeardropDashSpokedPinwheelAsterisk,
    /// \u{2744}: '❄'
    Snowflake,
    /// \u{2745}: '❅'
    TightTrifoliateSnowflake,
    /// \u{2746}: '❆'
    HeavyChevronSnowflake,
    /// \u{2747}: '❇'
    Sparkle,
    /// \u{2748}: '❈'
    HeavySparkle,
    /// \u{2749}: '❉'
    BalloonDashSpokedAsterisk,
    /// \u{274a}: '❊'
    EightTeardropDashSpokedPropellerAsterisk,
    /// \u{274b}: '❋'
    HeavyEightTeardropDashSpokedPropellerAsterisk,
    /// \u{274c}: '❌'
    CrossMark,
    /// \u{274d}: '❍'
    ShadowedWhiteCircle,
    /// \u{274e}: '❎'
    NegativeSquaredCrossMark,
    /// \u{274f}: '❏'
    LowerRightDropDashShadowedWhiteSquare,
    /// \u{2750}: '❐'
    UpperRightDropDashShadowedWhiteSquare,
    /// \u{2751}: '❑'
    LowerRightShadowedWhiteSquare,
    /// \u{2752}: '❒'
    UpperRightShadowedWhiteSquare,
    /// \u{2753}: '❓'
    BlackQuestionMarkOrnament,
    /// \u{2754}: '❔'
    WhiteQuestionMarkOrnament,
    /// \u{2755}: '❕'
    WhiteExclamationMarkOrnament,
    /// \u{2756}: '❖'
    BlackDiamondMinusWhiteX,
    /// \u{2757}: '❗'
    HeavyExclamationMarkSymbol,
    /// \u{2758}: '❘'
    LightVerticalBar,
    /// \u{2759}: '❙'
    MediumVerticalBar,
    /// \u{275a}: '❚'
    HeavyVerticalBar,
    /// \u{275b}: '❛'
    HeavySingleTurnedCommaQuotationMarkOrnament,
    /// \u{275c}: '❜'
    HeavySingleCommaQuotationMarkOrnament,
    /// \u{275d}: '❝'
    HeavyDoubleTurnedCommaQuotationMarkOrnament,
    /// \u{275e}: '❞'
    HeavyDoubleCommaQuotationMarkOrnament,
    /// \u{275f}: '❟'
    HeavyLowSingleCommaQuotationMarkOrnament,
    /// \u{2760}: '❠'
    HeavyLowDoubleCommaQuotationMarkOrnament,
    /// \u{2761}: '❡'
    CurvedStemParagraphSignOrnament,
    /// \u{2762}: '❢'
    HeavyExclamationMarkOrnament,
    /// \u{2763}: '❣'
    HeavyHeartExclamationMarkOrnament,
    /// \u{2764}: '❤'
    HeavyBlackHeart,
    /// \u{2765}: '❥'
    RotatedHeavyBlackHeartBullet,
    /// \u{2766}: '❦'
    FloralHeart,
    /// \u{2767}: '❧'
    RotatedFloralHeartBullet,
    /// \u{2768}: '❨'
    MediumLeftParenthesisOrnament,
    /// \u{2769}: '❩'
    MediumRightParenthesisOrnament,
    /// \u{276a}: '❪'
    MediumFlattenedLeftParenthesisOrnament,
    /// \u{276b}: '❫'
    MediumFlattenedRightParenthesisOrnament,
    /// \u{276c}: '❬'
    MediumLeftDashPointingAngleBracketOrnament,
    /// \u{276d}: '❭'
    MediumRightDashPointingAngleBracketOrnament,
    /// \u{276e}: '❮'
    HeavyLeftDashPointingAngleQuotationMarkOrnament,
    /// \u{276f}: '❯'
    HeavyRightDashPointingAngleQuotationMarkOrnament,
    /// \u{2770}: '❰'
    HeavyLeftDashPointingAngleBracketOrnament,
    /// \u{2771}: '❱'
    HeavyRightDashPointingAngleBracketOrnament,
    /// \u{2772}: '❲'
    LightLeftTortoiseShellBracketOrnament,
    /// \u{2773}: '❳'
    LightRightTortoiseShellBracketOrnament,
    /// \u{2774}: '❴'
    MediumLeftCurlyBracketOrnament,
    /// \u{2775}: '❵'
    MediumRightCurlyBracketOrnament,
    /// \u{2776}: '❶'
    DingbatNegativeCircledDigitOne,
    /// \u{2777}: '❷'
    DingbatNegativeCircledDigitTwo,
    /// \u{2778}: '❸'
    DingbatNegativeCircledDigitThree,
    /// \u{2779}: '❹'
    DingbatNegativeCircledDigitFour,
    /// \u{277a}: '❺'
    DingbatNegativeCircledDigitFive,
    /// \u{277b}: '❻'
    DingbatNegativeCircledDigitSix,
    /// \u{277c}: '❼'
    DingbatNegativeCircledDigitSeven,
    /// \u{277d}: '❽'
    DingbatNegativeCircledDigitEight,
    /// \u{277e}: '❾'
    DingbatNegativeCircledDigitNine,
    /// \u{277f}: '❿'
    DingbatNegativeCircledNumberTen,
    /// \u{2780}: '➀'
    DingbatCircledSansDashSerifDigitOne,
    /// \u{2781}: '➁'
    DingbatCircledSansDashSerifDigitTwo,
    /// \u{2782}: '➂'
    DingbatCircledSansDashSerifDigitThree,
    /// \u{2783}: '➃'
    DingbatCircledSansDashSerifDigitFour,
    /// \u{2784}: '➄'
    DingbatCircledSansDashSerifDigitFive,
    /// \u{2785}: '➅'
    DingbatCircledSansDashSerifDigitSix,
    /// \u{2786}: '➆'
    DingbatCircledSansDashSerifDigitSeven,
    /// \u{2787}: '➇'
    DingbatCircledSansDashSerifDigitEight,
    /// \u{2788}: '➈'
    DingbatCircledSansDashSerifDigitNine,
    /// \u{2789}: '➉'
    DingbatCircledSansDashSerifNumberTen,
    /// \u{278a}: '➊'
    DingbatNegativeCircledSansDashSerifDigitOne,
    /// \u{278b}: '➋'
    DingbatNegativeCircledSansDashSerifDigitTwo,
    /// \u{278c}: '➌'
    DingbatNegativeCircledSansDashSerifDigitThree,
    /// \u{278d}: '➍'
    DingbatNegativeCircledSansDashSerifDigitFour,
    /// \u{278e}: '➎'
    DingbatNegativeCircledSansDashSerifDigitFive,
    /// \u{278f}: '➏'
    DingbatNegativeCircledSansDashSerifDigitSix,
    /// \u{2790}: '➐'
    DingbatNegativeCircledSansDashSerifDigitSeven,
    /// \u{2791}: '➑'
    DingbatNegativeCircledSansDashSerifDigitEight,
    /// \u{2792}: '➒'
    DingbatNegativeCircledSansDashSerifDigitNine,
    /// \u{2793}: '➓'
    DingbatNegativeCircledSansDashSerifNumberTen,
    /// \u{2794}: '➔'
    HeavyWideDashHeadedRightwardsArrow,
    /// \u{2795}: '➕'
    HeavyPlusSign,
    /// \u{2796}: '➖'
    HeavyMinusSign,
    /// \u{2797}: '➗'
    HeavyDivisionSign,
    /// \u{2798}: '➘'
    HeavySouthEastArrow,
    /// \u{2799}: '➙'
    HeavyRightwardsArrow,
    /// \u{279a}: '➚'
    HeavyNorthEastArrow,
    /// \u{279b}: '➛'
    DraftingPointRightwardsArrow,
    /// \u{279c}: '➜'
    HeavyRoundDashTippedRightwardsArrow,
    /// \u{279d}: '➝'
    TriangleDashHeadedRightwardsArrow,
    /// \u{279e}: '➞'
    HeavyTriangleDashHeadedRightwardsArrow,
    /// \u{279f}: '➟'
    DashedTriangleDashHeadedRightwardsArrow,
    /// \u{27a0}: '➠'
    HeavyDashedTriangleDashHeadedRightwardsArrow,
    /// \u{27a1}: '➡'
    BlackRightwardsArrow,
    /// \u{27a2}: '➢'
    ThreeDashDTopDashLightedRightwardsArrowhead,
    /// \u{27a3}: '➣'
    ThreeDashDBottomDashLightedRightwardsArrowhead,
    /// \u{27a4}: '➤'
    BlackRightwardsArrowhead,
    /// \u{27a5}: '➥'
    HeavyBlackCurvedDownwardsAndRightwardsArrow,
    /// \u{27a6}: '➦'
    HeavyBlackCurvedUpwardsAndRightwardsArrow,
    /// \u{27a7}: '➧'
    SquatBlackRightwardsArrow,
    /// \u{27a8}: '➨'
    HeavyConcaveDashPointedBlackRightwardsArrow,
    /// \u{27a9}: '➩'
    RightDashShadedWhiteRightwardsArrow,
    /// \u{27aa}: '➪'
    LeftDashShadedWhiteRightwardsArrow,
    /// \u{27ab}: '➫'
    BackDashTiltedShadowedWhiteRightwardsArrow,
    /// \u{27ac}: '➬'
    FrontDashTiltedShadowedWhiteRightwardsArrow,
    /// \u{27ad}: '➭'
    HeavyLowerRightDashShadowedWhiteRightwardsArrow,
    /// \u{27ae}: '➮'
    HeavyUpperRightDashShadowedWhiteRightwardsArrow,
    /// \u{27af}: '➯'
    NotchedLowerRightDashShadowedWhiteRightwardsArrow,
    /// \u{27b0}: '➰'
    CurlyLoop,
    /// \u{27b1}: '➱'
    NotchedUpperRightDashShadowedWhiteRightwardsArrow,
    /// \u{27b2}: '➲'
    CircledHeavyWhiteRightwardsArrow,
    /// \u{27b3}: '➳'
    WhiteDashFeatheredRightwardsArrow,
    /// \u{27b4}: '➴'
    BlackDashFeatheredSouthEastArrow,
    /// \u{27b5}: '➵'
    BlackDashFeatheredRightwardsArrow,
    /// \u{27b6}: '➶'
    BlackDashFeatheredNorthEastArrow,
    /// \u{27b7}: '➷'
    HeavyBlackDashFeatheredSouthEastArrow,
    /// \u{27b8}: '➸'
    HeavyBlackDashFeatheredRightwardsArrow,
    /// \u{27b9}: '➹'
    HeavyBlackDashFeatheredNorthEastArrow,
    /// \u{27ba}: '➺'
    TeardropDashBarbedRightwardsArrow,
    /// \u{27bb}: '➻'
    HeavyTeardropDashShankedRightwardsArrow,
    /// \u{27bc}: '➼'
    WedgeDashTailedRightwardsArrow,
    /// \u{27bd}: '➽'
    HeavyWedgeDashTailedRightwardsArrow,
    /// \u{27be}: '➾'
    OpenDashOutlinedRightwardsArrow,
}

impl Into<char> for Dingbats {
    fn into(self) -> char {
        match self {
            Dingbats::BlackSafetyScissors => '✀',
            Dingbats::UpperBladeScissors => '✁',
            Dingbats::BlackScissors => '✂',
            Dingbats::LowerBladeScissors => '✃',
            Dingbats::WhiteScissors => '✄',
            Dingbats::WhiteHeavyCheckMark => '✅',
            Dingbats::TelephoneLocationSign => '✆',
            Dingbats::TapeDrive => '✇',
            Dingbats::Airplane => '✈',
            Dingbats::Envelope => '✉',
            Dingbats::RaisedFist => '✊',
            Dingbats::RaisedHand => '✋',
            Dingbats::VictoryHand => '✌',
            Dingbats::WritingHand => '✍',
            Dingbats::LowerRightPencil => '✎',
            Dingbats::Pencil => '✏',
            Dingbats::UpperRightPencil => '✐',
            Dingbats::WhiteNib => '✑',
            Dingbats::BlackNib => '✒',
            Dingbats::CheckMark => '✓',
            Dingbats::HeavyCheckMark => '✔',
            Dingbats::MultiplicationX => '✕',
            Dingbats::HeavyMultiplicationX => '✖',
            Dingbats::BallotX => '✗',
            Dingbats::HeavyBallotX => '✘',
            Dingbats::OutlinedGreekCross => '✙',
            Dingbats::HeavyGreekCross => '✚',
            Dingbats::OpenCentreCross => '✛',
            Dingbats::HeavyOpenCentreCross => '✜',
            Dingbats::LatinCross => '✝',
            Dingbats::ShadowedWhiteLatinCross => '✞',
            Dingbats::OutlinedLatinCross => '✟',
            Dingbats::MalteseCross => '✠',
            Dingbats::StarOfDavid => '✡',
            Dingbats::FourTeardropDashSpokedAsterisk => '✢',
            Dingbats::FourBalloonDashSpokedAsterisk => '✣',
            Dingbats::HeavyFourBalloonDashSpokedAsterisk => '✤',
            Dingbats::FourClubDashSpokedAsterisk => '✥',
            Dingbats::BlackFourPointedStar => '✦',
            Dingbats::WhiteFourPointedStar => '✧',
            Dingbats::Sparkles => '✨',
            Dingbats::StressOutlinedWhiteStar => '✩',
            Dingbats::CircledWhiteStar => '✪',
            Dingbats::OpenCentreBlackStar => '✫',
            Dingbats::BlackCentreWhiteStar => '✬',
            Dingbats::OutlinedBlackStar => '✭',
            Dingbats::HeavyOutlinedBlackStar => '✮',
            Dingbats::PinwheelStar => '✯',
            Dingbats::ShadowedWhiteStar => '✰',
            Dingbats::HeavyAsterisk => '✱',
            Dingbats::OpenCentreAsterisk => '✲',
            Dingbats::EightSpokedAsterisk => '✳',
            Dingbats::EightPointedBlackStar => '✴',
            Dingbats::EightPointedPinwheelStar => '✵',
            Dingbats::SixPointedBlackStar => '✶',
            Dingbats::EightPointedRectilinearBlackStar => '✷',
            Dingbats::HeavyEightPointedRectilinearBlackStar => '✸',
            Dingbats::TwelvePointedBlackStar => '✹',
            Dingbats::SixteenPointedAsterisk => '✺',
            Dingbats::TeardropDashSpokedAsterisk => '✻',
            Dingbats::OpenCentreTeardropDashSpokedAsterisk => '✼',
            Dingbats::HeavyTeardropDashSpokedAsterisk => '✽',
            Dingbats::SixPetalledBlackAndWhiteFlorette => '✾',
            Dingbats::BlackFlorette => '✿',
            Dingbats::WhiteFlorette => '❀',
            Dingbats::EightPetalledOutlinedBlackFlorette => '❁',
            Dingbats::CircledOpenCentreEightPointedStar => '❂',
            Dingbats::HeavyTeardropDashSpokedPinwheelAsterisk => '❃',
            Dingbats::Snowflake => '❄',
            Dingbats::TightTrifoliateSnowflake => '❅',
            Dingbats::HeavyChevronSnowflake => '❆',
            Dingbats::Sparkle => '❇',
            Dingbats::HeavySparkle => '❈',
            Dingbats::BalloonDashSpokedAsterisk => '❉',
            Dingbats::EightTeardropDashSpokedPropellerAsterisk => '❊',
            Dingbats::HeavyEightTeardropDashSpokedPropellerAsterisk => '❋',
            Dingbats::CrossMark => '❌',
            Dingbats::ShadowedWhiteCircle => '❍',
            Dingbats::NegativeSquaredCrossMark => '❎',
            Dingbats::LowerRightDropDashShadowedWhiteSquare => '❏',
            Dingbats::UpperRightDropDashShadowedWhiteSquare => '❐',
            Dingbats::LowerRightShadowedWhiteSquare => '❑',
            Dingbats::UpperRightShadowedWhiteSquare => '❒',
            Dingbats::BlackQuestionMarkOrnament => '❓',
            Dingbats::WhiteQuestionMarkOrnament => '❔',
            Dingbats::WhiteExclamationMarkOrnament => '❕',
            Dingbats::BlackDiamondMinusWhiteX => '❖',
            Dingbats::HeavyExclamationMarkSymbol => '❗',
            Dingbats::LightVerticalBar => '❘',
            Dingbats::MediumVerticalBar => '❙',
            Dingbats::HeavyVerticalBar => '❚',
            Dingbats::HeavySingleTurnedCommaQuotationMarkOrnament => '❛',
            Dingbats::HeavySingleCommaQuotationMarkOrnament => '❜',
            Dingbats::HeavyDoubleTurnedCommaQuotationMarkOrnament => '❝',
            Dingbats::HeavyDoubleCommaQuotationMarkOrnament => '❞',
            Dingbats::HeavyLowSingleCommaQuotationMarkOrnament => '❟',
            Dingbats::HeavyLowDoubleCommaQuotationMarkOrnament => '❠',
            Dingbats::CurvedStemParagraphSignOrnament => '❡',
            Dingbats::HeavyExclamationMarkOrnament => '❢',
            Dingbats::HeavyHeartExclamationMarkOrnament => '❣',
            Dingbats::HeavyBlackHeart => '❤',
            Dingbats::RotatedHeavyBlackHeartBullet => '❥',
            Dingbats::FloralHeart => '❦',
            Dingbats::RotatedFloralHeartBullet => '❧',
            Dingbats::MediumLeftParenthesisOrnament => '❨',
            Dingbats::MediumRightParenthesisOrnament => '❩',
            Dingbats::MediumFlattenedLeftParenthesisOrnament => '❪',
            Dingbats::MediumFlattenedRightParenthesisOrnament => '❫',
            Dingbats::MediumLeftDashPointingAngleBracketOrnament => '❬',
            Dingbats::MediumRightDashPointingAngleBracketOrnament => '❭',
            Dingbats::HeavyLeftDashPointingAngleQuotationMarkOrnament => '❮',
            Dingbats::HeavyRightDashPointingAngleQuotationMarkOrnament => '❯',
            Dingbats::HeavyLeftDashPointingAngleBracketOrnament => '❰',
            Dingbats::HeavyRightDashPointingAngleBracketOrnament => '❱',
            Dingbats::LightLeftTortoiseShellBracketOrnament => '❲',
            Dingbats::LightRightTortoiseShellBracketOrnament => '❳',
            Dingbats::MediumLeftCurlyBracketOrnament => '❴',
            Dingbats::MediumRightCurlyBracketOrnament => '❵',
            Dingbats::DingbatNegativeCircledDigitOne => '❶',
            Dingbats::DingbatNegativeCircledDigitTwo => '❷',
            Dingbats::DingbatNegativeCircledDigitThree => '❸',
            Dingbats::DingbatNegativeCircledDigitFour => '❹',
            Dingbats::DingbatNegativeCircledDigitFive => '❺',
            Dingbats::DingbatNegativeCircledDigitSix => '❻',
            Dingbats::DingbatNegativeCircledDigitSeven => '❼',
            Dingbats::DingbatNegativeCircledDigitEight => '❽',
            Dingbats::DingbatNegativeCircledDigitNine => '❾',
            Dingbats::DingbatNegativeCircledNumberTen => '❿',
            Dingbats::DingbatCircledSansDashSerifDigitOne => '➀',
            Dingbats::DingbatCircledSansDashSerifDigitTwo => '➁',
            Dingbats::DingbatCircledSansDashSerifDigitThree => '➂',
            Dingbats::DingbatCircledSansDashSerifDigitFour => '➃',
            Dingbats::DingbatCircledSansDashSerifDigitFive => '➄',
            Dingbats::DingbatCircledSansDashSerifDigitSix => '➅',
            Dingbats::DingbatCircledSansDashSerifDigitSeven => '➆',
            Dingbats::DingbatCircledSansDashSerifDigitEight => '➇',
            Dingbats::DingbatCircledSansDashSerifDigitNine => '➈',
            Dingbats::DingbatCircledSansDashSerifNumberTen => '➉',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitOne => '➊',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitTwo => '➋',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitThree => '➌',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitFour => '➍',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitFive => '➎',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitSix => '➏',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitSeven => '➐',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitEight => '➑',
            Dingbats::DingbatNegativeCircledSansDashSerifDigitNine => '➒',
            Dingbats::DingbatNegativeCircledSansDashSerifNumberTen => '➓',
            Dingbats::HeavyWideDashHeadedRightwardsArrow => '➔',
            Dingbats::HeavyPlusSign => '➕',
            Dingbats::HeavyMinusSign => '➖',
            Dingbats::HeavyDivisionSign => '➗',
            Dingbats::HeavySouthEastArrow => '➘',
            Dingbats::HeavyRightwardsArrow => '➙',
            Dingbats::HeavyNorthEastArrow => '➚',
            Dingbats::DraftingPointRightwardsArrow => '➛',
            Dingbats::HeavyRoundDashTippedRightwardsArrow => '➜',
            Dingbats::TriangleDashHeadedRightwardsArrow => '➝',
            Dingbats::HeavyTriangleDashHeadedRightwardsArrow => '➞',
            Dingbats::DashedTriangleDashHeadedRightwardsArrow => '➟',
            Dingbats::HeavyDashedTriangleDashHeadedRightwardsArrow => '➠',
            Dingbats::BlackRightwardsArrow => '➡',
            Dingbats::ThreeDashDTopDashLightedRightwardsArrowhead => '➢',
            Dingbats::ThreeDashDBottomDashLightedRightwardsArrowhead => '➣',
            Dingbats::BlackRightwardsArrowhead => '➤',
            Dingbats::HeavyBlackCurvedDownwardsAndRightwardsArrow => '➥',
            Dingbats::HeavyBlackCurvedUpwardsAndRightwardsArrow => '➦',
            Dingbats::SquatBlackRightwardsArrow => '➧',
            Dingbats::HeavyConcaveDashPointedBlackRightwardsArrow => '➨',
            Dingbats::RightDashShadedWhiteRightwardsArrow => '➩',
            Dingbats::LeftDashShadedWhiteRightwardsArrow => '➪',
            Dingbats::BackDashTiltedShadowedWhiteRightwardsArrow => '➫',
            Dingbats::FrontDashTiltedShadowedWhiteRightwardsArrow => '➬',
            Dingbats::HeavyLowerRightDashShadowedWhiteRightwardsArrow => '➭',
            Dingbats::HeavyUpperRightDashShadowedWhiteRightwardsArrow => '➮',
            Dingbats::NotchedLowerRightDashShadowedWhiteRightwardsArrow => '➯',
            Dingbats::CurlyLoop => '➰',
            Dingbats::NotchedUpperRightDashShadowedWhiteRightwardsArrow => '➱',
            Dingbats::CircledHeavyWhiteRightwardsArrow => '➲',
            Dingbats::WhiteDashFeatheredRightwardsArrow => '➳',
            Dingbats::BlackDashFeatheredSouthEastArrow => '➴',
            Dingbats::BlackDashFeatheredRightwardsArrow => '➵',
            Dingbats::BlackDashFeatheredNorthEastArrow => '➶',
            Dingbats::HeavyBlackDashFeatheredSouthEastArrow => '➷',
            Dingbats::HeavyBlackDashFeatheredRightwardsArrow => '➸',
            Dingbats::HeavyBlackDashFeatheredNorthEastArrow => '➹',
            Dingbats::TeardropDashBarbedRightwardsArrow => '➺',
            Dingbats::HeavyTeardropDashShankedRightwardsArrow => '➻',
            Dingbats::WedgeDashTailedRightwardsArrow => '➼',
            Dingbats::HeavyWedgeDashTailedRightwardsArrow => '➽',
            Dingbats::OpenDashOutlinedRightwardsArrow => '➾',
        }
    }
}

impl std::convert::TryFrom<char> for Dingbats {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '✀' => Ok(Dingbats::BlackSafetyScissors),
            '✁' => Ok(Dingbats::UpperBladeScissors),
            '✂' => Ok(Dingbats::BlackScissors),
            '✃' => Ok(Dingbats::LowerBladeScissors),
            '✄' => Ok(Dingbats::WhiteScissors),
            '✅' => Ok(Dingbats::WhiteHeavyCheckMark),
            '✆' => Ok(Dingbats::TelephoneLocationSign),
            '✇' => Ok(Dingbats::TapeDrive),
            '✈' => Ok(Dingbats::Airplane),
            '✉' => Ok(Dingbats::Envelope),
            '✊' => Ok(Dingbats::RaisedFist),
            '✋' => Ok(Dingbats::RaisedHand),
            '✌' => Ok(Dingbats::VictoryHand),
            '✍' => Ok(Dingbats::WritingHand),
            '✎' => Ok(Dingbats::LowerRightPencil),
            '✏' => Ok(Dingbats::Pencil),
            '✐' => Ok(Dingbats::UpperRightPencil),
            '✑' => Ok(Dingbats::WhiteNib),
            '✒' => Ok(Dingbats::BlackNib),
            '✓' => Ok(Dingbats::CheckMark),
            '✔' => Ok(Dingbats::HeavyCheckMark),
            '✕' => Ok(Dingbats::MultiplicationX),
            '✖' => Ok(Dingbats::HeavyMultiplicationX),
            '✗' => Ok(Dingbats::BallotX),
            '✘' => Ok(Dingbats::HeavyBallotX),
            '✙' => Ok(Dingbats::OutlinedGreekCross),
            '✚' => Ok(Dingbats::HeavyGreekCross),
            '✛' => Ok(Dingbats::OpenCentreCross),
            '✜' => Ok(Dingbats::HeavyOpenCentreCross),
            '✝' => Ok(Dingbats::LatinCross),
            '✞' => Ok(Dingbats::ShadowedWhiteLatinCross),
            '✟' => Ok(Dingbats::OutlinedLatinCross),
            '✠' => Ok(Dingbats::MalteseCross),
            '✡' => Ok(Dingbats::StarOfDavid),
            '✢' => Ok(Dingbats::FourTeardropDashSpokedAsterisk),
            '✣' => Ok(Dingbats::FourBalloonDashSpokedAsterisk),
            '✤' => Ok(Dingbats::HeavyFourBalloonDashSpokedAsterisk),
            '✥' => Ok(Dingbats::FourClubDashSpokedAsterisk),
            '✦' => Ok(Dingbats::BlackFourPointedStar),
            '✧' => Ok(Dingbats::WhiteFourPointedStar),
            '✨' => Ok(Dingbats::Sparkles),
            '✩' => Ok(Dingbats::StressOutlinedWhiteStar),
            '✪' => Ok(Dingbats::CircledWhiteStar),
            '✫' => Ok(Dingbats::OpenCentreBlackStar),
            '✬' => Ok(Dingbats::BlackCentreWhiteStar),
            '✭' => Ok(Dingbats::OutlinedBlackStar),
            '✮' => Ok(Dingbats::HeavyOutlinedBlackStar),
            '✯' => Ok(Dingbats::PinwheelStar),
            '✰' => Ok(Dingbats::ShadowedWhiteStar),
            '✱' => Ok(Dingbats::HeavyAsterisk),
            '✲' => Ok(Dingbats::OpenCentreAsterisk),
            '✳' => Ok(Dingbats::EightSpokedAsterisk),
            '✴' => Ok(Dingbats::EightPointedBlackStar),
            '✵' => Ok(Dingbats::EightPointedPinwheelStar),
            '✶' => Ok(Dingbats::SixPointedBlackStar),
            '✷' => Ok(Dingbats::EightPointedRectilinearBlackStar),
            '✸' => Ok(Dingbats::HeavyEightPointedRectilinearBlackStar),
            '✹' => Ok(Dingbats::TwelvePointedBlackStar),
            '✺' => Ok(Dingbats::SixteenPointedAsterisk),
            '✻' => Ok(Dingbats::TeardropDashSpokedAsterisk),
            '✼' => Ok(Dingbats::OpenCentreTeardropDashSpokedAsterisk),
            '✽' => Ok(Dingbats::HeavyTeardropDashSpokedAsterisk),
            '✾' => Ok(Dingbats::SixPetalledBlackAndWhiteFlorette),
            '✿' => Ok(Dingbats::BlackFlorette),
            '❀' => Ok(Dingbats::WhiteFlorette),
            '❁' => Ok(Dingbats::EightPetalledOutlinedBlackFlorette),
            '❂' => Ok(Dingbats::CircledOpenCentreEightPointedStar),
            '❃' => Ok(Dingbats::HeavyTeardropDashSpokedPinwheelAsterisk),
            '❄' => Ok(Dingbats::Snowflake),
            '❅' => Ok(Dingbats::TightTrifoliateSnowflake),
            '❆' => Ok(Dingbats::HeavyChevronSnowflake),
            '❇' => Ok(Dingbats::Sparkle),
            '❈' => Ok(Dingbats::HeavySparkle),
            '❉' => Ok(Dingbats::BalloonDashSpokedAsterisk),
            '❊' => Ok(Dingbats::EightTeardropDashSpokedPropellerAsterisk),
            '❋' => Ok(Dingbats::HeavyEightTeardropDashSpokedPropellerAsterisk),
            '❌' => Ok(Dingbats::CrossMark),
            '❍' => Ok(Dingbats::ShadowedWhiteCircle),
            '❎' => Ok(Dingbats::NegativeSquaredCrossMark),
            '❏' => Ok(Dingbats::LowerRightDropDashShadowedWhiteSquare),
            '❐' => Ok(Dingbats::UpperRightDropDashShadowedWhiteSquare),
            '❑' => Ok(Dingbats::LowerRightShadowedWhiteSquare),
            '❒' => Ok(Dingbats::UpperRightShadowedWhiteSquare),
            '❓' => Ok(Dingbats::BlackQuestionMarkOrnament),
            '❔' => Ok(Dingbats::WhiteQuestionMarkOrnament),
            '❕' => Ok(Dingbats::WhiteExclamationMarkOrnament),
            '❖' => Ok(Dingbats::BlackDiamondMinusWhiteX),
            '❗' => Ok(Dingbats::HeavyExclamationMarkSymbol),
            '❘' => Ok(Dingbats::LightVerticalBar),
            '❙' => Ok(Dingbats::MediumVerticalBar),
            '❚' => Ok(Dingbats::HeavyVerticalBar),
            '❛' => Ok(Dingbats::HeavySingleTurnedCommaQuotationMarkOrnament),
            '❜' => Ok(Dingbats::HeavySingleCommaQuotationMarkOrnament),
            '❝' => Ok(Dingbats::HeavyDoubleTurnedCommaQuotationMarkOrnament),
            '❞' => Ok(Dingbats::HeavyDoubleCommaQuotationMarkOrnament),
            '❟' => Ok(Dingbats::HeavyLowSingleCommaQuotationMarkOrnament),
            '❠' => Ok(Dingbats::HeavyLowDoubleCommaQuotationMarkOrnament),
            '❡' => Ok(Dingbats::CurvedStemParagraphSignOrnament),
            '❢' => Ok(Dingbats::HeavyExclamationMarkOrnament),
            '❣' => Ok(Dingbats::HeavyHeartExclamationMarkOrnament),
            '❤' => Ok(Dingbats::HeavyBlackHeart),
            '❥' => Ok(Dingbats::RotatedHeavyBlackHeartBullet),
            '❦' => Ok(Dingbats::FloralHeart),
            '❧' => Ok(Dingbats::RotatedFloralHeartBullet),
            '❨' => Ok(Dingbats::MediumLeftParenthesisOrnament),
            '❩' => Ok(Dingbats::MediumRightParenthesisOrnament),
            '❪' => Ok(Dingbats::MediumFlattenedLeftParenthesisOrnament),
            '❫' => Ok(Dingbats::MediumFlattenedRightParenthesisOrnament),
            '❬' => Ok(Dingbats::MediumLeftDashPointingAngleBracketOrnament),
            '❭' => Ok(Dingbats::MediumRightDashPointingAngleBracketOrnament),
            '❮' => Ok(Dingbats::HeavyLeftDashPointingAngleQuotationMarkOrnament),
            '❯' => Ok(Dingbats::HeavyRightDashPointingAngleQuotationMarkOrnament),
            '❰' => Ok(Dingbats::HeavyLeftDashPointingAngleBracketOrnament),
            '❱' => Ok(Dingbats::HeavyRightDashPointingAngleBracketOrnament),
            '❲' => Ok(Dingbats::LightLeftTortoiseShellBracketOrnament),
            '❳' => Ok(Dingbats::LightRightTortoiseShellBracketOrnament),
            '❴' => Ok(Dingbats::MediumLeftCurlyBracketOrnament),
            '❵' => Ok(Dingbats::MediumRightCurlyBracketOrnament),
            '❶' => Ok(Dingbats::DingbatNegativeCircledDigitOne),
            '❷' => Ok(Dingbats::DingbatNegativeCircledDigitTwo),
            '❸' => Ok(Dingbats::DingbatNegativeCircledDigitThree),
            '❹' => Ok(Dingbats::DingbatNegativeCircledDigitFour),
            '❺' => Ok(Dingbats::DingbatNegativeCircledDigitFive),
            '❻' => Ok(Dingbats::DingbatNegativeCircledDigitSix),
            '❼' => Ok(Dingbats::DingbatNegativeCircledDigitSeven),
            '❽' => Ok(Dingbats::DingbatNegativeCircledDigitEight),
            '❾' => Ok(Dingbats::DingbatNegativeCircledDigitNine),
            '❿' => Ok(Dingbats::DingbatNegativeCircledNumberTen),
            '➀' => Ok(Dingbats::DingbatCircledSansDashSerifDigitOne),
            '➁' => Ok(Dingbats::DingbatCircledSansDashSerifDigitTwo),
            '➂' => Ok(Dingbats::DingbatCircledSansDashSerifDigitThree),
            '➃' => Ok(Dingbats::DingbatCircledSansDashSerifDigitFour),
            '➄' => Ok(Dingbats::DingbatCircledSansDashSerifDigitFive),
            '➅' => Ok(Dingbats::DingbatCircledSansDashSerifDigitSix),
            '➆' => Ok(Dingbats::DingbatCircledSansDashSerifDigitSeven),
            '➇' => Ok(Dingbats::DingbatCircledSansDashSerifDigitEight),
            '➈' => Ok(Dingbats::DingbatCircledSansDashSerifDigitNine),
            '➉' => Ok(Dingbats::DingbatCircledSansDashSerifNumberTen),
            '➊' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitOne),
            '➋' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitTwo),
            '➌' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitThree),
            '➍' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitFour),
            '➎' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitFive),
            '➏' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitSix),
            '➐' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitSeven),
            '➑' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitEight),
            '➒' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitNine),
            '➓' => Ok(Dingbats::DingbatNegativeCircledSansDashSerifNumberTen),
            '➔' => Ok(Dingbats::HeavyWideDashHeadedRightwardsArrow),
            '➕' => Ok(Dingbats::HeavyPlusSign),
            '➖' => Ok(Dingbats::HeavyMinusSign),
            '➗' => Ok(Dingbats::HeavyDivisionSign),
            '➘' => Ok(Dingbats::HeavySouthEastArrow),
            '➙' => Ok(Dingbats::HeavyRightwardsArrow),
            '➚' => Ok(Dingbats::HeavyNorthEastArrow),
            '➛' => Ok(Dingbats::DraftingPointRightwardsArrow),
            '➜' => Ok(Dingbats::HeavyRoundDashTippedRightwardsArrow),
            '➝' => Ok(Dingbats::TriangleDashHeadedRightwardsArrow),
            '➞' => Ok(Dingbats::HeavyTriangleDashHeadedRightwardsArrow),
            '➟' => Ok(Dingbats::DashedTriangleDashHeadedRightwardsArrow),
            '➠' => Ok(Dingbats::HeavyDashedTriangleDashHeadedRightwardsArrow),
            '➡' => Ok(Dingbats::BlackRightwardsArrow),
            '➢' => Ok(Dingbats::ThreeDashDTopDashLightedRightwardsArrowhead),
            '➣' => Ok(Dingbats::ThreeDashDBottomDashLightedRightwardsArrowhead),
            '➤' => Ok(Dingbats::BlackRightwardsArrowhead),
            '➥' => Ok(Dingbats::HeavyBlackCurvedDownwardsAndRightwardsArrow),
            '➦' => Ok(Dingbats::HeavyBlackCurvedUpwardsAndRightwardsArrow),
            '➧' => Ok(Dingbats::SquatBlackRightwardsArrow),
            '➨' => Ok(Dingbats::HeavyConcaveDashPointedBlackRightwardsArrow),
            '➩' => Ok(Dingbats::RightDashShadedWhiteRightwardsArrow),
            '➪' => Ok(Dingbats::LeftDashShadedWhiteRightwardsArrow),
            '➫' => Ok(Dingbats::BackDashTiltedShadowedWhiteRightwardsArrow),
            '➬' => Ok(Dingbats::FrontDashTiltedShadowedWhiteRightwardsArrow),
            '➭' => Ok(Dingbats::HeavyLowerRightDashShadowedWhiteRightwardsArrow),
            '➮' => Ok(Dingbats::HeavyUpperRightDashShadowedWhiteRightwardsArrow),
            '➯' => Ok(Dingbats::NotchedLowerRightDashShadowedWhiteRightwardsArrow),
            '➰' => Ok(Dingbats::CurlyLoop),
            '➱' => Ok(Dingbats::NotchedUpperRightDashShadowedWhiteRightwardsArrow),
            '➲' => Ok(Dingbats::CircledHeavyWhiteRightwardsArrow),
            '➳' => Ok(Dingbats::WhiteDashFeatheredRightwardsArrow),
            '➴' => Ok(Dingbats::BlackDashFeatheredSouthEastArrow),
            '➵' => Ok(Dingbats::BlackDashFeatheredRightwardsArrow),
            '➶' => Ok(Dingbats::BlackDashFeatheredNorthEastArrow),
            '➷' => Ok(Dingbats::HeavyBlackDashFeatheredSouthEastArrow),
            '➸' => Ok(Dingbats::HeavyBlackDashFeatheredRightwardsArrow),
            '➹' => Ok(Dingbats::HeavyBlackDashFeatheredNorthEastArrow),
            '➺' => Ok(Dingbats::TeardropDashBarbedRightwardsArrow),
            '➻' => Ok(Dingbats::HeavyTeardropDashShankedRightwardsArrow),
            '➼' => Ok(Dingbats::WedgeDashTailedRightwardsArrow),
            '➽' => Ok(Dingbats::HeavyWedgeDashTailedRightwardsArrow),
            '➾' => Ok(Dingbats::OpenDashOutlinedRightwardsArrow),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Dingbats {
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

impl std::convert::TryFrom<u32> for Dingbats {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Dingbats {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Dingbats {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Dingbats::BlackSafetyScissors
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Dingbats{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
