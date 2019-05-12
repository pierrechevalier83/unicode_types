/// \u{e0000} → \u{e007f}\
///\
/// 󠀁 󠀠 󠀡 󠀢 󠀣 󠀤 󠀥 󠀦 󠀧 󠀨 󠀩 󠀪 󠀫 󠀬 󠀭 󠀮\
/// 󠀯 󠀰 󠀱 󠀲 󠀳 󠀴 󠀵 󠀶 󠀷 󠀸 󠀹 󠀺 󠀻 󠀼 󠀽 󠀾\
/// 󠀿 󠁀 󠁁 󠁂 󠁃 󠁄 󠁅 󠁆 󠁇 󠁈 󠁉 󠁊 󠁋 󠁌 󠁍 󠁎\
/// 󠁏 󠁐 󠁑 󠁒 󠁓 󠁔 󠁕 󠁖 󠁗 󠁘 󠁙 󠁚 󠁛 󠁜 󠁝 󠁞\
/// 󠁟 󠁠 󠁡 󠁢 󠁣 󠁤 󠁥 󠁦 󠁧 󠁨 󠁩 󠁪 󠁫 󠁬 󠁭 󠁮\
/// 󠁯 󠁰 󠁱 󠁲 󠁳 󠁴 󠁵 󠁶 󠁷 󠁸 󠁹 󠁺 󠁻 󠁼 󠁽 󠁾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{e0001}: '󠀁'
    pub const LANGUAGE_TAG: char = '󠀁';
    /// \u{e0020}: '󠀠'
    pub const TAG_SPACE: char = '󠀠';
    /// \u{e0021}: '󠀡'
    pub const TAG_EXCLAMATION_MARK: char = '󠀡';
    /// \u{e0022}: '󠀢'
    pub const TAG_QUOTATION_MARK: char = '󠀢';
    /// \u{e0023}: '󠀣'
    pub const TAG_NUMBER_SIGN: char = '󠀣';
    /// \u{e0024}: '󠀤'
    pub const TAG_DOLLAR_SIGN: char = '󠀤';
    /// \u{e0025}: '󠀥'
    pub const TAG_PERCENT_SIGN: char = '󠀥';
    /// \u{e0026}: '󠀦'
    pub const TAG_AMPERSAND: char = '󠀦';
    /// \u{e0027}: '󠀧'
    pub const TAG_APOSTROPHE: char = '󠀧';
    /// \u{e0028}: '󠀨'
    pub const TAG_LEFT_PARENTHESIS: char = '󠀨';
    /// \u{e0029}: '󠀩'
    pub const TAG_RIGHT_PARENTHESIS: char = '󠀩';
    /// \u{e002a}: '󠀪'
    pub const TAG_ASTERISK: char = '󠀪';
    /// \u{e002b}: '󠀫'
    pub const TAG_PLUS_SIGN: char = '󠀫';
    /// \u{e002c}: '󠀬'
    pub const TAG_COMMA: char = '󠀬';
    /// \u{e002d}: '󠀭'
    pub const TAG_HYPHEN_DASH_MINUS: char = '󠀭';
    /// \u{e002e}: '󠀮'
    pub const TAG_FULL_STOP: char = '󠀮';
    /// \u{e002f}: '󠀯'
    pub const TAG_SOLIDUS: char = '󠀯';
    /// \u{e0030}: '󠀰'
    pub const TAG_DIGIT_ZERO: char = '󠀰';
    /// \u{e0031}: '󠀱'
    pub const TAG_DIGIT_ONE: char = '󠀱';
    /// \u{e0032}: '󠀲'
    pub const TAG_DIGIT_TWO: char = '󠀲';
    /// \u{e0033}: '󠀳'
    pub const TAG_DIGIT_THREE: char = '󠀳';
    /// \u{e0034}: '󠀴'
    pub const TAG_DIGIT_FOUR: char = '󠀴';
    /// \u{e0035}: '󠀵'
    pub const TAG_DIGIT_FIVE: char = '󠀵';
    /// \u{e0036}: '󠀶'
    pub const TAG_DIGIT_SIX: char = '󠀶';
    /// \u{e0037}: '󠀷'
    pub const TAG_DIGIT_SEVEN: char = '󠀷';
    /// \u{e0038}: '󠀸'
    pub const TAG_DIGIT_EIGHT: char = '󠀸';
    /// \u{e0039}: '󠀹'
    pub const TAG_DIGIT_NINE: char = '󠀹';
    /// \u{e003a}: '󠀺'
    pub const TAG_COLON: char = '󠀺';
    /// \u{e003b}: '󠀻'
    pub const TAG_SEMICOLON: char = '󠀻';
    /// \u{e003c}: '󠀼'
    pub const TAG_LESS_DASH_THAN_SIGN: char = '󠀼';
    /// \u{e003d}: '󠀽'
    pub const TAG_EQUALS_SIGN: char = '󠀽';
    /// \u{e003e}: '󠀾'
    pub const TAG_GREATER_DASH_THAN_SIGN: char = '󠀾';
    /// \u{e003f}: '󠀿'
    pub const TAG_QUESTION_MARK: char = '󠀿';
    /// \u{e0040}: '󠁀'
    pub const TAG_COMMERCIAL_AT: char = '󠁀';
    /// \u{e0041}: '󠁁'
    pub const TAG_LATIN_CAPITAL_LETTER_A: char = '󠁁';
    /// \u{e0042}: '󠁂'
    pub const TAG_LATIN_CAPITAL_LETTER_B: char = '󠁂';
    /// \u{e0043}: '󠁃'
    pub const TAG_LATIN_CAPITAL_LETTER_C: char = '󠁃';
    /// \u{e0044}: '󠁄'
    pub const TAG_LATIN_CAPITAL_LETTER_D: char = '󠁄';
    /// \u{e0045}: '󠁅'
    pub const TAG_LATIN_CAPITAL_LETTER_E: char = '󠁅';
    /// \u{e0046}: '󠁆'
    pub const TAG_LATIN_CAPITAL_LETTER_F: char = '󠁆';
    /// \u{e0047}: '󠁇'
    pub const TAG_LATIN_CAPITAL_LETTER_G: char = '󠁇';
    /// \u{e0048}: '󠁈'
    pub const TAG_LATIN_CAPITAL_LETTER_H: char = '󠁈';
    /// \u{e0049}: '󠁉'
    pub const TAG_LATIN_CAPITAL_LETTER_I: char = '󠁉';
    /// \u{e004a}: '󠁊'
    pub const TAG_LATIN_CAPITAL_LETTER_J: char = '󠁊';
    /// \u{e004b}: '󠁋'
    pub const TAG_LATIN_CAPITAL_LETTER_K: char = '󠁋';
    /// \u{e004c}: '󠁌'
    pub const TAG_LATIN_CAPITAL_LETTER_L: char = '󠁌';
    /// \u{e004d}: '󠁍'
    pub const TAG_LATIN_CAPITAL_LETTER_M: char = '󠁍';
    /// \u{e004e}: '󠁎'
    pub const TAG_LATIN_CAPITAL_LETTER_N: char = '󠁎';
    /// \u{e004f}: '󠁏'
    pub const TAG_LATIN_CAPITAL_LETTER_O: char = '󠁏';
    /// \u{e0050}: '󠁐'
    pub const TAG_LATIN_CAPITAL_LETTER_P: char = '󠁐';
    /// \u{e0051}: '󠁑'
    pub const TAG_LATIN_CAPITAL_LETTER_Q: char = '󠁑';
    /// \u{e0052}: '󠁒'
    pub const TAG_LATIN_CAPITAL_LETTER_R: char = '󠁒';
    /// \u{e0053}: '󠁓'
    pub const TAG_LATIN_CAPITAL_LETTER_S: char = '󠁓';
    /// \u{e0054}: '󠁔'
    pub const TAG_LATIN_CAPITAL_LETTER_T: char = '󠁔';
    /// \u{e0055}: '󠁕'
    pub const TAG_LATIN_CAPITAL_LETTER_U: char = '󠁕';
    /// \u{e0056}: '󠁖'
    pub const TAG_LATIN_CAPITAL_LETTER_V: char = '󠁖';
    /// \u{e0057}: '󠁗'
    pub const TAG_LATIN_CAPITAL_LETTER_W: char = '󠁗';
    /// \u{e0058}: '󠁘'
    pub const TAG_LATIN_CAPITAL_LETTER_X: char = '󠁘';
    /// \u{e0059}: '󠁙'
    pub const TAG_LATIN_CAPITAL_LETTER_Y: char = '󠁙';
    /// \u{e005a}: '󠁚'
    pub const TAG_LATIN_CAPITAL_LETTER_Z: char = '󠁚';
    /// \u{e005b}: '󠁛'
    pub const TAG_LEFT_SQUARE_BRACKET: char = '󠁛';
    /// \u{e005c}: '󠁜'
    pub const TAG_REVERSE_SOLIDUS: char = '󠁜';
    /// \u{e005d}: '󠁝'
    pub const TAG_RIGHT_SQUARE_BRACKET: char = '󠁝';
    /// \u{e005e}: '󠁞'
    pub const TAG_CIRCUMFLEX_ACCENT: char = '󠁞';
    /// \u{e005f}: '󠁟'
    pub const TAG_LOW_LINE: char = '󠁟';
    /// \u{e0060}: '󠁠'
    pub const TAG_GRAVE_ACCENT: char = '󠁠';
    /// \u{e0061}: '󠁡'
    pub const TAG_LATIN_SMALL_LETTER_A: char = '󠁡';
    /// \u{e0062}: '󠁢'
    pub const TAG_LATIN_SMALL_LETTER_B: char = '󠁢';
    /// \u{e0063}: '󠁣'
    pub const TAG_LATIN_SMALL_LETTER_C: char = '󠁣';
    /// \u{e0064}: '󠁤'
    pub const TAG_LATIN_SMALL_LETTER_D: char = '󠁤';
    /// \u{e0065}: '󠁥'
    pub const TAG_LATIN_SMALL_LETTER_E: char = '󠁥';
    /// \u{e0066}: '󠁦'
    pub const TAG_LATIN_SMALL_LETTER_F: char = '󠁦';
    /// \u{e0067}: '󠁧'
    pub const TAG_LATIN_SMALL_LETTER_G: char = '󠁧';
    /// \u{e0068}: '󠁨'
    pub const TAG_LATIN_SMALL_LETTER_H: char = '󠁨';
    /// \u{e0069}: '󠁩'
    pub const TAG_LATIN_SMALL_LETTER_I: char = '󠁩';
    /// \u{e006a}: '󠁪'
    pub const TAG_LATIN_SMALL_LETTER_J: char = '󠁪';
    /// \u{e006b}: '󠁫'
    pub const TAG_LATIN_SMALL_LETTER_K: char = '󠁫';
    /// \u{e006c}: '󠁬'
    pub const TAG_LATIN_SMALL_LETTER_L: char = '󠁬';
    /// \u{e006d}: '󠁭'
    pub const TAG_LATIN_SMALL_LETTER_M: char = '󠁭';
    /// \u{e006e}: '󠁮'
    pub const TAG_LATIN_SMALL_LETTER_N: char = '󠁮';
    /// \u{e006f}: '󠁯'
    pub const TAG_LATIN_SMALL_LETTER_O: char = '󠁯';
    /// \u{e0070}: '󠁰'
    pub const TAG_LATIN_SMALL_LETTER_P: char = '󠁰';
    /// \u{e0071}: '󠁱'
    pub const TAG_LATIN_SMALL_LETTER_Q: char = '󠁱';
    /// \u{e0072}: '󠁲'
    pub const TAG_LATIN_SMALL_LETTER_R: char = '󠁲';
    /// \u{e0073}: '󠁳'
    pub const TAG_LATIN_SMALL_LETTER_S: char = '󠁳';
    /// \u{e0074}: '󠁴'
    pub const TAG_LATIN_SMALL_LETTER_T: char = '󠁴';
    /// \u{e0075}: '󠁵'
    pub const TAG_LATIN_SMALL_LETTER_U: char = '󠁵';
    /// \u{e0076}: '󠁶'
    pub const TAG_LATIN_SMALL_LETTER_V: char = '󠁶';
    /// \u{e0077}: '󠁷'
    pub const TAG_LATIN_SMALL_LETTER_W: char = '󠁷';
    /// \u{e0078}: '󠁸'
    pub const TAG_LATIN_SMALL_LETTER_X: char = '󠁸';
    /// \u{e0079}: '󠁹'
    pub const TAG_LATIN_SMALL_LETTER_Y: char = '󠁹';
    /// \u{e007a}: '󠁺'
    pub const TAG_LATIN_SMALL_LETTER_Z: char = '󠁺';
    /// \u{e007b}: '󠁻'
    pub const TAG_LEFT_CURLY_BRACKET: char = '󠁻';
    /// \u{e007c}: '󠁼'
    pub const TAG_VERTICAL_LINE: char = '󠁼';
    /// \u{e007d}: '󠁽'
    pub const TAG_RIGHT_CURLY_BRACKET: char = '󠁽';
    /// \u{e007e}: '󠁾'
    pub const TAG_TILDE: char = '󠁾';
}

