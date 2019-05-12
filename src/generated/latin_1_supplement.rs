/// \u{80} → \u{ff}
///
///                \
///                \
///   ¡ ¢ £ ¤ ¥ ¦ § ¨ © ª « ¬ ­ ® ¯\
/// ° ± ² ³ ´ µ ¶ · ¸ ¹ º » ¼ ½ ¾ ¿\
/// À Á Â Ã Ä Å Æ Ç È É Ê Ë Ì Í Î Ï\
/// Ð Ñ Ò Ó Ô Õ Ö × Ø Ù Ú Û Ü Ý Þ ß\
/// à á â ã ä å æ ç è é ê ë ì í î ï\
/// ð ñ ò ó ô õ ö ÷ ø ù ú û ü ý þ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{80}: ''
    pub const CONTROL_0080: char = '';
    /// \u{81}: ''
    pub const CONTROL_0081: char = '';
    /// \u{82}: ''
    pub const CONTROL_0082: char = '';
    /// \u{83}: ''
    pub const CONTROL_0083: char = '';
    /// \u{84}: ''
    pub const CONTROL_0084: char = '';
    /// \u{85}: ''
    pub const CONTROL_0085: char = '';
    /// \u{86}: ''
    pub const CONTROL_0086: char = '';
    /// \u{87}: ''
    pub const CONTROL_0087: char = '';
    /// \u{88}: ''
    pub const CONTROL_0088: char = '';
    /// \u{89}: ''
    pub const CONTROL_0089: char = '';
    /// \u{8a}: ''
    pub const CONTROL_008A: char = '';
    /// \u{8b}: ''
    pub const CONTROL_008B: char = '';
    /// \u{8c}: ''
    pub const CONTROL_008C: char = '';
    /// \u{8d}: ''
    pub const CONTROL_008D: char = '';
    /// \u{8e}: ''
    pub const CONTROL_008E: char = '';
    /// \u{8f}: ''
    pub const CONTROL_008F: char = '';
    /// \u{90}: ''
    pub const CONTROL_0090: char = '';
    /// \u{91}: ''
    pub const CONTROL_0091: char = '';
    /// \u{92}: ''
    pub const CONTROL_0092: char = '';
    /// \u{93}: ''
    pub const CONTROL_0093: char = '';
    /// \u{94}: ''
    pub const CONTROL_0094: char = '';
    /// \u{95}: ''
    pub const CONTROL_0095: char = '';
    /// \u{96}: ''
    pub const CONTROL_0096: char = '';
    /// \u{97}: ''
    pub const CONTROL_0097: char = '';
    /// \u{98}: ''
    pub const CONTROL_0098: char = '';
    /// \u{99}: ''
    pub const CONTROL_0099: char = '';
    /// \u{9a}: ''
    pub const CONTROL_009A: char = '';
    /// \u{9b}: ''
    pub const CONTROL_009B: char = '';
    /// \u{9c}: ''
    pub const CONTROL_009C: char = '';
    /// \u{9d}: ''
    pub const CONTROL_009D: char = '';
    /// \u{9e}: ''
    pub const CONTROL_009E: char = '';
    /// \u{9f}: ''
    pub const CONTROL_009F: char = '';
    /// \u{a0}: ' '
    pub const NO_DASH_BREAK_SPACE: char = ' ';
    /// \u{a1}: '¡'
    pub const INVERTED_EXCLAMATION_MARK: char = '¡';
    /// \u{a2}: '¢'
    pub const CENT_SIGN: char = '¢';
    /// \u{a3}: '£'
    pub const POUND_SIGN: char = '£';
    /// \u{a4}: '¤'
    pub const CURRENCY_SIGN: char = '¤';
    /// \u{a5}: '¥'
    pub const YEN_SIGN: char = '¥';
    /// \u{a6}: '¦'
    pub const BROKEN_BAR: char = '¦';
    /// \u{a7}: '§'
    pub const SECTION_SIGN: char = '§';
    /// \u{a8}: '¨'
    pub const DIAERESIS: char = '¨';
    /// \u{a9}: '©'
    pub const COPYRIGHT_SIGN: char = '©';
    /// \u{aa}: 'ª'
    pub const FEMININE_ORDINAL_INDICATOR: char = 'ª';
    /// \u{ab}: '«'
    pub const LEFT_DASH_POINTING_DOUBLE_ANGLE_QUOTATION_MARK: char = '«';
    /// \u{ac}: '¬'
    pub const NOT_SIGN: char = '¬';
    /// \u{ad}: '­'
    pub const SOFT_HYPHEN: char = '­';
    /// \u{ae}: '®'
    pub const REGISTERED_SIGN: char = '®';
    /// \u{af}: '¯'
    pub const MACRON: char = '¯';
    /// \u{b0}: '°'
    pub const DEGREE_SIGN: char = '°';
    /// \u{b1}: '±'
    pub const PLUS_DASH_MINUS_SIGN: char = '±';
    /// \u{b2}: '²'
    pub const SUPERSCRIPT_TWO: char = '²';
    /// \u{b3}: '³'
    pub const SUPERSCRIPT_THREE: char = '³';
    /// \u{b4}: '´'
    pub const ACUTE_ACCENT: char = '´';
    /// \u{b5}: 'µ'
    pub const MICRO_SIGN: char = 'µ';
    /// \u{b6}: '¶'
    pub const PILCROW_SIGN: char = '¶';
    /// \u{b7}: '·'
    pub const MIDDLE_DOT: char = '·';
    /// \u{b8}: '¸'
    pub const CEDILLA: char = '¸';
    /// \u{b9}: '¹'
    pub const SUPERSCRIPT_ONE: char = '¹';
    /// \u{ba}: 'º'
    pub const MASCULINE_ORDINAL_INDICATOR: char = 'º';
    /// \u{bb}: '»'
    pub const RIGHT_DASH_POINTING_DOUBLE_ANGLE_QUOTATION_MARK: char = '»';
    /// \u{bc}: '¼'
    pub const VULGAR_FRACTION_ONE_QUARTER: char = '¼';
    /// \u{bd}: '½'
    pub const VULGAR_FRACTION_ONE_HALF: char = '½';
    /// \u{be}: '¾'
    pub const VULGAR_FRACTION_THREE_QUARTERS: char = '¾';
    /// \u{bf}: '¿'
    pub const INVERTED_QUESTION_MARK: char = '¿';
    /// \u{c0}: 'À'
    pub const LATIN_CAPITAL_LETTER_A_WITH_GRAVE: char = 'À';
    /// \u{c1}: 'Á'
    pub const LATIN_CAPITAL_LETTER_A_WITH_ACUTE: char = 'Á';
    /// \u{c2}: 'Â'
    pub const LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX: char = 'Â';
    /// \u{c3}: 'Ã'
    pub const LATIN_CAPITAL_LETTER_A_WITH_TILDE: char = 'Ã';
    /// \u{c4}: 'Ä'
    pub const LATIN_CAPITAL_LETTER_A_WITH_DIAERESIS: char = 'Ä';
    /// \u{c5}: 'Å'
    pub const LATIN_CAPITAL_LETTER_A_WITH_RING_ABOVE: char = 'Å';
    /// \u{c6}: 'Æ'
    pub const LATIN_CAPITAL_LETTER_AE: char = 'Æ';
    /// \u{c7}: 'Ç'
    pub const LATIN_CAPITAL_LETTER_C_WITH_CEDILLA: char = 'Ç';
    /// \u{c8}: 'È'
    pub const LATIN_CAPITAL_LETTER_E_WITH_GRAVE: char = 'È';
    /// \u{c9}: 'É'
    pub const LATIN_CAPITAL_LETTER_E_WITH_ACUTE: char = 'É';
    /// \u{ca}: 'Ê'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX: char = 'Ê';
    /// \u{cb}: 'Ë'
    pub const LATIN_CAPITAL_LETTER_E_WITH_DIAERESIS: char = 'Ë';
    /// \u{cc}: 'Ì'
    pub const LATIN_CAPITAL_LETTER_I_WITH_GRAVE: char = 'Ì';
    /// \u{cd}: 'Í'
    pub const LATIN_CAPITAL_LETTER_I_WITH_ACUTE: char = 'Í';
    /// \u{ce}: 'Î'
    pub const LATIN_CAPITAL_LETTER_I_WITH_CIRCUMFLEX: char = 'Î';
    /// \u{cf}: 'Ï'
    pub const LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS: char = 'Ï';
    /// \u{d0}: 'Ð'
    pub const LATIN_CAPITAL_LETTER_ETH: char = 'Ð';
    /// \u{d1}: 'Ñ'
    pub const LATIN_CAPITAL_LETTER_N_WITH_TILDE: char = 'Ñ';
    /// \u{d2}: 'Ò'
    pub const LATIN_CAPITAL_LETTER_O_WITH_GRAVE: char = 'Ò';
    /// \u{d3}: 'Ó'
    pub const LATIN_CAPITAL_LETTER_O_WITH_ACUTE: char = 'Ó';
    /// \u{d4}: 'Ô'
    pub const LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX: char = 'Ô';
    /// \u{d5}: 'Õ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_TILDE: char = 'Õ';
    /// \u{d6}: 'Ö'
    pub const LATIN_CAPITAL_LETTER_O_WITH_DIAERESIS: char = 'Ö';
    /// \u{d7}: '×'
    pub const MULTIPLICATION_SIGN: char = '×';
    /// \u{d8}: 'Ø'
    pub const LATIN_CAPITAL_LETTER_O_WITH_STROKE: char = 'Ø';
    /// \u{d9}: 'Ù'
    pub const LATIN_CAPITAL_LETTER_U_WITH_GRAVE: char = 'Ù';
    /// \u{da}: 'Ú'
    pub const LATIN_CAPITAL_LETTER_U_WITH_ACUTE: char = 'Ú';
    /// \u{db}: 'Û'
    pub const LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX: char = 'Û';
    /// \u{dc}: 'Ü'
    pub const LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS: char = 'Ü';
    /// \u{dd}: 'Ý'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_ACUTE: char = 'Ý';
    /// \u{de}: 'Þ'
    pub const LATIN_CAPITAL_LETTER_THORN: char = 'Þ';
    /// \u{df}: 'ß'
    pub const LATIN_SMALL_LETTER_SHARP_S: char = 'ß';
    /// \u{e0}: 'à'
    pub const LATIN_SMALL_LETTER_A_WITH_GRAVE: char = 'à';
    /// \u{e1}: 'á'
    pub const LATIN_SMALL_LETTER_A_WITH_ACUTE: char = 'á';
    /// \u{e2}: 'â'
    pub const LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX: char = 'â';
    /// \u{e3}: 'ã'
    pub const LATIN_SMALL_LETTER_A_WITH_TILDE: char = 'ã';
    /// \u{e4}: 'ä'
    pub const LATIN_SMALL_LETTER_A_WITH_DIAERESIS: char = 'ä';
    /// \u{e5}: 'å'
    pub const LATIN_SMALL_LETTER_A_WITH_RING_ABOVE: char = 'å';
    /// \u{e6}: 'æ'
    pub const LATIN_SMALL_LETTER_AE: char = 'æ';
    /// \u{e7}: 'ç'
    pub const LATIN_SMALL_LETTER_C_WITH_CEDILLA: char = 'ç';
    /// \u{e8}: 'è'
    pub const LATIN_SMALL_LETTER_E_WITH_GRAVE: char = 'è';
    /// \u{e9}: 'é'
    pub const LATIN_SMALL_LETTER_E_WITH_ACUTE: char = 'é';
    /// \u{ea}: 'ê'
    pub const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX: char = 'ê';
    /// \u{eb}: 'ë'
    pub const LATIN_SMALL_LETTER_E_WITH_DIAERESIS: char = 'ë';
    /// \u{ec}: 'ì'
    pub const LATIN_SMALL_LETTER_I_WITH_GRAVE: char = 'ì';
    /// \u{ed}: 'í'
    pub const LATIN_SMALL_LETTER_I_WITH_ACUTE: char = 'í';
    /// \u{ee}: 'î'
    pub const LATIN_SMALL_LETTER_I_WITH_CIRCUMFLEX: char = 'î';
    /// \u{ef}: 'ï'
    pub const LATIN_SMALL_LETTER_I_WITH_DIAERESIS: char = 'ï';
    /// \u{f0}: 'ð'
    pub const LATIN_SMALL_LETTER_ETH: char = 'ð';
    /// \u{f1}: 'ñ'
    pub const LATIN_SMALL_LETTER_N_WITH_TILDE: char = 'ñ';
    /// \u{f2}: 'ò'
    pub const LATIN_SMALL_LETTER_O_WITH_GRAVE: char = 'ò';
    /// \u{f3}: 'ó'
    pub const LATIN_SMALL_LETTER_O_WITH_ACUTE: char = 'ó';
    /// \u{f4}: 'ô'
    pub const LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX: char = 'ô';
    /// \u{f5}: 'õ'
    pub const LATIN_SMALL_LETTER_O_WITH_TILDE: char = 'õ';
    /// \u{f6}: 'ö'
    pub const LATIN_SMALL_LETTER_O_WITH_DIAERESIS: char = 'ö';
    /// \u{f7}: '÷'
    pub const DIVISION_SIGN: char = '÷';
    /// \u{f8}: 'ø'
    pub const LATIN_SMALL_LETTER_O_WITH_STROKE: char = 'ø';
    /// \u{f9}: 'ù'
    pub const LATIN_SMALL_LETTER_U_WITH_GRAVE: char = 'ù';
    /// \u{fa}: 'ú'
    pub const LATIN_SMALL_LETTER_U_WITH_ACUTE: char = 'ú';
    /// \u{fb}: 'û'
    pub const LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX: char = 'û';
    /// \u{fc}: 'ü'
    pub const LATIN_SMALL_LETTER_U_WITH_DIAERESIS: char = 'ü';
    /// \u{fd}: 'ý'
    pub const LATIN_SMALL_LETTER_Y_WITH_ACUTE: char = 'ý';
    /// \u{fe}: 'þ'
    pub const LATIN_SMALL_LETTER_THORN: char = 'þ';
}

