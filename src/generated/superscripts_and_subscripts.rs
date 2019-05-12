/// \u{2070} → \u{209f}\
///\
/// ⁰ ⁱ ⁴ ⁵ ⁶ ⁷ ⁸ ⁹ ⁺ ⁻ ⁼ ⁽ ⁾ ⁿ ₀ ₁
/// ₂ ₃ ₄ ₅ ₆ ₇ ₈ ₉ ₊ ₋ ₌ ₍ ₎ ₐ ₑ ₒ
/// ₓ ₔ ₕ ₖ ₗ ₘ ₙ ₚ ₛ ₜ
pub mod constants {
    /// \u{2070}: '⁰'
    pub const SUPERSCRIPT_ZERO: char = '⁰';
    /// \u{2071}: 'ⁱ'
    pub const SUPERSCRIPT_LATIN_SMALL_LETTER_I: char = 'ⁱ';
    /// \u{2074}: '⁴'
    pub const SUPERSCRIPT_FOUR: char = '⁴';
    /// \u{2075}: '⁵'
    pub const SUPERSCRIPT_FIVE: char = '⁵';
    /// \u{2076}: '⁶'
    pub const SUPERSCRIPT_SIX: char = '⁶';
    /// \u{2077}: '⁷'
    pub const SUPERSCRIPT_SEVEN: char = '⁷';
    /// \u{2078}: '⁸'
    pub const SUPERSCRIPT_EIGHT: char = '⁸';
    /// \u{2079}: '⁹'
    pub const SUPERSCRIPT_NINE: char = '⁹';
    /// \u{207a}: '⁺'
    pub const SUPERSCRIPT_PLUS_SIGN: char = '⁺';
    /// \u{207b}: '⁻'
    pub const SUPERSCRIPT_MINUS: char = '⁻';
    /// \u{207c}: '⁼'
    pub const SUPERSCRIPT_EQUALS_SIGN: char = '⁼';
    /// \u{207d}: '⁽'
    pub const SUPERSCRIPT_LEFT_PARENTHESIS: char = '⁽';
    /// \u{207e}: '⁾'
    pub const SUPERSCRIPT_RIGHT_PARENTHESIS: char = '⁾';
    /// \u{207f}: 'ⁿ'
    pub const SUPERSCRIPT_LATIN_SMALL_LETTER_N: char = 'ⁿ';
    /// \u{2080}: '₀'
    pub const SUBSCRIPT_ZERO: char = '₀';
    /// \u{2081}: '₁'
    pub const SUBSCRIPT_ONE: char = '₁';
    /// \u{2082}: '₂'
    pub const SUBSCRIPT_TWO: char = '₂';
    /// \u{2083}: '₃'
    pub const SUBSCRIPT_THREE: char = '₃';
    /// \u{2084}: '₄'
    pub const SUBSCRIPT_FOUR: char = '₄';
    /// \u{2085}: '₅'
    pub const SUBSCRIPT_FIVE: char = '₅';
    /// \u{2086}: '₆'
    pub const SUBSCRIPT_SIX: char = '₆';
    /// \u{2087}: '₇'
    pub const SUBSCRIPT_SEVEN: char = '₇';
    /// \u{2088}: '₈'
    pub const SUBSCRIPT_EIGHT: char = '₈';
    /// \u{2089}: '₉'
    pub const SUBSCRIPT_NINE: char = '₉';
    /// \u{208a}: '₊'
    pub const SUBSCRIPT_PLUS_SIGN: char = '₊';
    /// \u{208b}: '₋'
    pub const SUBSCRIPT_MINUS: char = '₋';
    /// \u{208c}: '₌'
    pub const SUBSCRIPT_EQUALS_SIGN: char = '₌';
    /// \u{208d}: '₍'
    pub const SUBSCRIPT_LEFT_PARENTHESIS: char = '₍';
    /// \u{208e}: '₎'
    pub const SUBSCRIPT_RIGHT_PARENTHESIS: char = '₎';
    /// \u{2090}: 'ₐ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_A: char = 'ₐ';
    /// \u{2091}: 'ₑ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_E: char = 'ₑ';
    /// \u{2092}: 'ₒ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_O: char = 'ₒ';
    /// \u{2093}: 'ₓ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_X: char = 'ₓ';
    /// \u{2094}: 'ₔ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_SCHWA: char = 'ₔ';
    /// \u{2095}: 'ₕ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_H: char = 'ₕ';
    /// \u{2096}: 'ₖ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_K: char = 'ₖ';
    /// \u{2097}: 'ₗ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_L: char = 'ₗ';
    /// \u{2098}: 'ₘ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_M: char = 'ₘ';
    /// \u{2099}: 'ₙ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_N: char = 'ₙ';
    /// \u{209a}: 'ₚ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_P: char = 'ₚ';
    /// \u{209b}: 'ₛ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_S: char = 'ₛ';
    /// \u{209c}: 'ₜ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_T: char = 'ₜ';
}

