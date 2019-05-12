
/// An enum to represent all characters in the SuperscriptsandSubscripts block.
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
        match self {
            SuperscriptsandSubscripts::SuperscriptZero => '⁰',
            SuperscriptsandSubscripts::SuperscriptLatinSmallLetterI => 'ⁱ',
            SuperscriptsandSubscripts::SuperscriptFour => '⁴',
            SuperscriptsandSubscripts::SuperscriptFive => '⁵',
            SuperscriptsandSubscripts::SuperscriptSix => '⁶',
            SuperscriptsandSubscripts::SuperscriptSeven => '⁷',
            SuperscriptsandSubscripts::SuperscriptEight => '⁸',
            SuperscriptsandSubscripts::SuperscriptNine => '⁹',
            SuperscriptsandSubscripts::SuperscriptPlusSign => '⁺',
            SuperscriptsandSubscripts::SuperscriptMinus => '⁻',
            SuperscriptsandSubscripts::SuperscriptEqualsSign => '⁼',
            SuperscriptsandSubscripts::SuperscriptLeftParenthesis => '⁽',
            SuperscriptsandSubscripts::SuperscriptRightParenthesis => '⁾',
            SuperscriptsandSubscripts::SuperscriptLatinSmallLetterN => 'ⁿ',
            SuperscriptsandSubscripts::SubscriptZero => '₀',
            SuperscriptsandSubscripts::SubscriptOne => '₁',
            SuperscriptsandSubscripts::SubscriptTwo => '₂',
            SuperscriptsandSubscripts::SubscriptThree => '₃',
            SuperscriptsandSubscripts::SubscriptFour => '₄',
            SuperscriptsandSubscripts::SubscriptFive => '₅',
            SuperscriptsandSubscripts::SubscriptSix => '₆',
            SuperscriptsandSubscripts::SubscriptSeven => '₇',
            SuperscriptsandSubscripts::SubscriptEight => '₈',
            SuperscriptsandSubscripts::SubscriptNine => '₉',
            SuperscriptsandSubscripts::SubscriptPlusSign => '₊',
            SuperscriptsandSubscripts::SubscriptMinus => '₋',
            SuperscriptsandSubscripts::SubscriptEqualsSign => '₌',
            SuperscriptsandSubscripts::SubscriptLeftParenthesis => '₍',
            SuperscriptsandSubscripts::SubscriptRightParenthesis => '₎',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterA => 'ₐ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterE => 'ₑ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterO => 'ₒ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterX => 'ₓ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterSchwa => 'ₔ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterH => 'ₕ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterK => 'ₖ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterL => 'ₗ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterM => 'ₘ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterN => 'ₙ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterP => 'ₚ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterS => 'ₛ',
            SuperscriptsandSubscripts::LatinSubscriptSmallLetterT => 'ₜ',
        }
    }
}

impl std::convert::TryFrom<char> for SuperscriptsandSubscripts {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⁰' => Ok(SuperscriptsandSubscripts::SuperscriptZero),
            'ⁱ' => Ok(SuperscriptsandSubscripts::SuperscriptLatinSmallLetterI),
            '⁴' => Ok(SuperscriptsandSubscripts::SuperscriptFour),
            '⁵' => Ok(SuperscriptsandSubscripts::SuperscriptFive),
            '⁶' => Ok(SuperscriptsandSubscripts::SuperscriptSix),
            '⁷' => Ok(SuperscriptsandSubscripts::SuperscriptSeven),
            '⁸' => Ok(SuperscriptsandSubscripts::SuperscriptEight),
            '⁹' => Ok(SuperscriptsandSubscripts::SuperscriptNine),
            '⁺' => Ok(SuperscriptsandSubscripts::SuperscriptPlusSign),
            '⁻' => Ok(SuperscriptsandSubscripts::SuperscriptMinus),
            '⁼' => Ok(SuperscriptsandSubscripts::SuperscriptEqualsSign),
            '⁽' => Ok(SuperscriptsandSubscripts::SuperscriptLeftParenthesis),
            '⁾' => Ok(SuperscriptsandSubscripts::SuperscriptRightParenthesis),
            'ⁿ' => Ok(SuperscriptsandSubscripts::SuperscriptLatinSmallLetterN),
            '₀' => Ok(SuperscriptsandSubscripts::SubscriptZero),
            '₁' => Ok(SuperscriptsandSubscripts::SubscriptOne),
            '₂' => Ok(SuperscriptsandSubscripts::SubscriptTwo),
            '₃' => Ok(SuperscriptsandSubscripts::SubscriptThree),
            '₄' => Ok(SuperscriptsandSubscripts::SubscriptFour),
            '₅' => Ok(SuperscriptsandSubscripts::SubscriptFive),
            '₆' => Ok(SuperscriptsandSubscripts::SubscriptSix),
            '₇' => Ok(SuperscriptsandSubscripts::SubscriptSeven),
            '₈' => Ok(SuperscriptsandSubscripts::SubscriptEight),
            '₉' => Ok(SuperscriptsandSubscripts::SubscriptNine),
            '₊' => Ok(SuperscriptsandSubscripts::SubscriptPlusSign),
            '₋' => Ok(SuperscriptsandSubscripts::SubscriptMinus),
            '₌' => Ok(SuperscriptsandSubscripts::SubscriptEqualsSign),
            '₍' => Ok(SuperscriptsandSubscripts::SubscriptLeftParenthesis),
            '₎' => Ok(SuperscriptsandSubscripts::SubscriptRightParenthesis),
            'ₐ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterA),
            'ₑ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterE),
            'ₒ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterO),
            'ₓ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterX),
            'ₔ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterSchwa),
            'ₕ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterH),
            'ₖ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterK),
            'ₗ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterL),
            'ₘ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterM),
            'ₙ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterN),
            'ₚ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterP),
            'ₛ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterS),
            'ₜ' => Ok(SuperscriptsandSubscripts::LatinSubscriptSmallLetterT),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SuperscriptsandSubscripts::SuperscriptZero
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SuperscriptsandSubscripts{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
