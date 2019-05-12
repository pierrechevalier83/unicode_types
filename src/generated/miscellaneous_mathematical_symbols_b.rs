/// \u{2980} → \u{29ff}
///
/// ⦀ ⦁ ⦂ ⦃ ⦄ ⦅ ⦆ ⦇ ⦈ ⦉ ⦊ ⦋ ⦌ ⦍ ⦎ ⦏\
/// ⦐ ⦑ ⦒ ⦓ ⦔ ⦕ ⦖ ⦗ ⦘ ⦙ ⦚ ⦛ ⦜ ⦝ ⦞ ⦟\
/// ⦠ ⦡ ⦢ ⦣ ⦤ ⦥ ⦦ ⦧ ⦨ ⦩ ⦪ ⦫ ⦬ ⦭ ⦮ ⦯\
/// ⦰ ⦱ ⦲ ⦳ ⦴ ⦵ ⦶ ⦷ ⦸ ⦹ ⦺ ⦻ ⦼ ⦽ ⦾ ⦿\
/// ⧀ ⧁ ⧂ ⧃ ⧄ ⧅ ⧆ ⧇ ⧈ ⧉ ⧊ ⧋ ⧌ ⧍ ⧎ ⧏\
/// ⧐ ⧑ ⧒ ⧓ ⧔ ⧕ ⧖ ⧗ ⧘ ⧙ ⧚ ⧛ ⧜ ⧝ ⧞ ⧟\
/// ⧠ ⧡ ⧢ ⧣ ⧤ ⧥ ⧦ ⧧ ⧨ ⧩ ⧪ ⧫ ⧬ ⧭ ⧮ ⧯\
/// ⧰ ⧱ ⧲ ⧳ ⧴ ⧵ ⧶ ⧷ ⧸ ⧹ ⧺ ⧻ ⧼ ⧽ ⧾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2980}: '⦀'
    pub const TRIPLE_VERTICAL_BAR_DELIMITER: char = '⦀';
    /// \u{2981}: '⦁'
    pub const Z_NOTATION_SPOT: char = '⦁';
    /// \u{2982}: '⦂'
    pub const Z_NOTATION_TYPE_COLON: char = '⦂';
    /// \u{2983}: '⦃'
    pub const LEFT_WHITE_CURLY_BRACKET: char = '⦃';
    /// \u{2984}: '⦄'
    pub const RIGHT_WHITE_CURLY_BRACKET: char = '⦄';
    /// \u{2985}: '⦅'
    pub const LEFT_WHITE_PARENTHESIS: char = '⦅';
    /// \u{2986}: '⦆'
    pub const RIGHT_WHITE_PARENTHESIS: char = '⦆';
    /// \u{2987}: '⦇'
    pub const Z_NOTATION_LEFT_IMAGE_BRACKET: char = '⦇';
    /// \u{2988}: '⦈'
    pub const Z_NOTATION_RIGHT_IMAGE_BRACKET: char = '⦈';
    /// \u{2989}: '⦉'
    pub const Z_NOTATION_LEFT_BINDING_BRACKET: char = '⦉';
    /// \u{298a}: '⦊'
    pub const Z_NOTATION_RIGHT_BINDING_BRACKET: char = '⦊';
    /// \u{298b}: '⦋'
    pub const LEFT_SQUARE_BRACKET_WITH_UNDERBAR: char = '⦋';
    /// \u{298c}: '⦌'
    pub const RIGHT_SQUARE_BRACKET_WITH_UNDERBAR: char = '⦌';
    /// \u{298d}: '⦍'
    pub const LEFT_SQUARE_BRACKET_WITH_TICK_IN_TOP_CORNER: char = '⦍';
    /// \u{298e}: '⦎'
    pub const RIGHT_SQUARE_BRACKET_WITH_TICK_IN_BOTTOM_CORNER: char = '⦎';
    /// \u{298f}: '⦏'
    pub const LEFT_SQUARE_BRACKET_WITH_TICK_IN_BOTTOM_CORNER: char = '⦏';
    /// \u{2990}: '⦐'
    pub const RIGHT_SQUARE_BRACKET_WITH_TICK_IN_TOP_CORNER: char = '⦐';
    /// \u{2991}: '⦑'
    pub const LEFT_ANGLE_BRACKET_WITH_DOT: char = '⦑';
    /// \u{2992}: '⦒'
    pub const RIGHT_ANGLE_BRACKET_WITH_DOT: char = '⦒';
    /// \u{2993}: '⦓'
    pub const LEFT_ARC_LESS_DASH_THAN_BRACKET: char = '⦓';
    /// \u{2994}: '⦔'
    pub const RIGHT_ARC_GREATER_DASH_THAN_BRACKET: char = '⦔';
    /// \u{2995}: '⦕'
    pub const DOUBLE_LEFT_ARC_GREATER_DASH_THAN_BRACKET: char = '⦕';
    /// \u{2996}: '⦖'
    pub const DOUBLE_RIGHT_ARC_LESS_DASH_THAN_BRACKET: char = '⦖';
    /// \u{2997}: '⦗'
    pub const LEFT_BLACK_TORTOISE_SHELL_BRACKET: char = '⦗';
    /// \u{2998}: '⦘'
    pub const RIGHT_BLACK_TORTOISE_SHELL_BRACKET: char = '⦘';
    /// \u{2999}: '⦙'
    pub const DOTTED_FENCE: char = '⦙';
    /// \u{299a}: '⦚'
    pub const VERTICAL_ZIGZAG_LINE: char = '⦚';
    /// \u{299b}: '⦛'
    pub const MEASURED_ANGLE_OPENING_LEFT: char = '⦛';
    /// \u{299c}: '⦜'
    pub const RIGHT_ANGLE_VARIANT_WITH_SQUARE: char = '⦜';
    /// \u{299d}: '⦝'
    pub const MEASURED_RIGHT_ANGLE_WITH_DOT: char = '⦝';
    /// \u{299e}: '⦞'
    pub const ANGLE_WITH_S_INSIDE: char = '⦞';
    /// \u{299f}: '⦟'
    pub const ACUTE_ANGLE: char = '⦟';
    /// \u{29a0}: '⦠'
    pub const SPHERICAL_ANGLE_OPENING_LEFT: char = '⦠';
    /// \u{29a1}: '⦡'
    pub const SPHERICAL_ANGLE_OPENING_UP: char = '⦡';
    /// \u{29a2}: '⦢'
    pub const TURNED_ANGLE: char = '⦢';
    /// \u{29a3}: '⦣'
    pub const REVERSED_ANGLE: char = '⦣';
    /// \u{29a4}: '⦤'
    pub const ANGLE_WITH_UNDERBAR: char = '⦤';
    /// \u{29a5}: '⦥'
    pub const REVERSED_ANGLE_WITH_UNDERBAR: char = '⦥';
    /// \u{29a6}: '⦦'
    pub const OBLIQUE_ANGLE_OPENING_UP: char = '⦦';
    /// \u{29a7}: '⦧'
    pub const OBLIQUE_ANGLE_OPENING_DOWN: char = '⦧';
    /// \u{29a8}: '⦨'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_UP_AND_RIGHT: char = '⦨';
    /// \u{29a9}: '⦩'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_UP_AND_LEFT: char = '⦩';
    /// \u{29aa}: '⦪'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_DOWN_AND_RIGHT: char = '⦪';
    /// \u{29ab}: '⦫'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_DOWN_AND_LEFT: char = '⦫';
    /// \u{29ac}: '⦬'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_RIGHT_AND_UP: char = '⦬';
    /// \u{29ad}: '⦭'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_LEFT_AND_UP: char = '⦭';
    /// \u{29ae}: '⦮'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_RIGHT_AND_DOWN: char = '⦮';
    /// \u{29af}: '⦯'
    pub const MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_LEFT_AND_DOWN: char = '⦯';
    /// \u{29b0}: '⦰'
    pub const REVERSED_EMPTY_SET: char = '⦰';
    /// \u{29b1}: '⦱'
    pub const EMPTY_SET_WITH_OVERBAR: char = '⦱';
    /// \u{29b2}: '⦲'
    pub const EMPTY_SET_WITH_SMALL_CIRCLE_ABOVE: char = '⦲';
    /// \u{29b3}: '⦳'
    pub const EMPTY_SET_WITH_RIGHT_ARROW_ABOVE: char = '⦳';
    /// \u{29b4}: '⦴'
    pub const EMPTY_SET_WITH_LEFT_ARROW_ABOVE: char = '⦴';
    /// \u{29b5}: '⦵'
    pub const CIRCLE_WITH_HORIZONTAL_BAR: char = '⦵';
    /// \u{29b6}: '⦶'
    pub const CIRCLED_VERTICAL_BAR: char = '⦶';
    /// \u{29b7}: '⦷'
    pub const CIRCLED_PARALLEL: char = '⦷';
    /// \u{29b8}: '⦸'
    pub const CIRCLED_REVERSE_SOLIDUS: char = '⦸';
    /// \u{29b9}: '⦹'
    pub const CIRCLED_PERPENDICULAR: char = '⦹';
    /// \u{29ba}: '⦺'
    pub const CIRCLE_DIVIDED_BY_HORIZONTAL_BAR_AND_TOP_HALF_DIVIDED_BY_VERTICAL_BAR: char = '⦺';
    /// \u{29bb}: '⦻'
    pub const CIRCLE_WITH_SUPERIMPOSED_X: char = '⦻';
    /// \u{29bc}: '⦼'
    pub const CIRCLED_ANTICLOCKWISE_DASH_ROTATED_DIVISION_SIGN: char = '⦼';
    /// \u{29bd}: '⦽'
    pub const UP_ARROW_THROUGH_CIRCLE: char = '⦽';
    /// \u{29be}: '⦾'
    pub const CIRCLED_WHITE_BULLET: char = '⦾';
    /// \u{29bf}: '⦿'
    pub const CIRCLED_BULLET: char = '⦿';
    /// \u{29c0}: '⧀'
    pub const CIRCLED_LESS_DASH_THAN: char = '⧀';
    /// \u{29c1}: '⧁'
    pub const CIRCLED_GREATER_DASH_THAN: char = '⧁';
    /// \u{29c2}: '⧂'
    pub const CIRCLE_WITH_SMALL_CIRCLE_TO_THE_RIGHT: char = '⧂';
    /// \u{29c3}: '⧃'
    pub const CIRCLE_WITH_TWO_HORIZONTAL_STROKES_TO_THE_RIGHT: char = '⧃';
    /// \u{29c4}: '⧄'
    pub const SQUARED_RISING_DIAGONAL_SLASH: char = '⧄';
    /// \u{29c5}: '⧅'
    pub const SQUARED_FALLING_DIAGONAL_SLASH: char = '⧅';
    /// \u{29c6}: '⧆'
    pub const SQUARED_ASTERISK: char = '⧆';
    /// \u{29c7}: '⧇'
    pub const SQUARED_SMALL_CIRCLE: char = '⧇';
    /// \u{29c8}: '⧈'
    pub const SQUARED_SQUARE: char = '⧈';
    /// \u{29c9}: '⧉'
    pub const TWO_JOINED_SQUARES: char = '⧉';
    /// \u{29ca}: '⧊'
    pub const TRIANGLE_WITH_DOT_ABOVE: char = '⧊';
    /// \u{29cb}: '⧋'
    pub const TRIANGLE_WITH_UNDERBAR: char = '⧋';
    /// \u{29cc}: '⧌'
    pub const S_IN_TRIANGLE: char = '⧌';
    /// \u{29cd}: '⧍'
    pub const TRIANGLE_WITH_SERIFS_AT_BOTTOM: char = '⧍';
    /// \u{29ce}: '⧎'
    pub const RIGHT_TRIANGLE_ABOVE_LEFT_TRIANGLE: char = '⧎';
    /// \u{29cf}: '⧏'
    pub const LEFT_TRIANGLE_BESIDE_VERTICAL_BAR: char = '⧏';
    /// \u{29d0}: '⧐'
    pub const VERTICAL_BAR_BESIDE_RIGHT_TRIANGLE: char = '⧐';
    /// \u{29d1}: '⧑'
    pub const BOWTIE_WITH_LEFT_HALF_BLACK: char = '⧑';
    /// \u{29d2}: '⧒'
    pub const BOWTIE_WITH_RIGHT_HALF_BLACK: char = '⧒';
    /// \u{29d3}: '⧓'
    pub const BLACK_BOWTIE: char = '⧓';
    /// \u{29d4}: '⧔'
    pub const TIMES_WITH_LEFT_HALF_BLACK: char = '⧔';
    /// \u{29d5}: '⧕'
    pub const TIMES_WITH_RIGHT_HALF_BLACK: char = '⧕';
    /// \u{29d6}: '⧖'
    pub const WHITE_HOURGLASS: char = '⧖';
    /// \u{29d7}: '⧗'
    pub const BLACK_HOURGLASS: char = '⧗';
    /// \u{29d8}: '⧘'
    pub const LEFT_WIGGLY_FENCE: char = '⧘';
    /// \u{29d9}: '⧙'
    pub const RIGHT_WIGGLY_FENCE: char = '⧙';
    /// \u{29da}: '⧚'
    pub const LEFT_DOUBLE_WIGGLY_FENCE: char = '⧚';
    /// \u{29db}: '⧛'
    pub const RIGHT_DOUBLE_WIGGLY_FENCE: char = '⧛';
    /// \u{29dc}: '⧜'
    pub const INCOMPLETE_INFINITY: char = '⧜';
    /// \u{29dd}: '⧝'
    pub const TIE_OVER_INFINITY: char = '⧝';
    /// \u{29de}: '⧞'
    pub const INFINITY_NEGATED_WITH_VERTICAL_BAR: char = '⧞';
    /// \u{29df}: '⧟'
    pub const DOUBLE_DASH_ENDED_MULTIMAP: char = '⧟';
    /// \u{29e0}: '⧠'
    pub const SQUARE_WITH_CONTOURED_OUTLINE: char = '⧠';
    /// \u{29e1}: '⧡'
    pub const INCREASES_AS: char = '⧡';
    /// \u{29e2}: '⧢'
    pub const SHUFFLE_PRODUCT: char = '⧢';
    /// \u{29e3}: '⧣'
    pub const EQUALS_SIGN_AND_SLANTED_PARALLEL: char = '⧣';
    /// \u{29e4}: '⧤'
    pub const EQUALS_SIGN_AND_SLANTED_PARALLEL_WITH_TILDE_ABOVE: char = '⧤';
    /// \u{29e5}: '⧥'
    pub const IDENTICAL_TO_AND_SLANTED_PARALLEL: char = '⧥';
    /// \u{29e6}: '⧦'
    pub const GLEICH_STARK: char = '⧦';
    /// \u{29e7}: '⧧'
    pub const THERMODYNAMIC: char = '⧧';
    /// \u{29e8}: '⧨'
    pub const DOWN_DASH_POINTING_TRIANGLE_WITH_LEFT_HALF_BLACK: char = '⧨';
    /// \u{29e9}: '⧩'
    pub const DOWN_DASH_POINTING_TRIANGLE_WITH_RIGHT_HALF_BLACK: char = '⧩';
    /// \u{29ea}: '⧪'
    pub const BLACK_DIAMOND_WITH_DOWN_ARROW: char = '⧪';
    /// \u{29eb}: '⧫'
    pub const BLACK_LOZENGE: char = '⧫';
    /// \u{29ec}: '⧬'
    pub const WHITE_CIRCLE_WITH_DOWN_ARROW: char = '⧬';
    /// \u{29ed}: '⧭'
    pub const BLACK_CIRCLE_WITH_DOWN_ARROW: char = '⧭';
    /// \u{29ee}: '⧮'
    pub const ERROR_DASH_BARRED_WHITE_SQUARE: char = '⧮';
    /// \u{29ef}: '⧯'
    pub const ERROR_DASH_BARRED_BLACK_SQUARE: char = '⧯';
    /// \u{29f0}: '⧰'
    pub const ERROR_DASH_BARRED_WHITE_DIAMOND: char = '⧰';
    /// \u{29f1}: '⧱'
    pub const ERROR_DASH_BARRED_BLACK_DIAMOND: char = '⧱';
    /// \u{29f2}: '⧲'
    pub const ERROR_DASH_BARRED_WHITE_CIRCLE: char = '⧲';
    /// \u{29f3}: '⧳'
    pub const ERROR_DASH_BARRED_BLACK_CIRCLE: char = '⧳';
    /// \u{29f4}: '⧴'
    pub const RULE_DASH_DELAYED: char = '⧴';
    /// \u{29f5}: '⧵'
    pub const REVERSE_SOLIDUS_OPERATOR: char = '⧵';
    /// \u{29f6}: '⧶'
    pub const SOLIDUS_WITH_OVERBAR: char = '⧶';
    /// \u{29f7}: '⧷'
    pub const REVERSE_SOLIDUS_WITH_HORIZONTAL_STROKE: char = '⧷';
    /// \u{29f8}: '⧸'
    pub const BIG_SOLIDUS: char = '⧸';
    /// \u{29f9}: '⧹'
    pub const BIG_REVERSE_SOLIDUS: char = '⧹';
    /// \u{29fa}: '⧺'
    pub const DOUBLE_PLUS: char = '⧺';
    /// \u{29fb}: '⧻'
    pub const TRIPLE_PLUS: char = '⧻';
    /// \u{29fc}: '⧼'
    pub const LEFT_DASH_POINTING_CURVED_ANGLE_BRACKET: char = '⧼';
    /// \u{29fd}: '⧽'
    pub const RIGHT_DASH_POINTING_CURVED_ANGLE_BRACKET: char = '⧽';
    /// \u{29fe}: '⧾'
    pub const TINY: char = '⧾';
}

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
        use constants::*;
        match self {
            MiscellaneousMathematicalSymbolsB::TripleVerticalBarDelimiter => TRIPLE_VERTICAL_BAR_DELIMITER,
            MiscellaneousMathematicalSymbolsB::ZNotationSpot => Z_NOTATION_SPOT,
            MiscellaneousMathematicalSymbolsB::ZNotationTypeColon => Z_NOTATION_TYPE_COLON,
            MiscellaneousMathematicalSymbolsB::LeftWhiteCurlyBracket => LEFT_WHITE_CURLY_BRACKET,
            MiscellaneousMathematicalSymbolsB::RightWhiteCurlyBracket => RIGHT_WHITE_CURLY_BRACKET,
            MiscellaneousMathematicalSymbolsB::LeftWhiteParenthesis => LEFT_WHITE_PARENTHESIS,
            MiscellaneousMathematicalSymbolsB::RightWhiteParenthesis => RIGHT_WHITE_PARENTHESIS,
            MiscellaneousMathematicalSymbolsB::ZNotationLeftImageBracket => Z_NOTATION_LEFT_IMAGE_BRACKET,
            MiscellaneousMathematicalSymbolsB::ZNotationRightImageBracket => Z_NOTATION_RIGHT_IMAGE_BRACKET,
            MiscellaneousMathematicalSymbolsB::ZNotationLeftBindingBracket => Z_NOTATION_LEFT_BINDING_BRACKET,
            MiscellaneousMathematicalSymbolsB::ZNotationRightBindingBracket => Z_NOTATION_RIGHT_BINDING_BRACKET,
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithUnderbar => LEFT_SQUARE_BRACKET_WITH_UNDERBAR,
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithUnderbar => RIGHT_SQUARE_BRACKET_WITH_UNDERBAR,
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInTopCorner => LEFT_SQUARE_BRACKET_WITH_TICK_IN_TOP_CORNER,
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInBottomCorner => RIGHT_SQUARE_BRACKET_WITH_TICK_IN_BOTTOM_CORNER,
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInBottomCorner => LEFT_SQUARE_BRACKET_WITH_TICK_IN_BOTTOM_CORNER,
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInTopCorner => RIGHT_SQUARE_BRACKET_WITH_TICK_IN_TOP_CORNER,
            MiscellaneousMathematicalSymbolsB::LeftAngleBracketWithDot => LEFT_ANGLE_BRACKET_WITH_DOT,
            MiscellaneousMathematicalSymbolsB::RightAngleBracketWithDot => RIGHT_ANGLE_BRACKET_WITH_DOT,
            MiscellaneousMathematicalSymbolsB::LeftArcLessDashThanBracket => LEFT_ARC_LESS_DASH_THAN_BRACKET,
            MiscellaneousMathematicalSymbolsB::RightArcGreaterDashThanBracket => RIGHT_ARC_GREATER_DASH_THAN_BRACKET,
            MiscellaneousMathematicalSymbolsB::DoubleLeftArcGreaterDashThanBracket => DOUBLE_LEFT_ARC_GREATER_DASH_THAN_BRACKET,
            MiscellaneousMathematicalSymbolsB::DoubleRightArcLessDashThanBracket => DOUBLE_RIGHT_ARC_LESS_DASH_THAN_BRACKET,
            MiscellaneousMathematicalSymbolsB::LeftBlackTortoiseShellBracket => LEFT_BLACK_TORTOISE_SHELL_BRACKET,
            MiscellaneousMathematicalSymbolsB::RightBlackTortoiseShellBracket => RIGHT_BLACK_TORTOISE_SHELL_BRACKET,
            MiscellaneousMathematicalSymbolsB::DottedFence => DOTTED_FENCE,
            MiscellaneousMathematicalSymbolsB::VerticalZigzagLine => VERTICAL_ZIGZAG_LINE,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleOpeningLeft => MEASURED_ANGLE_OPENING_LEFT,
            MiscellaneousMathematicalSymbolsB::RightAngleVariantWithSquare => RIGHT_ANGLE_VARIANT_WITH_SQUARE,
            MiscellaneousMathematicalSymbolsB::MeasuredRightAngleWithDot => MEASURED_RIGHT_ANGLE_WITH_DOT,
            MiscellaneousMathematicalSymbolsB::AngleWithSInside => ANGLE_WITH_S_INSIDE,
            MiscellaneousMathematicalSymbolsB::AcuteAngle => ACUTE_ANGLE,
            MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningLeft => SPHERICAL_ANGLE_OPENING_LEFT,
            MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningUp => SPHERICAL_ANGLE_OPENING_UP,
            MiscellaneousMathematicalSymbolsB::TurnedAngle => TURNED_ANGLE,
            MiscellaneousMathematicalSymbolsB::ReversedAngle => REVERSED_ANGLE,
            MiscellaneousMathematicalSymbolsB::AngleWithUnderbar => ANGLE_WITH_UNDERBAR,
            MiscellaneousMathematicalSymbolsB::ReversedAngleWithUnderbar => REVERSED_ANGLE_WITH_UNDERBAR,
            MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningUp => OBLIQUE_ANGLE_OPENING_UP,
            MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningDown => OBLIQUE_ANGLE_OPENING_DOWN,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndRight => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_UP_AND_RIGHT,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndLeft => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_UP_AND_LEFT,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndRight => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_DOWN_AND_RIGHT,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndLeft => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_DOWN_AND_LEFT,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndUp => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_RIGHT_AND_UP,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndUp => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_LEFT_AND_UP,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndDown => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_RIGHT_AND_DOWN,
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndDown => MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_LEFT_AND_DOWN,
            MiscellaneousMathematicalSymbolsB::ReversedEmptySet => REVERSED_EMPTY_SET,
            MiscellaneousMathematicalSymbolsB::EmptySetWithOverbar => EMPTY_SET_WITH_OVERBAR,
            MiscellaneousMathematicalSymbolsB::EmptySetWithSmallCircleAbove => EMPTY_SET_WITH_SMALL_CIRCLE_ABOVE,
            MiscellaneousMathematicalSymbolsB::EmptySetWithRightArrowAbove => EMPTY_SET_WITH_RIGHT_ARROW_ABOVE,
            MiscellaneousMathematicalSymbolsB::EmptySetWithLeftArrowAbove => EMPTY_SET_WITH_LEFT_ARROW_ABOVE,
            MiscellaneousMathematicalSymbolsB::CircleWithHorizontalBar => CIRCLE_WITH_HORIZONTAL_BAR,
            MiscellaneousMathematicalSymbolsB::CircledVerticalBar => CIRCLED_VERTICAL_BAR,
            MiscellaneousMathematicalSymbolsB::CircledParallel => CIRCLED_PARALLEL,
            MiscellaneousMathematicalSymbolsB::CircledReverseSolidus => CIRCLED_REVERSE_SOLIDUS,
            MiscellaneousMathematicalSymbolsB::CircledPerpendicular => CIRCLED_PERPENDICULAR,
            MiscellaneousMathematicalSymbolsB::CircleDividedByHorizontalBarAndTopHalfDividedByVerticalBar => CIRCLE_DIVIDED_BY_HORIZONTAL_BAR_AND_TOP_HALF_DIVIDED_BY_VERTICAL_BAR,
            MiscellaneousMathematicalSymbolsB::CircleWithSuperimposedX => CIRCLE_WITH_SUPERIMPOSED_X,
            MiscellaneousMathematicalSymbolsB::CircledAnticlockwiseDashRotatedDivisionSign => CIRCLED_ANTICLOCKWISE_DASH_ROTATED_DIVISION_SIGN,
            MiscellaneousMathematicalSymbolsB::UpArrowThroughCircle => UP_ARROW_THROUGH_CIRCLE,
            MiscellaneousMathematicalSymbolsB::CircledWhiteBullet => CIRCLED_WHITE_BULLET,
            MiscellaneousMathematicalSymbolsB::CircledBullet => CIRCLED_BULLET,
            MiscellaneousMathematicalSymbolsB::CircledLessDashThan => CIRCLED_LESS_DASH_THAN,
            MiscellaneousMathematicalSymbolsB::CircledGreaterDashThan => CIRCLED_GREATER_DASH_THAN,
            MiscellaneousMathematicalSymbolsB::CircleWithSmallCircleToTheRight => CIRCLE_WITH_SMALL_CIRCLE_TO_THE_RIGHT,
            MiscellaneousMathematicalSymbolsB::CircleWithTwoHorizontalStrokesToTheRight => CIRCLE_WITH_TWO_HORIZONTAL_STROKES_TO_THE_RIGHT,
            MiscellaneousMathematicalSymbolsB::SquaredRisingDiagonalSlash => SQUARED_RISING_DIAGONAL_SLASH,
            MiscellaneousMathematicalSymbolsB::SquaredFallingDiagonalSlash => SQUARED_FALLING_DIAGONAL_SLASH,
            MiscellaneousMathematicalSymbolsB::SquaredAsterisk => SQUARED_ASTERISK,
            MiscellaneousMathematicalSymbolsB::SquaredSmallCircle => SQUARED_SMALL_CIRCLE,
            MiscellaneousMathematicalSymbolsB::SquaredSquare => SQUARED_SQUARE,
            MiscellaneousMathematicalSymbolsB::TwoJoinedSquares => TWO_JOINED_SQUARES,
            MiscellaneousMathematicalSymbolsB::TriangleWithDotAbove => TRIANGLE_WITH_DOT_ABOVE,
            MiscellaneousMathematicalSymbolsB::TriangleWithUnderbar => TRIANGLE_WITH_UNDERBAR,
            MiscellaneousMathematicalSymbolsB::SInTriangle => S_IN_TRIANGLE,
            MiscellaneousMathematicalSymbolsB::TriangleWithSerifsAtBottom => TRIANGLE_WITH_SERIFS_AT_BOTTOM,
            MiscellaneousMathematicalSymbolsB::RightTriangleAboveLeftTriangle => RIGHT_TRIANGLE_ABOVE_LEFT_TRIANGLE,
            MiscellaneousMathematicalSymbolsB::LeftTriangleBesideVerticalBar => LEFT_TRIANGLE_BESIDE_VERTICAL_BAR,
            MiscellaneousMathematicalSymbolsB::VerticalBarBesideRightTriangle => VERTICAL_BAR_BESIDE_RIGHT_TRIANGLE,
            MiscellaneousMathematicalSymbolsB::BowtieWithLeftHalfBlack => BOWTIE_WITH_LEFT_HALF_BLACK,
            MiscellaneousMathematicalSymbolsB::BowtieWithRightHalfBlack => BOWTIE_WITH_RIGHT_HALF_BLACK,
            MiscellaneousMathematicalSymbolsB::BlackBowtie => BLACK_BOWTIE,
            MiscellaneousMathematicalSymbolsB::TimesWithLeftHalfBlack => TIMES_WITH_LEFT_HALF_BLACK,
            MiscellaneousMathematicalSymbolsB::TimesWithRightHalfBlack => TIMES_WITH_RIGHT_HALF_BLACK,
            MiscellaneousMathematicalSymbolsB::WhiteHourglass => WHITE_HOURGLASS,
            MiscellaneousMathematicalSymbolsB::BlackHourglass => BLACK_HOURGLASS,
            MiscellaneousMathematicalSymbolsB::LeftWigglyFence => LEFT_WIGGLY_FENCE,
            MiscellaneousMathematicalSymbolsB::RightWigglyFence => RIGHT_WIGGLY_FENCE,
            MiscellaneousMathematicalSymbolsB::LeftDoubleWigglyFence => LEFT_DOUBLE_WIGGLY_FENCE,
            MiscellaneousMathematicalSymbolsB::RightDoubleWigglyFence => RIGHT_DOUBLE_WIGGLY_FENCE,
            MiscellaneousMathematicalSymbolsB::IncompleteInfinity => INCOMPLETE_INFINITY,
            MiscellaneousMathematicalSymbolsB::TieOverInfinity => TIE_OVER_INFINITY,
            MiscellaneousMathematicalSymbolsB::InfinityNegatedWithVerticalBar => INFINITY_NEGATED_WITH_VERTICAL_BAR,
            MiscellaneousMathematicalSymbolsB::DoubleDashEndedMultimap => DOUBLE_DASH_ENDED_MULTIMAP,
            MiscellaneousMathematicalSymbolsB::SquareWithContouredOutline => SQUARE_WITH_CONTOURED_OUTLINE,
            MiscellaneousMathematicalSymbolsB::IncreasesAs => INCREASES_AS,
            MiscellaneousMathematicalSymbolsB::ShuffleProduct => SHUFFLE_PRODUCT,
            MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallel => EQUALS_SIGN_AND_SLANTED_PARALLEL,
            MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallelWithTildeAbove => EQUALS_SIGN_AND_SLANTED_PARALLEL_WITH_TILDE_ABOVE,
            MiscellaneousMathematicalSymbolsB::IdenticalToAndSlantedParallel => IDENTICAL_TO_AND_SLANTED_PARALLEL,
            MiscellaneousMathematicalSymbolsB::GleichStark => GLEICH_STARK,
            MiscellaneousMathematicalSymbolsB::Thermodynamic => THERMODYNAMIC,
            MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithLeftHalfBlack => DOWN_DASH_POINTING_TRIANGLE_WITH_LEFT_HALF_BLACK,
            MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithRightHalfBlack => DOWN_DASH_POINTING_TRIANGLE_WITH_RIGHT_HALF_BLACK,
            MiscellaneousMathematicalSymbolsB::BlackDiamondWithDownArrow => BLACK_DIAMOND_WITH_DOWN_ARROW,
            MiscellaneousMathematicalSymbolsB::BlackLozenge => BLACK_LOZENGE,
            MiscellaneousMathematicalSymbolsB::WhiteCircleWithDownArrow => WHITE_CIRCLE_WITH_DOWN_ARROW,
            MiscellaneousMathematicalSymbolsB::BlackCircleWithDownArrow => BLACK_CIRCLE_WITH_DOWN_ARROW,
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteSquare => ERROR_DASH_BARRED_WHITE_SQUARE,
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackSquare => ERROR_DASH_BARRED_BLACK_SQUARE,
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteDiamond => ERROR_DASH_BARRED_WHITE_DIAMOND,
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackDiamond => ERROR_DASH_BARRED_BLACK_DIAMOND,
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteCircle => ERROR_DASH_BARRED_WHITE_CIRCLE,
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackCircle => ERROR_DASH_BARRED_BLACK_CIRCLE,
            MiscellaneousMathematicalSymbolsB::RuleDashDelayed => RULE_DASH_DELAYED,
            MiscellaneousMathematicalSymbolsB::ReverseSolidusOperator => REVERSE_SOLIDUS_OPERATOR,
            MiscellaneousMathematicalSymbolsB::SolidusWithOverbar => SOLIDUS_WITH_OVERBAR,
            MiscellaneousMathematicalSymbolsB::ReverseSolidusWithHorizontalStroke => REVERSE_SOLIDUS_WITH_HORIZONTAL_STROKE,
            MiscellaneousMathematicalSymbolsB::BigSolidus => BIG_SOLIDUS,
            MiscellaneousMathematicalSymbolsB::BigReverseSolidus => BIG_REVERSE_SOLIDUS,
            MiscellaneousMathematicalSymbolsB::DoublePlus => DOUBLE_PLUS,
            MiscellaneousMathematicalSymbolsB::TriplePlus => TRIPLE_PLUS,
            MiscellaneousMathematicalSymbolsB::LeftDashPointingCurvedAngleBracket => LEFT_DASH_POINTING_CURVED_ANGLE_BRACKET,
            MiscellaneousMathematicalSymbolsB::RightDashPointingCurvedAngleBracket => RIGHT_DASH_POINTING_CURVED_ANGLE_BRACKET,
            MiscellaneousMathematicalSymbolsB::Tiny => TINY,
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousMathematicalSymbolsB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TRIPLE_VERTICAL_BAR_DELIMITER => Ok(MiscellaneousMathematicalSymbolsB::TripleVerticalBarDelimiter),
            Z_NOTATION_SPOT => Ok(MiscellaneousMathematicalSymbolsB::ZNotationSpot),
            Z_NOTATION_TYPE_COLON => Ok(MiscellaneousMathematicalSymbolsB::ZNotationTypeColon),
            LEFT_WHITE_CURLY_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::LeftWhiteCurlyBracket),
            RIGHT_WHITE_CURLY_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::RightWhiteCurlyBracket),
            LEFT_WHITE_PARENTHESIS => Ok(MiscellaneousMathematicalSymbolsB::LeftWhiteParenthesis),
            RIGHT_WHITE_PARENTHESIS => Ok(MiscellaneousMathematicalSymbolsB::RightWhiteParenthesis),
            Z_NOTATION_LEFT_IMAGE_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::ZNotationLeftImageBracket),
            Z_NOTATION_RIGHT_IMAGE_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::ZNotationRightImageBracket),
            Z_NOTATION_LEFT_BINDING_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::ZNotationLeftBindingBracket),
            Z_NOTATION_RIGHT_BINDING_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::ZNotationRightBindingBracket),
            LEFT_SQUARE_BRACKET_WITH_UNDERBAR => Ok(MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithUnderbar),
            RIGHT_SQUARE_BRACKET_WITH_UNDERBAR => Ok(MiscellaneousMathematicalSymbolsB::RightSquareBracketWithUnderbar),
            LEFT_SQUARE_BRACKET_WITH_TICK_IN_TOP_CORNER => Ok(MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInTopCorner),
            RIGHT_SQUARE_BRACKET_WITH_TICK_IN_BOTTOM_CORNER => Ok(MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInBottomCorner),
            LEFT_SQUARE_BRACKET_WITH_TICK_IN_BOTTOM_CORNER => Ok(MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInBottomCorner),
            RIGHT_SQUARE_BRACKET_WITH_TICK_IN_TOP_CORNER => Ok(MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInTopCorner),
            LEFT_ANGLE_BRACKET_WITH_DOT => Ok(MiscellaneousMathematicalSymbolsB::LeftAngleBracketWithDot),
            RIGHT_ANGLE_BRACKET_WITH_DOT => Ok(MiscellaneousMathematicalSymbolsB::RightAngleBracketWithDot),
            LEFT_ARC_LESS_DASH_THAN_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::LeftArcLessDashThanBracket),
            RIGHT_ARC_GREATER_DASH_THAN_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::RightArcGreaterDashThanBracket),
            DOUBLE_LEFT_ARC_GREATER_DASH_THAN_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::DoubleLeftArcGreaterDashThanBracket),
            DOUBLE_RIGHT_ARC_LESS_DASH_THAN_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::DoubleRightArcLessDashThanBracket),
            LEFT_BLACK_TORTOISE_SHELL_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::LeftBlackTortoiseShellBracket),
            RIGHT_BLACK_TORTOISE_SHELL_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::RightBlackTortoiseShellBracket),
            DOTTED_FENCE => Ok(MiscellaneousMathematicalSymbolsB::DottedFence),
            VERTICAL_ZIGZAG_LINE => Ok(MiscellaneousMathematicalSymbolsB::VerticalZigzagLine),
            MEASURED_ANGLE_OPENING_LEFT => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleOpeningLeft),
            RIGHT_ANGLE_VARIANT_WITH_SQUARE => Ok(MiscellaneousMathematicalSymbolsB::RightAngleVariantWithSquare),
            MEASURED_RIGHT_ANGLE_WITH_DOT => Ok(MiscellaneousMathematicalSymbolsB::MeasuredRightAngleWithDot),
            ANGLE_WITH_S_INSIDE => Ok(MiscellaneousMathematicalSymbolsB::AngleWithSInside),
            ACUTE_ANGLE => Ok(MiscellaneousMathematicalSymbolsB::AcuteAngle),
            SPHERICAL_ANGLE_OPENING_LEFT => Ok(MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningLeft),
            SPHERICAL_ANGLE_OPENING_UP => Ok(MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningUp),
            TURNED_ANGLE => Ok(MiscellaneousMathematicalSymbolsB::TurnedAngle),
            REVERSED_ANGLE => Ok(MiscellaneousMathematicalSymbolsB::ReversedAngle),
            ANGLE_WITH_UNDERBAR => Ok(MiscellaneousMathematicalSymbolsB::AngleWithUnderbar),
            REVERSED_ANGLE_WITH_UNDERBAR => Ok(MiscellaneousMathematicalSymbolsB::ReversedAngleWithUnderbar),
            OBLIQUE_ANGLE_OPENING_UP => Ok(MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningUp),
            OBLIQUE_ANGLE_OPENING_DOWN => Ok(MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningDown),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_UP_AND_RIGHT => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndRight),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_UP_AND_LEFT => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndLeft),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_DOWN_AND_RIGHT => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndRight),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_DOWN_AND_LEFT => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndLeft),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_RIGHT_AND_UP => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndUp),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_LEFT_AND_UP => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndUp),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_RIGHT_AND_DOWN => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndDown),
            MEASURED_ANGLE_WITH_OPEN_ARM_ENDING_IN_ARROW_POINTING_LEFT_AND_DOWN => Ok(MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndDown),
            REVERSED_EMPTY_SET => Ok(MiscellaneousMathematicalSymbolsB::ReversedEmptySet),
            EMPTY_SET_WITH_OVERBAR => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithOverbar),
            EMPTY_SET_WITH_SMALL_CIRCLE_ABOVE => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithSmallCircleAbove),
            EMPTY_SET_WITH_RIGHT_ARROW_ABOVE => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithRightArrowAbove),
            EMPTY_SET_WITH_LEFT_ARROW_ABOVE => Ok(MiscellaneousMathematicalSymbolsB::EmptySetWithLeftArrowAbove),
            CIRCLE_WITH_HORIZONTAL_BAR => Ok(MiscellaneousMathematicalSymbolsB::CircleWithHorizontalBar),
            CIRCLED_VERTICAL_BAR => Ok(MiscellaneousMathematicalSymbolsB::CircledVerticalBar),
            CIRCLED_PARALLEL => Ok(MiscellaneousMathematicalSymbolsB::CircledParallel),
            CIRCLED_REVERSE_SOLIDUS => Ok(MiscellaneousMathematicalSymbolsB::CircledReverseSolidus),
            CIRCLED_PERPENDICULAR => Ok(MiscellaneousMathematicalSymbolsB::CircledPerpendicular),
            CIRCLE_DIVIDED_BY_HORIZONTAL_BAR_AND_TOP_HALF_DIVIDED_BY_VERTICAL_BAR => Ok(MiscellaneousMathematicalSymbolsB::CircleDividedByHorizontalBarAndTopHalfDividedByVerticalBar),
            CIRCLE_WITH_SUPERIMPOSED_X => Ok(MiscellaneousMathematicalSymbolsB::CircleWithSuperimposedX),
            CIRCLED_ANTICLOCKWISE_DASH_ROTATED_DIVISION_SIGN => Ok(MiscellaneousMathematicalSymbolsB::CircledAnticlockwiseDashRotatedDivisionSign),
            UP_ARROW_THROUGH_CIRCLE => Ok(MiscellaneousMathematicalSymbolsB::UpArrowThroughCircle),
            CIRCLED_WHITE_BULLET => Ok(MiscellaneousMathematicalSymbolsB::CircledWhiteBullet),
            CIRCLED_BULLET => Ok(MiscellaneousMathematicalSymbolsB::CircledBullet),
            CIRCLED_LESS_DASH_THAN => Ok(MiscellaneousMathematicalSymbolsB::CircledLessDashThan),
            CIRCLED_GREATER_DASH_THAN => Ok(MiscellaneousMathematicalSymbolsB::CircledGreaterDashThan),
            CIRCLE_WITH_SMALL_CIRCLE_TO_THE_RIGHT => Ok(MiscellaneousMathematicalSymbolsB::CircleWithSmallCircleToTheRight),
            CIRCLE_WITH_TWO_HORIZONTAL_STROKES_TO_THE_RIGHT => Ok(MiscellaneousMathematicalSymbolsB::CircleWithTwoHorizontalStrokesToTheRight),
            SQUARED_RISING_DIAGONAL_SLASH => Ok(MiscellaneousMathematicalSymbolsB::SquaredRisingDiagonalSlash),
            SQUARED_FALLING_DIAGONAL_SLASH => Ok(MiscellaneousMathematicalSymbolsB::SquaredFallingDiagonalSlash),
            SQUARED_ASTERISK => Ok(MiscellaneousMathematicalSymbolsB::SquaredAsterisk),
            SQUARED_SMALL_CIRCLE => Ok(MiscellaneousMathematicalSymbolsB::SquaredSmallCircle),
            SQUARED_SQUARE => Ok(MiscellaneousMathematicalSymbolsB::SquaredSquare),
            TWO_JOINED_SQUARES => Ok(MiscellaneousMathematicalSymbolsB::TwoJoinedSquares),
            TRIANGLE_WITH_DOT_ABOVE => Ok(MiscellaneousMathematicalSymbolsB::TriangleWithDotAbove),
            TRIANGLE_WITH_UNDERBAR => Ok(MiscellaneousMathematicalSymbolsB::TriangleWithUnderbar),
            S_IN_TRIANGLE => Ok(MiscellaneousMathematicalSymbolsB::SInTriangle),
            TRIANGLE_WITH_SERIFS_AT_BOTTOM => Ok(MiscellaneousMathematicalSymbolsB::TriangleWithSerifsAtBottom),
            RIGHT_TRIANGLE_ABOVE_LEFT_TRIANGLE => Ok(MiscellaneousMathematicalSymbolsB::RightTriangleAboveLeftTriangle),
            LEFT_TRIANGLE_BESIDE_VERTICAL_BAR => Ok(MiscellaneousMathematicalSymbolsB::LeftTriangleBesideVerticalBar),
            VERTICAL_BAR_BESIDE_RIGHT_TRIANGLE => Ok(MiscellaneousMathematicalSymbolsB::VerticalBarBesideRightTriangle),
            BOWTIE_WITH_LEFT_HALF_BLACK => Ok(MiscellaneousMathematicalSymbolsB::BowtieWithLeftHalfBlack),
            BOWTIE_WITH_RIGHT_HALF_BLACK => Ok(MiscellaneousMathematicalSymbolsB::BowtieWithRightHalfBlack),
            BLACK_BOWTIE => Ok(MiscellaneousMathematicalSymbolsB::BlackBowtie),
            TIMES_WITH_LEFT_HALF_BLACK => Ok(MiscellaneousMathematicalSymbolsB::TimesWithLeftHalfBlack),
            TIMES_WITH_RIGHT_HALF_BLACK => Ok(MiscellaneousMathematicalSymbolsB::TimesWithRightHalfBlack),
            WHITE_HOURGLASS => Ok(MiscellaneousMathematicalSymbolsB::WhiteHourglass),
            BLACK_HOURGLASS => Ok(MiscellaneousMathematicalSymbolsB::BlackHourglass),
            LEFT_WIGGLY_FENCE => Ok(MiscellaneousMathematicalSymbolsB::LeftWigglyFence),
            RIGHT_WIGGLY_FENCE => Ok(MiscellaneousMathematicalSymbolsB::RightWigglyFence),
            LEFT_DOUBLE_WIGGLY_FENCE => Ok(MiscellaneousMathematicalSymbolsB::LeftDoubleWigglyFence),
            RIGHT_DOUBLE_WIGGLY_FENCE => Ok(MiscellaneousMathematicalSymbolsB::RightDoubleWigglyFence),
            INCOMPLETE_INFINITY => Ok(MiscellaneousMathematicalSymbolsB::IncompleteInfinity),
            TIE_OVER_INFINITY => Ok(MiscellaneousMathematicalSymbolsB::TieOverInfinity),
            INFINITY_NEGATED_WITH_VERTICAL_BAR => Ok(MiscellaneousMathematicalSymbolsB::InfinityNegatedWithVerticalBar),
            DOUBLE_DASH_ENDED_MULTIMAP => Ok(MiscellaneousMathematicalSymbolsB::DoubleDashEndedMultimap),
            SQUARE_WITH_CONTOURED_OUTLINE => Ok(MiscellaneousMathematicalSymbolsB::SquareWithContouredOutline),
            INCREASES_AS => Ok(MiscellaneousMathematicalSymbolsB::IncreasesAs),
            SHUFFLE_PRODUCT => Ok(MiscellaneousMathematicalSymbolsB::ShuffleProduct),
            EQUALS_SIGN_AND_SLANTED_PARALLEL => Ok(MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallel),
            EQUALS_SIGN_AND_SLANTED_PARALLEL_WITH_TILDE_ABOVE => Ok(MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallelWithTildeAbove),
            IDENTICAL_TO_AND_SLANTED_PARALLEL => Ok(MiscellaneousMathematicalSymbolsB::IdenticalToAndSlantedParallel),
            GLEICH_STARK => Ok(MiscellaneousMathematicalSymbolsB::GleichStark),
            THERMODYNAMIC => Ok(MiscellaneousMathematicalSymbolsB::Thermodynamic),
            DOWN_DASH_POINTING_TRIANGLE_WITH_LEFT_HALF_BLACK => Ok(MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithLeftHalfBlack),
            DOWN_DASH_POINTING_TRIANGLE_WITH_RIGHT_HALF_BLACK => Ok(MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithRightHalfBlack),
            BLACK_DIAMOND_WITH_DOWN_ARROW => Ok(MiscellaneousMathematicalSymbolsB::BlackDiamondWithDownArrow),
            BLACK_LOZENGE => Ok(MiscellaneousMathematicalSymbolsB::BlackLozenge),
            WHITE_CIRCLE_WITH_DOWN_ARROW => Ok(MiscellaneousMathematicalSymbolsB::WhiteCircleWithDownArrow),
            BLACK_CIRCLE_WITH_DOWN_ARROW => Ok(MiscellaneousMathematicalSymbolsB::BlackCircleWithDownArrow),
            ERROR_DASH_BARRED_WHITE_SQUARE => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteSquare),
            ERROR_DASH_BARRED_BLACK_SQUARE => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackSquare),
            ERROR_DASH_BARRED_WHITE_DIAMOND => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteDiamond),
            ERROR_DASH_BARRED_BLACK_DIAMOND => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackDiamond),
            ERROR_DASH_BARRED_WHITE_CIRCLE => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteCircle),
            ERROR_DASH_BARRED_BLACK_CIRCLE => Ok(MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackCircle),
            RULE_DASH_DELAYED => Ok(MiscellaneousMathematicalSymbolsB::RuleDashDelayed),
            REVERSE_SOLIDUS_OPERATOR => Ok(MiscellaneousMathematicalSymbolsB::ReverseSolidusOperator),
            SOLIDUS_WITH_OVERBAR => Ok(MiscellaneousMathematicalSymbolsB::SolidusWithOverbar),
            REVERSE_SOLIDUS_WITH_HORIZONTAL_STROKE => Ok(MiscellaneousMathematicalSymbolsB::ReverseSolidusWithHorizontalStroke),
            BIG_SOLIDUS => Ok(MiscellaneousMathematicalSymbolsB::BigSolidus),
            BIG_REVERSE_SOLIDUS => Ok(MiscellaneousMathematicalSymbolsB::BigReverseSolidus),
            DOUBLE_PLUS => Ok(MiscellaneousMathematicalSymbolsB::DoublePlus),
            TRIPLE_PLUS => Ok(MiscellaneousMathematicalSymbolsB::TriplePlus),
            LEFT_DASH_POINTING_CURVED_ANGLE_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::LeftDashPointingCurvedAngleBracket),
            RIGHT_DASH_POINTING_CURVED_ANGLE_BRACKET => Ok(MiscellaneousMathematicalSymbolsB::RightDashPointingCurvedAngleBracket),
            TINY => Ok(MiscellaneousMathematicalSymbolsB::Tiny),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MiscellaneousMathematicalSymbolsB::TripleVerticalBarDelimiter => "triple vertical bar delimiter",
            MiscellaneousMathematicalSymbolsB::ZNotationSpot => "z notation spot",
            MiscellaneousMathematicalSymbolsB::ZNotationTypeColon => "z notation type colon",
            MiscellaneousMathematicalSymbolsB::LeftWhiteCurlyBracket => "left white curly bracket",
            MiscellaneousMathematicalSymbolsB::RightWhiteCurlyBracket => "right white curly bracket",
            MiscellaneousMathematicalSymbolsB::LeftWhiteParenthesis => "left white parenthesis",
            MiscellaneousMathematicalSymbolsB::RightWhiteParenthesis => "right white parenthesis",
            MiscellaneousMathematicalSymbolsB::ZNotationLeftImageBracket => "z notation left image bracket",
            MiscellaneousMathematicalSymbolsB::ZNotationRightImageBracket => "z notation right image bracket",
            MiscellaneousMathematicalSymbolsB::ZNotationLeftBindingBracket => "z notation left binding bracket",
            MiscellaneousMathematicalSymbolsB::ZNotationRightBindingBracket => "z notation right binding bracket",
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithUnderbar => "left square bracket with underbar",
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithUnderbar => "right square bracket with underbar",
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInTopCorner => "left square bracket with tick in top corner",
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInBottomCorner => "right square bracket with tick in bottom corner",
            MiscellaneousMathematicalSymbolsB::LeftSquareBracketWithTickInBottomCorner => "left square bracket with tick in bottom corner",
            MiscellaneousMathematicalSymbolsB::RightSquareBracketWithTickInTopCorner => "right square bracket with tick in top corner",
            MiscellaneousMathematicalSymbolsB::LeftAngleBracketWithDot => "left angle bracket with dot",
            MiscellaneousMathematicalSymbolsB::RightAngleBracketWithDot => "right angle bracket with dot",
            MiscellaneousMathematicalSymbolsB::LeftArcLessDashThanBracket => "left arc less-than bracket",
            MiscellaneousMathematicalSymbolsB::RightArcGreaterDashThanBracket => "right arc greater-than bracket",
            MiscellaneousMathematicalSymbolsB::DoubleLeftArcGreaterDashThanBracket => "double left arc greater-than bracket",
            MiscellaneousMathematicalSymbolsB::DoubleRightArcLessDashThanBracket => "double right arc less-than bracket",
            MiscellaneousMathematicalSymbolsB::LeftBlackTortoiseShellBracket => "left black tortoise shell bracket",
            MiscellaneousMathematicalSymbolsB::RightBlackTortoiseShellBracket => "right black tortoise shell bracket",
            MiscellaneousMathematicalSymbolsB::DottedFence => "dotted fence",
            MiscellaneousMathematicalSymbolsB::VerticalZigzagLine => "vertical zigzag line",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleOpeningLeft => "measured angle opening left",
            MiscellaneousMathematicalSymbolsB::RightAngleVariantWithSquare => "right angle variant with square",
            MiscellaneousMathematicalSymbolsB::MeasuredRightAngleWithDot => "measured right angle with dot",
            MiscellaneousMathematicalSymbolsB::AngleWithSInside => "angle with s inside",
            MiscellaneousMathematicalSymbolsB::AcuteAngle => "acute angle",
            MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningLeft => "spherical angle opening left",
            MiscellaneousMathematicalSymbolsB::SphericalAngleOpeningUp => "spherical angle opening up",
            MiscellaneousMathematicalSymbolsB::TurnedAngle => "turned angle",
            MiscellaneousMathematicalSymbolsB::ReversedAngle => "reversed angle",
            MiscellaneousMathematicalSymbolsB::AngleWithUnderbar => "angle with underbar",
            MiscellaneousMathematicalSymbolsB::ReversedAngleWithUnderbar => "reversed angle with underbar",
            MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningUp => "oblique angle opening up",
            MiscellaneousMathematicalSymbolsB::ObliqueAngleOpeningDown => "oblique angle opening down",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndRight => "measured angle with open arm ending in arrow pointing up and right",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingUpAndLeft => "measured angle with open arm ending in arrow pointing up and left",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndRight => "measured angle with open arm ending in arrow pointing down and right",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingDownAndLeft => "measured angle with open arm ending in arrow pointing down and left",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndUp => "measured angle with open arm ending in arrow pointing right and up",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndUp => "measured angle with open arm ending in arrow pointing left and up",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingRightAndDown => "measured angle with open arm ending in arrow pointing right and down",
            MiscellaneousMathematicalSymbolsB::MeasuredAngleWithOpenArmEndingInArrowPointingLeftAndDown => "measured angle with open arm ending in arrow pointing left and down",
            MiscellaneousMathematicalSymbolsB::ReversedEmptySet => "reversed empty set",
            MiscellaneousMathematicalSymbolsB::EmptySetWithOverbar => "empty set with overbar",
            MiscellaneousMathematicalSymbolsB::EmptySetWithSmallCircleAbove => "empty set with small circle above",
            MiscellaneousMathematicalSymbolsB::EmptySetWithRightArrowAbove => "empty set with right arrow above",
            MiscellaneousMathematicalSymbolsB::EmptySetWithLeftArrowAbove => "empty set with left arrow above",
            MiscellaneousMathematicalSymbolsB::CircleWithHorizontalBar => "circle with horizontal bar",
            MiscellaneousMathematicalSymbolsB::CircledVerticalBar => "circled vertical bar",
            MiscellaneousMathematicalSymbolsB::CircledParallel => "circled parallel",
            MiscellaneousMathematicalSymbolsB::CircledReverseSolidus => "circled reverse solidus",
            MiscellaneousMathematicalSymbolsB::CircledPerpendicular => "circled perpendicular",
            MiscellaneousMathematicalSymbolsB::CircleDividedByHorizontalBarAndTopHalfDividedByVerticalBar => "circle divided by horizontal bar and top half divided by vertical bar",
            MiscellaneousMathematicalSymbolsB::CircleWithSuperimposedX => "circle with superimposed x",
            MiscellaneousMathematicalSymbolsB::CircledAnticlockwiseDashRotatedDivisionSign => "circled anticlockwise-rotated division sign",
            MiscellaneousMathematicalSymbolsB::UpArrowThroughCircle => "up arrow through circle",
            MiscellaneousMathematicalSymbolsB::CircledWhiteBullet => "circled white bullet",
            MiscellaneousMathematicalSymbolsB::CircledBullet => "circled bullet",
            MiscellaneousMathematicalSymbolsB::CircledLessDashThan => "circled less-than",
            MiscellaneousMathematicalSymbolsB::CircledGreaterDashThan => "circled greater-than",
            MiscellaneousMathematicalSymbolsB::CircleWithSmallCircleToTheRight => "circle with small circle to the right",
            MiscellaneousMathematicalSymbolsB::CircleWithTwoHorizontalStrokesToTheRight => "circle with two horizontal strokes to the right",
            MiscellaneousMathematicalSymbolsB::SquaredRisingDiagonalSlash => "squared rising diagonal slash",
            MiscellaneousMathematicalSymbolsB::SquaredFallingDiagonalSlash => "squared falling diagonal slash",
            MiscellaneousMathematicalSymbolsB::SquaredAsterisk => "squared asterisk",
            MiscellaneousMathematicalSymbolsB::SquaredSmallCircle => "squared small circle",
            MiscellaneousMathematicalSymbolsB::SquaredSquare => "squared square",
            MiscellaneousMathematicalSymbolsB::TwoJoinedSquares => "two joined squares",
            MiscellaneousMathematicalSymbolsB::TriangleWithDotAbove => "triangle with dot above",
            MiscellaneousMathematicalSymbolsB::TriangleWithUnderbar => "triangle with underbar",
            MiscellaneousMathematicalSymbolsB::SInTriangle => "s in triangle",
            MiscellaneousMathematicalSymbolsB::TriangleWithSerifsAtBottom => "triangle with serifs at bottom",
            MiscellaneousMathematicalSymbolsB::RightTriangleAboveLeftTriangle => "right triangle above left triangle",
            MiscellaneousMathematicalSymbolsB::LeftTriangleBesideVerticalBar => "left triangle beside vertical bar",
            MiscellaneousMathematicalSymbolsB::VerticalBarBesideRightTriangle => "vertical bar beside right triangle",
            MiscellaneousMathematicalSymbolsB::BowtieWithLeftHalfBlack => "bowtie with left half black",
            MiscellaneousMathematicalSymbolsB::BowtieWithRightHalfBlack => "bowtie with right half black",
            MiscellaneousMathematicalSymbolsB::BlackBowtie => "black bowtie",
            MiscellaneousMathematicalSymbolsB::TimesWithLeftHalfBlack => "times with left half black",
            MiscellaneousMathematicalSymbolsB::TimesWithRightHalfBlack => "times with right half black",
            MiscellaneousMathematicalSymbolsB::WhiteHourglass => "white hourglass",
            MiscellaneousMathematicalSymbolsB::BlackHourglass => "black hourglass",
            MiscellaneousMathematicalSymbolsB::LeftWigglyFence => "left wiggly fence",
            MiscellaneousMathematicalSymbolsB::RightWigglyFence => "right wiggly fence",
            MiscellaneousMathematicalSymbolsB::LeftDoubleWigglyFence => "left double wiggly fence",
            MiscellaneousMathematicalSymbolsB::RightDoubleWigglyFence => "right double wiggly fence",
            MiscellaneousMathematicalSymbolsB::IncompleteInfinity => "incomplete infinity",
            MiscellaneousMathematicalSymbolsB::TieOverInfinity => "tie over infinity",
            MiscellaneousMathematicalSymbolsB::InfinityNegatedWithVerticalBar => "infinity negated with vertical bar",
            MiscellaneousMathematicalSymbolsB::DoubleDashEndedMultimap => "double-ended multimap",
            MiscellaneousMathematicalSymbolsB::SquareWithContouredOutline => "square with contoured outline",
            MiscellaneousMathematicalSymbolsB::IncreasesAs => "increases as",
            MiscellaneousMathematicalSymbolsB::ShuffleProduct => "shuffle product",
            MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallel => "equals sign and slanted parallel",
            MiscellaneousMathematicalSymbolsB::EqualsSignAndSlantedParallelWithTildeAbove => "equals sign and slanted parallel with tilde above",
            MiscellaneousMathematicalSymbolsB::IdenticalToAndSlantedParallel => "identical to and slanted parallel",
            MiscellaneousMathematicalSymbolsB::GleichStark => "gleich stark",
            MiscellaneousMathematicalSymbolsB::Thermodynamic => "thermodynamic",
            MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithLeftHalfBlack => "down-pointing triangle with left half black",
            MiscellaneousMathematicalSymbolsB::DownDashPointingTriangleWithRightHalfBlack => "down-pointing triangle with right half black",
            MiscellaneousMathematicalSymbolsB::BlackDiamondWithDownArrow => "black diamond with down arrow",
            MiscellaneousMathematicalSymbolsB::BlackLozenge => "black lozenge",
            MiscellaneousMathematicalSymbolsB::WhiteCircleWithDownArrow => "white circle with down arrow",
            MiscellaneousMathematicalSymbolsB::BlackCircleWithDownArrow => "black circle with down arrow",
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteSquare => "error-barred white square",
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackSquare => "error-barred black square",
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteDiamond => "error-barred white diamond",
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackDiamond => "error-barred black diamond",
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredWhiteCircle => "error-barred white circle",
            MiscellaneousMathematicalSymbolsB::ErrorDashBarredBlackCircle => "error-barred black circle",
            MiscellaneousMathematicalSymbolsB::RuleDashDelayed => "rule-delayed",
            MiscellaneousMathematicalSymbolsB::ReverseSolidusOperator => "reverse solidus operator",
            MiscellaneousMathematicalSymbolsB::SolidusWithOverbar => "solidus with overbar",
            MiscellaneousMathematicalSymbolsB::ReverseSolidusWithHorizontalStroke => "reverse solidus with horizontal stroke",
            MiscellaneousMathematicalSymbolsB::BigSolidus => "big solidus",
            MiscellaneousMathematicalSymbolsB::BigReverseSolidus => "big reverse solidus",
            MiscellaneousMathematicalSymbolsB::DoublePlus => "double plus",
            MiscellaneousMathematicalSymbolsB::TriplePlus => "triple plus",
            MiscellaneousMathematicalSymbolsB::LeftDashPointingCurvedAngleBracket => "left-pointing curved angle bracket",
            MiscellaneousMathematicalSymbolsB::RightDashPointingCurvedAngleBracket => "right-pointing curved angle bracket",
            MiscellaneousMathematicalSymbolsB::Tiny => "tiny",
        }
    }
}
