
/// An enum to represent all characters in the Mandaic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Mandaic {
    /// \u{840}: 'ࡀ'
    LetterHalqa,
    /// \u{841}: 'ࡁ'
    LetterAb,
    /// \u{842}: 'ࡂ'
    LetterAg,
    /// \u{843}: 'ࡃ'
    LetterAd,
    /// \u{844}: 'ࡄ'
    LetterAh,
    /// \u{845}: 'ࡅ'
    LetterUshenna,
    /// \u{846}: 'ࡆ'
    LetterAz,
    /// \u{847}: 'ࡇ'
    LetterIt,
    /// \u{848}: 'ࡈ'
    LetterAtt,
    /// \u{849}: 'ࡉ'
    LetterAksa,
    /// \u{84a}: 'ࡊ'
    LetterAk,
    /// \u{84b}: 'ࡋ'
    LetterAl,
    /// \u{84c}: 'ࡌ'
    LetterAm,
    /// \u{84d}: 'ࡍ'
    LetterAn,
    /// \u{84e}: 'ࡎ'
    LetterAs,
    /// \u{84f}: 'ࡏ'
    LetterIn,
    /// \u{850}: 'ࡐ'
    LetterAp,
    /// \u{851}: 'ࡑ'
    LetterAsz,
    /// \u{852}: 'ࡒ'
    LetterAq,
    /// \u{853}: 'ࡓ'
    LetterAr,
    /// \u{854}: 'ࡔ'
    LetterAsh,
    /// \u{855}: 'ࡕ'
    LetterAt,
    /// \u{856}: 'ࡖ'
    LetterDushenna,
    /// \u{857}: 'ࡗ'
    LetterKad,
    /// \u{858}: 'ࡘ'
    LetterAin,
    /// \u{859}: '࡙'
    AffricationMark,
    /// \u{85a}: '࡚'
    VocalizationMark,
    /// \u{85b}: '࡛'
    GeminationMark,
    /// \u{85e}: '࡞'
    Punctuation,
}

impl Into<char> for Mandaic {
    fn into(self) -> char {
        match self {
            Mandaic::LetterHalqa => 'ࡀ',
            Mandaic::LetterAb => 'ࡁ',
            Mandaic::LetterAg => 'ࡂ',
            Mandaic::LetterAd => 'ࡃ',
            Mandaic::LetterAh => 'ࡄ',
            Mandaic::LetterUshenna => 'ࡅ',
            Mandaic::LetterAz => 'ࡆ',
            Mandaic::LetterIt => 'ࡇ',
            Mandaic::LetterAtt => 'ࡈ',
            Mandaic::LetterAksa => 'ࡉ',
            Mandaic::LetterAk => 'ࡊ',
            Mandaic::LetterAl => 'ࡋ',
            Mandaic::LetterAm => 'ࡌ',
            Mandaic::LetterAn => 'ࡍ',
            Mandaic::LetterAs => 'ࡎ',
            Mandaic::LetterIn => 'ࡏ',
            Mandaic::LetterAp => 'ࡐ',
            Mandaic::LetterAsz => 'ࡑ',
            Mandaic::LetterAq => 'ࡒ',
            Mandaic::LetterAr => 'ࡓ',
            Mandaic::LetterAsh => 'ࡔ',
            Mandaic::LetterAt => 'ࡕ',
            Mandaic::LetterDushenna => 'ࡖ',
            Mandaic::LetterKad => 'ࡗ',
            Mandaic::LetterAin => 'ࡘ',
            Mandaic::AffricationMark => '࡙',
            Mandaic::VocalizationMark => '࡚',
            Mandaic::GeminationMark => '࡛',
            Mandaic::Punctuation => '࡞',
        }
    }
}

impl std::convert::TryFrom<char> for Mandaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ࡀ' => Ok(Mandaic::LetterHalqa),
            'ࡁ' => Ok(Mandaic::LetterAb),
            'ࡂ' => Ok(Mandaic::LetterAg),
            'ࡃ' => Ok(Mandaic::LetterAd),
            'ࡄ' => Ok(Mandaic::LetterAh),
            'ࡅ' => Ok(Mandaic::LetterUshenna),
            'ࡆ' => Ok(Mandaic::LetterAz),
            'ࡇ' => Ok(Mandaic::LetterIt),
            'ࡈ' => Ok(Mandaic::LetterAtt),
            'ࡉ' => Ok(Mandaic::LetterAksa),
            'ࡊ' => Ok(Mandaic::LetterAk),
            'ࡋ' => Ok(Mandaic::LetterAl),
            'ࡌ' => Ok(Mandaic::LetterAm),
            'ࡍ' => Ok(Mandaic::LetterAn),
            'ࡎ' => Ok(Mandaic::LetterAs),
            'ࡏ' => Ok(Mandaic::LetterIn),
            'ࡐ' => Ok(Mandaic::LetterAp),
            'ࡑ' => Ok(Mandaic::LetterAsz),
            'ࡒ' => Ok(Mandaic::LetterAq),
            'ࡓ' => Ok(Mandaic::LetterAr),
            'ࡔ' => Ok(Mandaic::LetterAsh),
            'ࡕ' => Ok(Mandaic::LetterAt),
            'ࡖ' => Ok(Mandaic::LetterDushenna),
            'ࡗ' => Ok(Mandaic::LetterKad),
            'ࡘ' => Ok(Mandaic::LetterAin),
            '࡙' => Ok(Mandaic::AffricationMark),
            '࡚' => Ok(Mandaic::VocalizationMark),
            '࡛' => Ok(Mandaic::GeminationMark),
            '࡞' => Ok(Mandaic::Punctuation),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Mandaic {
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

impl std::convert::TryFrom<u32> for Mandaic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Mandaic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Mandaic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Mandaic::LetterHalqa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Mandaic{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
