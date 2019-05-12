/// \u{fe50} → \u{fe6f}\
///\
/// ﹐ ﹑ ﹒ ﹔ ﹕ ﹖ ﹗ ﹘ ﹙ ﹚ ﹛ ﹜ ﹝ ﹞ ﹟ ﹠
/// ﹡ ﹢ ﹣ ﹤ ﹥ ﹦ ﹨ ﹩ ﹪ ﹫
pub mod constants {
    /// \u{fe50}: '﹐'
    pub const SMALL_COMMA: char = '﹐';
    /// \u{fe51}: '﹑'
    pub const SMALL_IDEOGRAPHIC_COMMA: char = '﹑';
    /// \u{fe52}: '﹒'
    pub const SMALL_FULL_STOP: char = '﹒';
    /// \u{fe54}: '﹔'
    pub const SMALL_SEMICOLON: char = '﹔';
    /// \u{fe55}: '﹕'
    pub const SMALL_COLON: char = '﹕';
    /// \u{fe56}: '﹖'
    pub const SMALL_QUESTION_MARK: char = '﹖';
    /// \u{fe57}: '﹗'
    pub const SMALL_EXCLAMATION_MARK: char = '﹗';
    /// \u{fe58}: '﹘'
    pub const SMALL_EM_DASH: char = '﹘';
    /// \u{fe59}: '﹙'
    pub const SMALL_LEFT_PARENTHESIS: char = '﹙';
    /// \u{fe5a}: '﹚'
    pub const SMALL_RIGHT_PARENTHESIS: char = '﹚';
    /// \u{fe5b}: '﹛'
    pub const SMALL_LEFT_CURLY_BRACKET: char = '﹛';
    /// \u{fe5c}: '﹜'
    pub const SMALL_RIGHT_CURLY_BRACKET: char = '﹜';
    /// \u{fe5d}: '﹝'
    pub const SMALL_LEFT_TORTOISE_SHELL_BRACKET: char = '﹝';
    /// \u{fe5e}: '﹞'
    pub const SMALL_RIGHT_TORTOISE_SHELL_BRACKET: char = '﹞';
    /// \u{fe5f}: '﹟'
    pub const SMALL_NUMBER_SIGN: char = '﹟';
    /// \u{fe60}: '﹠'
    pub const SMALL_AMPERSAND: char = '﹠';
    /// \u{fe61}: '﹡'
    pub const SMALL_ASTERISK: char = '﹡';
    /// \u{fe62}: '﹢'
    pub const SMALL_PLUS_SIGN: char = '﹢';
    /// \u{fe63}: '﹣'
    pub const SMALL_HYPHEN_DASH_MINUS: char = '﹣';
    /// \u{fe64}: '﹤'
    pub const SMALL_LESS_DASH_THAN_SIGN: char = '﹤';
    /// \u{fe65}: '﹥'
    pub const SMALL_GREATER_DASH_THAN_SIGN: char = '﹥';
    /// \u{fe66}: '﹦'
    pub const SMALL_EQUALS_SIGN: char = '﹦';
    /// \u{fe68}: '﹨'
    pub const SMALL_REVERSE_SOLIDUS: char = '﹨';
    /// \u{fe69}: '﹩'
    pub const SMALL_DOLLAR_SIGN: char = '﹩';
    /// \u{fe6a}: '﹪'
    pub const SMALL_PERCENT_SIGN: char = '﹪';
    /// \u{fe6b}: '﹫'
    pub const SMALL_COMMERCIAL_AT: char = '﹫';
}

