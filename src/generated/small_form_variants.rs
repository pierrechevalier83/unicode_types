
/// An enum to represent all characters in the SmallFormVariants block.
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
        match self {
            SmallFormVariants::SmallComma => '﹐',
            SmallFormVariants::SmallIdeographicComma => '﹑',
            SmallFormVariants::SmallFullStop => '﹒',
            SmallFormVariants::SmallSemicolon => '﹔',
            SmallFormVariants::SmallColon => '﹕',
            SmallFormVariants::SmallQuestionMark => '﹖',
            SmallFormVariants::SmallExclamationMark => '﹗',
            SmallFormVariants::SmallEmDash => '﹘',
            SmallFormVariants::SmallLeftParenthesis => '﹙',
            SmallFormVariants::SmallRightParenthesis => '﹚',
            SmallFormVariants::SmallLeftCurlyBracket => '﹛',
            SmallFormVariants::SmallRightCurlyBracket => '﹜',
            SmallFormVariants::SmallLeftTortoiseShellBracket => '﹝',
            SmallFormVariants::SmallRightTortoiseShellBracket => '﹞',
            SmallFormVariants::SmallNumberSign => '﹟',
            SmallFormVariants::SmallAmpersand => '﹠',
            SmallFormVariants::SmallAsterisk => '﹡',
            SmallFormVariants::SmallPlusSign => '﹢',
            SmallFormVariants::SmallHyphenDashMinus => '﹣',
            SmallFormVariants::SmallLessDashThanSign => '﹤',
            SmallFormVariants::SmallGreaterDashThanSign => '﹥',
            SmallFormVariants::SmallEqualsSign => '﹦',
            SmallFormVariants::SmallReverseSolidus => '﹨',
            SmallFormVariants::SmallDollarSign => '﹩',
            SmallFormVariants::SmallPercentSign => '﹪',
            SmallFormVariants::SmallCommercialAt => '﹫',
        }
    }
}

impl std::convert::TryFrom<char> for SmallFormVariants {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '﹐' => Ok(SmallFormVariants::SmallComma),
            '﹑' => Ok(SmallFormVariants::SmallIdeographicComma),
            '﹒' => Ok(SmallFormVariants::SmallFullStop),
            '﹔' => Ok(SmallFormVariants::SmallSemicolon),
            '﹕' => Ok(SmallFormVariants::SmallColon),
            '﹖' => Ok(SmallFormVariants::SmallQuestionMark),
            '﹗' => Ok(SmallFormVariants::SmallExclamationMark),
            '﹘' => Ok(SmallFormVariants::SmallEmDash),
            '﹙' => Ok(SmallFormVariants::SmallLeftParenthesis),
            '﹚' => Ok(SmallFormVariants::SmallRightParenthesis),
            '﹛' => Ok(SmallFormVariants::SmallLeftCurlyBracket),
            '﹜' => Ok(SmallFormVariants::SmallRightCurlyBracket),
            '﹝' => Ok(SmallFormVariants::SmallLeftTortoiseShellBracket),
            '﹞' => Ok(SmallFormVariants::SmallRightTortoiseShellBracket),
            '﹟' => Ok(SmallFormVariants::SmallNumberSign),
            '﹠' => Ok(SmallFormVariants::SmallAmpersand),
            '﹡' => Ok(SmallFormVariants::SmallAsterisk),
            '﹢' => Ok(SmallFormVariants::SmallPlusSign),
            '﹣' => Ok(SmallFormVariants::SmallHyphenDashMinus),
            '﹤' => Ok(SmallFormVariants::SmallLessDashThanSign),
            '﹥' => Ok(SmallFormVariants::SmallGreaterDashThanSign),
            '﹦' => Ok(SmallFormVariants::SmallEqualsSign),
            '﹨' => Ok(SmallFormVariants::SmallReverseSolidus),
            '﹩' => Ok(SmallFormVariants::SmallDollarSign),
            '﹪' => Ok(SmallFormVariants::SmallPercentSign),
            '﹫' => Ok(SmallFormVariants::SmallCommercialAt),
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
    /// The character with the lowest index in this unicode block
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
