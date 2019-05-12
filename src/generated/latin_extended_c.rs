/// \u{2c60} → \u{2c7f}\
///\
/// Ⱡ ⱡ Ɫ Ᵽ Ɽ ⱥ ⱦ Ⱨ ⱨ Ⱪ ⱪ Ⱬ ⱬ Ɑ Ɱ Ɐ
/// Ɒ ⱱ Ⱳ ⱳ ⱴ Ⱶ ⱶ ⱷ ⱸ ⱹ ⱺ ⱻ ⱼ ⱽ Ȿ
pub mod constants {
    /// \u{2c60}: 'Ⱡ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_DOUBLE_BAR: char = 'Ⱡ';
    /// \u{2c61}: 'ⱡ'
    pub const LATIN_SMALL_LETTER_L_WITH_DOUBLE_BAR: char = 'ⱡ';
    /// \u{2c62}: 'Ɫ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_MIDDLE_TILDE: char = 'Ɫ';
    /// \u{2c63}: 'Ᵽ'
    pub const LATIN_CAPITAL_LETTER_P_WITH_STROKE: char = 'Ᵽ';
    /// \u{2c64}: 'Ɽ'
    pub const LATIN_CAPITAL_LETTER_R_WITH_TAIL: char = 'Ɽ';
    /// \u{2c65}: 'ⱥ'
    pub const LATIN_SMALL_LETTER_A_WITH_STROKE: char = 'ⱥ';
    /// \u{2c66}: 'ⱦ'
    pub const LATIN_SMALL_LETTER_T_WITH_DIAGONAL_STROKE: char = 'ⱦ';
    /// \u{2c67}: 'Ⱨ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_DESCENDER: char = 'Ⱨ';
    /// \u{2c68}: 'ⱨ'
    pub const LATIN_SMALL_LETTER_H_WITH_DESCENDER: char = 'ⱨ';
    /// \u{2c69}: 'Ⱪ'
    pub const LATIN_CAPITAL_LETTER_K_WITH_DESCENDER: char = 'Ⱪ';
    /// \u{2c6a}: 'ⱪ'
    pub const LATIN_SMALL_LETTER_K_WITH_DESCENDER: char = 'ⱪ';
    /// \u{2c6b}: 'Ⱬ'
    pub const LATIN_CAPITAL_LETTER_Z_WITH_DESCENDER: char = 'Ⱬ';
    /// \u{2c6c}: 'ⱬ'
    pub const LATIN_SMALL_LETTER_Z_WITH_DESCENDER: char = 'ⱬ';
    /// \u{2c6d}: 'Ɑ'
    pub const LATIN_CAPITAL_LETTER_ALPHA: char = 'Ɑ';
    /// \u{2c6e}: 'Ɱ'
    pub const LATIN_CAPITAL_LETTER_M_WITH_HOOK: char = 'Ɱ';
    /// \u{2c6f}: 'Ɐ'
    pub const LATIN_CAPITAL_LETTER_TURNED_A: char = 'Ɐ';
    /// \u{2c70}: 'Ɒ'
    pub const LATIN_CAPITAL_LETTER_TURNED_ALPHA: char = 'Ɒ';
    /// \u{2c71}: 'ⱱ'
    pub const LATIN_SMALL_LETTER_V_WITH_RIGHT_HOOK: char = 'ⱱ';
    /// \u{2c72}: 'Ⱳ'
    pub const LATIN_CAPITAL_LETTER_W_WITH_HOOK: char = 'Ⱳ';
    /// \u{2c73}: 'ⱳ'
    pub const LATIN_SMALL_LETTER_W_WITH_HOOK: char = 'ⱳ';
    /// \u{2c74}: 'ⱴ'
    pub const LATIN_SMALL_LETTER_V_WITH_CURL: char = 'ⱴ';
    /// \u{2c75}: 'Ⱶ'
    pub const LATIN_CAPITAL_LETTER_HALF_H: char = 'Ⱶ';
    /// \u{2c76}: 'ⱶ'
    pub const LATIN_SMALL_LETTER_HALF_H: char = 'ⱶ';
    /// \u{2c77}: 'ⱷ'
    pub const LATIN_SMALL_LETTER_TAILLESS_PHI: char = 'ⱷ';
    /// \u{2c78}: 'ⱸ'
    pub const LATIN_SMALL_LETTER_E_WITH_NOTCH: char = 'ⱸ';
    /// \u{2c79}: 'ⱹ'
    pub const LATIN_SMALL_LETTER_TURNED_R_WITH_TAIL: char = 'ⱹ';
    /// \u{2c7a}: 'ⱺ'
    pub const LATIN_SMALL_LETTER_O_WITH_LOW_RING_INSIDE: char = 'ⱺ';
    /// \u{2c7b}: 'ⱻ'
    pub const LATIN_LETTER_SMALL_CAPITAL_TURNED_E: char = 'ⱻ';
    /// \u{2c7c}: 'ⱼ'
    pub const LATIN_SUBSCRIPT_SMALL_LETTER_J: char = 'ⱼ';
    /// \u{2c7d}: 'ⱽ'
    pub const MODIFIER_LETTER_CAPITAL_V: char = 'ⱽ';
    /// \u{2c7e}: 'Ȿ'
    pub const LATIN_CAPITAL_LETTER_S_WITH_SWASH_TAIL: char = 'Ȿ';
}