/// \u{fe50} → \u{fe6f}\
///\
/// ﹐ ﹑ ﹒ ﹔ ﹕ ﹖ ﹗ ﹘ ﹙ ﹚ ﹛ ﹜ ﹝ ﹞ ﹟ ﹠
/// ﹡ ﹢ ﹣ ﹤ ﹥ ﹦ ﹨ ﹩ ﹪ ﹫
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SmallFormVariants {
    /// \u{fe50}: '﹐'
    SmallComma,
    /// \u{fe51}: '﹑'
    SmallIdeographicComma,
    /// \u{fe52}: '﹒'
    SmallFullStop,
    /// \u{fe54}: '﹔'
    SmallSemicolon,
    /// \u{fe55}: '﹕'
    SmallColon,
    /// \u{fe56}: '﹖'
    SmallQuestionMark,
    /// \u{fe57}: '﹗'
    SmallExclamationMark,
    /// \u{fe58}: '﹘'
    SmallEmDash,
    /// \u{fe59}: '﹙'
    SmallLeftParenthesis,
    /// \u{fe5a}: '﹚'
    SmallRightParenthesis,
    /// \u{fe5b}: '﹛'
    SmallLeftCurlyBracket,
    /// \u{fe5c}: '﹜'
    SmallRightCurlyBracket,
    /// \u{fe5d}: '﹝'
    SmallLeftTortoiseShellBracket,
    /// \u{fe5e}: '﹞'
    SmallRightTortoiseShellBracket,
    /// \u{fe5f}: '﹟'
    SmallNumberSign,
    /// \u{fe60}: '﹠'
    SmallAmpersand,
    /// \u{fe61}: '﹡'
    SmallAsterisk,
    /// \u{fe62}: '﹢'
    SmallPlusSign,
    /// \u{fe63}: '﹣'
    SmallHyphenDashMinus,
    /// \u{fe64}: '﹤'
    SmallLessDashThanSign,
    /// \u{fe65}: '﹥'
    SmallGreaterDashThanSign,
    /// \u{fe66}: '﹦'
    SmallEqualsSign,
    /// \u{fe68}: '﹨'
    SmallReverseSolidus,
    /// \u{fe69}: '﹩'
    SmallDollarSign,
    /// \u{fe6a}: '﹪'
    SmallPercentSign,
    /// \u{fe6b}: '﹫'
    SmallCommercialAt,
}

impl Into<char> for SmallFormVariants {
    fn into(self) -> char {
        use constants::*;
        match self {
            SmallFormVariants::SmallComma => SMALL_COMMA,
            SmallFormVariants::SmallIdeographicComma => SMALL_IDEOGRAPHIC_COMMA,
            SmallFormVariants::SmallFullStop => SMALL_FULL_STOP,
            SmallFormVariants::SmallSemicolon => SMALL_SEMICOLON,
            SmallFormVariants::SmallColon => SMALL_COLON,
            SmallFormVariants::SmallQuestionMark => SMALL_QUESTION_MARK,
            SmallFormVariants::SmallExclamationMark => SMALL_EXCLAMATION_MARK,
            SmallFormVariants::SmallEmDash => SMALL_EM_DASH,
            SmallFormVariants::SmallLeftParenthesis => SMALL_LEFT_PARENTHESIS,
            SmallFormVariants::SmallRightParenthesis => SMALL_RIGHT_PARENTHESIS,
            SmallFormVariants::SmallLeftCurlyBracket => SMALL_LEFT_CURLY_BRACKET,
            SmallFormVariants::SmallRightCurlyBracket => SMALL_RIGHT_CURLY_BRACKET,
            SmallFormVariants::SmallLeftTortoiseShellBracket => SMALL_LEFT_TORTOISE_SHELL_BRACKET,
            SmallFormVariants::SmallRightTortoiseShellBracket => SMALL_RIGHT_TORTOISE_SHELL_BRACKET,
            SmallFormVariants::SmallNumberSign => SMALL_NUMBER_SIGN,
            SmallFormVariants::SmallAmpersand => SMALL_AMPERSAND,
            SmallFormVariants::SmallAsterisk => SMALL_ASTERISK,
            SmallFormVariants::SmallPlusSign => SMALL_PLUS_SIGN,
            SmallFormVariants::SmallHyphenDashMinus => SMALL_HYPHEN_DASH_MINUS,
            SmallFormVariants::SmallLessDashThanSign => SMALL_LESS_DASH_THAN_SIGN,
            SmallFormVariants::SmallGreaterDashThanSign => SMALL_GREATER_DASH_THAN_SIGN,
            SmallFormVariants::SmallEqualsSign => SMALL_EQUALS_SIGN,
            SmallFormVariants::SmallReverseSolidus => SMALL_REVERSE_SOLIDUS,
            SmallFormVariants::SmallDollarSign => SMALL_DOLLAR_SIGN,
            SmallFormVariants::SmallPercentSign => SMALL_PERCENT_SIGN,
            SmallFormVariants::SmallCommercialAt => SMALL_COMMERCIAL_AT,
        }
    }
}

