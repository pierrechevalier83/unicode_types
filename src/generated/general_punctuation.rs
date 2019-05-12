/// \u{2000} → \u{206f}\
///\
///                       ​ ‌ ‍ ‎ ‏\
/// ‐ ‑ ‒ – — ― ‖ ‗ ‘ ’ ‚ ‛ “ ” „ ‟\
/// † ‡ • ‣ ․ ‥ … ‧     ‪ ‫ ‬ ‭ ‮  \
/// ‰ ‱ ′ ″ ‴ ‵ ‶ ‷ ‸ ‹ › ※ ‼ ‽ ‾ ‿\
/// ⁀ ⁁ ⁂ ⁃ ⁄ ⁅ ⁆ ⁇ ⁈ ⁉ ⁊ ⁋ ⁌ ⁍ ⁎ ⁏\
/// ⁐ ⁑ ⁒ ⁓ ⁔ ⁕ ⁖ ⁗ ⁘ ⁙ ⁚ ⁛ ⁜ ⁝ ⁞  \
/// ⁠ ⁡ ⁢ ⁣ ⁤ ⁦ ⁧ ⁨ ⁩ ⁪ ⁫ ⁬ ⁭ ⁮\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2000}: ' '
    pub const EN_QUAD: char = ' ';
    /// \u{2001}: ' '
    pub const EM_QUAD: char = ' ';
    /// \u{2002}: ' '
    pub const EN_SPACE: char = ' ';
    /// \u{2003}: ' '
    pub const EM_SPACE: char = ' ';
    /// \u{2004}: ' '
    pub const THREE_DASH_PER_DASH_EM_SPACE: char = ' ';
    /// \u{2005}: ' '
    pub const FOUR_DASH_PER_DASH_EM_SPACE: char = ' ';
    /// \u{2006}: ' '
    pub const SIX_DASH_PER_DASH_EM_SPACE: char = ' ';
    /// \u{2007}: ' '
    pub const FIGURE_SPACE: char = ' ';
    /// \u{2008}: ' '
    pub const PUNCTUATION_SPACE: char = ' ';
    /// \u{2009}: ' '
    pub const THIN_SPACE: char = ' ';
    /// \u{200a}: ' '
    pub const HAIR_SPACE: char = ' ';
    /// \u{200b}: '​'
    pub const ZERO_WIDTH_SPACE: char = '​';
    /// \u{200c}: '‌'
    pub const ZERO_WIDTH_NON_DASH_JOINER: char = '‌';
    /// \u{200d}: '‍'
    pub const ZERO_WIDTH_JOINER: char = '‍';
    /// \u{200e}: '‎'
    pub const LEFT_DASH_TO_DASH_RIGHT_MARK: char = '‎';
    /// \u{200f}: '‏'
    pub const RIGHT_DASH_TO_DASH_LEFT_MARK: char = '‏';
    /// \u{2010}: '‐'
    pub const HYPHEN: char = '‐';
    /// \u{2011}: '‑'
    pub const NON_DASH_BREAKING_HYPHEN: char = '‑';
    /// \u{2012}: '‒'
    pub const FIGURE_DASH: char = '‒';
    /// \u{2013}: '–'
    pub const EN_DASH: char = '–';
    /// \u{2014}: '—'
    pub const EM_DASH: char = '—';
    /// \u{2015}: '―'
    pub const HORIZONTAL_BAR: char = '―';
    /// \u{2016}: '‖'
    pub const DOUBLE_VERTICAL_LINE: char = '‖';
    /// \u{2017}: '‗'
    pub const DOUBLE_LOW_LINE: char = '‗';
    /// \u{2018}: '‘'
    pub const LEFT_SINGLE_QUOTATION_MARK: char = '‘';
    /// \u{2019}: '’'
    pub const RIGHT_SINGLE_QUOTATION_MARK: char = '’';
    /// \u{201a}: '‚'
    pub const SINGLE_LOW_DASH_9_QUOTATION_MARK: char = '‚';
    /// \u{201b}: '‛'
    pub const SINGLE_HIGH_DASH_REVERSED_DASH_9_QUOTATION_MARK: char = '‛';
    /// \u{201c}: '“'
    pub const LEFT_DOUBLE_QUOTATION_MARK: char = '“';
    /// \u{201d}: '”'
    pub const RIGHT_DOUBLE_QUOTATION_MARK: char = '”';
    /// \u{201e}: '„'
    pub const DOUBLE_LOW_DASH_9_QUOTATION_MARK: char = '„';
    /// \u{201f}: '‟'
    pub const DOUBLE_HIGH_DASH_REVERSED_DASH_9_QUOTATION_MARK: char = '‟';
    /// \u{2020}: '†'
    pub const DAGGER: char = '†';
    /// \u{2021}: '‡'
    pub const DOUBLE_DAGGER: char = '‡';
    /// \u{2022}: '•'
    pub const BULLET: char = '•';
    /// \u{2023}: '‣'
    pub const TRIANGULAR_BULLET: char = '‣';
    /// \u{2024}: '․'
    pub const ONE_DOT_LEADER: char = '․';
    /// \u{2025}: '‥'
    pub const TWO_DOT_LEADER: char = '‥';
    /// \u{2026}: '…'
    pub const HORIZONTAL_ELLIPSIS: char = '…';
    /// \u{2027}: '‧'
    pub const HYPHENATION_POINT: char = '‧';
    /// \u{2028}: ' '
    pub const LINE_SEPARATOR: char = ' ';
    /// \u{2029}: ' '
    pub const PARAGRAPH_SEPARATOR: char = ' ';
    /// \u{202a}: '‪'
    pub const LEFT_DASH_TO_DASH_RIGHT_EMBEDDING: char = '‪';
    /// \u{202b}: '‫'
    pub const RIGHT_DASH_TO_DASH_LEFT_EMBEDDING: char = '‫';
    /// \u{202c}: '‬'
    pub const POP_DIRECTIONAL_FORMATTING: char = '‬';
    /// \u{202d}: '‭'
    pub const LEFT_DASH_TO_DASH_RIGHT_OVERRIDE: char = '‭';
    /// \u{202e}: '‮'
    pub const RIGHT_DASH_TO_DASH_LEFT_OVERRIDE: char = '‮';
    /// \u{202f}: ' '
    pub const NARROW_NO_DASH_BREAK_SPACE: char = ' ';
    /// \u{2030}: '‰'
    pub const PER_MILLE_SIGN: char = '‰';
    /// \u{2031}: '‱'
    pub const PER_TEN_THOUSAND_SIGN: char = '‱';
    /// \u{2032}: '′'
    pub const PRIME: char = '′';
    /// \u{2033}: '″'
    pub const DOUBLE_PRIME: char = '″';
    /// \u{2034}: '‴'
    pub const TRIPLE_PRIME: char = '‴';
    /// \u{2035}: '‵'
    pub const REVERSED_PRIME: char = '‵';
    /// \u{2036}: '‶'
    pub const REVERSED_DOUBLE_PRIME: char = '‶';
    /// \u{2037}: '‷'
    pub const REVERSED_TRIPLE_PRIME: char = '‷';
    /// \u{2038}: '‸'
    pub const CARET: char = '‸';
    /// \u{2039}: '‹'
    pub const SINGLE_LEFT_DASH_POINTING_ANGLE_QUOTATION_MARK: char = '‹';
    /// \u{203a}: '›'
    pub const SINGLE_RIGHT_DASH_POINTING_ANGLE_QUOTATION_MARK: char = '›';
    /// \u{203b}: '※'
    pub const REFERENCE_MARK: char = '※';
    /// \u{203c}: '‼'
    pub const DOUBLE_EXCLAMATION_MARK: char = '‼';
    /// \u{203d}: '‽'
    pub const INTERROBANG: char = '‽';
    /// \u{203e}: '‾'
    pub const OVERLINE: char = '‾';
    /// \u{203f}: '‿'
    pub const UNDERTIE: char = '‿';
    /// \u{2040}: '⁀'
    pub const CHARACTER_TIE: char = '⁀';
    /// \u{2041}: '⁁'
    pub const CARET_INSERTION_POINT: char = '⁁';
    /// \u{2042}: '⁂'
    pub const ASTERISM: char = '⁂';
    /// \u{2043}: '⁃'
    pub const HYPHEN_BULLET: char = '⁃';
    /// \u{2044}: '⁄'
    pub const FRACTION_SLASH: char = '⁄';
    /// \u{2045}: '⁅'
    pub const LEFT_SQUARE_BRACKET_WITH_QUILL: char = '⁅';
    /// \u{2046}: '⁆'
    pub const RIGHT_SQUARE_BRACKET_WITH_QUILL: char = '⁆';
    /// \u{2047}: '⁇'
    pub const DOUBLE_QUESTION_MARK: char = '⁇';
    /// \u{2048}: '⁈'
    pub const QUESTION_EXCLAMATION_MARK: char = '⁈';
    /// \u{2049}: '⁉'
    pub const EXCLAMATION_QUESTION_MARK: char = '⁉';
    /// \u{204a}: '⁊'
    pub const TIRONIAN_SIGN_ET: char = '⁊';
    /// \u{204b}: '⁋'
    pub const REVERSED_PILCROW_SIGN: char = '⁋';
    /// \u{204c}: '⁌'
    pub const BLACK_LEFTWARDS_BULLET: char = '⁌';
    /// \u{204d}: '⁍'
    pub const BLACK_RIGHTWARDS_BULLET: char = '⁍';
    /// \u{204e}: '⁎'
    pub const LOW_ASTERISK: char = '⁎';
    /// \u{204f}: '⁏'
    pub const REVERSED_SEMICOLON: char = '⁏';
    /// \u{2050}: '⁐'
    pub const CLOSE_UP: char = '⁐';
    /// \u{2051}: '⁑'
    pub const TWO_ASTERISKS_ALIGNED_VERTICALLY: char = '⁑';
    /// \u{2052}: '⁒'
    pub const COMMERCIAL_MINUS_SIGN: char = '⁒';
    /// \u{2053}: '⁓'
    pub const SWUNG_DASH: char = '⁓';
    /// \u{2054}: '⁔'
    pub const INVERTED_UNDERTIE: char = '⁔';
    /// \u{2055}: '⁕'
    pub const FLOWER_PUNCTUATION_MARK: char = '⁕';
    /// \u{2056}: '⁖'
    pub const THREE_DOT_PUNCTUATION: char = '⁖';
    /// \u{2057}: '⁗'
    pub const QUADRUPLE_PRIME: char = '⁗';
    /// \u{2058}: '⁘'
    pub const FOUR_DOT_PUNCTUATION: char = '⁘';
    /// \u{2059}: '⁙'
    pub const FIVE_DOT_PUNCTUATION: char = '⁙';
    /// \u{205a}: '⁚'
    pub const TWO_DOT_PUNCTUATION: char = '⁚';
    /// \u{205b}: '⁛'
    pub const FOUR_DOT_MARK: char = '⁛';
    /// \u{205c}: '⁜'
    pub const DOTTED_CROSS: char = '⁜';
    /// \u{205d}: '⁝'
    pub const TRICOLON: char = '⁝';
    /// \u{205e}: '⁞'
    pub const VERTICAL_FOUR_DOTS: char = '⁞';
    /// \u{205f}: ' '
    pub const MEDIUM_MATHEMATICAL_SPACE: char = ' ';
    /// \u{2060}: '⁠'
    pub const WORD_JOINER: char = '⁠';
    /// \u{2061}: '⁡'
    pub const FUNCTION_APPLICATION: char = '⁡';
    /// \u{2062}: '⁢'
    pub const INVISIBLE_TIMES: char = '⁢';
    /// \u{2063}: '⁣'
    pub const INVISIBLE_SEPARATOR: char = '⁣';
    /// \u{2064}: '⁤'
    pub const INVISIBLE_PLUS: char = '⁤';
    /// \u{2066}: '⁦'
    pub const LEFT_DASH_TO_DASH_RIGHT_ISOLATE: char = '⁦';
    /// \u{2067}: '⁧'
    pub const RIGHT_DASH_TO_DASH_LEFT_ISOLATE: char = '⁧';
    /// \u{2068}: '⁨'
    pub const FIRST_STRONG_ISOLATE: char = '⁨';
    /// \u{2069}: '⁩'
    pub const POP_DIRECTIONAL_ISOLATE: char = '⁩';
    /// \u{206a}: '⁪'
    pub const INHIBIT_SYMMETRIC_SWAPPING: char = '⁪';
    /// \u{206b}: '⁫'
    pub const ACTIVATE_SYMMETRIC_SWAPPING: char = '⁫';
    /// \u{206c}: '⁬'
    pub const INHIBIT_ARABIC_FORM_SHAPING: char = '⁬';
    /// \u{206d}: '⁭'
    pub const ACTIVATE_ARABIC_FORM_SHAPING: char = '⁭';
    /// \u{206e}: '⁮'
    pub const NATIONAL_DIGIT_SHAPES: char = '⁮';
}

