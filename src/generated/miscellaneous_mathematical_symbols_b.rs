
/// An enum to represent all characters in the MiscellaneousMathematicalSymbolsB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MiscellaneousMathematicalSymbolsB {
    /// \u{2980}: '⦀'
    TripleVerticalBarDelimiter,
    /// \u{2981}: '⦁'
    ZNotationSpot,
    /// \u{2982}: '⦂'
    ZNotationTypeColon,
    /// \u{2983}: '⦃'
    LeftWhiteCurlyBracket,
    /// \u{2984}: '⦄'
    RightWhiteCurlyBracket,
    /// \u{2985}: '⦅'
    LeftWhiteParenthesis,
    /// \u{2986}: '⦆'
    RightWhiteParenthesis,
    /// \u{2987}: '⦇'
    ZNotationLeftImageBracket,
    /// \u{2988}: '⦈'
    ZNotationRightImageBracket,
    /// \u{2989}: '⦉'
    ZNotationLeftBindingBracket,
    /// \u{298a}: '⦊'
    ZNotationRightBindingBracket,
    /// \u{298b}: '⦋'
    LeftSquareBracketWithUnderbar,
    /// \u{298c}: '⦌'
    RightSquareBracketWithUnderbar,
    /// \u{298d}: '⦍'
    LeftSquareBracketWithTickInTopCorner,
    /// \u{298e}: '⦎'
    RightSquareBracketWithTickInBottomCorner,
    /// \u{298f}: '⦏'
    LeftSquareBracketWithTickInBottomCorner,
    /// \u{2990}: '⦐'
    RightSquareBracketWithTickInTopCorner,
    /// \u{2991}: '⦑'
    LeftAngleBracketWithDot,
    /// \u{2992}: '⦒'
    RightAngleBracketWithDot,
    /// \u{2993}: '⦓'
    LeftArcLessDashThanBracket,
    /// \u{2994}: '⦔'
    RightArcGreaterDashThanBracket,
    /// \u{2995}: '⦕'
    DoubleLeftArcGreaterDashThanBracket,
    /// \u{2996}: '⦖'
    DoubleRightArcLessDashThanBracket,
    /// \u{2997}: '⦗'
    LeftBlackTortoiseShellBracket,
    /// \u{2998}: '⦘'
    RightBlackTortoiseShellBracket,
    /// \u{2999}: '⦙'
    DottedFence,
    /// \u{299a}: '⦚'
    VerticalZigzagLine,
    /// \u{299b}: '⦛'
    MeasuredAngleOpeningLeft,
    /// \u{299c}: '⦜'
    RightAngleVariantWithSquare,
    /// \u{299d}: '⦝'
    MeasuredRightAngleWithDot,
    /// \u{299e}: '⦞'
    AngleWithSInside,
    /// \u{299f}: '⦟'
    AcuteAngle,
    /// \u{29a0}: '⦠'
    SphericalAngleOpeningLeft,
    /// \u{29a1}: '⦡'
    SphericalAngleOpeningUp,
    /// \u{29a2}: '⦢'
    TurnedAngle,
    /// \u{29a3}: '⦣'
    ReversedAngle,
    /// \u{29a4}: '⦤'
    AngleWithUnderbar,
    /// \u{29a5}: '⦥'
    ReversedAngleWithUnderbar,
    /// \u{29a6}: '⦦'
    ObliqueAngleOpeningUp,
    /// \u{29a7}: '⦧'
    ObliqueAngleOpeningDown,
    /// \u{29a8}: '⦨'
    MeasuredAngleWithOpenArmEndingInArrowPointingUpAndRight,
    /// \u{29a9}: '⦩'
    MeasuredAngleWithOpenArmEndingInArrowPointingUpAndLeft,
    /// \u{29aa}: '⦪'
    MeasuredAngleWithOpenArmEndingInArrowPointingDownAndRight,
    /// \u{29ab}: '⦫'
    MeasuredAngleWithOpenArmEndingInArrowPointingDownAndLeft,
    /// \u{29ac}: '⦬'
    MeasuredAngleWithOpenArmEndingInArrowPointingRightAndUp,
    /// \u{29ad}: '⦭'
    MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndUp,
    /// \u{29ae}: '⦮'
    MeasuredAngleWithOpenArmEndingInArrowPointingRightAndDown,
    /// \u{29af}: '⦯'
    MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndDown,
    /// \u{29b0}: '⦰'
    ReversedEmptySet,
    /// \u{29b1}: '⦱'
    EmptySetWithOverbar,
    /// \u{29b2}: '⦲'
    EmptySetWithSmallCircleAbove,
    /// \u{29b3}: '⦳'
    EmptySetWithRightArrowAbove,
    /// \u{29b4}: '⦴'
    EmptySetWithLeftArrowAbove,
    /// \u{29b5}: '⦵'
    CircleWithHorizontalBar,
    /// \u{29b6}: '⦶'
    CircledVerticalBar,
    /// \u{29b7}: '⦷'
    CircledParallel,
    /// \u{29b8}: '⦸'
    CircledReverseSolidus,
    /// \u{29b9}: '⦹'
    CircledPerpendicular,
    /// \u{29ba}: '⦺'
    CircleDividedByHorizontalBarAndTopHalfDividedByVerticalBar,
    /// \u{29bb}: '⦻'
    CircleWithSuperimposedX,
    /// \u{29bc}: '⦼'
    CircledAnticlockwiseDashRotatedDivisionSign,
    /// \u{29bd}: '⦽'
    UpArrowThroughCircle,
    /// \u{29be}: '⦾'
    CircledWhiteBullet,
    /// \u{29bf}: '⦿'
    CircledBullet,
    /// \u{29c0}: '⧀'
    CircledLessDashThan,
    /// \u{29c1}: '⧁'
    CircledGreaterDashThan,
    /// \u{29c2}: '⧂'
    CircleWithSmallCircleToTheRight,
    /// \u{29c3}: '⧃'
    CircleWithTwoHorizontalStrokesToTheRight,
    /// \u{29c4}: '⧄'
    SquaredRisingDiagonalSlash,
    /// \u{29c5}: '⧅'
    SquaredFallingDiagonalSlash,
    /// \u{29c6}: '⧆'
    SquaredAsterisk,
    /// \u{29c7}: '⧇'
    SquaredSmallCircle,
    /// \u{29c8}: '⧈'
    SquaredSquare,
    /// \u{29c9}: '⧉'
    TwoJoinedSquares,
    /// \u{29ca}: '⧊'
    TriangleWithDotAbove,
    /// \u{29cb}: '⧋'
    TriangleWithUnderbar,
    /// \u{29cc}: '⧌'
    SInTriangle,
    /// \u{29cd}: '⧍'
    TriangleWithSerifsAtBottom,
    /// \u{29ce}: '⧎'
    RightTriangleAboveLeftTriangle,
    /// \u{29cf}: '⧏'
    LeftTriangleBesideVerticalBar,
    /// \u{29d0}: '⧐'
    VerticalBarBesideRightTriangle,
    /// \u{29d1}: '⧑'
    BowtieWithLeftHalfBlack,
    /// \u{29d2}: '⧒'
    BowtieWithRightHalfBlack,
    /// \u{29d3}: '⧓'
    BlackBowtie,
    /// \u{29d4}: '⧔'
    TimesWithLeftHalfBlack,
    /// \u{29d5}: '⧕'
    TimesWithRightHalfBlack,
    /// \u{29d6}: '⧖'
    WhiteHourglass,
    /// \u{29d7}: '⧗'
    BlackHourglass,
    /// \u{29d8}: '⧘'
    LeftWigglyFence,
    /// \u{29d9}: '⧙'
    RightWigglyFence,
    /// \u{29da}: '⧚'
    LeftDoubleWigglyFence,
    /// \u{29db}: '⧛'
    RightDoubleWigglyFence,
    /// \u{29dc}: '⧜'
    IncompleteInfinity,
    /// \u{29dd}: '⧝'
    TieOverInfinity,
    /// \u{29de}: '⧞'
    InfinityNegatedWithVerticalBar,
    /// \u{29df}: '⧟'
    DoubleDashEndedMultimap,
    /// \u{29e0}: '⧠'
    SquareWithContouredOutline,
    /// \u{29e1}: '⧡'
    IncreasesAs,
    /// \u{29e2}: '⧢'
    ShuffleProduct,
    /// \u{29e3}: '⧣'
    EqualsSignAndSlantedParallel,
    /// \u{29e4}: '⧤'
    EqualsSignAndSlantedParallelWithTildeAbove,
    /// \u{29e5}: '⧥'
    IdenticalToAndSlantedParallel,
    /// \u{29e6}: '⧦'
    GleichStark,
    /// \u{29e7}: '⧧'
    Thermodynamic,
    /// \u{29e8}: '⧨'
    DownDashPointingTriangleWithLeftHalfBlack,
    /// \u{29e9}: '⧩'
    DownDashPointingTriangleWithRightHalfBlack,
    /// \u{29ea}: '⧪'
    BlackDiamondWithDownArrow,
    /// \u{29eb}: '⧫'
    BlackLozenge,
    /// \u{29ec}: '⧬'
    WhiteCircleWithDownArrow,
    /// \u{29ed}: '⧭'
    BlackCircleWithDownArrow,
    /// \u{29ee}: '⧮'
    ErrorDashBarredWhiteSquare,
    /// \u{29ef}: '⧯'
    ErrorDashBarredBlackSquare,
    /// \u{29f0}: '⧰'
    ErrorDashBarredWhiteDiamond,
    /// \u{29f1}: '⧱'
    ErrorDashBarredBlackDiamond,
    /// \u{29f2}: '⧲'
    ErrorDashBarredWhiteCircle,
    /// \u{29f3}: '⧳'
    ErrorDashBarredBlackCircle,
    /// \u{29f4}: '⧴'
    RuleDashDelayed,
    /// \u{29f5}: '⧵'
    ReverseSolidusOperator,
    /// \u{29f6}: '⧶'
    SolidusWithOverbar,
    /// \u{29f7}: '⧷'
    ReverseSolidusWithHorizontalStroke,
    /// \u{29f8}: '⧸'
    BigSolidus,
    /// \u{29f9}: '⧹'
    BigReverseSolidus,
    /// \u{29fa}: '⧺'
    DoublePlus,
    /// \u{29fb}: '⧻'
    TriplePlus,
    /// \u{29fc}: '⧼'
    LeftDashPointingCurvedAngleBracket,
    /// \u{29fd}: '⧽'
    RightDashPointingCurvedAngleBracket,
    /// \u{29fe}: '⧾'
    Tiny,
}