impl std::convert::TryFrom<char> for SmallFormVariants {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SMALL_COMMA => Ok(SmallFormVariants::SmallComma),
            SMALL_IDEOGRAPHIC_COMMA => Ok(SmallFormVariants::SmallIdeographicComma),
            SMALL_FULL_STOP => Ok(SmallFormVariants::SmallFullStop),
            SMALL_SEMICOLON => Ok(SmallFormVariants::SmallSemicolon),
            SMALL_COLON => Ok(SmallFormVariants::SmallColon),
            SMALL_QUESTION_MARK => Ok(SmallFormVariants::SmallQuestionMark),
            SMALL_EXCLAMATION_MARK => Ok(SmallFormVariants::SmallExclamationMark),
            SMALL_EM_DASH => Ok(SmallFormVariants::SmallEmDash),
            SMALL_LEFT_PARENTHESIS => Ok(SmallFormVariants::SmallLeftParenthesis),
            SMALL_RIGHT_PARENTHESIS => Ok(SmallFormVariants::SmallRightParenthesis),
            SMALL_LEFT_CURLY_BRACKET => Ok(SmallFormVariants::SmallLeftCurlyBracket),
            SMALL_RIGHT_CURLY_BRACKET => Ok(SmallFormVariants::SmallRightCurlyBracket),
            SMALL_LEFT_TORTOISE_SHELL_BRACKET => Ok(SmallFormVariants::SmallLeftTortoiseShellBracket),
            SMALL_RIGHT_TORTOISE_SHELL_BRACKET => Ok(SmallFormVariants::SmallRightTortoiseShellBracket),
            SMALL_NUMBER_SIGN => Ok(SmallFormVariants::SmallNumberSign),
            SMALL_AMPERSAND => Ok(SmallFormVariants::SmallAmpersand),
            SMALL_ASTERISK => Ok(SmallFormVariants::SmallAsterisk),
            SMALL_PLUS_SIGN => Ok(SmallFormVariants::SmallPlusSign),
            SMALL_HYPHEN_DASH_MINUS => Ok(SmallFormVariants::SmallHyphenDashMinus),
            SMALL_LESS_DASH_THAN_SIGN => Ok(SmallFormVariants::SmallLessDashThanSign),
            SMALL_GREATER_DASH_THAN_SIGN => Ok(SmallFormVariants::SmallGreaterDashThanSign),
            SMALL_EQUALS_SIGN => Ok(SmallFormVariants::SmallEqualsSign),
            SMALL_REVERSE_SOLIDUS => Ok(SmallFormVariants::SmallReverseSolidus),
            SMALL_DOLLAR_SIGN => Ok(SmallFormVariants::SmallDollarSign),
            SMALL_PERCENT_SIGN => Ok(SmallFormVariants::SmallPercentSign),
            SMALL_COMMERCIAL_AT => Ok(SmallFormVariants::SmallCommercialAt),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SmallFormVariants {
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

impl std::convert::TryFrom<u32> for SmallFormVariants {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SmallFormVariants {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SmallFormVariants {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        SmallFormVariants::SmallComma
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SmallFormVariants::SmallComma => "small comma",
            SmallFormVariants::SmallIdeographicComma => "small ideographic comma",
            SmallFormVariants::SmallFullStop => "small full stop",
            SmallFormVariants::SmallSemicolon => "small semicolon",
            SmallFormVariants::SmallColon => "small colon",
            SmallFormVariants::SmallQuestionMark => "small question mark",
            SmallFormVariants::SmallExclamationMark => "small exclamation mark",
            SmallFormVariants::SmallEmDash => "small em dash",
            SmallFormVariants::SmallLeftParenthesis => "small left parenthesis",
            SmallFormVariants::SmallRightParenthesis => "small right parenthesis",
            SmallFormVariants::SmallLeftCurlyBracket => "small left curly bracket",
            SmallFormVariants::SmallRightCurlyBracket => "small right curly bracket",
            SmallFormVariants::SmallLeftTortoiseShellBracket => "small left tortoise shell bracket",
            SmallFormVariants::SmallRightTortoiseShellBracket => "small right tortoise shell bracket",
            SmallFormVariants::SmallNumberSign => "small number sign",
            SmallFormVariants::SmallAmpersand => "small ampersand",
            SmallFormVariants::SmallAsterisk => "small asterisk",
            SmallFormVariants::SmallPlusSign => "small plus sign",
            SmallFormVariants::SmallHyphenDashMinus => "small hyphen-minus",
            SmallFormVariants::SmallLessDashThanSign => "small less-than sign",
            SmallFormVariants::SmallGreaterDashThanSign => "small greater-than sign",
            SmallFormVariants::SmallEqualsSign => "small equals sign",
            SmallFormVariants::SmallReverseSolidus => "small reverse solidus",
            SmallFormVariants::SmallDollarSign => "small dollar sign",
            SmallFormVariants::SmallPercentSign => "small percent sign",
            SmallFormVariants::SmallCommercialAt => "small commercial at",
        }
    }
}