/// An enum to represent all characters in the Tags block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tags {
    /// \u{e0001}: '󠀁'
    LanguageTag,
    /// \u{e0020}: '󠀠'
    TagSpace,
    /// \u{e0021}: '󠀡'
    TagExclamationMark,
    /// \u{e0022}: '󠀢'
    TagQuotationMark,
    /// \u{e0023}: '󠀣'
    TagNumberSign,
    /// \u{e0024}: '󠀤'
    TagDollarSign,
    /// \u{e0025}: '󠀥'
    TagPercentSign,
    /// \u{e0026}: '󠀦'
    TagAmpersand,
    /// \u{e0027}: '󠀧'
    TagApostrophe,
    /// \u{e0028}: '󠀨'
    TagLeftParenthesis,
    /// \u{e0029}: '󠀩'
    TagRightParenthesis,
    /// \u{e002a}: '󠀪'
    TagAsterisk,
    /// \u{e002b}: '󠀫'
    TagPlusSign,
    /// \u{e002c}: '󠀬'
    TagComma,
    /// \u{e002d}: '󠀭'
    TagHyphenDashMinus,
    /// \u{e002e}: '󠀮'
    TagFullStop,
    /// \u{e002f}: '󠀯'
    TagSolidus,
    /// \u{e0030}: '󠀰'
    TagDigitZero,
    /// \u{e0031}: '󠀱'
    TagDigitOne,
    /// \u{e0032}: '󠀲'
    TagDigitTwo,
    /// \u{e0033}: '󠀳'
    TagDigitThree,
    /// \u{e0034}: '󠀴'
    TagDigitFour,
    /// \u{e0035}: '󠀵'
    TagDigitFive,
    /// \u{e0036}: '󠀶'
    TagDigitSix,
    /// \u{e0037}: '󠀷'
    TagDigitSeven,
    /// \u{e0038}: '󠀸'
    TagDigitEight,
    /// \u{e0039}: '󠀹'
    TagDigitNine,
    /// \u{e003a}: '󠀺'
    TagColon,
    /// \u{e003b}: '󠀻'
    TagSemicolon,
    /// \u{e003c}: '󠀼'
    TagLessDashThanSign,
    /// \u{e003d}: '󠀽'
    TagEqualsSign,
    /// \u{e003e}: '󠀾'
    TagGreaterDashThanSign,
    /// \u{e003f}: '󠀿'
    TagQuestionMark,
    /// \u{e0040}: '󠁀'
    TagCommercialAt,
    /// \u{e0041}: '󠁁'
    TagLatinCapitalLetterA,
    /// \u{e0042}: '󠁂'
    TagLatinCapitalLetterB,
    /// \u{e0043}: '󠁃'
    TagLatinCapitalLetterC,
    /// \u{e0044}: '󠁄'
    TagLatinCapitalLetterD,
    /// \u{e0045}: '󠁅'
    TagLatinCapitalLetterE,
    /// \u{e0046}: '󠁆'
    TagLatinCapitalLetterF,
    /// \u{e0047}: '󠁇'
    TagLatinCapitalLetterG,
    /// \u{e0048}: '󠁈'
    TagLatinCapitalLetterH,
    /// \u{e0049}: '󠁉'
    TagLatinCapitalLetterI,
    /// \u{e004a}: '󠁊'
    TagLatinCapitalLetterJ,
    /// \u{e004b}: '󠁋'
    TagLatinCapitalLetterK,
    /// \u{e004c}: '󠁌'
    TagLatinCapitalLetterL,
    /// \u{e004d}: '󠁍'
    TagLatinCapitalLetterM,
    /// \u{e004e}: '󠁎'
    TagLatinCapitalLetterN,
    /// \u{e004f}: '󠁏'
    TagLatinCapitalLetterO,
    /// \u{e0050}: '󠁐'
    TagLatinCapitalLetterP,
    /// \u{e0051}: '󠁑'
    TagLatinCapitalLetterQ,
    /// \u{e0052}: '󠁒'
    TagLatinCapitalLetterR,
    /// \u{e0053}: '󠁓'
    TagLatinCapitalLetterS,
    /// \u{e0054}: '󠁔'
    TagLatinCapitalLetterT,
    /// \u{e0055}: '󠁕'
    TagLatinCapitalLetterU,
    /// \u{e0056}: '󠁖'
    TagLatinCapitalLetterV,
    /// \u{e0057}: '󠁗'
    TagLatinCapitalLetterW,
    /// \u{e0058}: '󠁘'
    TagLatinCapitalLetterX,
    /// \u{e0059}: '󠁙'
    TagLatinCapitalLetterY,
    /// \u{e005a}: '󠁚'
    TagLatinCapitalLetterZ,
    /// \u{e005b}: '󠁛'
    TagLeftSquareBracket,
    /// \u{e005c}: '󠁜'
    TagReverseSolidus,
    /// \u{e005d}: '󠁝'
    TagRightSquareBracket,
    /// \u{e005e}: '󠁞'
    TagCircumflexAccent,
    /// \u{e005f}: '󠁟'
    TagLowLine,
    /// \u{e0060}: '󠁠'
    TagGraveAccent,
    /// \u{e0061}: '󠁡'
    TagLatinSmallLetterA,
    /// \u{e0062}: '󠁢'
    TagLatinSmallLetterB,
    /// \u{e0063}: '󠁣'
    TagLatinSmallLetterC,
    /// \u{e0064}: '󠁤'
    TagLatinSmallLetterD,
    /// \u{e0065}: '󠁥'
    TagLatinSmallLetterE,
    /// \u{e0066}: '󠁦'
    TagLatinSmallLetterF,
    /// \u{e0067}: '󠁧'
    TagLatinSmallLetterG,
    /// \u{e0068}: '󠁨'
    TagLatinSmallLetterH,
    /// \u{e0069}: '󠁩'
    TagLatinSmallLetterI,
    /// \u{e006a}: '󠁪'
    TagLatinSmallLetterJ,
    /// \u{e006b}: '󠁫'
    TagLatinSmallLetterK,
    /// \u{e006c}: '󠁬'
    TagLatinSmallLetterL,
    /// \u{e006d}: '󠁭'
    TagLatinSmallLetterM,
    /// \u{e006e}: '󠁮'
    TagLatinSmallLetterN,
    /// \u{e006f}: '󠁯'
    TagLatinSmallLetterO,
    /// \u{e0070}: '󠁰'
    TagLatinSmallLetterP,
    /// \u{e0071}: '󠁱'
    TagLatinSmallLetterQ,
    /// \u{e0072}: '󠁲'
    TagLatinSmallLetterR,
    /// \u{e0073}: '󠁳'
    TagLatinSmallLetterS,
    /// \u{e0074}: '󠁴'
    TagLatinSmallLetterT,
    /// \u{e0075}: '󠁵'
    TagLatinSmallLetterU,
    /// \u{e0076}: '󠁶'
    TagLatinSmallLetterV,
    /// \u{e0077}: '󠁷'
    TagLatinSmallLetterW,
    /// \u{e0078}: '󠁸'
    TagLatinSmallLetterX,
    /// \u{e0079}: '󠁹'
    TagLatinSmallLetterY,
    /// \u{e007a}: '󠁺'
    TagLatinSmallLetterZ,
    /// \u{e007b}: '󠁻'
    TagLeftCurlyBracket,
    /// \u{e007c}: '󠁼'
    TagVerticalLine,
    /// \u{e007d}: '󠁽'
    TagRightCurlyBracket,
    /// \u{e007e}: '󠁾'
    TagTilde,
}

