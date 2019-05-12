/// \u{1720} → \u{173f}\
///\
/// ᜠ ᜡ ᜢ ᜣ ᜤ ᜥ ᜦ ᜧ ᜨ ᜩ ᜪ ᜫ ᜬ ᜭ ᜮ ᜯ
/// ᜰ ᜱ ᜲ ᜳ ᜴ ᜵ ᜶
pub mod constants {
    /// \u{1720}: 'ᜠ'
    pub const HANUNOO_LETTER_A: char = 'ᜠ';
    /// \u{1721}: 'ᜡ'
    pub const HANUNOO_LETTER_I: char = 'ᜡ';
    /// \u{1722}: 'ᜢ'
    pub const HANUNOO_LETTER_U: char = 'ᜢ';
    /// \u{1723}: 'ᜣ'
    pub const HANUNOO_LETTER_KA: char = 'ᜣ';
    /// \u{1724}: 'ᜤ'
    pub const HANUNOO_LETTER_GA: char = 'ᜤ';
    /// \u{1725}: 'ᜥ'
    pub const HANUNOO_LETTER_NGA: char = 'ᜥ';
    /// \u{1726}: 'ᜦ'
    pub const HANUNOO_LETTER_TA: char = 'ᜦ';
    /// \u{1727}: 'ᜧ'
    pub const HANUNOO_LETTER_DA: char = 'ᜧ';
    /// \u{1728}: 'ᜨ'
    pub const HANUNOO_LETTER_NA: char = 'ᜨ';
    /// \u{1729}: 'ᜩ'
    pub const HANUNOO_LETTER_PA: char = 'ᜩ';
    /// \u{172a}: 'ᜪ'
    pub const HANUNOO_LETTER_BA: char = 'ᜪ';
    /// \u{172b}: 'ᜫ'
    pub const HANUNOO_LETTER_MA: char = 'ᜫ';
    /// \u{172c}: 'ᜬ'
    pub const HANUNOO_LETTER_YA: char = 'ᜬ';
    /// \u{172d}: 'ᜭ'
    pub const HANUNOO_LETTER_RA: char = 'ᜭ';
    /// \u{172e}: 'ᜮ'
    pub const HANUNOO_LETTER_LA: char = 'ᜮ';
    /// \u{172f}: 'ᜯ'
    pub const HANUNOO_LETTER_WA: char = 'ᜯ';
    /// \u{1730}: 'ᜰ'
    pub const HANUNOO_LETTER_SA: char = 'ᜰ';
    /// \u{1731}: 'ᜱ'
    pub const HANUNOO_LETTER_HA: char = 'ᜱ';
    /// \u{1732}: 'ᜲ'
    pub const HANUNOO_VOWEL_SIGN_I: char = 'ᜲ';
    /// \u{1733}: 'ᜳ'
    pub const HANUNOO_VOWEL_SIGN_U: char = 'ᜳ';
    /// \u{1734}: '᜴'
    pub const HANUNOO_SIGN_PAMUDPOD: char = '᜴';
    /// \u{1735}: '᜵'
    pub const PHILIPPINE_SINGLE_PUNCTUATION: char = '᜵';
    /// \u{1736}: '᜶'
    pub const PHILIPPINE_DOUBLE_PUNCTUATION: char = '᜶';
}

