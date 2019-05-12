/// \u{840} → \u{85f}
///
/// ࡀ ࡁ ࡂ ࡃ ࡄ ࡅ ࡆ ࡇ ࡈ ࡉ ࡊ ࡋ ࡌ ࡍ ࡎ ࡏ\
/// ࡐ ࡑ ࡒ ࡓ ࡔ ࡕ ࡖ ࡗ ࡘ ࡙ ࡚ ࡛ ࡞\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{840}: 'ࡀ'
    pub const LETTER_HALQA: char = 'ࡀ';
    /// \u{841}: 'ࡁ'
    pub const LETTER_AB: char = 'ࡁ';
    /// \u{842}: 'ࡂ'
    pub const LETTER_AG: char = 'ࡂ';
    /// \u{843}: 'ࡃ'
    pub const LETTER_AD: char = 'ࡃ';
    /// \u{844}: 'ࡄ'
    pub const LETTER_AH: char = 'ࡄ';
    /// \u{845}: 'ࡅ'
    pub const LETTER_USHENNA: char = 'ࡅ';
    /// \u{846}: 'ࡆ'
    pub const LETTER_AZ: char = 'ࡆ';
    /// \u{847}: 'ࡇ'
    pub const LETTER_IT: char = 'ࡇ';
    /// \u{848}: 'ࡈ'
    pub const LETTER_ATT: char = 'ࡈ';
    /// \u{849}: 'ࡉ'
    pub const LETTER_AKSA: char = 'ࡉ';
    /// \u{84a}: 'ࡊ'
    pub const LETTER_AK: char = 'ࡊ';
    /// \u{84b}: 'ࡋ'
    pub const LETTER_AL: char = 'ࡋ';
    /// \u{84c}: 'ࡌ'
    pub const LETTER_AM: char = 'ࡌ';
    /// \u{84d}: 'ࡍ'
    pub const LETTER_AN: char = 'ࡍ';
    /// \u{84e}: 'ࡎ'
    pub const LETTER_AS: char = 'ࡎ';
    /// \u{84f}: 'ࡏ'
    pub const LETTER_IN: char = 'ࡏ';
    /// \u{850}: 'ࡐ'
    pub const LETTER_AP: char = 'ࡐ';
    /// \u{851}: 'ࡑ'
    pub const LETTER_ASZ: char = 'ࡑ';
    /// \u{852}: 'ࡒ'
    pub const LETTER_AQ: char = 'ࡒ';
    /// \u{853}: 'ࡓ'
    pub const LETTER_AR: char = 'ࡓ';
    /// \u{854}: 'ࡔ'
    pub const LETTER_ASH: char = 'ࡔ';
    /// \u{855}: 'ࡕ'
    pub const LETTER_AT: char = 'ࡕ';
    /// \u{856}: 'ࡖ'
    pub const LETTER_DUSHENNA: char = 'ࡖ';
    /// \u{857}: 'ࡗ'
    pub const LETTER_KAD: char = 'ࡗ';
    /// \u{858}: 'ࡘ'
    pub const LETTER_AIN: char = 'ࡘ';
    /// \u{859}: '࡙'
    pub const AFFRICATION_MARK: char = '࡙';
    /// \u{85a}: '࡚'
    pub const VOCALIZATION_MARK: char = '࡚';
    /// \u{85b}: '࡛'
    pub const GEMINATION_MARK: char = '࡛';
    /// \u{85e}: '࡞'
    pub const PUNCTUATION: char = '࡞';
}

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
        use constants::*;
        match self {
            Mandaic::LetterHalqa => LETTER_HALQA,
            Mandaic::LetterAb => LETTER_AB,
            Mandaic::LetterAg => LETTER_AG,
            Mandaic::LetterAd => LETTER_AD,
            Mandaic::LetterAh => LETTER_AH,
            Mandaic::LetterUshenna => LETTER_USHENNA,
            Mandaic::LetterAz => LETTER_AZ,
            Mandaic::LetterIt => LETTER_IT,
            Mandaic::LetterAtt => LETTER_ATT,
            Mandaic::LetterAksa => LETTER_AKSA,
            Mandaic::LetterAk => LETTER_AK,
            Mandaic::LetterAl => LETTER_AL,
            Mandaic::LetterAm => LETTER_AM,
            Mandaic::LetterAn => LETTER_AN,
            Mandaic::LetterAs => LETTER_AS,
            Mandaic::LetterIn => LETTER_IN,
            Mandaic::LetterAp => LETTER_AP,
            Mandaic::LetterAsz => LETTER_ASZ,
            Mandaic::LetterAq => LETTER_AQ,
            Mandaic::LetterAr => LETTER_AR,
            Mandaic::LetterAsh => LETTER_ASH,
            Mandaic::LetterAt => LETTER_AT,
            Mandaic::LetterDushenna => LETTER_DUSHENNA,
            Mandaic::LetterKad => LETTER_KAD,
            Mandaic::LetterAin => LETTER_AIN,
            Mandaic::AffricationMark => AFFRICATION_MARK,
            Mandaic::VocalizationMark => VOCALIZATION_MARK,
            Mandaic::GeminationMark => GEMINATION_MARK,
            Mandaic::Punctuation => PUNCTUATION,
        }
    }
}