/// \u{2070} → \u{209f}\
///\
/// ⁰ ⁱ ⁴ ⁵ ⁶ ⁷ ⁸ ⁹ ⁺ ⁻ ⁼ ⁽ ⁾ ⁿ ₀ ₁
/// ₂ ₃ ₄ ₅ ₆ ₇ ₈ ₉ ₊ ₋ ₌ ₍ ₎ ₐ ₑ ₒ
/// ₓ ₔ ₕ ₖ ₗ ₘ ₙ ₚ ₛ ₜ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SuperscriptsandSubscripts {
    /// \u{2070}: '⁰'
    SuperscriptZero,
    /// \u{2071}: 'ⁱ'
    SuperscriptLatinSmallLetterI,
    /// \u{2074}: '⁴'
    SuperscriptFour,
    /// \u{2075}: '⁵'
    SuperscriptFive,
    /// \u{2076}: '⁶'
    SuperscriptSix,
    /// \u{2077}: '⁷'
    SuperscriptSeven,
    /// \u{2078}: '⁸'
    SuperscriptEight,
    /// \u{2079}: '⁹'
    SuperscriptNine,
    /// \u{207a}: '⁺'
    SuperscriptPlusSign,
    /// \u{207b}: '⁻'
    SuperscriptMinus,
    /// \u{207c}: '⁼'
    SuperscriptEqualsSign,
    /// \u{207d}: '⁽'
    SuperscriptLeftParenthesis,
    /// \u{207e}: '⁾'
    SuperscriptRightParenthesis,
    /// \u{207f}: 'ⁿ'
    SuperscriptLatinSmallLetterN,
    /// \u{2080}: '₀'
    SubscriptZero,
    /// \u{2081}: '₁'
    SubscriptOne,
    /// \u{2082}: '₂'
    SubscriptTwo,
    /// \u{2083}: '₃'
    SubscriptThree,
    /// \u{2084}: '₄'
    SubscriptFour,
    /// \u{2085}: '₅'
    SubscriptFive,
    /// \u{2086}: '₆'
    SubscriptSix,
    /// \u{2087}: '₇'
    SubscriptSeven,
    /// \u{2088}: '₈'
    SubscriptEight,
    /// \u{2089}: '₉'
    SubscriptNine,
    /// \u{208a}: '₊'
    SubscriptPlusSign,
    /// \u{208b}: '₋'
    SubscriptMinus,
    /// \u{208c}: '₌'
    SubscriptEqualsSign,
    /// \u{208d}: '₍'
    SubscriptLeftParenthesis,
    /// \u{208e}: '₎'
    SubscriptRightParenthesis,
    /// \u{2090}: 'ₐ'
    LatinSubscriptSmallLetterA,
    /// \u{2091}: 'ₑ'
    LatinSubscriptSmallLetterE,
    /// \u{2092}: 'ₒ'
    LatinSubscriptSmallLetterO,
    /// \u{2093}: 'ₓ'
    LatinSubscriptSmallLetterX,
    /// \u{2094}: 'ₔ'
    LatinSubscriptSmallLetterSchwa,
    /// \u{2095}: 'ₕ'
    LatinSubscriptSmallLetterH,
    /// \u{2096}: 'ₖ'
    LatinSubscriptSmallLetterK,
    /// \u{2097}: 'ₗ'
    LatinSubscriptSmallLetterL,
    /// \u{2098}: 'ₘ'
    LatinSubscriptSmallLetterM,
    /// \u{2099}: 'ₙ'
    LatinSubscriptSmallLetterN,
    /// \u{209a}: 'ₚ'
    LatinSubscriptSmallLetterP,
    /// \u{209b}: 'ₛ'
    LatinSubscriptSmallLetterS,
    /// \u{209c}: 'ₜ'
    LatinSubscriptSmallLetterT,
}