/// An enum to represent all characters in the GeneralPunctuation block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GeneralPunctuation {
    /// \u{2000}: ' '
    EnQuad,
    /// \u{2001}: ' '
    EmQuad,
    /// \u{2002}: ' '
    EnSpace,
    /// \u{2003}: ' '
    EmSpace,
    /// \u{2004}: ' '
    ThreeDashPerDashEmSpace,
    /// \u{2005}: ' '
    FourDashPerDashEmSpace,
    /// \u{2006}: ' '
    SixDashPerDashEmSpace,
    /// \u{2007}: ' '
    FigureSpace,
    /// \u{2008}: ' '
    PunctuationSpace,
    /// \u{2009}: ' '
    ThinSpace,
    /// \u{200a}: ' '
    HairSpace,
    /// \u{200b}: '​'
    ZeroWidthSpace,
    /// \u{200c}: '‌'
    ZeroWidthNonDashJoiner,
    /// \u{200d}: '‍'
    ZeroWidthJoiner,
    /// \u{200e}: '‎'
    LeftDashToDashRightMark,
    /// \u{200f}: '‏'
    RightDashToDashLeftMark,
    /// \u{2010}: '‐'
    Hyphen,
    /// \u{2011}: '‑'
    NonDashBreakingHyphen,
    /// \u{2012}: '‒'
    FigureDash,
    /// \u{2013}: '–'
    EnDash,
    /// \u{2014}: '—'
    EmDash,
    /// \u{2015}: '―'
    HorizontalBar,
    /// \u{2016}: '‖'
    DoubleVerticalLine,
    /// \u{2017}: '‗'
    DoubleLowLine,
    /// \u{2018}: '‘'
    LeftSingleQuotationMark,
    /// \u{2019}: '’'
    RightSingleQuotationMark,
    /// \u{201a}: '‚'
    SingleLowDash9QuotationMark,
    /// \u{201b}: '‛'
    SingleHighDashReversedDash9QuotationMark,
    /// \u{201c}: '“'
    LeftDoubleQuotationMark,
    /// \u{201d}: '”'
    RightDoubleQuotationMark,
    /// \u{201e}: '„'
    DoubleLowDash9QuotationMark,
    /// \u{201f}: '‟'
    DoubleHighDashReversedDash9QuotationMark,
    /// \u{2020}: '†'
    Dagger,
    /// \u{2021}: '‡'
    DoubleDagger,
    /// \u{2022}: '•'
    Bullet,
    /// \u{2023}: '‣'
    TriangularBullet,
    /// \u{2024}: '․'
    OneDotLeader,
    /// \u{2025}: '‥'
    TwoDotLeader,
    /// \u{2026}: '…'
    HorizontalEllipsis,
    /// \u{2027}: '‧'
    HyphenationPoint,
    /// \u{2028}: ' '
    LineSeparator,
    /// \u{2029}: ' '
    ParagraphSeparator,
    /// \u{202a}: '‪'
    LeftDashToDashRightEmbedding,
    /// \u{202b}: '‫'
    RightDashToDashLeftEmbedding,
    /// \u{202c}: '‬'
    PopDirectionalFormatting,
    /// \u{202d}: '‭'
    LeftDashToDashRightOverride,
    /// \u{202e}: '‮'
    RightDashToDashLeftOverride,
    /// \u{202f}: ' '
    NarrowNoDashBreakSpace,
    /// \u{2030}: '‰'
    PerMilleSign,
    /// \u{2031}: '‱'
    PerTenThousandSign,
    /// \u{2032}: '′'
    Prime,
    /// \u{2033}: '″'
    DoublePrime,
    /// \u{2034}: '‴'
    TriplePrime,
    /// \u{2035}: '‵'
    ReversedPrime,
    /// \u{2036}: '‶'
    ReversedDoublePrime,
    /// \u{2037}: '‷'
    ReversedTriplePrime,
    /// \u{2038}: '‸'
    Caret,
    /// \u{2039}: '‹'
    SingleLeftDashPointingAngleQuotationMark,
    /// \u{203a}: '›'
    SingleRightDashPointingAngleQuotationMark,
    /// \u{203b}: '※'
    ReferenceMark,
    /// \u{203c}: '‼'
    DoubleExclamationMark,
    /// \u{203d}: '‽'
    Interrobang,
    /// \u{203e}: '‾'
    Overline,
    /// \u{203f}: '‿'
    Undertie,
    /// \u{2040}: '⁀'
    CharacterTie,
    /// \u{2041}: '⁁'
    CaretInsertionPoint,
    /// \u{2042}: '⁂'
    Asterism,
    /// \u{2043}: '⁃'
    HyphenBullet,
    /// \u{2044}: '⁄'
    FractionSlash,
    /// \u{2045}: '⁅'
    LeftSquareBracketWithQuill,
    /// \u{2046}: '⁆'
    RightSquareBracketWithQuill,
    /// \u{2047}: '⁇'
    DoubleQuestionMark,
    /// \u{2048}: '⁈'
    QuestionExclamationMark,
    /// \u{2049}: '⁉'
    ExclamationQuestionMark,
    /// \u{204a}: '⁊'
    TironianSignEt,
    /// \u{204b}: '⁋'
    ReversedPilcrowSign,
    /// \u{204c}: '⁌'
    BlackLeftwardsBullet,
    /// \u{204d}: '⁍'
    BlackRightwardsBullet,
    /// \u{204e}: '⁎'
    LowAsterisk,
    /// \u{204f}: '⁏'
    ReversedSemicolon,
    /// \u{2050}: '⁐'
    CloseUp,
    /// \u{2051}: '⁑'
    TwoAsterisksAlignedVertically,
    /// \u{2052}: '⁒'
    CommercialMinusSign,
    /// \u{2053}: '⁓'
    SwungDash,
    /// \u{2054}: '⁔'
    InvertedUndertie,
    /// \u{2055}: '⁕'
    FlowerPunctuationMark,
    /// \u{2056}: '⁖'
    ThreeDotPunctuation,
    /// \u{2057}: '⁗'
    QuadruplePrime,
    /// \u{2058}: '⁘'
    FourDotPunctuation,
    /// \u{2059}: '⁙'
    FiveDotPunctuation,
    /// \u{205a}: '⁚'
    TwoDotPunctuation,
    /// \u{205b}: '⁛'
    FourDotMark,
    /// \u{205c}: '⁜'
    DottedCross,
    /// \u{205d}: '⁝'
    Tricolon,
    /// \u{205e}: '⁞'
    VerticalFourDots,
    /// \u{205f}: ' '
    MediumMathematicalSpace,
    /// \u{2060}: '⁠'
    WordJoiner,
    /// \u{2061}: '⁡'
    FunctionApplication,
    /// \u{2062}: '⁢'
    InvisibleTimes,
    /// \u{2063}: '⁣'
    InvisibleSeparator,
    /// \u{2064}: '⁤'
    InvisiblePlus,
    /// \u{2066}: '⁦'
    LeftDashToDashRightIsolate,
    /// \u{2067}: '⁧'
    RightDashToDashLeftIsolate,
    /// \u{2068}: '⁨'
    FirstStrongIsolate,
    /// \u{2069}: '⁩'
    PopDirectionalIsolate,
    /// \u{206a}: '⁪'
    InhibitSymmetricSwapping,
    /// \u{206b}: '⁫'
    ActivateSymmetricSwapping,
    /// \u{206c}: '⁬'
    InhibitArabicFormShaping,
    /// \u{206d}: '⁭'
    ActivateArabicFormShaping,
    /// \u{206e}: '⁮'
    NationalDigitShapes,
}

