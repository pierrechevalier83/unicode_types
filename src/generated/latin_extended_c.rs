
/// An enum to represent all characters in the LatinExtendedC block.
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
        match self {
            LatinExtendedC::LatinCapitalLetterLWithDoubleBar => 'Ⱡ',
            LatinExtendedC::LatinSmallLetterLWithDoubleBar => 'ⱡ',
            LatinExtendedC::LatinCapitalLetterLWithMiddleTilde => 'Ɫ',
            LatinExtendedC::LatinCapitalLetterPWithStroke => 'Ᵽ',
            LatinExtendedC::LatinCapitalLetterRWithTail => 'Ɽ',
            LatinExtendedC::LatinSmallLetterAWithStroke => 'ⱥ',
            LatinExtendedC::LatinSmallLetterTWithDiagonalStroke => 'ⱦ',
            LatinExtendedC::LatinCapitalLetterHWithDescender => 'Ⱨ',
            LatinExtendedC::LatinSmallLetterHWithDescender => 'ⱨ',
            LatinExtendedC::LatinCapitalLetterKWithDescender => 'Ⱪ',
            LatinExtendedC::LatinSmallLetterKWithDescender => 'ⱪ',
            LatinExtendedC::LatinCapitalLetterZWithDescender => 'Ⱬ',
            LatinExtendedC::LatinSmallLetterZWithDescender => 'ⱬ',
            LatinExtendedC::LatinCapitalLetterAlpha => 'Ɑ',
            LatinExtendedC::LatinCapitalLetterMWithHook => 'Ɱ',
            LatinExtendedC::LatinCapitalLetterTurnedA => 'Ɐ',
            LatinExtendedC::LatinCapitalLetterTurnedAlpha => 'Ɒ',
            LatinExtendedC::LatinSmallLetterVWithRightHook => 'ⱱ',
            LatinExtendedC::LatinCapitalLetterWWithHook => 'Ⱳ',
            LatinExtendedC::LatinSmallLetterWWithHook => 'ⱳ',
            LatinExtendedC::LatinSmallLetterVWithCurl => 'ⱴ',
            LatinExtendedC::LatinCapitalLetterHalfH => 'Ⱶ',
            LatinExtendedC::LatinSmallLetterHalfH => 'ⱶ',
            LatinExtendedC::LatinSmallLetterTaillessPhi => 'ⱷ',
            LatinExtendedC::LatinSmallLetterEWithNotch => 'ⱸ',
            LatinExtendedC::LatinSmallLetterTurnedRWithTail => 'ⱹ',
            LatinExtendedC::LatinSmallLetterOWithLowRingInside => 'ⱺ',
            LatinExtendedC::LatinLetterSmallCapitalTurnedE => 'ⱻ',
            LatinExtendedC::LatinSubscriptSmallLetterJ => 'ⱼ',
            LatinExtendedC::ModifierLetterCapitalV => 'ⱽ',
            LatinExtendedC::LatinCapitalLetterSWithSwashTail => 'Ȿ',
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedC {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ⱡ' => Ok(LatinExtendedC::LatinCapitalLetterLWithDoubleBar),
            'ⱡ' => Ok(LatinExtendedC::LatinSmallLetterLWithDoubleBar),
            'Ɫ' => Ok(LatinExtendedC::LatinCapitalLetterLWithMiddleTilde),
            'Ᵽ' => Ok(LatinExtendedC::LatinCapitalLetterPWithStroke),
            'Ɽ' => Ok(LatinExtendedC::LatinCapitalLetterRWithTail),
            'ⱥ' => Ok(LatinExtendedC::LatinSmallLetterAWithStroke),
            'ⱦ' => Ok(LatinExtendedC::LatinSmallLetterTWithDiagonalStroke),
            'Ⱨ' => Ok(LatinExtendedC::LatinCapitalLetterHWithDescender),
            'ⱨ' => Ok(LatinExtendedC::LatinSmallLetterHWithDescender),
            'Ⱪ' => Ok(LatinExtendedC::LatinCapitalLetterKWithDescender),
            'ⱪ' => Ok(LatinExtendedC::LatinSmallLetterKWithDescender),
            'Ⱬ' => Ok(LatinExtendedC::LatinCapitalLetterZWithDescender),
            'ⱬ' => Ok(LatinExtendedC::LatinSmallLetterZWithDescender),
            'Ɑ' => Ok(LatinExtendedC::LatinCapitalLetterAlpha),
            'Ɱ' => Ok(LatinExtendedC::LatinCapitalLetterMWithHook),
            'Ɐ' => Ok(LatinExtendedC::LatinCapitalLetterTurnedA),
            'Ɒ' => Ok(LatinExtendedC::LatinCapitalLetterTurnedAlpha),
            'ⱱ' => Ok(LatinExtendedC::LatinSmallLetterVWithRightHook),
            'Ⱳ' => Ok(LatinExtendedC::LatinCapitalLetterWWithHook),
            'ⱳ' => Ok(LatinExtendedC::LatinSmallLetterWWithHook),
            'ⱴ' => Ok(LatinExtendedC::LatinSmallLetterVWithCurl),
            'Ⱶ' => Ok(LatinExtendedC::LatinCapitalLetterHalfH),
            'ⱶ' => Ok(LatinExtendedC::LatinSmallLetterHalfH),
            'ⱷ' => Ok(LatinExtendedC::LatinSmallLetterTaillessPhi),
            'ⱸ' => Ok(LatinExtendedC::LatinSmallLetterEWithNotch),
            'ⱹ' => Ok(LatinExtendedC::LatinSmallLetterTurnedRWithTail),
            'ⱺ' => Ok(LatinExtendedC::LatinSmallLetterOWithLowRingInside),
            'ⱻ' => Ok(LatinExtendedC::LatinLetterSmallCapitalTurnedE),
            'ⱼ' => Ok(LatinExtendedC::LatinSubscriptSmallLetterJ),
            'ⱽ' => Ok(LatinExtendedC::ModifierLetterCapitalV),
            'Ȿ' => Ok(LatinExtendedC::LatinCapitalLetterSWithSwashTail),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedC::LatinCapitalLetterLWithDoubleBar
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("LatinExtendedC{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