/// \u{2c60} → \u{2c7f}\
///\
/// Ⱡ ⱡ Ɫ Ᵽ Ɽ ⱥ ⱦ Ⱨ ⱨ Ⱪ ⱪ Ⱬ ⱬ Ɑ Ɱ Ɐ
/// Ɒ ⱱ Ⱳ ⱳ ⱴ Ⱶ ⱶ ⱷ ⱸ ⱹ ⱺ ⱻ ⱼ ⱽ Ȿ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedC {
    /// \u{2c60}: 'Ⱡ'
    LatinCapitalLetterLWithDoubleBar,
    /// \u{2c61}: 'ⱡ'
    LatinSmallLetterLWithDoubleBar,
    /// \u{2c62}: 'Ɫ'
    LatinCapitalLetterLWithMiddleTilde,
    /// \u{2c63}: 'Ᵽ'
    LatinCapitalLetterPWithStroke,
    /// \u{2c64}: 'Ɽ'
    LatinCapitalLetterRWithTail,
    /// \u{2c65}: 'ⱥ'
    LatinSmallLetterAWithStroke,
    /// \u{2c66}: 'ⱦ'
    LatinSmallLetterTWithDiagonalStroke,
    /// \u{2c67}: 'Ⱨ'
    LatinCapitalLetterHWithDescender,
    /// \u{2c68}: 'ⱨ'
    LatinSmallLetterHWithDescender,
    /// \u{2c69}: 'Ⱪ'
    LatinCapitalLetterKWithDescender,
    /// \u{2c6a}: 'ⱪ'
    LatinSmallLetterKWithDescender,
    /// \u{2c6b}: 'Ⱬ'
    LatinCapitalLetterZWithDescender,
    /// \u{2c6c}: 'ⱬ'
    LatinSmallLetterZWithDescender,
    /// \u{2c6d}: 'Ɑ'
    LatinCapitalLetterAlpha,
    /// \u{2c6e}: 'Ɱ'
    LatinCapitalLetterMWithHook,
    /// \u{2c6f}: 'Ɐ'
    LatinCapitalLetterTurnedA,
    /// \u{2c70}: 'Ɒ'
    LatinCapitalLetterTurnedAlpha,
    /// \u{2c71}: 'ⱱ'
    LatinSmallLetterVWithRightHook,
    /// \u{2c72}: 'Ⱳ'
    LatinCapitalLetterWWithHook,
    /// \u{2c73}: 'ⱳ'
    LatinSmallLetterWWithHook,
    /// \u{2c74}: 'ⱴ'
    LatinSmallLetterVWithCurl,
    /// \u{2c75}: 'Ⱶ'
    LatinCapitalLetterHalfH,
    /// \u{2c76}: 'ⱶ'
    LatinSmallLetterHalfH,
    /// \u{2c77}: 'ⱷ'
    LatinSmallLetterTaillessPhi,
    /// \u{2c78}: 'ⱸ'
    LatinSmallLetterEWithNotch,
    /// \u{2c79}: 'ⱹ'
    LatinSmallLetterTurnedRWithTail,
    /// \u{2c7a}: 'ⱺ'
    LatinSmallLetterOWithLowRingInside,
    /// \u{2c7b}: 'ⱻ'
    LatinLetterSmallCapitalTurnedE,
    /// \u{2c7c}: 'ⱼ'
    LatinSubscriptSmallLetterJ,
    /// \u{2c7d}: 'ⱽ'
    ModifierLetterCapitalV,
    /// \u{2c7e}: 'Ȿ'
    LatinCapitalLetterSWithSwashTail,
}

