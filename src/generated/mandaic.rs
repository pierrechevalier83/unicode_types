/// \u{840} → \u{85f}\
///\
/// ࡀ ࡁ ࡂ ࡃ ࡄ ࡅ ࡆ ࡇ ࡈ ࡉ ࡊ ࡋ ࡌ ࡍ ࡎ ࡏ
/// ࡐ ࡑ ࡒ ࡓ ࡔ ࡕ ࡖ ࡗ ࡘ ࡙ ࡚ ࡛ ࡞
pub mod constants {
    /// \u{840}: 'ࡀ'
    pub const MANDAIC_LETTER_HALQA: char = 'ࡀ';
    /// \u{841}: 'ࡁ'
    pub const MANDAIC_LETTER_AB: char = 'ࡁ';
    /// \u{842}: 'ࡂ'
    pub const MANDAIC_LETTER_AG: char = 'ࡂ';
    /// \u{843}: 'ࡃ'
    pub const MANDAIC_LETTER_AD: char = 'ࡃ';
    /// \u{844}: 'ࡄ'
    pub const MANDAIC_LETTER_AH: char = 'ࡄ';
    /// \u{845}: 'ࡅ'
    pub const MANDAIC_LETTER_USHENNA: char = 'ࡅ';
    /// \u{846}: 'ࡆ'
    pub const MANDAIC_LETTER_AZ: char = 'ࡆ';
    /// \u{847}: 'ࡇ'
    pub const MANDAIC_LETTER_IT: char = 'ࡇ';
    /// \u{848}: 'ࡈ'
    pub const MANDAIC_LETTER_ATT: char = 'ࡈ';
    /// \u{849}: 'ࡉ'
    pub const MANDAIC_LETTER_AKSA: char = 'ࡉ';
    /// \u{84a}: 'ࡊ'
    pub const MANDAIC_LETTER_AK: char = 'ࡊ';
    /// \u{84b}: 'ࡋ'
    pub const MANDAIC_LETTER_AL: char = 'ࡋ';
    /// \u{84c}: 'ࡌ'
    pub const MANDAIC_LETTER_AM: char = 'ࡌ';
    /// \u{84d}: 'ࡍ'
    pub const MANDAIC_LETTER_AN: char = 'ࡍ';
    /// \u{84e}: 'ࡎ'
    pub const MANDAIC_LETTER_AS: char = 'ࡎ';
    /// \u{84f}: 'ࡏ'
    pub const MANDAIC_LETTER_IN: char = 'ࡏ';
    /// \u{850}: 'ࡐ'
    pub const MANDAIC_LETTER_AP: char = 'ࡐ';
    /// \u{851}: 'ࡑ'
    pub const MANDAIC_LETTER_ASZ: char = 'ࡑ';
    /// \u{852}: 'ࡒ'
    pub const MANDAIC_LETTER_AQ: char = 'ࡒ';
    /// \u{853}: 'ࡓ'
    pub const MANDAIC_LETTER_AR: char = 'ࡓ';
    /// \u{854}: 'ࡔ'
    pub const MANDAIC_LETTER_ASH: char = 'ࡔ';
    /// \u{855}: 'ࡕ'
    pub const MANDAIC_LETTER_AT: char = 'ࡕ';
    /// \u{856}: 'ࡖ'
    pub const MANDAIC_LETTER_DUSHENNA: char = 'ࡖ';
    /// \u{857}: 'ࡗ'
    pub const MANDAIC_LETTER_KAD: char = 'ࡗ';
    /// \u{858}: 'ࡘ'
    pub const MANDAIC_LETTER_AIN: char = 'ࡘ';
    /// \u{859}: '࡙'
    pub const MANDAIC_AFFRICATION_MARK: char = '࡙';
    /// \u{85a}: '࡚'
    pub const MANDAIC_VOCALIZATION_MARK: char = '࡚';
    /// \u{85b}: '࡛'
    pub const MANDAIC_GEMINATION_MARK: char = '࡛';
    /// \u{85e}: '࡞'
    pub const MANDAIC_PUNCTUATION: char = '࡞';
}