/// \u{1720} → \u{173f}\
///\
/// ᜠ ᜡ ᜢ ᜣ ᜤ ᜥ ᜦ ᜧ ᜨ ᜩ ᜪ ᜫ ᜬ ᜭ ᜮ ᜯ
/// ᜰ ᜱ ᜲ ᜳ ᜴ ᜵ ᜶
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hanunoo {
    /// \u{1720}: 'ᜠ'
    HanunooLetterA,
    /// \u{1721}: 'ᜡ'
    HanunooLetterI,
    /// \u{1722}: 'ᜢ'
    HanunooLetterU,
    /// \u{1723}: 'ᜣ'
    HanunooLetterKa,
    /// \u{1724}: 'ᜤ'
    HanunooLetterGa,
    /// \u{1725}: 'ᜥ'
    HanunooLetterNga,
    /// \u{1726}: 'ᜦ'
    HanunooLetterTa,
    /// \u{1727}: 'ᜧ'
    HanunooLetterDa,
    /// \u{1728}: 'ᜨ'
    HanunooLetterNa,
    /// \u{1729}: 'ᜩ'
    HanunooLetterPa,
    /// \u{172a}: 'ᜪ'
    HanunooLetterBa,
    /// \u{172b}: 'ᜫ'
    HanunooLetterMa,
    /// \u{172c}: 'ᜬ'
    HanunooLetterYa,
    /// \u{172d}: 'ᜭ'
    HanunooLetterRa,
    /// \u{172e}: 'ᜮ'
    HanunooLetterLa,
    /// \u{172f}: 'ᜯ'
    HanunooLetterWa,
    /// \u{1730}: 'ᜰ'
    HanunooLetterSa,
    /// \u{1731}: 'ᜱ'
    HanunooLetterHa,
    /// \u{1732}: 'ᜲ'
    HanunooVowelSignI,
    /// \u{1733}: 'ᜳ'
    HanunooVowelSignU,
    /// \u{1734}: '᜴'
    HanunooSignPamudpod,
    /// \u{1735}: '᜵'
    PhilippineSinglePunctuation,
    /// \u{1736}: '᜶'
    PhilippineDoublePunctuation,
}

impl Into<char> for Hanunoo {
    fn into(self) -> char {
        use constants::*;
        match self {
            Hanunoo::HanunooLetterA => HANUNOO_LETTER_A,
            Hanunoo::HanunooLetterI => HANUNOO_LETTER_I,
            Hanunoo::HanunooLetterU => HANUNOO_LETTER_U,
            Hanunoo::HanunooLetterKa => HANUNOO_LETTER_KA,
            Hanunoo::HanunooLetterGa => HANUNOO_LETTER_GA,
            Hanunoo::HanunooLetterNga => HANUNOO_LETTER_NGA,
            Hanunoo::HanunooLetterTa => HANUNOO_LETTER_TA,
            Hanunoo::HanunooLetterDa => HANUNOO_LETTER_DA,
            Hanunoo::HanunooLetterNa => HANUNOO_LETTER_NA,
            Hanunoo::HanunooLetterPa => HANUNOO_LETTER_PA,
            Hanunoo::HanunooLetterBa => HANUNOO_LETTER_BA,
            Hanunoo::HanunooLetterMa => HANUNOO_LETTER_MA,
            Hanunoo::HanunooLetterYa => HANUNOO_LETTER_YA,
            Hanunoo::HanunooLetterRa => HANUNOO_LETTER_RA,
            Hanunoo::HanunooLetterLa => HANUNOO_LETTER_LA,
            Hanunoo::HanunooLetterWa => HANUNOO_LETTER_WA,
            Hanunoo::HanunooLetterSa => HANUNOO_LETTER_SA,
            Hanunoo::HanunooLetterHa => HANUNOO_LETTER_HA,
            Hanunoo::HanunooVowelSignI => HANUNOO_VOWEL_SIGN_I,
            Hanunoo::HanunooVowelSignU => HANUNOO_VOWEL_SIGN_U,
            Hanunoo::HanunooSignPamudpod => HANUNOO_SIGN_PAMUDPOD,
            Hanunoo::PhilippineSinglePunctuation => PHILIPPINE_SINGLE_PUNCTUATION,
            Hanunoo::PhilippineDoublePunctuation => PHILIPPINE_DOUBLE_PUNCTUATION,
        }
    }
}