/// An enum to represent all characters in the Latin1Supplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Latin1Supplement {
    /// \u{80}: ''
    Control0080,
    /// \u{81}: ''
    Control0081,
    /// \u{82}: ''
    Control0082,
    /// \u{83}: ''
    Control0083,
    /// \u{84}: ''
    Control0084,
    /// \u{85}: ''
    Control0085,
    /// \u{86}: ''
    Control0086,
    /// \u{87}: ''
    Control0087,
    /// \u{88}: ''
    Control0088,
    /// \u{89}: ''
    Control0089,
    /// \u{8a}: ''
    Control008a,
    /// \u{8b}: ''
    Control008b,
    /// \u{8c}: ''
    Control008c,
    /// \u{8d}: ''
    Control008d,
    /// \u{8e}: ''
    Control008e,
    /// \u{8f}: ''
    Control008f,
    /// \u{90}: ''
    Control0090,
    /// \u{91}: ''
    Control0091,
    /// \u{92}: ''
    Control0092,
    /// \u{93}: ''
    Control0093,
    /// \u{94}: ''
    Control0094,
    /// \u{95}: ''
    Control0095,
    /// \u{96}: ''
    Control0096,
    /// \u{97}: ''
    Control0097,
    /// \u{98}: ''
    Control0098,
    /// \u{99}: ''
    Control0099,
    /// \u{9a}: ''
    Control009a,
    /// \u{9b}: ''
    Control009b,
    /// \u{9c}: ''
    Control009c,
    /// \u{9d}: ''
    Control009d,
    /// \u{9e}: ''
    Control009e,
    /// \u{9f}: ''
    Control009f,
    /// \u{a0}: ' '
    NoDashBreakSpace,
    /// \u{a1}: '¡'
    InvertedExclamationMark,
    /// \u{a2}: '¢'
    CentSign,
    /// \u{a3}: '£'
    PoundSign,
    /// \u{a4}: '¤'
    CurrencySign,
    /// \u{a5}: '¥'
    YenSign,
    /// \u{a6}: '¦'
    BrokenBar,
    /// \u{a7}: '§'
    SectionSign,
    /// \u{a8}: '¨'
    Diaeresis,
    /// \u{a9}: '©'
    CopyrightSign,
    /// \u{aa}: 'ª'
    FeminineOrdinalIndicator,
    /// \u{ab}: '«'
    LeftDashPointingDoubleAngleQuotationMark,
    /// \u{ac}: '¬'
    NotSign,
    /// \u{ad}: '­'
    SoftHyphen,
    /// \u{ae}: '®'
    RegisteredSign,
    /// \u{af}: '¯'
    Macron,
    /// \u{b0}: '°'
    DegreeSign,
    /// \u{b1}: '±'
    PlusDashMinusSign,
    /// \u{b2}: '²'
    SuperscriptTwo,
    /// \u{b3}: '³'
    SuperscriptThree,
    /// \u{b4}: '´'
    AcuteAccent,
    /// \u{b5}: 'µ'
    MicroSign,
    /// \u{b6}: '¶'
    PilcrowSign,
    /// \u{b7}: '·'
    MiddleDot,
    /// \u{b8}: '¸'
    Cedilla,
    /// \u{b9}: '¹'
    SuperscriptOne,
    /// \u{ba}: 'º'
    MasculineOrdinalIndicator,
    /// \u{bb}: '»'
    RightDashPointingDoubleAngleQuotationMark,
    /// \u{bc}: '¼'
    VulgarFractionOneQuarter,
    /// \u{bd}: '½'
    VulgarFractionOneHalf,
    /// \u{be}: '¾'
    VulgarFractionThreeQuarters,
    /// \u{bf}: '¿'
    InvertedQuestionMark,
    /// \u{c0}: 'À'
    LatinCapitalLetterAWithGrave,
    /// \u{c1}: 'Á'
    LatinCapitalLetterAWithAcute,
    /// \u{c2}: 'Â'
    LatinCapitalLetterAWithCircumflex,
    /// \u{c3}: 'Ã'
    LatinCapitalLetterAWithTilde,
    /// \u{c4}: 'Ä'
    LatinCapitalLetterAWithDiaeresis,
    /// \u{c5}: 'Å'
    LatinCapitalLetterAWithRingAbove,
    /// \u{c6}: 'Æ'
    LatinCapitalLetterAe,
    /// \u{c7}: 'Ç'
    LatinCapitalLetterCWithCedilla,
    /// \u{c8}: 'È'
    LatinCapitalLetterEWithGrave,
    /// \u{c9}: 'É'
    LatinCapitalLetterEWithAcute,
    /// \u{ca}: 'Ê'
    LatinCapitalLetterEWithCircumflex,
    /// \u{cb}: 'Ë'
    LatinCapitalLetterEWithDiaeresis,
    /// \u{cc}: 'Ì'
    LatinCapitalLetterIWithGrave,
    /// \u{cd}: 'Í'
    LatinCapitalLetterIWithAcute,
    /// \u{ce}: 'Î'
    LatinCapitalLetterIWithCircumflex,
    /// \u{cf}: 'Ï'
    LatinCapitalLetterIWithDiaeresis,
    /// \u{d0}: 'Ð'
    LatinCapitalLetterEth,
    /// \u{d1}: 'Ñ'
    LatinCapitalLetterNWithTilde,
    /// \u{d2}: 'Ò'
    LatinCapitalLetterOWithGrave,
    /// \u{d3}: 'Ó'
    LatinCapitalLetterOWithAcute,
    /// \u{d4}: 'Ô'
    LatinCapitalLetterOWithCircumflex,
    /// \u{d5}: 'Õ'
    LatinCapitalLetterOWithTilde,
    /// \u{d6}: 'Ö'
    LatinCapitalLetterOWithDiaeresis,
    /// \u{d7}: '×'
    MultiplicationSign,
    /// \u{d8}: 'Ø'
    LatinCapitalLetterOWithStroke,
    /// \u{d9}: 'Ù'
    LatinCapitalLetterUWithGrave,
    /// \u{da}: 'Ú'
    LatinCapitalLetterUWithAcute,
    /// \u{db}: 'Û'
    LatinCapitalLetterUWithCircumflex,
    /// \u{dc}: 'Ü'
    LatinCapitalLetterUWithDiaeresis,
    /// \u{dd}: 'Ý'
    LatinCapitalLetterYWithAcute,
    /// \u{de}: 'Þ'
    LatinCapitalLetterThorn,
    /// \u{df}: 'ß'
    LatinSmallLetterSharpS,
    /// \u{e0}: 'à'
    LatinSmallLetterAWithGrave,
    /// \u{e1}: 'á'
    LatinSmallLetterAWithAcute,
    /// \u{e2}: 'â'
    LatinSmallLetterAWithCircumflex,
    /// \u{e3}: 'ã'
    LatinSmallLetterAWithTilde,
    /// \u{e4}: 'ä'
    LatinSmallLetterAWithDiaeresis,
    /// \u{e5}: 'å'
    LatinSmallLetterAWithRingAbove,
    /// \u{e6}: 'æ'
    LatinSmallLetterAe,
    /// \u{e7}: 'ç'
    LatinSmallLetterCWithCedilla,
    /// \u{e8}: 'è'
    LatinSmallLetterEWithGrave,
    /// \u{e9}: 'é'
    LatinSmallLetterEWithAcute,
    /// \u{ea}: 'ê'
    LatinSmallLetterEWithCircumflex,
    /// \u{eb}: 'ë'
    LatinSmallLetterEWithDiaeresis,
    /// \u{ec}: 'ì'
    LatinSmallLetterIWithGrave,
    /// \u{ed}: 'í'
    LatinSmallLetterIWithAcute,
    /// \u{ee}: 'î'
    LatinSmallLetterIWithCircumflex,
    /// \u{ef}: 'ï'
    LatinSmallLetterIWithDiaeresis,
    /// \u{f0}: 'ð'
    LatinSmallLetterEth,
    /// \u{f1}: 'ñ'
    LatinSmallLetterNWithTilde,
    /// \u{f2}: 'ò'
    LatinSmallLetterOWithGrave,
    /// \u{f3}: 'ó'
    LatinSmallLetterOWithAcute,
    /// \u{f4}: 'ô'
    LatinSmallLetterOWithCircumflex,
    /// \u{f5}: 'õ'
    LatinSmallLetterOWithTilde,
    /// \u{f6}: 'ö'
    LatinSmallLetterOWithDiaeresis,
    /// \u{f7}: '÷'
    DivisionSign,
    /// \u{f8}: 'ø'
    LatinSmallLetterOWithStroke,
    /// \u{f9}: 'ù'
    LatinSmallLetterUWithGrave,
    /// \u{fa}: 'ú'
    LatinSmallLetterUWithAcute,
    /// \u{fb}: 'û'
    LatinSmallLetterUWithCircumflex,
    /// \u{fc}: 'ü'
    LatinSmallLetterUWithDiaeresis,
    /// \u{fd}: 'ý'
    LatinSmallLetterYWithAcute,
    /// \u{fe}: 'þ'
    LatinSmallLetterThorn,
}