impl Into<char> for SuperscriptsandSubscripts {
    fn into(self) -> char {
        use constants::*;
        match self {
            SuperscriptsandSubscripts::SuperscriptZero => SUPERSCRIPT_ZERO,
            SuperscriptsandSubscripts::SuperscriptLatinSmallLetterI => SUPERSCRIPT_LATIN_SMALL_LETTER_I,
            SuperscriptsandSubscripts::SuperscriptFour => SUPERSCRIPT_FOUR,
            SuperscriptsandSubscripts::SuperscriptFive => SUPERSCRIPT_FIVE,
            SuperscriptsandSubscripts::SuperscriptSix => SUPERSCRIPT_SIX,
            SuperscriptsandSubscripts::SuperscriptSeven => SUPERSCRIPT_SEVEN,
            SuperscriptsandSubscripts::SuperscriptEight => SUPERSCRIPT_EIGHT,
            SuperscriptsandSubscripts::SuperscriptNine => SUPERSCRIPT_NINE,
            SuperscriptsandSubscripts::SuperscriptPlusSign => SUPERSCRIPT_PLUS_SIGN,
            SuperscriptsandSubscripts::SuperscriptMinus => SUPERSCRIPT_MINUS,
            SuperscriptsandSubscripts::SuperscriptEqualsSign => SUPERSCRIPT_EQUALS_SIGN,
            SuperscriptsandSubscripts::SuperscriptLeftParenthesis => SUPERSCRIPT_LEFT_PARENTHESIS,
            SuperscriptsandSubscripts::SuperscriptRightParenthesis => SUPERSCRIPT_RIGHT_PARENTHESIS,
            SuperscriptsandSubscripts::SuperscriptLatinSmallLetterN => SUPERSCRIPT_LATIN_SMALL_LETTER_N,
            SuperscriptsandSubscripts::SubscriptZero => SUBSCRIPT_ZERO,
            SuperscriptsandSubscripts::SubscriptOne => SUBSCRIPT_ONE,
            SuperscriptsandSubscripts::SubscriptTwo => SUBSCRIPT_TWO,
            SuperscriptsandSubscripts::SubscriptThree => SUBSCRIPT_THREE,
            SuperscriptsandSubscripts::SubscriptFour => SUBSCRIPT_FOUR,
            SuperscriptsandSubscripts::SubscriptFive => SUBSCRIPT_FIVE,
            SuperscriptsandSubscripts::SubscriptSix => SUBSCRIPT_SIX,
            SuperscriptsandSubscripts::SubscriptSeven => SUBSCRIPT_SEVEN,
            SuperscriptsandSubscripts::SubscriptEight => SUBSCRIPT_EIGHT,
            SuperscriptsandSubscripts::SubscriptNine => SUBSCRIPT_NINE,
            SuperscriptsandSubscripts::SubscriptPlusSign => SUBSCRIPT_PLUS_SIGN,
            SuperscriptsandSubscripts::SubscriptMinus => SUBSCRIPT_MINUS,
            SuperscriptsandSubscripts::SubscriptEqualsSign => SUBSCRIPT_EQUALS_SIGN,
            SuperscriptsandSubscripts::SubscriptLeftParenthesis => SUBSCRIPT_LEFT_PARENTHESIS,
            SuperscriptsandSubscripts::SubscriptRightParenthesis => SUBSCRIPT_RIGHT_PARENTHESIS,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterA => LATIN_SUBSCRIPT_SMALL_LETTER_A,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterE => LATIN_SUBSCRIPT_SMALL_LETTER_E,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterO => LATIN_SUBSCRIPT_SMALL_LETTER_O,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterX => LATIN_SUBSCRIPT_SMALL_LETTER_X,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterSchwa => LATIN_SUBSCRIPT_SMALL_LETTER_SCHWA,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterH => LATIN_SUBSCRIPT_SMALL_LETTER_H,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterK => LATIN_SUBSCRIPT_SMALL_LETTER_K,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterL => LATIN_SUBSCRIPT_SMALL_LETTER_L,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterM => LATIN_SUBSCRIPT_SMALL_LETTER_M,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterN => LATIN_SUBSCRIPT_SMALL_LETTER_N,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterP => LATIN_SUBSCRIPT_SMALL_LETTER_P,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterS => LATIN_SUBSCRIPT_SMALL_LETTER_S,
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterT => LATIN_SUBSCRIPT_SMALL_LETTER_T,
        }
    }
}