impl std::convert::TryFrom<char> for Hanunoo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HANUNOO_LETTER_A => Ok(Hanunoo::HanunooLetterA),
            HANUNOO_LETTER_I => Ok(Hanunoo::HanunooLetterI),
            HANUNOO_LETTER_U => Ok(Hanunoo::HanunooLetterU),
            HANUNOO_LETTER_KA => Ok(Hanunoo::HanunooLetterKa),
            HANUNOO_LETTER_GA => Ok(Hanunoo::HanunooLetterGa),
            HANUNOO_LETTER_NGA => Ok(Hanunoo::HanunooLetterNga),
            HANUNOO_LETTER_TA => Ok(Hanunoo::HanunooLetterTa),
            HANUNOO_LETTER_DA => Ok(Hanunoo::HanunooLetterDa),
            HANUNOO_LETTER_NA => Ok(Hanunoo::HanunooLetterNa),
            HANUNOO_LETTER_PA => Ok(Hanunoo::HanunooLetterPa),
            HANUNOO_LETTER_BA => Ok(Hanunoo::HanunooLetterBa),
            HANUNOO_LETTER_MA => Ok(Hanunoo::HanunooLetterMa),
            HANUNOO_LETTER_YA => Ok(Hanunoo::HanunooLetterYa),
            HANUNOO_LETTER_RA => Ok(Hanunoo::HanunooLetterRa),
            HANUNOO_LETTER_LA => Ok(Hanunoo::HanunooLetterLa),
            HANUNOO_LETTER_WA => Ok(Hanunoo::HanunooLetterWa),
            HANUNOO_LETTER_SA => Ok(Hanunoo::HanunooLetterSa),
            HANUNOO_LETTER_HA => Ok(Hanunoo::HanunooLetterHa),
            HANUNOO_VOWEL_SIGN_I => Ok(Hanunoo::HanunooVowelSignI),
            HANUNOO_VOWEL_SIGN_U => Ok(Hanunoo::HanunooVowelSignU),
            HANUNOO_SIGN_PAMUDPOD => Ok(Hanunoo::HanunooSignPamudpod),
            PHILIPPINE_SINGLE_PUNCTUATION => Ok(Hanunoo::PhilippineSinglePunctuation),
            PHILIPPINE_DOUBLE_PUNCTUATION => Ok(Hanunoo::PhilippineDoublePunctuation),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Hanunoo {
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

impl std::convert::TryFrom<u32> for Hanunoo {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Hanunoo {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Hanunoo {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Hanunoo::HanunooLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Hanunoo::HanunooLetterA => "hanunoo letter a",
            Hanunoo::HanunooLetterI => "hanunoo letter i",
            Hanunoo::HanunooLetterU => "hanunoo letter u",
            Hanunoo::HanunooLetterKa => "hanunoo letter ka",
            Hanunoo::HanunooLetterGa => "hanunoo letter ga",
            Hanunoo::HanunooLetterNga => "hanunoo letter nga",
            Hanunoo::HanunooLetterTa => "hanunoo letter ta",
            Hanunoo::HanunooLetterDa => "hanunoo letter da",
            Hanunoo::HanunooLetterNa => "hanunoo letter na",
            Hanunoo::HanunooLetterPa => "hanunoo letter pa",
            Hanunoo::HanunooLetterBa => "hanunoo letter ba",
            Hanunoo::HanunooLetterMa => "hanunoo letter ma",
            Hanunoo::HanunooLetterYa => "hanunoo letter ya",
            Hanunoo::HanunooLetterRa => "hanunoo letter ra",
            Hanunoo::HanunooLetterLa => "hanunoo letter la",
            Hanunoo::HanunooLetterWa => "hanunoo letter wa",
            Hanunoo::HanunooLetterSa => "hanunoo letter sa",
            Hanunoo::HanunooLetterHa => "hanunoo letter ha",
            Hanunoo::HanunooVowelSignI => "hanunoo vowel sign i",
            Hanunoo::HanunooVowelSignU => "hanunoo vowel sign u",
            Hanunoo::HanunooSignPamudpod => "hanunoo sign pamudpod",
            Hanunoo::PhilippineSinglePunctuation => "philippine single punctuation",
            Hanunoo::PhilippineDoublePunctuation => "philippine double punctuation",
        }
    }
}