impl Into<char> for MiscellaneousMathematicalSymbolsB {
    fn into(self) -> char {
        match self {
            MiscellaneousMathematicalSymbolsB::TripleVerticalBarDelimiter => '⦀',
            MiscellaneousMathematicalSymbolsB::ZNotationSpot => '⦁',
            MiscellaneousMathematicalSymbolsB::ZNotationTypeColon => '⦂',
            MiscellaneousMathematicalSymbolsB::LeftWhiteCurlyBracket => '⦃',
            MiscellaneousMathematicalSymbolsB::RightWhiteCurlyBracket => '⦄',
            MiscellaneousMathematicalSymbolsB::LeftWhiteParenthesis => '⦅',
            MiscellaneousMathematicalSymbolsB::RightWhiteParenthesis => '⦆',
            MiscellaneousMathematicalSymbolsB::ZNotationLeftImageBracket => '⦇',
            MiscellaneousMathematicalSymbolsB::ZNotationRightImageBracket => '⦈',
            MiscellaneousMathematicalSymbolsB::ZNotationLeftBindingBracket => '⦉',
            MiscellaneousMathematicalSymbolsB::ZNotationRightBindingBracket => '⦊',
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithUnderbar => '⦋',
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithUnderbar => '⦌',
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInTopCorner => '⦍',
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInBottomCorner => '⦎',
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInBottomCorner => '⦏',
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInTopCorner => '⦐',
            MiscellaneousMathematicalSymbolsB::LeftAngleBracketWithDot => '⦑',
            MiscellaneousMathematicalSymbolsB::RightAngleBracketWithDot => '⦒',
            MiscellaneousMathematicalSymbolsB::LeftArcLessDashThanBracket => '⦓',
            MiscellaneousMathematicalSymbolsB::RightArcGreaterDashThanBracket => '⦔',
            MiscellaneousMathematicalSymbolsB::DoubleLeftArcGreaterDashThanBracket => '⦕',
            MiscellaneousMathematicalSymbolsB::DoubleRightArcLessDashThanBracket => '⦖',
            MiscellaneousMathematicalSymbolsB::LeftBlackTortoiseShellBracket => '⦗',
            MiscellaneousMathematicalSymbolsB::RightBlackTortoiseShellBracket => '⦘',
            MiscellaneousMathematicalSymbolsB::DottedFence => '⦙',
            MiscellaneousMathematicalSymbolsB::VerticalZigzagLine => '⦚',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleOpeningLeft => '⦛',
            MiscellaneousMathematicalSymbolsB::RightAngleVariantWithSquare => '⦜',
            MiscellaneousMathematicalSymbolsB::MeasuredRightAngleWithDot => '⦝',
            MiscellaneousMathematicalSymbolsB::AngleWithSInside => '⦞',
            MiscellaneousMathematicalSymbolsB::AcuteAngle => '⦟',
            MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningLeft => '⦠',
            MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningUp => '⦡',
            MiscellaneousMathematicalSymbolsB::TurnedAngle => '⦢',
            MiscellaneousMathematicalSymbolsB::ReversedAngle => '⦣',
            MiscellaneousMathematicalSymbolsB::AngleWithUnderbar => '⦤',
            MiscellaneousMathematicalSymbolsB::ReversedAngleWithUnderbar => '⦥',
            MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningUp => '⦦',
            MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningDown => '⦧',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndRight => '⦨',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndLeft => '⦩',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndRight => '⦪',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndLeft => '⦫',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndUp => '⦬',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndUp => '⦭',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndDown => '⦮',
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndDown => '⦯',
            MiscellaneousMathematicalSymbolsB::ReversedEmptySet => '⦰',
            MiscellaneousMathematicalSymbolsB::EmptySetWithOverbar => '⦱',
            MiscellaneousMathematicalSymbolsB::EmptySetWithSmallCircleAbove => '⦲',
            MiscellaneousMathematicalSymbolsB::EmptySetWithRightArrowAbove => '⦳',
            MiscellaneousMathematicalSymbolsB::EmptySetWithLeftArrowAbove => '⦴',
            MiscellaneousMathematicalSymbolsB::CircleWithHorizontalBar => '⦵',
            MiscellaneousMathematicalSymbolsB::CircledVerticalBar => '⦶',
            MiscellaneousMathematicalSymbolsB::CircledParallel => '⦷',
            MiscellaneousMathematicalSymbolsB::CircledReverseSolidus => '⦸',
            MiscellaneousMathematicalSymbolsB::CircledPerpendicular => '⦹',
            MiscellaneousMathematicalSymbolsB::CircleDividedByHorizontalBarAndTopHalfDividedByVerticalBar => '⦺',
            MiscellaneousMathematicalSymbolsB::CircleWithSuperimposedX => '⦻',
            MiscellaneousMathematicalSymbolsB::CircledAnticlockwiseDashRotatedDivisionSign => '⦼',
            MiscellaneousMathematicalSymbolsB::UpArrowThroughCircle => '⦽',
            MiscellaneousMathematicalSymbolsB::CircledWhiteBullet => '⦾',
            MiscellaneousMathematicalSymbolsB::CircledBullet => '⦿',
            MiscellaneousMathematicalSymbolsB::CircledLessDashThan => '⧀',
            MiscellaneousMathematicalSymbolsB::CircledGreaterDashThan => '⧁',
            MiscellaneousMathematicalSymbolsB::CircleWithSmallCircleToTheRight => '⧂',
            MiscellaneousMathematicalSymbolsB::CircleWithTwoHorizontalStrokesToTheRight => '⧃',
            MiscellaneousMathematicalSymbolsB::SquaredRisingDiagonalSlash => '⧄',
            MiscellaneousMathematicalSymbolsB::SquaredFallingDiagonalSlash => '⧅',
            MiscellaneousMathematicalSymbolsB::SquaredAsterisk => '⧆',
            MiscellaneousMathematicalSymbolsB::SquaredSmallCircle => '⧇',
            MiscellaneousMathematicalSymbolsB::SquaredSquare => '⧈',
            MiscellaneousMathematicalSymbolsB::TwoJoinedSquares => '⧉',
            MiscellaneousMathematicalSymbolsB::TriangleWithDotAbove => '⧊',
            MiscellaneousMathematicalSymbolsB::TriangleWithUnderbar => '⧋',
            MiscellaneousMathematicalSymbolsB::SInTriangle => '⧌',
            MiscellaneousMathematicalSymbolsB::TriangleWithSerifsAtBottom => '⧍',
            MiscellaneousMathematicalSymbolsB::RightTriangleAboveLeftTriangle => '⧎',
            MiscellaneousMathematicalSymbolsB::LeftTriangleBesideVerticalBar => '⧏',
            MiscellaneousMathematicalSymbolsB::VerticalBarBesideRightTriangle => '⧐',
            MiscellaneousMathematicalSymbolsB::BowtieWithLeftHalfBlack => '⧑',
            MiscellaneousMathematicalSymbolsB::BowtieWithRightHalfBlack => '⧒',
            MiscellaneousMathematicalSymbolsB::BlackBowtie => '⧓',
            MiscellaneousMathematicalSymbolsB::TimesWithLeftHalfBlack => '⧔',
            MiscellaneousMathematicalSymbolsB::TimesWithRightHalfBlack => '⧕',
            MiscellaneousMathematicalSymbolsB::WhiteHourglass => '⧖',
            MiscellaneousMathematicalSymbolsB::BlackHourglass => '⧗',
            MiscellaneousMathematicalSymbolsB::LeftWigglyFence => '⧘',
            MiscellaneousMathematicalSymbolsB::RightWigglyFence => '⧙',
            MiscellaneousMathematicalSymbolsB::LeftDoubleWigglyFence => '⧚',
            MiscellaneousMathematicalSymbolsB::RightDoubleWigglyFence => '⧛',
            MiscellaneousMathematicalSymbolsB::IncompleteInfinity => '⧜',
            MiscellaneousMathematicalSymbolsB::TieOverInfinity => '⧝',
            MiscellaneousMathematicalSymbolsB::InfinityNegatedWithVerticalBar => '⧞',
            MiscellaneousMathematicalSymbolsB::DoubleDashEndedMultimap => '⧟',
            MiscellaneousMathematicalSymbolsB::SquareWithContouredOutline => '⧠',
            MiscellaneousMathematicalSymbolsB::IncreasesAs => '⧡',
            MiscellaneousMathematicalSymbolsB::ShuffleProduct => '⧢',
            MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallel => '⧣',
            MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallelWithTildeAbove => '⧤',
            MiscellaneousMathematicalSymbolsB::IdenticalToAndSlantedParallel => '⧥',
            MiscellaneousMathematicalSymbolsB::GleichStark => '⧦',
            MiscellaneousMathematicalSymbolsB::Thermodynamic => '⧧',
            MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithLeftHalfBlack => '⧨',
            MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithRightHalfBlack => '⧩',
            MiscellaneousMathematicalSymbolsB::BlackDiamondWithDownArrow => '⧪',
            MiscellaneousMathematicalSymbolsB::BlackLozenge => '⧫',
            MiscellaneousMathematicalSymbolsB::WhiteCircleWithDownArrow => '⧬',
            MiscellaneousMathematicalSymbolsB::BlackCircleWithDownArrow => '⧭',
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteSquare => '⧮',
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackSquare => '⧯',
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteDiamond => '⧰',
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackDiamond => '⧱',
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteCircle => '⧲',
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackCircle => '⧳',
            MiscellaneousMathematicalSymbolsB::RuleDashDelayed => '⧴',
            MiscellaneousMathematicalSymbolsB::ReverseSolidusOperator => '⧵',
            MiscellaneousMathematicalSymbolsB::SolidusWithOverbar => '⧶',
            MiscellaneousMathematicalSymbolsB::ReverseSolidusWithHorizontalStroke => '⧷',
            MiscellaneousMathematicalSymbolsB::BigSolidus => '⧸',
            MiscellaneousMathematicalSymbolsB::BigReverseSolidus => '⧹',
            MiscellaneousMathematicalSymbolsB::DoublePlus => '⧺',
            MiscellaneousMathematicalSymbolsB::TriplePlus => '⧻',
            MiscellaneousMathematicalSymbolsB::LeftDashPointingCurvedAngleBracket => '⧼',
            MiscellaneousMathematicalSymbolsB::RightDashPointingCurvedAngleBracket => '⧽',
            MiscellaneousMathematicalSymbolsB::Tiny => '⧾',
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousMathematicalSymbolsB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⦀' => Ok(MiscellaneousMathematicalSymbolsB::TripleVerticalBarDelimiter),
            '⦁' => Ok(MiscellaneousMathematicalSymbolsB::ZNotationSpot),
            '⦂' => Ok(MiscellaneousMathematicalSymbolsB::ZNotationTypeColon),
            '⦃' => Ok(MiscellaneousMathematicalSymbolsB::LeftWhiteCurlyBracket),
            '⦄' => Ok(MiscellaneousMathematicalSymbolsB::RightWhiteCurlyBracket),
            '⦅' => Ok(MiscellaneousMathematicalSymbolsB::LeftWhiteParenthesis),
            '⦆' => Ok(MiscellaneousMathematicalSymbolsB::RightWhiteParenthesis),
            '⦇' => Ok(MiscellaneousMathematicalSymbolsB::ZNotationLeftImageBracket),
            '⦈' => Ok(MiscellaneousMathematicalSymbolsB::ZNotationRightImageBracket),
            '⦉' => Ok(MiscellaneousMathematicalSymbolsB::ZNotationLeftBindingBracket),
            '⦊' => Ok(MiscellaneousMathematicalSymbolsB::ZNotationRightBindingBracket),
            '⦋' => Ok(MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithUnderbar),
            '⦌' => Ok(MiscellaneousMathematicalSymbolsB::RightSquareBracketWithUnderbar),
            '⦍' => Ok(MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInTopCorner),
            '⦎' => Ok(MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInBottomCorner),
            '⦏' => Ok(MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInBottomCorner),
            '⦐' => Ok(MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInTopCorner),
            '⦑' => Ok(MiscellaneousMathematicalSymbolsB::LeftAngleBracketWithDot),
            '⦒' => Ok(MiscellaneousMathematicalSymbolsB::RightAngleBracketWithDot),
            '⦓' => Ok(MiscellaneousMathematicalSymbolsB::LeftArcLessDashThanBracket),
            '⦔' => Ok(MiscellaneousMathematicalSymbolsB::RightArcGreaterDashThanBracket),
            '⦕' => Ok(MiscellaneousMathematicalSymbolsB::DoubleLeftArcGreaterDashThanBracket),
            '⦖' => Ok(MiscellaneousMathematicalSymbolsB::DoubleRightArcLessDashThanBracket),
            '⦗' => Ok(MiscellaneousMathematicalSymbolsB::LeftBlackTortoiseShellBracket),
            '⦘' => Ok(MiscellaneousMathematicalSymbolsB::RightBlackTortoiseShellBracket),
            '⦙' => Ok(MiscellaneousMathematicalSymbolsB::DottedFence),
            '⦚' => Ok(MiscellaneousMathematicalSymbolsB::VerticalZigzagLine),
            '⦛' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleOpeningLeft),
            '⦜' => Ok(MiscellaneousMathematicalSymbolsB::RightAngleVariantWithSquare),
            '⦝' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredRightAngleWithDot),
            '⦞' => Ok(MiscellaneousMathematicalSymbolsB::AngleWithSInside),
            '⦟' => Ok(MiscellaneousMathematicalSymbolsB::AcuteAngle),
            '⦠' => Ok(MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningLeft),
            '⦡' => Ok(MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningUp),
            '⦢' => Ok(MiscellaneousMathematicalSymbolsB::TurnedAngle),
            '⦣' => Ok(MiscellaneousMathematicalSymbolsB::ReversedAngle),
            '⦤' => Ok(MiscellaneousMathematicalSymbolsB::AngleWithUnderbar),
            '⦥' => Ok(MiscellaneousMathematicalSymbolsB::ReversedAngleWithUnderbar),
            '⦦' => Ok(MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningUp),
            '⦧' => Ok(MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningDown),
            '⦨' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndRight),
            '⦩' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndLeft),
            '⦪' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndRight),
            '⦫' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndLeft),
            '⦬' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndUp),
            '⦭' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndUp),
            '⦮' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndDown),
            '⦯' => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndDown),
            '⦰' => Ok(MiscellaneousMathematicalSymbolsB::ReversedEmptySet),
            '⦱' => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithOverbar),
            '⦲' => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithSmallCircleAbove),
            '⦳' => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithRightArrowAbove),
            '⦴' => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithLeftArrowAbove),
            '⦵' => Ok(MiscellaneousMathematicalSymbolsB::CircleWithHorizontalBar),
            '⦶' => Ok(MiscellaneousMathematicalSymbolsB::CircledVerticalBar),
            '⦷' => Ok(MiscellaneousMathematicalSymbolsB::CircledParallel),
            '⦸' => Ok(MiscellaneousMathematicalSymbolsB::CircledReverseSolidus),
            '⦹' => Ok(MiscellaneousMathematicalSymbolsB::CircledPerpendicular),
            '⦺' => Ok(MiscellaneousMathematicalSymbolsB::CircleDividedByHorizontalBarAndTopHalfDividedByVerticalBar),
            '⦻' => Ok(MiscellaneousMathematicalSymbolsB::CircleWithSuperimposedX),
            '⦼' => Ok(MiscellaneousMathematicalSymbolsB::CircledAnticlockwiseDashRotatedDivisionSign),
            '⦽' => Ok(MiscellaneousMathematicalSymbolsB::UpArrowThroughCircle),
            '⦾' => Ok(MiscellaneousMathematicalSymbolsB::CircledWhiteBullet),
            '⦿' => Ok(MiscellaneousMathematicalSymbolsB::CircledBullet),
            '⧀' => Ok(MiscellaneousMathematicalSymbolsB::CircledLessDashThan),
            '⧁' => Ok(MiscellaneousMathematicalSymbolsB::CircledGreaterDashThan),
            '⧂' => Ok(MiscellaneousMathematicalSymbolsB::CircleWithSmallCircleToTheRight),
            '⧃' => Ok(MiscellaneousMathematicalSymbolsB::CircleWithTwoHorizontalStrokesToTheRight),
            '⧄' => Ok(MiscellaneousMathematicalSymbolsB::SquaredRisingDiagonalSlash),
            '⧅' => Ok(MiscellaneousMathematicalSymbolsB::SquaredFallingDiagonalSlash),
            '⧆' => Ok(MiscellaneousMathematicalSymbolsB::SquaredAsterisk),
            '⧇' => Ok(MiscellaneousMathematicalSymbolsB::SquaredSmallCircle),
            '⧈' => Ok(MiscellaneousMathematicalSymbolsB::SquaredSquare),
            '⧉' => Ok(MiscellaneousMathematicalSymbolsB::TwoJoinedSquares),
            '⧊' => Ok(MiscellaneousMathematicalSymbolsB::TriangleWithDotAbove),
            '⧋' => Ok(MiscellaneousMathematicalSymbolsB::TriangleWithUnderbar),
            '⧌' => Ok(MiscellaneousMathematicalSymbolsB::SInTriangle),
            '⧍' => Ok(MiscellaneousMathematicalSymbolsB::TriangleWithSerifsAtBottom),
            '⧎' => Ok(MiscellaneousMathematicalSymbolsB::RightTriangleAboveLeftTriangle),
            '⧏' => Ok(MiscellaneousMathematicalSymbolsB::LeftTriangleBesideVerticalBar),
            '⧐' => Ok(MiscellaneousMathematicalSymbolsB::VerticalBarBesideRightTriangle),
            '⧑' => Ok(MiscellaneousMathematicalSymbolsB::BowtieWithLeftHalfBlack),
            '⧒' => Ok(MiscellaneousMathematicalSymbolsB::BowtieWithRightHalfBlack),
            '⧓' => Ok(MiscellaneousMathematicalSymbolsB::BlackBowtie),
            '⧔' => Ok(MiscellaneousMathematicalSymbolsB::TimesWithLeftHalfBlack),
            '⧕' => Ok(MiscellaneousMathematicalSymbolsB::TimesWithRightHalfBlack),
            '⧖' => Ok(MiscellaneousMathematicalSymbolsB::WhiteHourglass),
            '⧗' => Ok(MiscellaneousMathematicalSymbolsB::BlackHourglass),
            '⧘' => Ok(MiscellaneousMathematicalSymbolsB::LeftWigglyFence),
            '⧙' => Ok(MiscellaneousMathematicalSymbolsB::RightWigglyFence),
            '⧚' => Ok(MiscellaneousMathematicalSymbolsB::LeftDoubleWigglyFence),
            '⧛' => Ok(MiscellaneousMathematicalSymbolsB::RightDoubleWigglyFence),
            '⧜' => Ok(MiscellaneousMathematicalSymbolsB::IncompleteInfinity),
            '⧝' => Ok(MiscellaneousMathematicalSymbolsB::TieOverInfinity),
            '⧞' => Ok(MiscellaneousMathematicalSymbolsB::InfinityNegatedWithVerticalBar),
            '⧟' => Ok(MiscellaneousMathematicalSymbolsB::DoubleDashEndedMultimap),
            '⧠' => Ok(MiscellaneousMathematicalSymbolsB::SquareWithContouredOutline),
            '⧡' => Ok(MiscellaneousMathematicalSymbolsB::IncreasesAs),
            '⧢' => Ok(MiscellaneousMathematicalSymbolsB::ShuffleProduct),
            '⧣' => Ok(MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallel),
            '⧤' => Ok(MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallelWithTildeAbove),
            '⧥' => Ok(MiscellaneousMathematicalSymbolsB::IdenticalToAndSlantedParallel),
            '⧦' => Ok(MiscellaneousMathematicalSymbolsB::GleichStark),
            '⧧' => Ok(MiscellaneousMathematicalSymbolsB::Thermodynamic),
            '⧨' => Ok(MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithLeftHalfBlack),
            '⧩' => Ok(MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithRightHalfBlack),
            '⧪' => Ok(MiscellaneousMathematicalSymbolsB::BlackDiamondWithDownArrow),
            '⧫' => Ok(MiscellaneousMathematicalSymbolsB::BlackLozenge),
            '⧬' => Ok(MiscellaneousMathematicalSymbolsB::WhiteCircleWithDownArrow),
            '⧭' => Ok(MiscellaneousMathematicalSymbolsB::BlackCircleWithDownArrow),
            '⧮' => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteSquare),
            '⧯' => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackSquare),
            '⧰' => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteDiamond),
            '⧱' => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackDiamond),
            '⧲' => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteCircle),
            '⧳' => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackCircle),
            '⧴' => Ok(MiscellaneousMathematicalSymbolsB::RuleDashDelayed),
            '⧵' => Ok(MiscellaneousMathematicalSymbolsB::ReverseSolidusOperator),
            '⧶' => Ok(MiscellaneousMathematicalSymbolsB::SolidusWithOverbar),
            '⧷' => Ok(MiscellaneousMathematicalSymbolsB::ReverseSolidusWithHorizontalStroke),
            '⧸' => Ok(MiscellaneousMathematicalSymbolsB::BigSolidus),
            '⧹' => Ok(MiscellaneousMathematicalSymbolsB::BigReverseSolidus),
            '⧺' => Ok(MiscellaneousMathematicalSymbolsB::DoublePlus),
            '⧻' => Ok(MiscellaneousMathematicalSymbolsB::TriplePlus),
            '⧼' => Ok(MiscellaneousMathematicalSymbolsB::LeftDashPointingCurvedAngleBracket),
            '⧽' => Ok(MiscellaneousMathematicalSymbolsB::RightDashPointingCurvedAngleBracket),
            '⧾' => Ok(MiscellaneousMathematicalSymbolsB::Tiny),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MiscellaneousMathematicalSymbolsB {
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

impl std::convert::TryFrom<u32> for MiscellaneousMathematicalSymbolsB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MiscellaneousMathematicalSymbolsB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MiscellaneousMathematicalSymbolsB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MiscellaneousMathematicalSymbolsB::TripleVerticalBarDelimiter
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MiscellaneousMathematicalSymbolsB{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