impl std::convert::TryFrom<char> for SuperscriptsandSubscripts {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SUPERSCRIPT_ZERO => Ok(SuperscriptsandSubscripts::SuperscriptZero),
            SUPERSCRIPT_LATIN_SMALL_LETTER_I => Ok(SuperscriptsandSubscripts::SuperscriptLatinSmallLetterI),
            SUPERSCRIPT_FOUR => Ok(SuperscriptsandSubscripts::SuperscriptFour),
            SUPERSCRIPT_FIVE => Ok(SuperscriptsandSubscripts::SuperscriptFive),
            SUPERSCRIPT_SIX => Ok(SuperscriptsandSubscripts::SuperscriptSix),
            SUPERSCRIPT_SEVEN => Ok(SuperscriptsandSubscripts::SuperscriptSeven),
            SUPERSCRIPT_EIGHT => Ok(SuperscriptsandSubscripts::SuperscriptEight),
            SUPERSCRIPT_NINE => Ok(SuperscriptsandSubscripts::SuperscriptNine),
            SUPERSCRIPT_PLUS_SIGN => Ok(SuperscriptsandSubscripts::SuperscriptPlusSign),
            SUPERSCRIPT_MINUS => Ok(SuperscriptsandSubscripts::SuperscriptMinus),
            SUPERSCRIPT_EQUALS_SIGN => Ok(SuperscriptsandSubscripts::SuperscriptEqualsSign),
            SUPERSCRIPT_LEFT_PARENTHESIS => Ok(SuperscriptsandSubscripts::SuperscriptLeftParenthesis),
            SUPERSCRIPT_RIGHT_PARENTHESIS => Ok(SuperscriptsandSubscripts::SuperscriptRightParenthesis),
            SUPERSCRIPT_LATIN_SMALL_LETTER_N => Ok(SuperscriptsandSubscripts::SuperscriptLatinSmallLetterN),
            SUBSCRIPT_ZERO => Ok(SuperscriptsandSubscripts::SubscriptZero),
            SUBSCRIPT_ONE => Ok(SuperscriptsandSubscripts::SubscriptOne),
            SUBSCRIPT_TWO => Ok(SuperscriptsandSubscripts::SubscriptTwo),
            SUBSCRIPT_THREE => Ok(SuperscriptsandSubscripts::SubscriptThree),
            SUBSCRIPT_FOUR => Ok(SuperscriptsandSubscripts::SubscriptFour),
            SUBSCRIPT_FIVE => Ok(SuperscriptsandSubscripts::SubscriptFive),
            SUBSCRIPT_SIX => Ok(SuperscriptsandSubscripts::SubscriptSix),
            SUBSCRIPT_SEVEN => Ok(SuperscriptsandSubscripts::SubscriptSeven),
            SUBSCRIPT_EIGHT => Ok(SuperscriptsandSubscripts::SubscriptEight),
            SUBSCRIPT_NINE => Ok(SuperscriptsandSubscripts::SubscriptNine),
            SUBSCRIPT_PLUS_SIGN => Ok(SuperscriptsandSubscripts::SubscriptPlusSign),
            SUBSCRIPT_MINUS => Ok(SuperscriptsandSubscripts::SubscriptMinus),
            SUBSCRIPT_EQUALS_SIGN => Ok(SuperscriptsandSubscripts::SubscriptEqualsSign),
            SUBSCRIPT_LEFT_PARENTHESIS => Ok(SuperscriptsandSubscripts::SubscriptLeftParenthesis),
            SUBSCRIPT_RIGHT_PARENTHESIS => Ok(SuperscriptsandSubscripts::SubscriptRightParenthesis),
            LATIN_SUBSCRIPT_SMALL_LETTER_A => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterA),
            LATIN_SUBSCRIPT_SMALL_LETTER_E => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterE),
            LATIN_SUBSCRIPT_SMALL_LETTER_O => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterO),
            LATIN_SUBSCRIPT_SMALL_LETTER_X => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterX),
            LATIN_SUBSCRIPT_SMALL_LETTER_SCHWA => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterSchwa),
            LATIN_SUBSCRIPT_SMALL_LETTER_H => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterH),
            LATIN_SUBSCRIPT_SMALL_LETTER_K => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterK),
            LATIN_SUBSCRIPT_SMALL_LETTER_L => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterL),
            LATIN_SUBSCRIPT_SMALL_LETTER_M => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterM),
            LATIN_SUBSCRIPT_SMALL_LETTER_N => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterN),
            LATIN_SUBSCRIPT_SMALL_LETTER_P => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterP),
            LATIN_SUBSCRIPT_SMALL_LETTER_S => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterS),
            LATIN_SUBSCRIPT_SMALL_LETTER_T => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterT),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SuperscriptsandSubscripts {
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

impl std::convert::TryFrom<u32> for SuperscriptsandSubscripts {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SuperscriptsandSubscripts {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SuperscriptsandSubscripts {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        SuperscriptsandSubscripts::SuperscriptZero
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SuperscriptsandSubscripts::SuperscriptZero => "superscript zero",
            SuperscriptsandSubscripts::SuperscriptLatinSmallLetterI => "superscript latin small letter i",
            SuperscriptsandSubscripts::SuperscriptFour => "superscript four",
            SuperscriptsandSubscripts::SuperscriptFive => "superscript five",
            SuperscriptsandSubscripts::SuperscriptSix => "superscript six",
            SuperscriptsandSubscripts::SuperscriptSeven => "superscript seven",
            SuperscriptsandSubscripts::SuperscriptEight => "superscript eight",
            SuperscriptsandSubscripts::SuperscriptNine => "superscript nine",
            SuperscriptsandSubscripts::SuperscriptPlusSign => "superscript plus sign",
            SuperscriptsandSubscripts::SuperscriptMinus => "superscript minus",
            SuperscriptsandSubscripts::SuperscriptEqualsSign => "superscript equals sign",
            SuperscriptsandSubscripts::SuperscriptLeftParenthesis => "superscript left parenthesis",
            SuperscriptsandSubscripts::SuperscriptRightParenthesis => "superscript right parenthesis",
            SuperscriptsandSubscripts::SuperscriptLatinSmallLetterN => "superscript latin small letter n",
            SuperscriptsandSubscripts::SubscriptZero => "subscript zero",
            SuperscriptsandSubscripts::SubscriptOne => "subscript one",
            SuperscriptsandSubscripts::SubscriptTwo => "subscript two",
            SuperscriptsandSubscripts::SubscriptThree => "subscript three",
            SuperscriptsandSubscripts::SubscriptFour => "subscript four",
            SuperscriptsandSubscripts::SubscriptFive => "subscript five",
            SuperscriptsandSubscripts::SubscriptSix => "subscript six",
            SuperscriptsandSubscripts::SubscriptSeven => "subscript seven",
            SuperscriptsandSubscripts::SubscriptEight => "subscript eight",
            SuperscriptsandSubscripts::SubscriptNine => "subscript nine",
            SuperscriptsandSubscripts::SubscriptPlusSign => "subscript plus sign",
            SuperscriptsandSubscripts::SubscriptMinus => "subscript minus",
            SuperscriptsandSubscripts::SubscriptEqualsSign => "subscript equals sign",
            SuperscriptsandSubscripts::SubscriptLeftParenthesis => "subscript left parenthesis",
            SuperscriptsandSubscripts::SubscriptRightParenthesis => "subscript right parenthesis",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterA => "latin subscript small letter a",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterE => "latin subscript small letter e",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterO => "latin subscript small letter o",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterX => "latin subscript small letter x",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterSchwa => "latin subscript small letter schwa",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterH => "latin subscript small letter h",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterK => "latin subscript small letter k",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterL => "latin subscript small letter l",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterM => "latin subscript small letter m",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterN => "latin subscript small letter n",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterP => "latin subscript small letter p",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterS => "latin subscript small letter s",
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterT => "latin subscript small letter t",
        }
    }
}