impl Into<char> for Tags {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tags::LanguageTag => LANGUAGE_TAG,
            Tags::TagSpace => TAG_SPACE,
            Tags::TagExclamationMark => TAG_EXCLAMATION_MARK,
            Tags::TagQuotationMark => TAG_QUOTATION_MARK,
            Tags::TagNumberSign => TAG_NUMBER_SIGN,
            Tags::TagDollarSign => TAG_DOLLAR_SIGN,
            Tags::TagPercentSign => TAG_PERCENT_SIGN,
            Tags::TagAmpersand => TAG_AMPERSAND,
            Tags::TagApostrophe => TAG_APOSTROPHE,
            Tags::TagLeftParenthesis => TAG_LEFT_PARENTHESIS,
            Tags::TagRightParenthesis => TAG_RIGHT_PARENTHESIS,
            Tags::TagAsterisk => TAG_ASTERISK,
            Tags::TagPlusSign => TAG_PLUS_SIGN,
            Tags::TagComma => TAG_COMMA,
            Tags::TagHyphenDashMinus => TAG_HYPHEN_DASH_MINUS,
            Tags::TagFullStop => TAG_FULL_STOP,
            Tags::TagSolidus => TAG_SOLIDUS,
            Tags::TagDigitZero => TAG_DIGIT_ZERO,
            Tags::TagDigitOne => TAG_DIGIT_ONE,
            Tags::TagDigitTwo => TAG_DIGIT_TWO,
            Tags::TagDigitThree => TAG_DIGIT_THREE,
            Tags::TagDigitFour => TAG_DIGIT_FOUR,
            Tags::TagDigitFive => TAG_DIGIT_FIVE,
            Tags::TagDigitSix => TAG_DIGIT_SIX,
            Tags::TagDigitSeven => TAG_DIGIT_SEVEN,
            Tags::TagDigitEight => TAG_DIGIT_EIGHT,
            Tags::TagDigitNine => TAG_DIGIT_NINE,
            Tags::TagColon => TAG_COLON,
            Tags::TagSemicolon => TAG_SEMICOLON,
            Tags::TagLessDashThanSign => TAG_LESS_DASH_THAN_SIGN,
            Tags::TagEqualsSign => TAG_EQUALS_SIGN,
            Tags::TagGreaterDashThanSign => TAG_GREATER_DASH_THAN_SIGN,
            Tags::TagQuestionMark => TAG_QUESTION_MARK,
            Tags::TagCommercialAt => TAG_COMMERCIAL_AT,
            Tags::TagLatinCapitalLetterA => TAG_LATIN_CAPITAL_LETTER_A,
            Tags::TagLatinCapitalLetterB => TAG_LATIN_CAPITAL_LETTER_B,
            Tags::TagLatinCapitalLetterC => TAG_LATIN_CAPITAL_LETTER_C,
            Tags::TagLatinCapitalLetterD => TAG_LATIN_CAPITAL_LETTER_D,
            Tags::TagLatinCapitalLetterE => TAG_LATIN_CAPITAL_LETTER_E,
            Tags::TagLatinCapitalLetterF => TAG_LATIN_CAPITAL_LETTER_F,
            Tags::TagLatinCapitalLetterG => TAG_LATIN_CAPITAL_LETTER_G,
            Tags::TagLatinCapitalLetterH => TAG_LATIN_CAPITAL_LETTER_H,
            Tags::TagLatinCapitalLetterI => TAG_LATIN_CAPITAL_LETTER_I,
            Tags::TagLatinCapitalLetterJ => TAG_LATIN_CAPITAL_LETTER_J,
            Tags::TagLatinCapitalLetterK => TAG_LATIN_CAPITAL_LETTER_K,
            Tags::TagLatinCapitalLetterL => TAG_LATIN_CAPITAL_LETTER_L,
            Tags::TagLatinCapitalLetterM => TAG_LATIN_CAPITAL_LETTER_M,
            Tags::TagLatinCapitalLetterN => TAG_LATIN_CAPITAL_LETTER_N,
            Tags::TagLatinCapitalLetterO => TAG_LATIN_CAPITAL_LETTER_O,
            Tags::TagLatinCapitalLetterP => TAG_LATIN_CAPITAL_LETTER_P,
            Tags::TagLatinCapitalLetterQ => TAG_LATIN_CAPITAL_LETTER_Q,
            Tags::TagLatinCapitalLetterR => TAG_LATIN_CAPITAL_LETTER_R,
            Tags::TagLatinCapitalLetterS => TAG_LATIN_CAPITAL_LETTER_S,
            Tags::TagLatinCapitalLetterT => TAG_LATIN_CAPITAL_LETTER_T,
            Tags::TagLatinCapitalLetterU => TAG_LATIN_CAPITAL_LETTER_U,
            Tags::TagLatinCapitalLetterV => TAG_LATIN_CAPITAL_LETTER_V,
            Tags::TagLatinCapitalLetterW => TAG_LATIN_CAPITAL_LETTER_W,
            Tags::TagLatinCapitalLetterX => TAG_LATIN_CAPITAL_LETTER_X,
            Tags::TagLatinCapitalLetterY => TAG_LATIN_CAPITAL_LETTER_Y,
            Tags::TagLatinCapitalLetterZ => TAG_LATIN_CAPITAL_LETTER_Z,
            Tags::TagLeftSquareBracket => TAG_LEFT_SQUARE_BRACKET,
            Tags::TagReverseSolidus => TAG_REVERSE_SOLIDUS,
            Tags::TagRightSquareBracket => TAG_RIGHT_SQUARE_BRACKET,
            Tags::TagCircumflexAccent => TAG_CIRCUMFLEX_ACCENT,
            Tags::TagLowLine => TAG_LOW_LINE,
            Tags::TagGraveAccent => TAG_GRAVE_ACCENT,
            Tags::TagLatinSmallLetterA => TAG_LATIN_SMALL_LETTER_A,
            Tags::TagLatinSmallLetterB => TAG_LATIN_SMALL_LETTER_B,
            Tags::TagLatinSmallLetterC => TAG_LATIN_SMALL_LETTER_C,
            Tags::TagLatinSmallLetterD => TAG_LATIN_SMALL_LETTER_D,
            Tags::TagLatinSmallLetterE => TAG_LATIN_SMALL_LETTER_E,
            Tags::TagLatinSmallLetterF => TAG_LATIN_SMALL_LETTER_F,
            Tags::TagLatinSmallLetterG => TAG_LATIN_SMALL_LETTER_G,
            Tags::TagLatinSmallLetterH => TAG_LATIN_SMALL_LETTER_H,
            Tags::TagLatinSmallLetterI => TAG_LATIN_SMALL_LETTER_I,
            Tags::TagLatinSmallLetterJ => TAG_LATIN_SMALL_LETTER_J,
            Tags::TagLatinSmallLetterK => TAG_LATIN_SMALL_LETTER_K,
            Tags::TagLatinSmallLetterL => TAG_LATIN_SMALL_LETTER_L,
            Tags::TagLatinSmallLetterM => TAG_LATIN_SMALL_LETTER_M,
            Tags::TagLatinSmallLetterN => TAG_LATIN_SMALL_LETTER_N,
            Tags::TagLatinSmallLetterO => TAG_LATIN_SMALL_LETTER_O,
            Tags::TagLatinSmallLetterP => TAG_LATIN_SMALL_LETTER_P,
            Tags::TagLatinSmallLetterQ => TAG_LATIN_SMALL_LETTER_Q,
            Tags::TagLatinSmallLetterR => TAG_LATIN_SMALL_LETTER_R,
            Tags::TagLatinSmallLetterS => TAG_LATIN_SMALL_LETTER_S,
            Tags::TagLatinSmallLetterT => TAG_LATIN_SMALL_LETTER_T,
            Tags::TagLatinSmallLetterU => TAG_LATIN_SMALL_LETTER_U,
            Tags::TagLatinSmallLetterV => TAG_LATIN_SMALL_LETTER_V,
            Tags::TagLatinSmallLetterW => TAG_LATIN_SMALL_LETTER_W,
            Tags::TagLatinSmallLetterX => TAG_LATIN_SMALL_LETTER_X,
            Tags::TagLatinSmallLetterY => TAG_LATIN_SMALL_LETTER_Y,
            Tags::TagLatinSmallLetterZ => TAG_LATIN_SMALL_LETTER_Z,
            Tags::TagLeftCurlyBracket => TAG_LEFT_CURLY_BRACKET,
            Tags::TagVerticalLine => TAG_VERTICAL_LINE,
            Tags::TagRightCurlyBracket => TAG_RIGHT_CURLY_BRACKET,
            Tags::TagTilde => TAG_TILDE,
        }
    }
}