impl Into<char> for LatinExtendedC {
    fn into(self) -> char {
        use constants::*;
        match self {
            LatinExtendedC::LatinCapitalLetterLWithDoubleBar => LATIN_CAPITAL_LETTER_L_WITH_DOUBLE_BAR,
            LatinExtendedC::LatinSmallLetterLWithDoubleBar => LATIN_SMALL_LETTER_L_WITH_DOUBLE_BAR,
            LatinExtendedC::LatinCapitalLetterLWithMiddleTilde => LATIN_CAPITAL_LETTER_L_WITH_MIDDLE_TILDE,
            LatinExtendedC::LatinCapitalLetterPWithStroke => LATIN_CAPITAL_LETTER_P_WITH_STROKE,
            LatinExtendedC::LatinCapitalLetterRWithTail => LATIN_CAPITAL_LETTER_R_WITH_TAIL,
            LatinExtendedC::LatinSmallLetterAWithStroke => LATIN_SMALL_LETTER_A_WITH_STROKE,
            LatinExtendedC::LatinSmallLetterTWithDiagonalStroke => LATIN_SMALL_LETTER_T_WITH_DIAGONAL_STROKE,
            LatinExtendedC::LatinCapitalLetterHWithDescender => LATIN_CAPITAL_LETTER_H_WITH_DESCENDER,
            LatinExtendedC::LatinSmallLetterHWithDescender => LATIN_SMALL_LETTER_H_WITH_DESCENDER,
            LatinExtendedC::LatinCapitalLetterKWithDescender => LATIN_CAPITAL_LETTER_K_WITH_DESCENDER,
            LatinExtendedC::LatinSmallLetterKWithDescender => LATIN_SMALL_LETTER_K_WITH_DESCENDER,
            LatinExtendedC::LatinCapitalLetterZWithDescender => LATIN_CAPITAL_LETTER_Z_WITH_DESCENDER,
            LatinExtendedC::LatinSmallLetterZWithDescender => LATIN_SMALL_LETTER_Z_WITH_DESCENDER,
            LatinExtendedC::LatinCapitalLetterAlpha => LATIN_CAPITAL_LETTER_ALPHA,
            LatinExtendedC::LatinCapitalLetterMWithHook => LATIN_CAPITAL_LETTER_M_WITH_HOOK,
            LatinExtendedC::LatinCapitalLetterTurnedA => LATIN_CAPITAL_LETTER_TURNED_A,
            LatinExtendedC::LatinCapitalLetterTurnedAlpha => LATIN_CAPITAL_LETTER_TURNED_ALPHA,
            LatinExtendedC::LatinSmallLetterVWithRightHook => LATIN_SMALL_LETTER_V_WITH_RIGHT_HOOK,
            LatinExtendedC::LatinCapitalLetterWWithHook => LATIN_CAPITAL_LETTER_W_WITH_HOOK,
            LatinExtendedC::LatinSmallLetterWWithHook => LATIN_SMALL_LETTER_W_WITH_HOOK,
            LatinExtendedC::LatinSmallLetterVWithCurl => LATIN_SMALL_LETTER_V_WITH_CURL,
            LatinExtendedC::LatinCapitalLetterHalfH => LATIN_CAPITAL_LETTER_HALF_H,
            LatinExtendedC::LatinSmallLetterHalfH => LATIN_SMALL_LETTER_HALF_H,
            LatinExtendedC::LatinSmallLetterTaillessPhi => LATIN_SMALL_LETTER_TAILLESS_PHI,
            LatinExtendedC::LatinSmallLetterEWithNotch => LATIN_SMALL_LETTER_E_WITH_NOTCH,
            LatinExtendedC::LatinSmallLetterTurnedRWithTail => LATIN_SMALL_LETTER_TURNED_R_WITH_TAIL,
            LatinExtendedC::LatinSmallLetterOWithLowRingInside => LATIN_SMALL_LETTER_O_WITH_LOW_RING_INSIDE,
            LatinExtendedC::LatinLetterSmallCapitalTurnedE => LATIN_LETTER_SMALL_CAPITAL_TURNED_E,
            LatinExtendedC::LatinSubscriptSmallLetterJ => LATIN_SUBSCRIPT_SMALL_LETTER_J,
            LatinExtendedC::ModifierLetterCapitalV => MODIFIER_LETTER_CAPITAL_V,
            LatinExtendedC::LatinCapitalLetterSWithSwashTail => LATIN_CAPITAL_LETTER_S_WITH_SWASH_TAIL,
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedC {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LATIN_CAPITAL_LETTER_L_WITH_DOUBLE_BAR => Ok(LatinExtendedC::LatinCapitalLetterLWithDoubleBar),
            LATIN_SMALL_LETTER_L_WITH_DOUBLE_BAR => Ok(LatinExtendedC::LatinSmallLetterLWithDoubleBar),
            LATIN_CAPITAL_LETTER_L_WITH_MIDDLE_TILDE => Ok(LatinExtendedC::LatinCapitalLetterLWithMiddleTilde),
            LATIN_CAPITAL_LETTER_P_WITH_STROKE => Ok(LatinExtendedC::LatinCapitalLetterPWithStroke),
            LATIN_CAPITAL_LETTER_R_WITH_TAIL => Ok(LatinExtendedC::LatinCapitalLetterRWithTail),
            LATIN_SMALL_LETTER_A_WITH_STROKE => Ok(LatinExtendedC::LatinSmallLetterAWithStroke),
            LATIN_SMALL_LETTER_T_WITH_DIAGONAL_STROKE => Ok(LatinExtendedC::LatinSmallLetterTWithDiagonalStroke),
            LATIN_CAPITAL_LETTER_H_WITH_DESCENDER => Ok(LatinExtendedC::LatinCapitalLetterHWithDescender),
            LATIN_SMALL_LETTER_H_WITH_DESCENDER => Ok(LatinExtendedC::LatinSmallLetterHWithDescender),
            LATIN_CAPITAL_LETTER_K_WITH_DESCENDER => Ok(LatinExtendedC::LatinCapitalLetterKWithDescender),
            LATIN_SMALL_LETTER_K_WITH_DESCENDER => Ok(LatinExtendedC::LatinSmallLetterKWithDescender),
            LATIN_CAPITAL_LETTER_Z_WITH_DESCENDER => Ok(LatinExtendedC::LatinCapitalLetterZWithDescender),
            LATIN_SMALL_LETTER_Z_WITH_DESCENDER => Ok(LatinExtendedC::LatinSmallLetterZWithDescender),
            LATIN_CAPITAL_LETTER_ALPHA => Ok(LatinExtendedC::LatinCapitalLetterAlpha),
            LATIN_CAPITAL_LETTER_M_WITH_HOOK => Ok(LatinExtendedC::LatinCapitalLetterMWithHook),
            LATIN_CAPITAL_LETTER_TURNED_A => Ok(LatinExtendedC::LatinCapitalLetterTurnedA),
            LATIN_CAPITAL_LETTER_TURNED_ALPHA => Ok(LatinExtendedC::LatinCapitalLetterTurnedAlpha),
            LATIN_SMALL_LETTER_V_WITH_RIGHT_HOOK => Ok(LatinExtendedC::LatinSmallLetterVWithRightHook),
            LATIN_CAPITAL_LETTER_W_WITH_HOOK => Ok(LatinExtendedC::LatinCapitalLetterWWithHook),
            LATIN_SMALL_LETTER_W_WITH_HOOK => Ok(LatinExtendedC::LatinSmallLetterWWithHook),
            LATIN_SMALL_LETTER_V_WITH_CURL => Ok(LatinExtendedC::LatinSmallLetterVWithCurl),
            LATIN_CAPITAL_LETTER_HALF_H => Ok(LatinExtendedC::LatinCapitalLetterHalfH),
            LATIN_SMALL_LETTER_HALF_H => Ok(LatinExtendedC::LatinSmallLetterHalfH),
            LATIN_SMALL_LETTER_TAILLESS_PHI => Ok(LatinExtendedC::LatinSmallLetterTaillessPhi),
            LATIN_SMALL_LETTER_E_WITH_NOTCH => Ok(LatinExtendedC::LatinSmallLetterEWithNotch),
            LATIN_SMALL_LETTER_TURNED_R_WITH_TAIL => Ok(LatinExtendedC::LatinSmallLetterTurnedRWithTail),
            LATIN_SMALL_LETTER_O_WITH_LOW_RING_INSIDE => Ok(LatinExtendedC::LatinSmallLetterOWithLowRingInside),
            LATIN_LETTER_SMALL_CAPITAL_TURNED_E => Ok(LatinExtendedC::LatinLetterSmallCapitalTurnedE),
            LATIN_SUBSCRIPT_SMALL_LETTER_J => Ok(LatinExtendedC::LatinSubscriptSmallLetterJ),
            MODIFIER_LETTER_CAPITAL_V => Ok(LatinExtendedC::ModifierLetterCapitalV),
            LATIN_CAPITAL_LETTER_S_WITH_SWASH_TAIL => Ok(LatinExtendedC::LatinCapitalLetterSWithSwashTail),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedC {
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

impl std::convert::TryFrom<u32> for LatinExtendedC {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedC {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedC {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        LatinExtendedC::LatinCapitalLetterLWithDoubleBar
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            LatinExtendedC::LatinCapitalLetterLWithDoubleBar => "latin capital letter l with double bar",
            LatinExtendedC::LatinSmallLetterLWithDoubleBar => "latin small letter l with double bar",
            LatinExtendedC::LatinCapitalLetterLWithMiddleTilde => "latin capital letter l with middle tilde",
            LatinExtendedC::LatinCapitalLetterPWithStroke => "latin capital letter p with stroke",
            LatinExtendedC::LatinCapitalLetterRWithTail => "latin capital letter r with tail",
            LatinExtendedC::LatinSmallLetterAWithStroke => "latin small letter a with stroke",
            LatinExtendedC::LatinSmallLetterTWithDiagonalStroke => "latin small letter t with diagonal stroke",
            LatinExtendedC::LatinCapitalLetterHWithDescender => "latin capital letter h with descender",
            LatinExtendedC::LatinSmallLetterHWithDescender => "latin small letter h with descender",
            LatinExtendedC::LatinCapitalLetterKWithDescender => "latin capital letter k with descender",
            LatinExtendedC::LatinSmallLetterKWithDescender => "latin small letter k with descender",
            LatinExtendedC::LatinCapitalLetterZWithDescender => "latin capital letter z with descender",
            LatinExtendedC::LatinSmallLetterZWithDescender => "latin small letter z with descender",
            LatinExtendedC::LatinCapitalLetterAlpha => "latin capital letter alpha",
            LatinExtendedC::LatinCapitalLetterMWithHook => "latin capital letter m with hook",
            LatinExtendedC::LatinCapitalLetterTurnedA => "latin capital letter turned a",
            LatinExtendedC::LatinCapitalLetterTurnedAlpha => "latin capital letter turned alpha",
            LatinExtendedC::LatinSmallLetterVWithRightHook => "latin small letter v with right hook",
            LatinExtendedC::LatinCapitalLetterWWithHook => "latin capital letter w with hook",
            LatinExtendedC::LatinSmallLetterWWithHook => "latin small letter w with hook",
            LatinExtendedC::LatinSmallLetterVWithCurl => "latin small letter v with curl",
            LatinExtendedC::LatinCapitalLetterHalfH => "latin capital letter half h",
            LatinExtendedC::LatinSmallLetterHalfH => "latin small letter half h",
            LatinExtendedC::LatinSmallLetterTaillessPhi => "latin small letter tailless phi",
            LatinExtendedC::LatinSmallLetterEWithNotch => "latin small letter e with notch",
            LatinExtendedC::LatinSmallLetterTurnedRWithTail => "latin small letter turned r with tail",
            LatinExtendedC::LatinSmallLetterOWithLowRingInside => "latin small letter o with low ring inside",
            LatinExtendedC::LatinLetterSmallCapitalTurnedE => "latin letter small capital turned e",
            LatinExtendedC::LatinSubscriptSmallLetterJ => "latin subscript small letter j",
            LatinExtendedC::ModifierLetterCapitalV => "modifier letter capital v",
            LatinExtendedC::LatinCapitalLetterSWithSwashTail => "latin capital letter s with swash tail",
        }
    }
}