impl Into<char> for Latin1Supplement {
    fn into(self) -> char {
        use constants::*;
        match self {
            Latin1Supplement::Control0080 => CONTROL_0080,
            Latin1Supplement::Control0081 => CONTROL_0081,
            Latin1Supplement::Control0082 => CONTROL_0082,
            Latin1Supplement::Control0083 => CONTROL_0083,
            Latin1Supplement::Control0084 => CONTROL_0084,
            Latin1Supplement::Control0085 => CONTROL_0085,
            Latin1Supplement::Control0086 => CONTROL_0086,
            Latin1Supplement::Control0087 => CONTROL_0087,
            Latin1Supplement::Control0088 => CONTROL_0088,
            Latin1Supplement::Control0089 => CONTROL_0089,
            Latin1Supplement::Control008a => CONTROL_008A,
            Latin1Supplement::Control008b => CONTROL_008B,
            Latin1Supplement::Control008c => CONTROL_008C,
            Latin1Supplement::Control008d => CONTROL_008D,
            Latin1Supplement::Control008e => CONTROL_008E,
            Latin1Supplement::Control008f => CONTROL_008F,
            Latin1Supplement::Control0090 => CONTROL_0090,
            Latin1Supplement::Control0091 => CONTROL_0091,
            Latin1Supplement::Control0092 => CONTROL_0092,
            Latin1Supplement::Control0093 => CONTROL_0093,
            Latin1Supplement::Control0094 => CONTROL_0094,
            Latin1Supplement::Control0095 => CONTROL_0095,
            Latin1Supplement::Control0096 => CONTROL_0096,
            Latin1Supplement::Control0097 => CONTROL_0097,
            Latin1Supplement::Control0098 => CONTROL_0098,
            Latin1Supplement::Control0099 => CONTROL_0099,
            Latin1Supplement::Control009a => CONTROL_009A,
            Latin1Supplement::Control009b => CONTROL_009B,
            Latin1Supplement::Control009c => CONTROL_009C,
            Latin1Supplement::Control009d => CONTROL_009D,
            Latin1Supplement::Control009e => CONTROL_009E,
            Latin1Supplement::Control009f => CONTROL_009F,
            Latin1Supplement::NoDashBreakSpace => NO_DASH_BREAK_SPACE,
            Latin1Supplement::InvertedExclamationMark => INVERTED_EXCLAMATION_MARK,
            Latin1Supplement::CentSign => CENT_SIGN,
            Latin1Supplement::PoundSign => POUND_SIGN,
            Latin1Supplement::CurrencySign => CURRENCY_SIGN,
            Latin1Supplement::YenSign => YEN_SIGN,
            Latin1Supplement::BrokenBar => BROKEN_BAR,
            Latin1Supplement::SectionSign => SECTION_SIGN,
            Latin1Supplement::Diaeresis => DIAERESIS,
            Latin1Supplement::CopyrightSign => COPYRIGHT_SIGN,
            Latin1Supplement::FeminineOrdinalIndicator => FEMININE_ORDINAL_INDICATOR,
            Latin1Supplement::LeftDashPointingDoubleAngleQuotationMark => LEFT_DASH_POINTING_DOUBLE_ANGLE_QUOTATION_MARK,
            Latin1Supplement::NotSign => NOT_SIGN,
            Latin1Supplement::SoftHyphen => SOFT_HYPHEN,
            Latin1Supplement::RegisteredSign => REGISTERED_SIGN,
            Latin1Supplement::Macron => MACRON,
            Latin1Supplement::DegreeSign => DEGREE_SIGN,
            Latin1Supplement::PlusDashMinusSign => PLUS_DASH_MINUS_SIGN,
            Latin1Supplement::SuperscriptTwo => SUPERSCRIPT_TWO,
            Latin1Supplement::SuperscriptThree => SUPERSCRIPT_THREE,
            Latin1Supplement::AcuteAccent => ACUTE_ACCENT,
            Latin1Supplement::MicroSign => MICRO_SIGN,
            Latin1Supplement::PilcrowSign => PILCROW_SIGN,
            Latin1Supplement::MiddleDot => MIDDLE_DOT,
            Latin1Supplement::Cedilla => CEDILLA,
            Latin1Supplement::SuperscriptOne => SUPERSCRIPT_ONE,
            Latin1Supplement::MasculineOrdinalIndicator => MASCULINE_ORDINAL_INDICATOR,
            Latin1Supplement::RightDashPointingDoubleAngleQuotationMark => RIGHT_DASH_POINTING_DOUBLE_ANGLE_QUOTATION_MARK,
            Latin1Supplement::VulgarFractionOneQuarter => VULGAR_FRACTION_ONE_QUARTER,
            Latin1Supplement::VulgarFractionOneHalf => VULGAR_FRACTION_ONE_HALF,
            Latin1Supplement::VulgarFractionThreeQuarters => VULGAR_FRACTION_THREE_QUARTERS,
            Latin1Supplement::InvertedQuestionMark => INVERTED_QUESTION_MARK,
            Latin1Supplement::LatinCapitalLetterAWithGrave => LATIN_CAPITAL_LETTER_A_WITH_GRAVE,
            Latin1Supplement::LatinCapitalLetterAWithAcute => LATIN_CAPITAL_LETTER_A_WITH_ACUTE,
            Latin1Supplement::LatinCapitalLetterAWithCircumflex => LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinCapitalLetterAWithTilde => LATIN_CAPITAL_LETTER_A_WITH_TILDE,
            Latin1Supplement::LatinCapitalLetterAWithDiaeresis => LATIN_CAPITAL_LETTER_A_WITH_DIAERESIS,
            Latin1Supplement::LatinCapitalLetterAWithRingAbove => LATIN_CAPITAL_LETTER_A_WITH_RING_ABOVE,
            Latin1Supplement::LatinCapitalLetterAe => LATIN_CAPITAL_LETTER_AE,
            Latin1Supplement::LatinCapitalLetterCWithCedilla => LATIN_CAPITAL_LETTER_C_WITH_CEDILLA,
            Latin1Supplement::LatinCapitalLetterEWithGrave => LATIN_CAPITAL_LETTER_E_WITH_GRAVE,
            Latin1Supplement::LatinCapitalLetterEWithAcute => LATIN_CAPITAL_LETTER_E_WITH_ACUTE,
            Latin1Supplement::LatinCapitalLetterEWithCircumflex => LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinCapitalLetterEWithDiaeresis => LATIN_CAPITAL_LETTER_E_WITH_DIAERESIS,
            Latin1Supplement::LatinCapitalLetterIWithGrave => LATIN_CAPITAL_LETTER_I_WITH_GRAVE,
            Latin1Supplement::LatinCapitalLetterIWithAcute => LATIN_CAPITAL_LETTER_I_WITH_ACUTE,
            Latin1Supplement::LatinCapitalLetterIWithCircumflex => LATIN_CAPITAL_LETTER_I_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinCapitalLetterIWithDiaeresis => LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS,
            Latin1Supplement::LatinCapitalLetterEth => LATIN_CAPITAL_LETTER_ETH,
            Latin1Supplement::LatinCapitalLetterNWithTilde => LATIN_CAPITAL_LETTER_N_WITH_TILDE,
            Latin1Supplement::LatinCapitalLetterOWithGrave => LATIN_CAPITAL_LETTER_O_WITH_GRAVE,
            Latin1Supplement::LatinCapitalLetterOWithAcute => LATIN_CAPITAL_LETTER_O_WITH_ACUTE,
            Latin1Supplement::LatinCapitalLetterOWithCircumflex => LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinCapitalLetterOWithTilde => LATIN_CAPITAL_LETTER_O_WITH_TILDE,
            Latin1Supplement::LatinCapitalLetterOWithDiaeresis => LATIN_CAPITAL_LETTER_O_WITH_DIAERESIS,
            Latin1Supplement::MultiplicationSign => MULTIPLICATION_SIGN,
            Latin1Supplement::LatinCapitalLetterOWithStroke => LATIN_CAPITAL_LETTER_O_WITH_STROKE,
            Latin1Supplement::LatinCapitalLetterUWithGrave => LATIN_CAPITAL_LETTER_U_WITH_GRAVE,
            Latin1Supplement::LatinCapitalLetterUWithAcute => LATIN_CAPITAL_LETTER_U_WITH_ACUTE,
            Latin1Supplement::LatinCapitalLetterUWithCircumflex => LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinCapitalLetterUWithDiaeresis => LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS,
            Latin1Supplement::LatinCapitalLetterYWithAcute => LATIN_CAPITAL_LETTER_Y_WITH_ACUTE,
            Latin1Supplement::LatinCapitalLetterThorn => LATIN_CAPITAL_LETTER_THORN,
            Latin1Supplement::LatinSmallLetterSharpS => LATIN_SMALL_LETTER_SHARP_S,
            Latin1Supplement::LatinSmallLetterAWithGrave => LATIN_SMALL_LETTER_A_WITH_GRAVE,
            Latin1Supplement::LatinSmallLetterAWithAcute => LATIN_SMALL_LETTER_A_WITH_ACUTE,
            Latin1Supplement::LatinSmallLetterAWithCircumflex => LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinSmallLetterAWithTilde => LATIN_SMALL_LETTER_A_WITH_TILDE,
            Latin1Supplement::LatinSmallLetterAWithDiaeresis => LATIN_SMALL_LETTER_A_WITH_DIAERESIS,
            Latin1Supplement::LatinSmallLetterAWithRingAbove => LATIN_SMALL_LETTER_A_WITH_RING_ABOVE,
            Latin1Supplement::LatinSmallLetterAe => LATIN_SMALL_LETTER_AE,
            Latin1Supplement::LatinSmallLetterCWithCedilla => LATIN_SMALL_LETTER_C_WITH_CEDILLA,
            Latin1Supplement::LatinSmallLetterEWithGrave => LATIN_SMALL_LETTER_E_WITH_GRAVE,
            Latin1Supplement::LatinSmallLetterEWithAcute => LATIN_SMALL_LETTER_E_WITH_ACUTE,
            Latin1Supplement::LatinSmallLetterEWithCircumflex => LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinSmallLetterEWithDiaeresis => LATIN_SMALL_LETTER_E_WITH_DIAERESIS,
            Latin1Supplement::LatinSmallLetterIWithGrave => LATIN_SMALL_LETTER_I_WITH_GRAVE,
            Latin1Supplement::LatinSmallLetterIWithAcute => LATIN_SMALL_LETTER_I_WITH_ACUTE,
            Latin1Supplement::LatinSmallLetterIWithCircumflex => LATIN_SMALL_LETTER_I_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinSmallLetterIWithDiaeresis => LATIN_SMALL_LETTER_I_WITH_DIAERESIS,
            Latin1Supplement::LatinSmallLetterEth => LATIN_SMALL_LETTER_ETH,
            Latin1Supplement::LatinSmallLetterNWithTilde => LATIN_SMALL_LETTER_N_WITH_TILDE,
            Latin1Supplement::LatinSmallLetterOWithGrave => LATIN_SMALL_LETTER_O_WITH_GRAVE,
            Latin1Supplement::LatinSmallLetterOWithAcute => LATIN_SMALL_LETTER_O_WITH_ACUTE,
            Latin1Supplement::LatinSmallLetterOWithCircumflex => LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinSmallLetterOWithTilde => LATIN_SMALL_LETTER_O_WITH_TILDE,
            Latin1Supplement::LatinSmallLetterOWithDiaeresis => LATIN_SMALL_LETTER_O_WITH_DIAERESIS,
            Latin1Supplement::DivisionSign => DIVISION_SIGN,
            Latin1Supplement::LatinSmallLetterOWithStroke => LATIN_SMALL_LETTER_O_WITH_STROKE,
            Latin1Supplement::LatinSmallLetterUWithGrave => LATIN_SMALL_LETTER_U_WITH_GRAVE,
            Latin1Supplement::LatinSmallLetterUWithAcute => LATIN_SMALL_LETTER_U_WITH_ACUTE,
            Latin1Supplement::LatinSmallLetterUWithCircumflex => LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX,
            Latin1Supplement::LatinSmallLetterUWithDiaeresis => LATIN_SMALL_LETTER_U_WITH_DIAERESIS,
            Latin1Supplement::LatinSmallLetterYWithAcute => LATIN_SMALL_LETTER_Y_WITH_ACUTE,
            Latin1Supplement::LatinSmallLetterThorn => LATIN_SMALL_LETTER_THORN,
        }
    }
}