impl std::convert::TryFrom<char> for Tags {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LANGUAGE_TAG => Ok(Tags::LanguageTag),
            TAG_SPACE => Ok(Tags::TagSpace),
            TAG_EXCLAMATION_MARK => Ok(Tags::TagExclamationMark),
            TAG_QUOTATION_MARK => Ok(Tags::TagQuotationMark),
            TAG_NUMBER_SIGN => Ok(Tags::TagNumberSign),
            TAG_DOLLAR_SIGN => Ok(Tags::TagDollarSign),
            TAG_PERCENT_SIGN => Ok(Tags::TagPercentSign),
            TAG_AMPERSAND => Ok(Tags::TagAmpersand),
            TAG_APOSTROPHE => Ok(Tags::TagApostrophe),
            TAG_LEFT_PARENTHESIS => Ok(Tags::TagLeftParenthesis),
            TAG_RIGHT_PARENTHESIS => Ok(Tags::TagRightParenthesis),
            TAG_ASTERISK => Ok(Tags::TagAsterisk),
            TAG_PLUS_SIGN => Ok(Tags::TagPlusSign),
            TAG_COMMA => Ok(Tags::TagComma),
            TAG_HYPHEN_DASH_MINUS => Ok(Tags::TagHyphenDashMinus),
            TAG_FULL_STOP => Ok(Tags::TagFullStop),
            TAG_SOLIDUS => Ok(Tags::TagSolidus),
            TAG_DIGIT_ZERO => Ok(Tags::TagDigitZero),
            TAG_DIGIT_ONE => Ok(Tags::TagDigitOne),
            TAG_DIGIT_TWO => Ok(Tags::TagDigitTwo),
            TAG_DIGIT_THREE => Ok(Tags::TagDigitThree),
            TAG_DIGIT_FOUR => Ok(Tags::TagDigitFour),
            TAG_DIGIT_FIVE => Ok(Tags::TagDigitFive),
            TAG_DIGIT_SIX => Ok(Tags::TagDigitSix),
            TAG_DIGIT_SEVEN => Ok(Tags::TagDigitSeven),
            TAG_DIGIT_EIGHT => Ok(Tags::TagDigitEight),
            TAG_DIGIT_NINE => Ok(Tags::TagDigitNine),
            TAG_COLON => Ok(Tags::TagColon),
            TAG_SEMICOLON => Ok(Tags::TagSemicolon),
            TAG_LESS_DASH_THAN_SIGN => Ok(Tags::TagLessDashThanSign),
            TAG_EQUALS_SIGN => Ok(Tags::TagEqualsSign),
            TAG_GREATER_DASH_THAN_SIGN => Ok(Tags::TagGreaterDashThanSign),
            TAG_QUESTION_MARK => Ok(Tags::TagQuestionMark),
            TAG_COMMERCIAL_AT => Ok(Tags::TagCommercialAt),
            TAG_LATIN_CAPITAL_LETTER_A => Ok(Tags::TagLatinCapitalLetterA),
            TAG_LATIN_CAPITAL_LETTER_B => Ok(Tags::TagLatinCapitalLetterB),
            TAG_LATIN_CAPITAL_LETTER_C => Ok(Tags::TagLatinCapitalLetterC),
            TAG_LATIN_CAPITAL_LETTER_D => Ok(Tags::TagLatinCapitalLetterD),
            TAG_LATIN_CAPITAL_LETTER_E => Ok(Tags::TagLatinCapitalLetterE),
            TAG_LATIN_CAPITAL_LETTER_F => Ok(Tags::TagLatinCapitalLetterF),
            TAG_LATIN_CAPITAL_LETTER_G => Ok(Tags::TagLatinCapitalLetterG),
            TAG_LATIN_CAPITAL_LETTER_H => Ok(Tags::TagLatinCapitalLetterH),
            TAG_LATIN_CAPITAL_LETTER_I => Ok(Tags::TagLatinCapitalLetterI),
            TAG_LATIN_CAPITAL_LETTER_J => Ok(Tags::TagLatinCapitalLetterJ),
            TAG_LATIN_CAPITAL_LETTER_K => Ok(Tags::TagLatinCapitalLetterK),
            TAG_LATIN_CAPITAL_LETTER_L => Ok(Tags::TagLatinCapitalLetterL),
            TAG_LATIN_CAPITAL_LETTER_M => Ok(Tags::TagLatinCapitalLetterM),
            TAG_LATIN_CAPITAL_LETTER_N => Ok(Tags::TagLatinCapitalLetterN),
            TAG_LATIN_CAPITAL_LETTER_O => Ok(Tags::TagLatinCapitalLetterO),
            TAG_LATIN_CAPITAL_LETTER_P => Ok(Tags::TagLatinCapitalLetterP),
            TAG_LATIN_CAPITAL_LETTER_Q => Ok(Tags::TagLatinCapitalLetterQ),
            TAG_LATIN_CAPITAL_LETTER_R => Ok(Tags::TagLatinCapitalLetterR),
            TAG_LATIN_CAPITAL_LETTER_S => Ok(Tags::TagLatinCapitalLetterS),
            TAG_LATIN_CAPITAL_LETTER_T => Ok(Tags::TagLatinCapitalLetterT),
            TAG_LATIN_CAPITAL_LETTER_U => Ok(Tags::TagLatinCapitalLetterU),
            TAG_LATIN_CAPITAL_LETTER_V => Ok(Tags::TagLatinCapitalLetterV),
            TAG_LATIN_CAPITAL_LETTER_W => Ok(Tags::TagLatinCapitalLetterW),
            TAG_LATIN_CAPITAL_LETTER_X => Ok(Tags::TagLatinCapitalLetterX),
            TAG_LATIN_CAPITAL_LETTER_Y => Ok(Tags::TagLatinCapitalLetterY),
            TAG_LATIN_CAPITAL_LETTER_Z => Ok(Tags::TagLatinCapitalLetterZ),
            TAG_LEFT_SQUARE_BRACKET => Ok(Tags::TagLeftSquareBracket),
            TAG_REVERSE_SOLIDUS => Ok(Tags::TagReverseSolidus),
            TAG_RIGHT_SQUARE_BRACKET => Ok(Tags::TagRightSquareBracket),
            TAG_CIRCUMFLEX_ACCENT => Ok(Tags::TagCircumflexAccent),
            TAG_LOW_LINE => Ok(Tags::TagLowLine),
            TAG_GRAVE_ACCENT => Ok(Tags::TagGraveAccent),
            TAG_LATIN_SMALL_LETTER_A => Ok(Tags::TagLatinSmallLetterA),
            TAG_LATIN_SMALL_LETTER_B => Ok(Tags::TagLatinSmallLetterB),
            TAG_LATIN_SMALL_LETTER_C => Ok(Tags::TagLatinSmallLetterC),
            TAG_LATIN_SMALL_LETTER_D => Ok(Tags::TagLatinSmallLetterD),
            TAG_LATIN_SMALL_LETTER_E => Ok(Tags::TagLatinSmallLetterE),
            TAG_LATIN_SMALL_LETTER_F => Ok(Tags::TagLatinSmallLetterF),
            TAG_LATIN_SMALL_LETTER_G => Ok(Tags::TagLatinSmallLetterG),
            TAG_LATIN_SMALL_LETTER_H => Ok(Tags::TagLatinSmallLetterH),
            TAG_LATIN_SMALL_LETTER_I => Ok(Tags::TagLatinSmallLetterI),
            TAG_LATIN_SMALL_LETTER_J => Ok(Tags::TagLatinSmallLetterJ),
            TAG_LATIN_SMALL_LETTER_K => Ok(Tags::TagLatinSmallLetterK),
            TAG_LATIN_SMALL_LETTER_L => Ok(Tags::TagLatinSmallLetterL),
            TAG_LATIN_SMALL_LETTER_M => Ok(Tags::TagLatinSmallLetterM),
            TAG_LATIN_SMALL_LETTER_N => Ok(Tags::TagLatinSmallLetterN),
            TAG_LATIN_SMALL_LETTER_O => Ok(Tags::TagLatinSmallLetterO),
            TAG_LATIN_SMALL_LETTER_P => Ok(Tags::TagLatinSmallLetterP),
            TAG_LATIN_SMALL_LETTER_Q => Ok(Tags::TagLatinSmallLetterQ),
            TAG_LATIN_SMALL_LETTER_R => Ok(Tags::TagLatinSmallLetterR),
            TAG_LATIN_SMALL_LETTER_S => Ok(Tags::TagLatinSmallLetterS),
            TAG_LATIN_SMALL_LETTER_T => Ok(Tags::TagLatinSmallLetterT),
            TAG_LATIN_SMALL_LETTER_U => Ok(Tags::TagLatinSmallLetterU),
            TAG_LATIN_SMALL_LETTER_V => Ok(Tags::TagLatinSmallLetterV),
            TAG_LATIN_SMALL_LETTER_W => Ok(Tags::TagLatinSmallLetterW),
            TAG_LATIN_SMALL_LETTER_X => Ok(Tags::TagLatinSmallLetterX),
            TAG_LATIN_SMALL_LETTER_Y => Ok(Tags::TagLatinSmallLetterY),
            TAG_LATIN_SMALL_LETTER_Z => Ok(Tags::TagLatinSmallLetterZ),
            TAG_LEFT_CURLY_BRACKET => Ok(Tags::TagLeftCurlyBracket),
            TAG_VERTICAL_LINE => Ok(Tags::TagVerticalLine),
            TAG_RIGHT_CURLY_BRACKET => Ok(Tags::TagRightCurlyBracket),
            TAG_TILDE => Ok(Tags::TagTilde),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tags {
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

impl std::convert::TryFrom<u32> for Tags {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tags {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tags {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tags::LanguageTag
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tags::LanguageTag => "language tag",
            Tags::TagSpace => "tag space",
            Tags::TagExclamationMark => "tag exclamation mark",
            Tags::TagQuotationMark => "tag quotation mark",
            Tags::TagNumberSign => "tag number sign",
            Tags::TagDollarSign => "tag dollar sign",
            Tags::TagPercentSign => "tag percent sign",
            Tags::TagAmpersand => "tag ampersand",
            Tags::TagApostrophe => "tag apostrophe",
            Tags::TagLeftParenthesis => "tag left parenthesis",
            Tags::TagRightParenthesis => "tag right parenthesis",
            Tags::TagAsterisk => "tag asterisk",
            Tags::TagPlusSign => "tag plus sign",
            Tags::TagComma => "tag comma",
            Tags::TagHyphenDashMinus => "tag hyphen-minus",
            Tags::TagFullStop => "tag full stop",
            Tags::TagSolidus => "tag solidus",
            Tags::TagDigitZero => "tag digit zero",
            Tags::TagDigitOne => "tag digit one",
            Tags::TagDigitTwo => "tag digit two",
            Tags::TagDigitThree => "tag digit three",
            Tags::TagDigitFour => "tag digit four",
            Tags::TagDigitFive => "tag digit five",
            Tags::TagDigitSix => "tag digit six",
            Tags::TagDigitSeven => "tag digit seven",
            Tags::TagDigitEight => "tag digit eight",
            Tags::TagDigitNine => "tag digit nine",
            Tags::TagColon => "tag colon",
            Tags::TagSemicolon => "tag semicolon",
            Tags::TagLessDashThanSign => "tag less-than sign",
            Tags::TagEqualsSign => "tag equals sign",
            Tags::TagGreaterDashThanSign => "tag greater-than sign",
            Tags::TagQuestionMark => "tag question mark",
            Tags::TagCommercialAt => "tag commercial at",
            Tags::TagLatinCapitalLetterA => "tag latin capital letter a",
            Tags::TagLatinCapitalLetterB => "tag latin capital letter b",
            Tags::TagLatinCapitalLetterC => "tag latin capital letter c",
            Tags::TagLatinCapitalLetterD => "tag latin capital letter d",
            Tags::TagLatinCapitalLetterE => "tag latin capital letter e",
            Tags::TagLatinCapitalLetterF => "tag latin capital letter f",
            Tags::TagLatinCapitalLetterG => "tag latin capital letter g",
            Tags::TagLatinCapitalLetterH => "tag latin capital letter h",
            Tags::TagLatinCapitalLetterI => "tag latin capital letter i",
            Tags::TagLatinCapitalLetterJ => "tag latin capital letter j",
            Tags::TagLatinCapitalLetterK => "tag latin capital letter k",
            Tags::TagLatinCapitalLetterL => "tag latin capital letter l",
            Tags::TagLatinCapitalLetterM => "tag latin capital letter m",
            Tags::TagLatinCapitalLetterN => "tag latin capital letter n",
            Tags::TagLatinCapitalLetterO => "tag latin capital letter o",
            Tags::TagLatinCapitalLetterP => "tag latin capital letter p",
            Tags::TagLatinCapitalLetterQ => "tag latin capital letter q",
            Tags::TagLatinCapitalLetterR => "tag latin capital letter r",
            Tags::TagLatinCapitalLetterS => "tag latin capital letter s",
            Tags::TagLatinCapitalLetterT => "tag latin capital letter t",
            Tags::TagLatinCapitalLetterU => "tag latin capital letter u",
            Tags::TagLatinCapitalLetterV => "tag latin capital letter v",
            Tags::TagLatinCapitalLetterW => "tag latin capital letter w",
            Tags::TagLatinCapitalLetterX => "tag latin capital letter x",
            Tags::TagLatinCapitalLetterY => "tag latin capital letter y",
            Tags::TagLatinCapitalLetterZ => "tag latin capital letter z",
            Tags::TagLeftSquareBracket => "tag left square bracket",
            Tags::TagReverseSolidus => "tag reverse solidus",
            Tags::TagRightSquareBracket => "tag right square bracket",
            Tags::TagCircumflexAccent => "tag circumflex accent",
            Tags::TagLowLine => "tag low line",
            Tags::TagGraveAccent => "tag grave accent",
            Tags::TagLatinSmallLetterA => "tag latin small letter a",
            Tags::TagLatinSmallLetterB => "tag latin small letter b",
            Tags::TagLatinSmallLetterC => "tag latin small letter c",
            Tags::TagLatinSmallLetterD => "tag latin small letter d",
            Tags::TagLatinSmallLetterE => "tag latin small letter e",
            Tags::TagLatinSmallLetterF => "tag latin small letter f",
            Tags::TagLatinSmallLetterG => "tag latin small letter g",
            Tags::TagLatinSmallLetterH => "tag latin small letter h",
            Tags::TagLatinSmallLetterI => "tag latin small letter i",
            Tags::TagLatinSmallLetterJ => "tag latin small letter j",
            Tags::TagLatinSmallLetterK => "tag latin small letter k",
            Tags::TagLatinSmallLetterL => "tag latin small letter l",
            Tags::TagLatinSmallLetterM => "tag latin small letter m",
            Tags::TagLatinSmallLetterN => "tag latin small letter n",
            Tags::TagLatinSmallLetterO => "tag latin small letter o",
            Tags::TagLatinSmallLetterP => "tag latin small letter p",
            Tags::TagLatinSmallLetterQ => "tag latin small letter q",
            Tags::TagLatinSmallLetterR => "tag latin small letter r",
            Tags::TagLatinSmallLetterS => "tag latin small letter s",
            Tags::TagLatinSmallLetterT => "tag latin small letter t",
            Tags::TagLatinSmallLetterU => "tag latin small letter u",
            Tags::TagLatinSmallLetterV => "tag latin small letter v",
            Tags::TagLatinSmallLetterW => "tag latin small letter w",
            Tags::TagLatinSmallLetterX => "tag latin small letter x",
            Tags::TagLatinSmallLetterY => "tag latin small letter y",
            Tags::TagLatinSmallLetterZ => "tag latin small letter z",
            Tags::TagLeftCurlyBracket => "tag left curly bracket",
            Tags::TagVerticalLine => "tag vertical line",
            Tags::TagRightCurlyBracket => "tag right curly bracket",
            Tags::TagTilde => "tag tilde",
        }
    }
}