impl Into<char> for GeneralPunctuation {
    fn into(self) -> char {
        use constants::*;
        match self {
            GeneralPunctuation::EnQuad => EN_QUAD,
            GeneralPunctuation::EmQuad => EM_QUAD,
            GeneralPunctuation::EnSpace => EN_SPACE,
            GeneralPunctuation::EmSpace => EM_SPACE,
            GeneralPunctuation::ThreeDashPerDashEmSpace => THREE_DASH_PER_DASH_EM_SPACE,
            GeneralPunctuation::FourDashPerDashEmSpace => FOUR_DASH_PER_DASH_EM_SPACE,
            GeneralPunctuation::SixDashPerDashEmSpace => SIX_DASH_PER_DASH_EM_SPACE,
            GeneralPunctuation::FigureSpace => FIGURE_SPACE,
            GeneralPunctuation::PunctuationSpace => PUNCTUATION_SPACE,
            GeneralPunctuation::ThinSpace => THIN_SPACE,
            GeneralPunctuation::HairSpace => HAIR_SPACE,
            GeneralPunctuation::ZeroWidthSpace => ZERO_WIDTH_SPACE,
            GeneralPunctuation::ZeroWidthNonDashJoiner => ZERO_WIDTH_NON_DASH_JOINER,
            GeneralPunctuation::ZeroWidthJoiner => ZERO_WIDTH_JOINER,
            GeneralPunctuation::LeftDashToDashRightMark => LEFT_DASH_TO_DASH_RIGHT_MARK,
            GeneralPunctuation::RightDashToDashLeftMark => RIGHT_DASH_TO_DASH_LEFT_MARK,
            GeneralPunctuation::Hyphen => HYPHEN,
            GeneralPunctuation::NonDashBreakingHyphen => NON_DASH_BREAKING_HYPHEN,
            GeneralPunctuation::FigureDash => FIGURE_DASH,
            GeneralPunctuation::EnDash => EN_DASH,
            GeneralPunctuation::EmDash => EM_DASH,
            GeneralPunctuation::HorizontalBar => HORIZONTAL_BAR,
            GeneralPunctuation::DoubleVerticalLine => DOUBLE_VERTICAL_LINE,
            GeneralPunctuation::DoubleLowLine => DOUBLE_LOW_LINE,
            GeneralPunctuation::LeftSingleQuotationMark => LEFT_SINGLE_QUOTATION_MARK,
            GeneralPunctuation::RightSingleQuotationMark => RIGHT_SINGLE_QUOTATION_MARK,
            GeneralPunctuation::SingleLowDash9QuotationMark => SINGLE_LOW_DASH_9_QUOTATION_MARK,
            GeneralPunctuation::SingleHighDashReversedDash9QuotationMark => SINGLE_HIGH_DASH_REVERSED_DASH_9_QUOTATION_MARK,
            GeneralPunctuation::LeftDoubleQuotationMark => LEFT_DOUBLE_QUOTATION_MARK,
            GeneralPunctuation::RightDoubleQuotationMark => RIGHT_DOUBLE_QUOTATION_MARK,
            GeneralPunctuation::DoubleLowDash9QuotationMark => DOUBLE_LOW_DASH_9_QUOTATION_MARK,
            GeneralPunctuation::DoubleHighDashReversedDash9QuotationMark => DOUBLE_HIGH_DASH_REVERSED_DASH_9_QUOTATION_MARK,
            GeneralPunctuation::Dagger => DAGGER,
            GeneralPunctuation::DoubleDagger => DOUBLE_DAGGER,
            GeneralPunctuation::Bullet => BULLET,
            GeneralPunctuation::TriangularBullet => TRIANGULAR_BULLET,
            GeneralPunctuation::OneDotLeader => ONE_DOT_LEADER,
            GeneralPunctuation::TwoDotLeader => TWO_DOT_LEADER,
            GeneralPunctuation::HorizontalEllipsis => HORIZONTAL_ELLIPSIS,
            GeneralPunctuation::HyphenationPoint => HYPHENATION_POINT,
            GeneralPunctuation::LineSeparator => LINE_SEPARATOR,
            GeneralPunctuation::ParagraphSeparator => PARAGRAPH_SEPARATOR,
            GeneralPunctuation::LeftDashToDashRightEmbedding => LEFT_DASH_TO_DASH_RIGHT_EMBEDDING,
            GeneralPunctuation::RightDashToDashLeftEmbedding => RIGHT_DASH_TO_DASH_LEFT_EMBEDDING,
            GeneralPunctuation::PopDirectionalFormatting => POP_DIRECTIONAL_FORMATTING,
            GeneralPunctuation::LeftDashToDashRightOverride => LEFT_DASH_TO_DASH_RIGHT_OVERRIDE,
            GeneralPunctuation::RightDashToDashLeftOverride => RIGHT_DASH_TO_DASH_LEFT_OVERRIDE,
            GeneralPunctuation::NarrowNoDashBreakSpace => NARROW_NO_DASH_BREAK_SPACE,
            GeneralPunctuation::PerMilleSign => PER_MILLE_SIGN,
            GeneralPunctuation::PerTenThousandSign => PER_TEN_THOUSAND_SIGN,
            GeneralPunctuation::Prime => PRIME,
            GeneralPunctuation::DoublePrime => DOUBLE_PRIME,
            GeneralPunctuation::TriplePrime => TRIPLE_PRIME,
            GeneralPunctuation::ReversedPrime => REVERSED_PRIME,
            GeneralPunctuation::ReversedDoublePrime => REVERSED_DOUBLE_PRIME,
            GeneralPunctuation::ReversedTriplePrime => REVERSED_TRIPLE_PRIME,
            GeneralPunctuation::Caret => CARET,
            GeneralPunctuation::SingleLeftDashPointingAngleQuotationMark => SINGLE_LEFT_DASH_POINTING_ANGLE_QUOTATION_MARK,
            GeneralPunctuation::SingleRightDashPointingAngleQuotationMark => SINGLE_RIGHT_DASH_POINTING_ANGLE_QUOTATION_MARK,
            GeneralPunctuation::ReferenceMark => REFERENCE_MARK,
            GeneralPunctuation::DoubleExclamationMark => DOUBLE_EXCLAMATION_MARK,
            GeneralPunctuation::Interrobang => INTERROBANG,
            GeneralPunctuation::Overline => OVERLINE,
            GeneralPunctuation::Undertie => UNDERTIE,
            GeneralPunctuation::CharacterTie => CHARACTER_TIE,
            GeneralPunctuation::CaretInsertionPoint => CARET_INSERTION_POINT,
            GeneralPunctuation::Asterism => ASTERISM,
            GeneralPunctuation::HyphenBullet => HYPHEN_BULLET,
            GeneralPunctuation::FractionSlash => FRACTION_SLASH,
            GeneralPunctuation::LeftSquareBracketWithQuill => LEFT_SQUARE_BRACKET_WITH_QUILL,
            GeneralPunctuation::RightSquareBracketWithQuill => RIGHT_SQUARE_BRACKET_WITH_QUILL,
            GeneralPunctuation::DoubleQuestionMark => DOUBLE_QUESTION_MARK,
            GeneralPunctuation::QuestionExclamationMark => QUESTION_EXCLAMATION_MARK,
            GeneralPunctuation::ExclamationQuestionMark => EXCLAMATION_QUESTION_MARK,
            GeneralPunctuation::TironianSignEt => TIRONIAN_SIGN_ET,
            GeneralPunctuation::ReversedPilcrowSign => REVERSED_PILCROW_SIGN,
            GeneralPunctuation::BlackLeftwardsBullet => BLACK_LEFTWARDS_BULLET,
            GeneralPunctuation::BlackRightwardsBullet => BLACK_RIGHTWARDS_BULLET,
            GeneralPunctuation::LowAsterisk => LOW_ASTERISK,
            GeneralPunctuation::ReversedSemicolon => REVERSED_SEMICOLON,
            GeneralPunctuation::CloseUp => CLOSE_UP,
            GeneralPunctuation::TwoAsterisksAlignedVertically => TWO_ASTERISKS_ALIGNED_VERTICALLY,
            GeneralPunctuation::CommercialMinusSign => COMMERCIAL_MINUS_SIGN,
            GeneralPunctuation::SwungDash => SWUNG_DASH,
            GeneralPunctuation::InvertedUndertie => INVERTED_UNDERTIE,
            GeneralPunctuation::FlowerPunctuationMark => FLOWER_PUNCTUATION_MARK,
            GeneralPunctuation::ThreeDotPunctuation => THREE_DOT_PUNCTUATION,
            GeneralPunctuation::QuadruplePrime => QUADRUPLE_PRIME,
            GeneralPunctuation::FourDotPunctuation => FOUR_DOT_PUNCTUATION,
            GeneralPunctuation::FiveDotPunctuation => FIVE_DOT_PUNCTUATION,
            GeneralPunctuation::TwoDotPunctuation => TWO_DOT_PUNCTUATION,
            GeneralPunctuation::FourDotMark => FOUR_DOT_MARK,
            GeneralPunctuation::DottedCross => DOTTED_CROSS,
            GeneralPunctuation::Tricolon => TRICOLON,
            GeneralPunctuation::VerticalFourDots => VERTICAL_FOUR_DOTS,
            GeneralPunctuation::MediumMathematicalSpace => MEDIUM_MATHEMATICAL_SPACE,
            GeneralPunctuation::WordJoiner => WORD_JOINER,
            GeneralPunctuation::FunctionApplication => FUNCTION_APPLICATION,
            GeneralPunctuation::InvisibleTimes => INVISIBLE_TIMES,
            GeneralPunctuation::InvisibleSeparator => INVISIBLE_SEPARATOR,
            GeneralPunctuation::InvisiblePlus => INVISIBLE_PLUS,
            GeneralPunctuation::LeftDashToDashRightIsolate => LEFT_DASH_TO_DASH_RIGHT_ISOLATE,
            GeneralPunctuation::RightDashToDashLeftIsolate => RIGHT_DASH_TO_DASH_LEFT_ISOLATE,
            GeneralPunctuation::FirstStrongIsolate => FIRST_STRONG_ISOLATE,
            GeneralPunctuation::PopDirectionalIsolate => POP_DIRECTIONAL_ISOLATE,
            GeneralPunctuation::InhibitSymmetricSwapping => INHIBIT_SYMMETRIC_SWAPPING,
            GeneralPunctuation::ActivateSymmetricSwapping => ACTIVATE_SYMMETRIC_SWAPPING,
            GeneralPunctuation::InhibitArabicFormShaping => INHIBIT_ARABIC_FORM_SHAPING,
            GeneralPunctuation::ActivateArabicFormShaping => ACTIVATE_ARABIC_FORM_SHAPING,
            GeneralPunctuation::NationalDigitShapes => NATIONAL_DIGIT_SHAPES,
        }
    }
}