impl std::convert::TryFrom<char> for Latin1Supplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CONTROL_0080 => Ok(Latin1Supplement::Control0080),
            CONTROL_0081 => Ok(Latin1Supplement::Control0081),
            CONTROL_0082 => Ok(Latin1Supplement::Control0082),
            CONTROL_0083 => Ok(Latin1Supplement::Control0083),
            CONTROL_0084 => Ok(Latin1Supplement::Control0084),
            CONTROL_0085 => Ok(Latin1Supplement::Control0085),
            CONTROL_0086 => Ok(Latin1Supplement::Control0086),
            CONTROL_0087 => Ok(Latin1Supplement::Control0087),
            CONTROL_0088 => Ok(Latin1Supplement::Control0088),
            CONTROL_0089 => Ok(Latin1Supplement::Control0089),
            CONTROL_008A => Ok(Latin1Supplement::Control008a),
            CONTROL_008B => Ok(Latin1Supplement::Control008b),
            CONTROL_008C => Ok(Latin1Supplement::Control008c),
            CONTROL_008D => Ok(Latin1Supplement::Control008d),
            CONTROL_008E => Ok(Latin1Supplement::Control008e),
            CONTROL_008F => Ok(Latin1Supplement::Control008f),
            CONTROL_0090 => Ok(Latin1Supplement::Control0090),
            CONTROL_0091 => Ok(Latin1Supplement::Control0091),
            CONTROL_0092 => Ok(Latin1Supplement::Control0092),
            CONTROL_0093 => Ok(Latin1Supplement::Control0093),
            CONTROL_0094 => Ok(Latin1Supplement::Control0094),
            CONTROL_0095 => Ok(Latin1Supplement::Control0095),
            CONTROL_0096 => Ok(Latin1Supplement::Control0096),
            CONTROL_0097 => Ok(Latin1Supplement::Control0097),
            CONTROL_0098 => Ok(Latin1Supplement::Control0098),
            CONTROL_0099 => Ok(Latin1Supplement::Control0099),
            CONTROL_009A => Ok(Latin1Supplement::Control009a),
            CONTROL_009B => Ok(Latin1Supplement::Control009b),
            CONTROL_009C => Ok(Latin1Supplement::Control009c),
            CONTROL_009D => Ok(Latin1Supplement::Control009d),
            CONTROL_009E => Ok(Latin1Supplement::Control009e),
            CONTROL_009F => Ok(Latin1Supplement::Control009f),
            NO_DASH_BREAK_SPACE => Ok(Latin1Supplement::NoDashBreakSpace),
            INVERTED_EXCLAMATION_MARK => Ok(Latin1Supplement::InvertedExclamationMark),
            CENT_SIGN => Ok(Latin1Supplement::CentSign),
            POUND_SIGN => Ok(Latin1Supplement::PoundSign),
            CURRENCY_SIGN => Ok(Latin1Supplement::CurrencySign),
            YEN_SIGN => Ok(Latin1Supplement::YenSign),
            BROKEN_BAR => Ok(Latin1Supplement::BrokenBar),
            SECTION_SIGN => Ok(Latin1Supplement::SectionSign),
            DIAERESIS => Ok(Latin1Supplement::Diaeresis),
            COPYRIGHT_SIGN => Ok(Latin1Supplement::CopyrightSign),
            FEMININE_ORDINAL_INDICATOR => Ok(Latin1Supplement::FeminineOrdinalIndicator),
            LEFT_DASH_POINTING_DOUBLE_ANGLE_QUOTATION_MARK => Ok(Latin1Supplement::LeftDashPointingDoubleAngleQuotationMark),
            NOT_SIGN => Ok(Latin1Supplement::NotSign),
            SOFT_HYPHEN => Ok(Latin1Supplement::SoftHyphen),
            REGISTERED_SIGN => Ok(Latin1Supplement::RegisteredSign),
            MACRON => Ok(Latin1Supplement::Macron),
            DEGREE_SIGN => Ok(Latin1Supplement::DegreeSign),
            PLUS_DASH_MINUS_SIGN => Ok(Latin1Supplement::PlusDashMinusSign),
            SUPERSCRIPT_TWO => Ok(Latin1Supplement::SuperscriptTwo),
            SUPERSCRIPT_THREE => Ok(Latin1Supplement::SuperscriptThree),
            ACUTE_ACCENT => Ok(Latin1Supplement::AcuteAccent),
            MICRO_SIGN => Ok(Latin1Supplement::MicroSign),
            PILCROW_SIGN => Ok(Latin1Supplement::PilcrowSign),
            MIDDLE_DOT => Ok(Latin1Supplement::MiddleDot),
            CEDILLA => Ok(Latin1Supplement::Cedilla),
            SUPERSCRIPT_ONE => Ok(Latin1Supplement::SuperscriptOne),
            MASCULINE_ORDINAL_INDICATOR => Ok(Latin1Supplement::MasculineOrdinalIndicator),
            RIGHT_DASH_POINTING_DOUBLE_ANGLE_QUOTATION_MARK => Ok(Latin1Supplement::RightDashPointingDoubleAngleQuotationMark),
            VULGAR_FRACTION_ONE_QUARTER => Ok(Latin1Supplement::VulgarFractionOneQuarter),
            VULGAR_FRACTION_ONE_HALF => Ok(Latin1Supplement::VulgarFractionOneHalf),
            VULGAR_FRACTION_THREE_QUARTERS => Ok(Latin1Supplement::VulgarFractionThreeQuarters),
            INVERTED_QUESTION_MARK => Ok(Latin1Supplement::InvertedQuestionMark),
            LATIN_CAPITAL_LETTER_A_WITH_GRAVE => Ok(Latin1Supplement::LatinCapitalLetterAWithGrave),
            LATIN_CAPITAL_LETTER_A_WITH_ACUTE => Ok(Latin1Supplement::LatinCapitalLetterAWithAcute),
            LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinCapitalLetterAWithCircumflex),
            LATIN_CAPITAL_LETTER_A_WITH_TILDE => Ok(Latin1Supplement::LatinCapitalLetterAWithTilde),
            LATIN_CAPITAL_LETTER_A_WITH_DIAERESIS => Ok(Latin1Supplement::LatinCapitalLetterAWithDiaeresis),
            LATIN_CAPITAL_LETTER_A_WITH_RING_ABOVE => Ok(Latin1Supplement::LatinCapitalLetterAWithRingAbove),
            LATIN_CAPITAL_LETTER_AE => Ok(Latin1Supplement::LatinCapitalLetterAe),
            LATIN_CAPITAL_LETTER_C_WITH_CEDILLA => Ok(Latin1Supplement::LatinCapitalLetterCWithCedilla),
            LATIN_CAPITAL_LETTER_E_WITH_GRAVE => Ok(Latin1Supplement::LatinCapitalLetterEWithGrave),
            LATIN_CAPITAL_LETTER_E_WITH_ACUTE => Ok(Latin1Supplement::LatinCapitalLetterEWithAcute),
            LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinCapitalLetterEWithCircumflex),
            LATIN_CAPITAL_LETTER_E_WITH_DIAERESIS => Ok(Latin1Supplement::LatinCapitalLetterEWithDiaeresis),
            LATIN_CAPITAL_LETTER_I_WITH_GRAVE => Ok(Latin1Supplement::LatinCapitalLetterIWithGrave),
            LATIN_CAPITAL_LETTER_I_WITH_ACUTE => Ok(Latin1Supplement::LatinCapitalLetterIWithAcute),
            LATIN_CAPITAL_LETTER_I_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinCapitalLetterIWithCircumflex),
            LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS => Ok(Latin1Supplement::LatinCapitalLetterIWithDiaeresis),
            LATIN_CAPITAL_LETTER_ETH => Ok(Latin1Supplement::LatinCapitalLetterEth),
            LATIN_CAPITAL_LETTER_N_WITH_TILDE => Ok(Latin1Supplement::LatinCapitalLetterNWithTilde),
            LATIN_CAPITAL_LETTER_O_WITH_GRAVE => Ok(Latin1Supplement::LatinCapitalLetterOWithGrave),
            LATIN_CAPITAL_LETTER_O_WITH_ACUTE => Ok(Latin1Supplement::LatinCapitalLetterOWithAcute),
            LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinCapitalLetterOWithCircumflex),
            LATIN_CAPITAL_LETTER_O_WITH_TILDE => Ok(Latin1Supplement::LatinCapitalLetterOWithTilde),
            LATIN_CAPITAL_LETTER_O_WITH_DIAERESIS => Ok(Latin1Supplement::LatinCapitalLetterOWithDiaeresis),
            MULTIPLICATION_SIGN => Ok(Latin1Supplement::MultiplicationSign),
            LATIN_CAPITAL_LETTER_O_WITH_STROKE => Ok(Latin1Supplement::LatinCapitalLetterOWithStroke),
            LATIN_CAPITAL_LETTER_U_WITH_GRAVE => Ok(Latin1Supplement::LatinCapitalLetterUWithGrave),
            LATIN_CAPITAL_LETTER_U_WITH_ACUTE => Ok(Latin1Supplement::LatinCapitalLetterUWithAcute),
            LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinCapitalLetterUWithCircumflex),
            LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS => Ok(Latin1Supplement::LatinCapitalLetterUWithDiaeresis),
            LATIN_CAPITAL_LETTER_Y_WITH_ACUTE => Ok(Latin1Supplement::LatinCapitalLetterYWithAcute),
            LATIN_CAPITAL_LETTER_THORN => Ok(Latin1Supplement::LatinCapitalLetterThorn),
            LATIN_SMALL_LETTER_SHARP_S => Ok(Latin1Supplement::LatinSmallLetterSharpS),
            LATIN_SMALL_LETTER_A_WITH_GRAVE => Ok(Latin1Supplement::LatinSmallLetterAWithGrave),
            LATIN_SMALL_LETTER_A_WITH_ACUTE => Ok(Latin1Supplement::LatinSmallLetterAWithAcute),
            LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinSmallLetterAWithCircumflex),
            LATIN_SMALL_LETTER_A_WITH_TILDE => Ok(Latin1Supplement::LatinSmallLetterAWithTilde),
            LATIN_SMALL_LETTER_A_WITH_DIAERESIS => Ok(Latin1Supplement::LatinSmallLetterAWithDiaeresis),
            LATIN_SMALL_LETTER_A_WITH_RING_ABOVE => Ok(Latin1Supplement::LatinSmallLetterAWithRingAbove),
            LATIN_SMALL_LETTER_AE => Ok(Latin1Supplement::LatinSmallLetterAe),
            LATIN_SMALL_LETTER_C_WITH_CEDILLA => Ok(Latin1Supplement::LatinSmallLetterCWithCedilla),
            LATIN_SMALL_LETTER_E_WITH_GRAVE => Ok(Latin1Supplement::LatinSmallLetterEWithGrave),
            LATIN_SMALL_LETTER_E_WITH_ACUTE => Ok(Latin1Supplement::LatinSmallLetterEWithAcute),
            LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinSmallLetterEWithCircumflex),
            LATIN_SMALL_LETTER_E_WITH_DIAERESIS => Ok(Latin1Supplement::LatinSmallLetterEWithDiaeresis),
            LATIN_SMALL_LETTER_I_WITH_GRAVE => Ok(Latin1Supplement::LatinSmallLetterIWithGrave),
            LATIN_SMALL_LETTER_I_WITH_ACUTE => Ok(Latin1Supplement::LatinSmallLetterIWithAcute),
            LATIN_SMALL_LETTER_I_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinSmallLetterIWithCircumflex),
            LATIN_SMALL_LETTER_I_WITH_DIAERESIS => Ok(Latin1Supplement::LatinSmallLetterIWithDiaeresis),
            LATIN_SMALL_LETTER_ETH => Ok(Latin1Supplement::LatinSmallLetterEth),
            LATIN_SMALL_LETTER_N_WITH_TILDE => Ok(Latin1Supplement::LatinSmallLetterNWithTilde),
            LATIN_SMALL_LETTER_O_WITH_GRAVE => Ok(Latin1Supplement::LatinSmallLetterOWithGrave),
            LATIN_SMALL_LETTER_O_WITH_ACUTE => Ok(Latin1Supplement::LatinSmallLetterOWithAcute),
            LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinSmallLetterOWithCircumflex),
            LATIN_SMALL_LETTER_O_WITH_TILDE => Ok(Latin1Supplement::LatinSmallLetterOWithTilde),
            LATIN_SMALL_LETTER_O_WITH_DIAERESIS => Ok(Latin1Supplement::LatinSmallLetterOWithDiaeresis),
            DIVISION_SIGN => Ok(Latin1Supplement::DivisionSign),
            LATIN_SMALL_LETTER_O_WITH_STROKE => Ok(Latin1Supplement::LatinSmallLetterOWithStroke),
            LATIN_SMALL_LETTER_U_WITH_GRAVE => Ok(Latin1Supplement::LatinSmallLetterUWithGrave),
            LATIN_SMALL_LETTER_U_WITH_ACUTE => Ok(Latin1Supplement::LatinSmallLetterUWithAcute),
            LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX => Ok(Latin1Supplement::LatinSmallLetterUWithCircumflex),
            LATIN_SMALL_LETTER_U_WITH_DIAERESIS => Ok(Latin1Supplement::LatinSmallLetterUWithDiaeresis),
            LATIN_SMALL_LETTER_Y_WITH_ACUTE => Ok(Latin1Supplement::LatinSmallLetterYWithAcute),
            LATIN_SMALL_LETTER_THORN => Ok(Latin1Supplement::LatinSmallLetterThorn),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Latin1Supplement {
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

impl std::convert::TryFrom<u32> for Latin1Supplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Latin1Supplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Latin1Supplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Latin1Supplement::Control0080
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Latin1Supplement::Control0080 => "control 0080",
            Latin1Supplement::Control0081 => "control 0081",
            Latin1Supplement::Control0082 => "control 0082",
            Latin1Supplement::Control0083 => "control 0083",
            Latin1Supplement::Control0084 => "control 0084",
            Latin1Supplement::Control0085 => "control 0085",
            Latin1Supplement::Control0086 => "control 0086",
            Latin1Supplement::Control0087 => "control 0087",
            Latin1Supplement::Control0088 => "control 0088",
            Latin1Supplement::Control0089 => "control 0089",
            Latin1Supplement::Control008a => "control 008a",
            Latin1Supplement::Control008b => "control 008b",
            Latin1Supplement::Control008c => "control 008c",
            Latin1Supplement::Control008d => "control 008d",
            Latin1Supplement::Control008e => "control 008e",
            Latin1Supplement::Control008f => "control 008f",
            Latin1Supplement::Control0090 => "control 0090",
            Latin1Supplement::Control0091 => "control 0091",
            Latin1Supplement::Control0092 => "control 0092",
            Latin1Supplement::Control0093 => "control 0093",
            Latin1Supplement::Control0094 => "control 0094",
            Latin1Supplement::Control0095 => "control 0095",
            Latin1Supplement::Control0096 => "control 0096",
            Latin1Supplement::Control0097 => "control 0097",
            Latin1Supplement::Control0098 => "control 0098",
            Latin1Supplement::Control0099 => "control 0099",
            Latin1Supplement::Control009a => "control 009a",
            Latin1Supplement::Control009b => "control 009b",
            Latin1Supplement::Control009c => "control 009c",
            Latin1Supplement::Control009d => "control 009d",
            Latin1Supplement::Control009e => "control 009e",
            Latin1Supplement::Control009f => "control 009f",
            Latin1Supplement::NoDashBreakSpace => "no-break space",
            Latin1Supplement::InvertedExclamationMark => "inverted exclamation mark",
            Latin1Supplement::CentSign => "cent sign",
            Latin1Supplement::PoundSign => "pound sign",
            Latin1Supplement::CurrencySign => "currency sign",
            Latin1Supplement::YenSign => "yen sign",
            Latin1Supplement::BrokenBar => "broken bar",
            Latin1Supplement::SectionSign => "section sign",
            Latin1Supplement::Diaeresis => "diaeresis",
            Latin1Supplement::CopyrightSign => "copyright sign",
            Latin1Supplement::FeminineOrdinalIndicator => "feminine ordinal indicator",
            Latin1Supplement::LeftDashPointingDoubleAngleQuotationMark => "left-pointing double angle quotation mark",
            Latin1Supplement::NotSign => "not sign",
            Latin1Supplement::SoftHyphen => "soft hyphen",
            Latin1Supplement::RegisteredSign => "registered sign",
            Latin1Supplement::Macron => "macron",
            Latin1Supplement::DegreeSign => "degree sign",
            Latin1Supplement::PlusDashMinusSign => "plus-minus sign",
            Latin1Supplement::SuperscriptTwo => "superscript two",
            Latin1Supplement::SuperscriptThree => "superscript three",
            Latin1Supplement::AcuteAccent => "acute accent",
            Latin1Supplement::MicroSign => "micro sign",
            Latin1Supplement::PilcrowSign => "pilcrow sign",
            Latin1Supplement::MiddleDot => "middle dot",
            Latin1Supplement::Cedilla => "cedilla",
            Latin1Supplement::SuperscriptOne => "superscript one",
            Latin1Supplement::MasculineOrdinalIndicator => "masculine ordinal indicator",
            Latin1Supplement::RightDashPointingDoubleAngleQuotationMark => "right-pointing double angle quotation mark",
            Latin1Supplement::VulgarFractionOneQuarter => "vulgar fraction one quarter",
            Latin1Supplement::VulgarFractionOneHalf => "vulgar fraction one half",
            Latin1Supplement::VulgarFractionThreeQuarters => "vulgar fraction three quarters",
            Latin1Supplement::InvertedQuestionMark => "inverted question mark",
            Latin1Supplement::LatinCapitalLetterAWithGrave => "latin capital letter a with grave",
            Latin1Supplement::LatinCapitalLetterAWithAcute => "latin capital letter a with acute",
            Latin1Supplement::LatinCapitalLetterAWithCircumflex => "latin capital letter a with circumflex",
            Latin1Supplement::LatinCapitalLetterAWithTilde => "latin capital letter a with tilde",
            Latin1Supplement::LatinCapitalLetterAWithDiaeresis => "latin capital letter a with diaeresis",
            Latin1Supplement::LatinCapitalLetterAWithRingAbove => "latin capital letter a with ring above",
            Latin1Supplement::LatinCapitalLetterAe => "latin capital letter ae",
            Latin1Supplement::LatinCapitalLetterCWithCedilla => "latin capital letter c with cedilla",
            Latin1Supplement::LatinCapitalLetterEWithGrave => "latin capital letter e with grave",
            Latin1Supplement::LatinCapitalLetterEWithAcute => "latin capital letter e with acute",
            Latin1Supplement::LatinCapitalLetterEWithCircumflex => "latin capital letter e with circumflex",
            Latin1Supplement::LatinCapitalLetterEWithDiaeresis => "latin capital letter e with diaeresis",
            Latin1Supplement::LatinCapitalLetterIWithGrave => "latin capital letter i with grave",
            Latin1Supplement::LatinCapitalLetterIWithAcute => "latin capital letter i with acute",
            Latin1Supplement::LatinCapitalLetterIWithCircumflex => "latin capital letter i with circumflex",
            Latin1Supplement::LatinCapitalLetterIWithDiaeresis => "latin capital letter i with diaeresis",
            Latin1Supplement::LatinCapitalLetterEth => "latin capital letter eth",
            Latin1Supplement::LatinCapitalLetterNWithTilde => "latin capital letter n with tilde",
            Latin1Supplement::LatinCapitalLetterOWithGrave => "latin capital letter o with grave",
            Latin1Supplement::LatinCapitalLetterOWithAcute => "latin capital letter o with acute",
            Latin1Supplement::LatinCapitalLetterOWithCircumflex => "latin capital letter o with circumflex",
            Latin1Supplement::LatinCapitalLetterOWithTilde => "latin capital letter o with tilde",
            Latin1Supplement::LatinCapitalLetterOWithDiaeresis => "latin capital letter o with diaeresis",
            Latin1Supplement::MultiplicationSign => "multiplication sign",
            Latin1Supplement::LatinCapitalLetterOWithStroke => "latin capital letter o with stroke",
            Latin1Supplement::LatinCapitalLetterUWithGrave => "latin capital letter u with grave",
            Latin1Supplement::LatinCapitalLetterUWithAcute => "latin capital letter u with acute",
            Latin1Supplement::LatinCapitalLetterUWithCircumflex => "latin capital letter u with circumflex",
            Latin1Supplement::LatinCapitalLetterUWithDiaeresis => "latin capital letter u with diaeresis",
            Latin1Supplement::LatinCapitalLetterYWithAcute => "latin capital letter y with acute",
            Latin1Supplement::LatinCapitalLetterThorn => "latin capital letter thorn",
            Latin1Supplement::LatinSmallLetterSharpS => "latin small letter sharp s",
            Latin1Supplement::LatinSmallLetterAWithGrave => "latin small letter a with grave",
            Latin1Supplement::LatinSmallLetterAWithAcute => "latin small letter a with acute",
            Latin1Supplement::LatinSmallLetterAWithCircumflex => "latin small letter a with circumflex",
            Latin1Supplement::LatinSmallLetterAWithTilde => "latin small letter a with tilde",
            Latin1Supplement::LatinSmallLetterAWithDiaeresis => "latin small letter a with diaeresis",
            Latin1Supplement::LatinSmallLetterAWithRingAbove => "latin small letter a with ring above",
            Latin1Supplement::LatinSmallLetterAe => "latin small letter ae",
            Latin1Supplement::LatinSmallLetterCWithCedilla => "latin small letter c with cedilla",
            Latin1Supplement::LatinSmallLetterEWithGrave => "latin small letter e with grave",
            Latin1Supplement::LatinSmallLetterEWithAcute => "latin small letter e with acute",
            Latin1Supplement::LatinSmallLetterEWithCircumflex => "latin small letter e with circumflex",
            Latin1Supplement::LatinSmallLetterEWithDiaeresis => "latin small letter e with diaeresis",
            Latin1Supplement::LatinSmallLetterIWithGrave => "latin small letter i with grave",
            Latin1Supplement::LatinSmallLetterIWithAcute => "latin small letter i with acute",
            Latin1Supplement::LatinSmallLetterIWithCircumflex => "latin small letter i with circumflex",
            Latin1Supplement::LatinSmallLetterIWithDiaeresis => "latin small letter i with diaeresis",
            Latin1Supplement::LatinSmallLetterEth => "latin small letter eth",
            Latin1Supplement::LatinSmallLetterNWithTilde => "latin small letter n with tilde",
            Latin1Supplement::LatinSmallLetterOWithGrave => "latin small letter o with grave",
            Latin1Supplement::LatinSmallLetterOWithAcute => "latin small letter o with acute",
            Latin1Supplement::LatinSmallLetterOWithCircumflex => "latin small letter o with circumflex",
            Latin1Supplement::LatinSmallLetterOWithTilde => "latin small letter o with tilde",
            Latin1Supplement::LatinSmallLetterOWithDiaeresis => "latin small letter o with diaeresis",
            Latin1Supplement::DivisionSign => "division sign",
            Latin1Supplement::LatinSmallLetterOWithStroke => "latin small letter o with stroke",
            Latin1Supplement::LatinSmallLetterUWithGrave => "latin small letter u with grave",
            Latin1Supplement::LatinSmallLetterUWithAcute => "latin small letter u with acute",
            Latin1Supplement::LatinSmallLetterUWithCircumflex => "latin small letter u with circumflex",
            Latin1Supplement::LatinSmallLetterUWithDiaeresis => "latin small letter u with diaeresis",
            Latin1Supplement::LatinSmallLetterYWithAcute => "latin small letter y with acute",
            Latin1Supplement::LatinSmallLetterThorn => "latin small letter thorn",
        }
    }
}