/// \u{840} → \u{85f}\
///\
/// ࡀ ࡁ ࡂ ࡃ ࡄ ࡅ ࡆ ࡇ ࡈ ࡉ ࡊ ࡋ ࡌ ࡍ ࡎ ࡏ
/// ࡐ ࡑ ࡒ ࡓ ࡔ ࡕ ࡖ ࡗ ࡘ ࡙ ࡚ ࡛ ࡞
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Mandaic {
    /// \u{840}: 'ࡀ'
    MandaicLetterHalqa,
    /// \u{841}: 'ࡁ'
    MandaicLetterAb,
    /// \u{842}: 'ࡂ'
    MandaicLetterAg,
    /// \u{843}: 'ࡃ'
    MandaicLetterAd,
    /// \u{844}: 'ࡄ'
    MandaicLetterAh,
    /// \u{845}: 'ࡅ'
    MandaicLetterUshenna,
    /// \u{846}: 'ࡆ'
    MandaicLetterAz,
    /// \u{847}: 'ࡇ'
    MandaicLetterIt,
    /// \u{848}: 'ࡈ'
    MandaicLetterAtt,
    /// \u{849}: 'ࡉ'
    MandaicLetterAksa,
    /// \u{84a}: 'ࡊ'
    MandaicLetterAk,
    /// \u{84b}: 'ࡋ'
    MandaicLetterAl,
    /// \u{84c}: 'ࡌ'
    MandaicLetterAm,
    /// \u{84d}: 'ࡍ'
    MandaicLetterAn,
    /// \u{84e}: 'ࡎ'
    MandaicLetterAs,
    /// \u{84f}: 'ࡏ'
    MandaicLetterIn,
    /// \u{850}: 'ࡐ'
    MandaicLetterAp,
    /// \u{851}: 'ࡑ'
    MandaicLetterAsz,
    /// \u{852}: 'ࡒ'
    MandaicLetterAq,
    /// \u{853}: 'ࡓ'
    MandaicLetterAr,
    /// \u{854}: 'ࡔ'
    MandaicLetterAsh,
    /// \u{855}: 'ࡕ'
    MandaicLetterAt,
    /// \u{856}: 'ࡖ'
    MandaicLetterDushenna,
    /// \u{857}: 'ࡗ'
    MandaicLetterKad,
    /// \u{858}: 'ࡘ'
    MandaicLetterAin,
    /// \u{859}: '࡙'
    MandaicAffricationMark,
    /// \u{85a}: '࡚'
    MandaicVocalizationMark,
    /// \u{85b}: '࡛'
    MandaicGeminationMark,
    /// \u{85e}: '࡞'
    MandaicPunctuation,
}

impl Into<char> for Mandaic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Mandaic::MandaicLetterHalqa => MANDAIC_LETTER_HALQA,
            Mandaic::MandaicLetterAb => MANDAIC_LETTER_AB,
            Mandaic::MandaicLetterAg => MANDAIC_LETTER_AG,
            Mandaic::MandaicLetterAd => MANDAIC_LETTER_AD,
            Mandaic::MandaicLetterAh => MANDAIC_LETTER_AH,
            Mandaic::MandaicLetterUshenna => MANDAIC_LETTER_USHENNA,
            Mandaic::MandaicLetterAz => MANDAIC_LETTER_AZ,
            Mandaic::MandaicLetterIt => MANDAIC_LETTER_IT,
            Mandaic::MandaicLetterAtt => MANDAIC_LETTER_ATT,
            Mandaic::MandaicLetterAksa => MANDAIC_LETTER_AKSA,
            Mandaic::MandaicLetterAk => MANDAIC_LETTER_AK,
            Mandaic::MandaicLetterAl => MANDAIC_LETTER_AL,
            Mandaic::MandaicLetterAm => MANDAIC_LETTER_AM,
            Mandaic::MandaicLetterAn => MANDAIC_LETTER_AN,
            Mandaic::MandaicLetterAs => MANDAIC_LETTER_AS,
            Mandaic::MandaicLetterIn => MANDAIC_LETTER_IN,
            Mandaic::MandaicLetterAp => MANDAIC_LETTER_AP,
            Mandaic::MandaicLetterAsz => MANDAIC_LETTER_ASZ,
            Mandaic::MandaicLetterAq => MANDAIC_LETTER_AQ,
            Mandaic::MandaicLetterAr => MANDAIC_LETTER_AR,
            Mandaic::MandaicLetterAsh => MANDAIC_LETTER_ASH,
            Mandaic::MandaicLetterAt => MANDAIC_LETTER_AT,
            Mandaic::MandaicLetterDushenna => MANDAIC_LETTER_DUSHENNA,
            Mandaic::MandaicLetterKad => MANDAIC_LETTER_KAD,
            Mandaic::MandaicLetterAin => MANDAIC_LETTER_AIN,
            Mandaic::MandaicAffricationMark => MANDAIC_AFFRICATION_MARK,
            Mandaic::MandaicVocalizationMark => MANDAIC_VOCALIZATION_MARK,
            Mandaic::MandaicGeminationMark => MANDAIC_GEMINATION_MARK,
            Mandaic::MandaicPunctuation => MANDAIC_PUNCTUATION,
        }
    }
}