impl std::convert::TryFrom<char> for Mandaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_HALQA => Ok(Mandaic::LetterHalqa),
            LETTER_AB => Ok(Mandaic::LetterAb),
            LETTER_AG => Ok(Mandaic::LetterAg),
            LETTER_AD => Ok(Mandaic::LetterAd),
            LETTER_AH => Ok(Mandaic::LetterAh),
            LETTER_USHENNA => Ok(Mandaic::LetterUshenna),
            LETTER_AZ => Ok(Mandaic::LetterAz),
            LETTER_IT => Ok(Mandaic::LetterIt),
            LETTER_ATT => Ok(Mandaic::LetterAtt),
            LETTER_AKSA => Ok(Mandaic::LetterAksa),
            LETTER_AK => Ok(Mandaic::LetterAk),
            LETTER_AL => Ok(Mandaic::LetterAl),
            LETTER_AM => Ok(Mandaic::LetterAm),
            LETTER_AN => Ok(Mandaic::LetterAn),
            LETTER_AS => Ok(Mandaic::LetterAs),
            LETTER_IN => Ok(Mandaic::LetterIn),
            LETTER_AP => Ok(Mandaic::LetterAp),
            LETTER_ASZ => Ok(Mandaic::LetterAsz),
            LETTER_AQ => Ok(Mandaic::LetterAq),
            LETTER_AR => Ok(Mandaic::LetterAr),
            LETTER_ASH => Ok(Mandaic::LetterAsh),
            LETTER_AT => Ok(Mandaic::LetterAt),
            LETTER_DUSHENNA => Ok(Mandaic::LetterDushenna),
            LETTER_KAD => Ok(Mandaic::LetterKad),
            LETTER_AIN => Ok(Mandaic::LetterAin),
            AFFRICATION_MARK => Ok(Mandaic::AffricationMark),
            VOCALIZATION_MARK => Ok(Mandaic::VocalizationMark),
            GEMINATION_MARK => Ok(Mandaic::GeminationMark),
            PUNCTUATION => Ok(Mandaic::Punctuation),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Mandaic::LetterHalqa => "mandaic letter halqa",
            Mandaic::LetterAb => "mandaic letter ab",
            Mandaic::LetterAg => "mandaic letter ag",
            Mandaic::LetterAd => "mandaic letter ad",
            Mandaic::LetterAh => "mandaic letter ah",
            Mandaic::LetterUshenna => "mandaic letter ushenna",
            Mandaic::LetterAz => "mandaic letter az",
            Mandaic::LetterIt => "mandaic letter it",
            Mandaic::LetterAtt => "mandaic letter att",
            Mandaic::LetterAksa => "mandaic letter aksa",
            Mandaic::LetterAk => "mandaic letter ak",
            Mandaic::LetterAl => "mandaic letter al",
            Mandaic::LetterAm => "mandaic letter am",
            Mandaic::LetterAn => "mandaic letter an",
            Mandaic::LetterAs => "mandaic letter as",
            Mandaic::LetterIn => "mandaic letter in",
            Mandaic::LetterAp => "mandaic letter ap",
            Mandaic::LetterAsz => "mandaic letter asz",
            Mandaic::LetterAq => "mandaic letter aq",
            Mandaic::LetterAr => "mandaic letter ar",
            Mandaic::LetterAsh => "mandaic letter ash",
            Mandaic::LetterAt => "mandaic letter at",
            Mandaic::LetterDushenna => "mandaic letter dushenna",
            Mandaic::LetterKad => "mandaic letter kad",
            Mandaic::LetterAin => "mandaic letter ain",
            Mandaic::AffricationMark => "mandaic affrication mark",
            Mandaic::VocalizationMark => "mandaic vocalization mark",
            Mandaic::GeminationMark => "mandaic gemination mark",
            Mandaic::Punctuation => "mandaic punctuation",
        }
    }
}