impl std::convert::TryFrom<char> for GeneralPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            EN_QUAD => Ok(GeneralPunctuation::EnQuad),
            EM_QUAD => Ok(GeneralPunctuation::EmQuad),
            EN_SPACE => Ok(GeneralPunctuation::EnSpace),
            EM_SPACE => Ok(GeneralPunctuation::EmSpace),
            THREE_DASH_PER_DASH_EM_SPACE => Ok(GeneralPunctuation::ThreeDashPerDashEmSpace),
            FOUR_DASH_PER_DASH_EM_SPACE => Ok(GeneralPunctuation::FourDashPerDashEmSpace),
            SIX_DASH_PER_DASH_EM_SPACE => Ok(GeneralPunctuation::SixDashPerDashEmSpace),
            FIGURE_SPACE => Ok(GeneralPunctuation::FigureSpace),
            PUNCTUATION_SPACE => Ok(GeneralPunctuation::PunctuationSpace),
            THIN_SPACE => Ok(GeneralPunctuation::ThinSpace),
            HAIR_SPACE => Ok(GeneralPunctuation::HairSpace),
            ZERO_WIDTH_SPACE => Ok(GeneralPunctuation::ZeroWidthSpace),
            ZERO_WIDTH_NON_DASH_JOINER => Ok(GeneralPunctuation::ZeroWidthNonDashJoiner),
            ZERO_WIDTH_JOINER => Ok(GeneralPunctuation::ZeroWidthJoiner),
            LEFT_DASH_TO_DASH_RIGHT_MARK => Ok(GeneralPunctuation::LeftDashToDashRightMark),
            RIGHT_DASH_TO_DASH_LEFT_MARK => Ok(GeneralPunctuation::RightDashToDashLeftMark),
            HYPHEN => Ok(GeneralPunctuation::Hyphen),
            NON_DASH_BREAKING_HYPHEN => Ok(GeneralPunctuation::NonDashBreakingHyphen),
            FIGURE_DASH => Ok(GeneralPunctuation::FigureDash),
            EN_DASH => Ok(GeneralPunctuation::EnDash),
            EM_DASH => Ok(GeneralPunctuation::EmDash),
            HORIZONTAL_BAR => Ok(GeneralPunctuation::HorizontalBar),
            DOUBLE_VERTICAL_LINE => Ok(GeneralPunctuation::DoubleVerticalLine),
            DOUBLE_LOW_LINE => Ok(GeneralPunctuation::DoubleLowLine),
            LEFT_SINGLE_QUOTATION_MARK => Ok(GeneralPunctuation::LeftSingleQuotationMark),
            RIGHT_SINGLE_QUOTATION_MARK => Ok(GeneralPunctuation::RightSingleQuotationMark),
            SINGLE_LOW_DASH_9_QUOTATION_MARK => Ok(GeneralPunctuation::SingleLowDash9QuotationMark),
            SINGLE_HIGH_DASH_REVERSED_DASH_9_QUOTATION_MARK => Ok(GeneralPunctuation::SingleHighDashReversedDash9QuotationMark),
            LEFT_DOUBLE_QUOTATION_MARK => Ok(GeneralPunctuation::LeftDoubleQuotationMark),
            RIGHT_DOUBLE_QUOTATION_MARK => Ok(GeneralPunctuation::RightDoubleQuotationMark),
            DOUBLE_LOW_DASH_9_QUOTATION_MARK => Ok(GeneralPunctuation::DoubleLowDash9QuotationMark),
            DOUBLE_HIGH_DASH_REVERSED_DASH_9_QUOTATION_MARK => Ok(GeneralPunctuation::DoubleHighDashReversedDash9QuotationMark),
            DAGGER => Ok(GeneralPunctuation::Dagger),
            DOUBLE_DAGGER => Ok(GeneralPunctuation::DoubleDagger),
            BULLET => Ok(GeneralPunctuation::Bullet),
            TRIANGULAR_BULLET => Ok(GeneralPunctuation::TriangularBullet),
            ONE_DOT_LEADER => Ok(GeneralPunctuation::OneDotLeader),
            TWO_DOT_LEADER => Ok(GeneralPunctuation::TwoDotLeader),
            HORIZONTAL_ELLIPSIS => Ok(GeneralPunctuation::HorizontalEllipsis),
            HYPHENATION_POINT => Ok(GeneralPunctuation::HyphenationPoint),
            LINE_SEPARATOR => Ok(GeneralPunctuation::LineSeparator),
            PARAGRAPH_SEPARATOR => Ok(GeneralPunctuation::ParagraphSeparator),
            LEFT_DASH_TO_DASH_RIGHT_EMBEDDING => Ok(GeneralPunctuation::LeftDashToDashRightEmbedding),
            RIGHT_DASH_TO_DASH_LEFT_EMBEDDING => Ok(GeneralPunctuation::RightDashToDashLeftEmbedding),
            POP_DIRECTIONAL_FORMATTING => Ok(GeneralPunctuation::PopDirectionalFormatting),
            LEFT_DASH_TO_DASH_RIGHT_OVERRIDE => Ok(GeneralPunctuation::LeftDashToDashRightOverride),
            RIGHT_DASH_TO_DASH_LEFT_OVERRIDE => Ok(GeneralPunctuation::RightDashToDashLeftOverride),
            NARROW_NO_DASH_BREAK_SPACE => Ok(GeneralPunctuation::NarrowNoDashBreakSpace),
            PER_MILLE_SIGN => Ok(GeneralPunctuation::PerMilleSign),
            PER_TEN_THOUSAND_SIGN => Ok(GeneralPunctuation::PerTenThousandSign),
            PRIME => Ok(GeneralPunctuation::Prime),
            DOUBLE_PRIME => Ok(GeneralPunctuation::DoublePrime),
            TRIPLE_PRIME => Ok(GeneralPunctuation::TriplePrime),
            REVERSED_PRIME => Ok(GeneralPunctuation::ReversedPrime),
            REVERSED_DOUBLE_PRIME => Ok(GeneralPunctuation::ReversedDoublePrime),
            REVERSED_TRIPLE_PRIME => Ok(GeneralPunctuation::ReversedTriplePrime),
            CARET => Ok(GeneralPunctuation::Caret),
            SINGLE_LEFT_DASH_POINTING_ANGLE_QUOTATION_MARK => Ok(GeneralPunctuation::SingleLeftDashPointingAngleQuotationMark),
            SINGLE_RIGHT_DASH_POINTING_ANGLE_QUOTATION_MARK => Ok(GeneralPunctuation::SingleRightDashPointingAngleQuotationMark),
            REFERENCE_MARK => Ok(GeneralPunctuation::ReferenceMark),
            DOUBLE_EXCLAMATION_MARK => Ok(GeneralPunctuation::DoubleExclamationMark),
            INTERROBANG => Ok(GeneralPunctuation::Interrobang),
            OVERLINE => Ok(GeneralPunctuation::Overline),
            UNDERTIE => Ok(GeneralPunctuation::Undertie),
            CHARACTER_TIE => Ok(GeneralPunctuation::CharacterTie),
            CARET_INSERTION_POINT => Ok(GeneralPunctuation::CaretInsertionPoint),
            ASTERISM => Ok(GeneralPunctuation::Asterism),
            HYPHEN_BULLET => Ok(GeneralPunctuation::HyphenBullet),
            FRACTION_SLASH => Ok(GeneralPunctuation::FractionSlash),
            LEFT_SQUARE_BRACKET_WITH_QUILL => Ok(GeneralPunctuation::LeftSquareBracketWithQuill),
            RIGHT_SQUARE_BRACKET_WITH_QUILL => Ok(GeneralPunctuation::RightSquareBracketWithQuill),
            DOUBLE_QUESTION_MARK => Ok(GeneralPunctuation::DoubleQuestionMark),
            QUESTION_EXCLAMATION_MARK => Ok(GeneralPunctuation::QuestionExclamationMark),
            EXCLAMATION_QUESTION_MARK => Ok(GeneralPunctuation::ExclamationQuestionMark),
            TIRONIAN_SIGN_ET => Ok(GeneralPunctuation::TironianSignEt),
            REVERSED_PILCROW_SIGN => Ok(GeneralPunctuation::ReversedPilcrowSign),
            BLACK_LEFTWARDS_BULLET => Ok(GeneralPunctuation::BlackLeftwardsBullet),
            BLACK_RIGHTWARDS_BULLET => Ok(GeneralPunctuation::BlackRightwardsBullet),
            LOW_ASTERISK => Ok(GeneralPunctuation::LowAsterisk),
            REVERSED_SEMICOLON => Ok(GeneralPunctuation::ReversedSemicolon),
            CLOSE_UP => Ok(GeneralPunctuation::CloseUp),
            TWO_ASTERISKS_ALIGNED_VERTICALLY => Ok(GeneralPunctuation::TwoAsterisksAlignedVertically),
            COMMERCIAL_MINUS_SIGN => Ok(GeneralPunctuation::CommercialMinusSign),
            SWUNG_DASH => Ok(GeneralPunctuation::SwungDash),
            INVERTED_UNDERTIE => Ok(GeneralPunctuation::InvertedUndertie),
            FLOWER_PUNCTUATION_MARK => Ok(GeneralPunctuation::FlowerPunctuationMark),
            THREE_DOT_PUNCTUATION => Ok(GeneralPunctuation::ThreeDotPunctuation),
            QUADRUPLE_PRIME => Ok(GeneralPunctuation::QuadruplePrime),
            FOUR_DOT_PUNCTUATION => Ok(GeneralPunctuation::FourDotPunctuation),
            FIVE_DOT_PUNCTUATION => Ok(GeneralPunctuation::FiveDotPunctuation),
            TWO_DOT_PUNCTUATION => Ok(GeneralPunctuation::TwoDotPunctuation),
            FOUR_DOT_MARK => Ok(GeneralPunctuation::FourDotMark),
            DOTTED_CROSS => Ok(GeneralPunctuation::DottedCross),
            TRICOLON => Ok(GeneralPunctuation::Tricolon),
            VERTICAL_FOUR_DOTS => Ok(GeneralPunctuation::VerticalFourDots),
            MEDIUM_MATHEMATICAL_SPACE => Ok(GeneralPunctuation::MediumMathematicalSpace),
            WORD_JOINER => Ok(GeneralPunctuation::WordJoiner),
            FUNCTION_APPLICATION => Ok(GeneralPunctuation::FunctionApplication),
            INVISIBLE_TIMES => Ok(GeneralPunctuation::InvisibleTimes),
            INVISIBLE_SEPARATOR => Ok(GeneralPunctuation::InvisibleSeparator),
            INVISIBLE_PLUS => Ok(GeneralPunctuation::InvisiblePlus),
            LEFT_DASH_TO_DASH_RIGHT_ISOLATE => Ok(GeneralPunctuation::LeftDashToDashRightIsolate),
            RIGHT_DASH_TO_DASH_LEFT_ISOLATE => Ok(GeneralPunctuation::RightDashToDashLeftIsolate),
            FIRST_STRONG_ISOLATE => Ok(GeneralPunctuation::FirstStrongIsolate),
            POP_DIRECTIONAL_ISOLATE => Ok(GeneralPunctuation::PopDirectionalIsolate),
            INHIBIT_SYMMETRIC_SWAPPING => Ok(GeneralPunctuation::InhibitSymmetricSwapping),
            ACTIVATE_SYMMETRIC_SWAPPING => Ok(GeneralPunctuation::ActivateSymmetricSwapping),
            INHIBIT_ARABIC_FORM_SHAPING => Ok(GeneralPunctuation::InhibitArabicFormShaping),
            ACTIVATE_ARABIC_FORM_SHAPING => Ok(GeneralPunctuation::ActivateArabicFormShaping),
            NATIONAL_DIGIT_SHAPES => Ok(GeneralPunctuation::NationalDigitShapes),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GeneralPunctuation {
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

impl std::convert::TryFrom<u32> for GeneralPunctuation {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GeneralPunctuation {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GeneralPunctuation {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GeneralPunctuation::EnQuad
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GeneralPunctuation::EnQuad => "en quad",
            GeneralPunctuation::EmQuad => "em quad",
            GeneralPunctuation::EnSpace => "en space",
            GeneralPunctuation::EmSpace => "em space",
            GeneralPunctuation::ThreeDashPerDashEmSpace => "three-per-em space",
            GeneralPunctuation::FourDashPerDashEmSpace => "four-per-em space",
            GeneralPunctuation::SixDashPerDashEmSpace => "six-per-em space",
            GeneralPunctuation::FigureSpace => "figure space",
            GeneralPunctuation::PunctuationSpace => "punctuation space",
            GeneralPunctuation::ThinSpace => "thin space",
            GeneralPunctuation::HairSpace => "hair space",
            GeneralPunctuation::ZeroWidthSpace => "zero width space",
            GeneralPunctuation::ZeroWidthNonDashJoiner => "zero width non-joiner",
            GeneralPunctuation::ZeroWidthJoiner => "zero width joiner",
            GeneralPunctuation::LeftDashToDashRightMark => "left-to-right mark",
            GeneralPunctuation::RightDashToDashLeftMark => "right-to-left mark",
            GeneralPunctuation::Hyphen => "hyphen",
            GeneralPunctuation::NonDashBreakingHyphen => "non-breaking hyphen",
            GeneralPunctuation::FigureDash => "figure dash",
            GeneralPunctuation::EnDash => "en dash",
            GeneralPunctuation::EmDash => "em dash",
            GeneralPunctuation::HorizontalBar => "horizontal bar",
            GeneralPunctuation::DoubleVerticalLine => "double vertical line",
            GeneralPunctuation::DoubleLowLine => "double low line",
            GeneralPunctuation::LeftSingleQuotationMark => "left single quotation mark",
            GeneralPunctuation::RightSingleQuotationMark => "right single quotation mark",
            GeneralPunctuation::SingleLowDash9QuotationMark => "single low-9 quotation mark",
            GeneralPunctuation::SingleHighDashReversedDash9QuotationMark => "single high-reversed-9 quotation mark",
            GeneralPunctuation::LeftDoubleQuotationMark => "left double quotation mark",
            GeneralPunctuation::RightDoubleQuotationMark => "right double quotation mark",
            GeneralPunctuation::DoubleLowDash9QuotationMark => "double low-9 quotation mark",
            GeneralPunctuation::DoubleHighDashReversedDash9QuotationMark => "double high-reversed-9 quotation mark",
            GeneralPunctuation::Dagger => "dagger",
            GeneralPunctuation::DoubleDagger => "double dagger",
            GeneralPunctuation::Bullet => "bullet",
            GeneralPunctuation::TriangularBullet => "triangular bullet",
            GeneralPunctuation::OneDotLeader => "one dot leader",
            GeneralPunctuation::TwoDotLeader => "two dot leader",
            GeneralPunctuation::HorizontalEllipsis => "horizontal ellipsis",
            GeneralPunctuation::HyphenationPoint => "hyphenation point",
            GeneralPunctuation::LineSeparator => "line separator",
            GeneralPunctuation::ParagraphSeparator => "paragraph separator",
            GeneralPunctuation::LeftDashToDashRightEmbedding => "left-to-right embedding",
            GeneralPunctuation::RightDashToDashLeftEmbedding => "right-to-left embedding",
            GeneralPunctuation::PopDirectionalFormatting => "pop directional formatting",
            GeneralPunctuation::LeftDashToDashRightOverride => "left-to-right override",
            GeneralPunctuation::RightDashToDashLeftOverride => "right-to-left override",
            GeneralPunctuation::NarrowNoDashBreakSpace => "narrow no-break space",
            GeneralPunctuation::PerMilleSign => "per mille sign",
            GeneralPunctuation::PerTenThousandSign => "per ten thousand sign",
            GeneralPunctuation::Prime => "prime",
            GeneralPunctuation::DoublePrime => "double prime",
            GeneralPunctuation::TriplePrime => "triple prime",
            GeneralPunctuation::ReversedPrime => "reversed prime",
            GeneralPunctuation::ReversedDoublePrime => "reversed double prime",
            GeneralPunctuation::ReversedTriplePrime => "reversed triple prime",
            GeneralPunctuation::Caret => "caret",
            GeneralPunctuation::SingleLeftDashPointingAngleQuotationMark => "single left-pointing angle quotation mark",
            GeneralPunctuation::SingleRightDashPointingAngleQuotationMark => "single right-pointing angle quotation mark",
            GeneralPunctuation::ReferenceMark => "reference mark",
            GeneralPunctuation::DoubleExclamationMark => "double exclamation mark",
            GeneralPunctuation::Interrobang => "interrobang",
            GeneralPunctuation::Overline => "overline",
            GeneralPunctuation::Undertie => "undertie",
            GeneralPunctuation::CharacterTie => "character tie",
            GeneralPunctuation::CaretInsertionPoint => "caret insertion point",
            GeneralPunctuation::Asterism => "asterism",
            GeneralPunctuation::HyphenBullet => "hyphen bullet",
            GeneralPunctuation::FractionSlash => "fraction slash",
            GeneralPunctuation::LeftSquareBracketWithQuill => "left square bracket with quill",
            GeneralPunctuation::RightSquareBracketWithQuill => "right square bracket with quill",
            GeneralPunctuation::DoubleQuestionMark => "double question mark",
            GeneralPunctuation::QuestionExclamationMark => "question exclamation mark",
            GeneralPunctuation::ExclamationQuestionMark => "exclamation question mark",
            GeneralPunctuation::TironianSignEt => "tironian sign et",
            GeneralPunctuation::ReversedPilcrowSign => "reversed pilcrow sign",
            GeneralPunctuation::BlackLeftwardsBullet => "black leftwards bullet",
            GeneralPunctuation::BlackRightwardsBullet => "black rightwards bullet",
            GeneralPunctuation::LowAsterisk => "low asterisk",
            GeneralPunctuation::ReversedSemicolon => "reversed semicolon",
            GeneralPunctuation::CloseUp => "close up",
            GeneralPunctuation::TwoAsterisksAlignedVertically => "two asterisks aligned vertically",
            GeneralPunctuation::CommercialMinusSign => "commercial minus sign",
            GeneralPunctuation::SwungDash => "swung dash",
            GeneralPunctuation::InvertedUndertie => "inverted undertie",
            GeneralPunctuation::FlowerPunctuationMark => "flower punctuation mark",
            GeneralPunctuation::ThreeDotPunctuation => "three dot punctuation",
            GeneralPunctuation::QuadruplePrime => "quadruple prime",
            GeneralPunctuation::FourDotPunctuation => "four dot punctuation",
            GeneralPunctuation::FiveDotPunctuation => "five dot punctuation",
            GeneralPunctuation::TwoDotPunctuation => "two dot punctuation",
            GeneralPunctuation::FourDotMark => "four dot mark",
            GeneralPunctuation::DottedCross => "dotted cross",
            GeneralPunctuation::Tricolon => "tricolon",
            GeneralPunctuation::VerticalFourDots => "vertical four dots",
            GeneralPunctuation::MediumMathematicalSpace => "medium mathematical space",
            GeneralPunctuation::WordJoiner => "word joiner",
            GeneralPunctuation::FunctionApplication => "function application",
            GeneralPunctuation::InvisibleTimes => "invisible times",
            GeneralPunctuation::InvisibleSeparator => "invisible separator",
            GeneralPunctuation::InvisiblePlus => "invisible plus",
            GeneralPunctuation::LeftDashToDashRightIsolate => "left-to-right isolate",
            GeneralPunctuation::RightDashToDashLeftIsolate => "right-to-left isolate",
            GeneralPunctuation::FirstStrongIsolate => "first strong isolate",
            GeneralPunctuation::PopDirectionalIsolate => "pop directional isolate",
            GeneralPunctuation::InhibitSymmetricSwapping => "inhibit symmetric swapping",
            GeneralPunctuation::ActivateSymmetricSwapping => "activate symmetric swapping",
            GeneralPunctuation::InhibitArabicFormShaping => "inhibit arabic form shaping",
            GeneralPunctuation::ActivateArabicFormShaping => "activate arabic form shaping",
            GeneralPunctuation::NationalDigitShapes => "national digit shapes",
        }
    }
}