impl std::convert::TryFrom<char> for Mandaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MANDAIC_LETTER_HALQA => Ok(Mandaic::MandaicLetterHalqa),
            MANDAIC_LETTER_AB => Ok(Mandaic::MandaicLetterAb),
            MANDAIC_LETTER_AG => Ok(Mandaic::MandaicLetterAg),
            MANDAIC_LETTER_AD => Ok(Mandaic::MandaicLetterAd),
            MANDAIC_LETTER_AH => Ok(Mandaic::MandaicLetterAh),
            MANDAIC_LETTER_USHENNA => Ok(Mandaic::MandaicLetterUshenna),
            MANDAIC_LETTER_AZ => Ok(Mandaic::MandaicLetterAz),
            MANDAIC_LETTER_IT => Ok(Mandaic::MandaicLetterIt),
            MANDAIC_LETTER_ATT => Ok(Mandaic::MandaicLetterAtt),
            MANDAIC_LETTER_AKSA => Ok(Mandaic::MandaicLetterAksa),
            MANDAIC_LETTER_AK => Ok(Mandaic::MandaicLetterAk),
            MANDAIC_LETTER_AL => Ok(Mandaic::MandaicLetterAl),
            MANDAIC_LETTER_AM => Ok(Mandaic::MandaicLetterAm),
            MANDAIC_LETTER_AN => Ok(Mandaic::MandaicLetterAn),
            MANDAIC_LETTER_AS => Ok(Mandaic::MandaicLetterAs),
            MANDAIC_LETTER_IN => Ok(Mandaic::MandaicLetterIn),
            MANDAIC_LETTER_AP => Ok(Mandaic::MandaicLetterAp),
            MANDAIC_LETTER_ASZ => Ok(Mandaic::MandaicLetterAsz),
            MANDAIC_LETTER_AQ => Ok(Mandaic::MandaicLetterAq),
            MANDAIC_LETTER_AR => Ok(Mandaic::MandaicLetterAr),
            MANDAIC_LETTER_ASH => Ok(Mandaic::MandaicLetterAsh),
            MANDAIC_LETTER_AT => Ok(Mandaic::MandaicLetterAt),
            MANDAIC_LETTER_DUSHENNA => Ok(Mandaic::MandaicLetterDushenna),
            MANDAIC_LETTER_KAD => Ok(Mandaic::MandaicLetterKad),
            MANDAIC_LETTER_AIN => Ok(Mandaic::MandaicLetterAin),
            MANDAIC_AFFRICATION_MARK => Ok(Mandaic::MandaicAffricationMark),
            MANDAIC_VOCALIZATION_MARK => Ok(Mandaic::MandaicVocalizationMark),
            MANDAIC_GEMINATION_MARK => Ok(Mandaic::MandaicGeminationMark),
            MANDAIC_PUNCTUATION => Ok(Mandaic::MandaicPunctuation),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Mandaic::MandaicLetterHalqa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Mandaic::MandaicLetterHalqa => "mandaic letter halqa",
            Mandaic::MandaicLetterAb => "mandaic letter ab",
            Mandaic::MandaicLetterAg => "mandaic letter ag",
            Mandaic::MandaicLetterAd => "mandaic letter ad",
            Mandaic::MandaicLetterAh => "mandaic letter ah",
            Mandaic::MandaicLetterUshenna => "mandaic letter ushenna",
            Mandaic::MandaicLetterAz => "mandaic letter az",
            Mandaic::MandaicLetterIt => "mandaic letter it",
            Mandaic::MandaicLetterAtt => "mandaic letter att",
            Mandaic::MandaicLetterAksa => "mandaic letter aksa",
            Mandaic::MandaicLetterAk => "mandaic letter ak",
            Mandaic::MandaicLetterAl => "mandaic letter al",
            Mandaic::MandaicLetterAm => "mandaic letter am",
            Mandaic::MandaicLetterAn => "mandaic letter an",
            Mandaic::MandaicLetterAs => "mandaic letter as",
            Mandaic::MandaicLetterIn => "mandaic letter in",
            Mandaic::MandaicLetterAp => "mandaic letter ap",
            Mandaic::MandaicLetterAsz => "mandaic letter asz",
            Mandaic::MandaicLetterAq => "mandaic letter aq",
            Mandaic::MandaicLetterAr => "mandaic letter ar",
            Mandaic::MandaicLetterAsh => "mandaic letter ash",
            Mandaic::MandaicLetterAt => "mandaic letter at",
            Mandaic::MandaicLetterDushenna => "mandaic letter dushenna",
            Mandaic::MandaicLetterKad => "mandaic letter kad",
            Mandaic::MandaicLetterAin => "mandaic letter ain",
            Mandaic::MandaicAffricationMark => "mandaic affrication mark",
            Mandaic::MandaicVocalizationMark => "mandaic vocalization mark",
            Mandaic::MandaicGeminationMark => "mandaic gemination mark",
            Mandaic::MandaicPunctuation => "mandaic punctuation",
        }
    }
}
