/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{1720}: 'ᜠ'
    pub const LETTER_A: char = 'ᜠ';
    /// \u{1721}: 'ᜡ'
    pub const LETTER_I: char = 'ᜡ';
    /// \u{1722}: 'ᜢ'
    pub const LETTER_U: char = 'ᜢ';
    /// \u{1723}: 'ᜣ'
    pub const LETTER_KA: char = 'ᜣ';
    /// \u{1724}: 'ᜤ'
    pub const LETTER_GA: char = 'ᜤ';
    /// \u{1725}: 'ᜥ'
    pub const LETTER_NGA: char = 'ᜥ';
    /// \u{1726}: 'ᜦ'
    pub const LETTER_TA: char = 'ᜦ';
    /// \u{1727}: 'ᜧ'
    pub const LETTER_DA: char = 'ᜧ';
    /// \u{1728}: 'ᜨ'
    pub const LETTER_NA: char = 'ᜨ';
    /// \u{1729}: 'ᜩ'
    pub const LETTER_PA: char = 'ᜩ';
    /// \u{172a}: 'ᜪ'
    pub const LETTER_BA: char = 'ᜪ';
    /// \u{172b}: 'ᜫ'
    pub const LETTER_MA: char = 'ᜫ';
    /// \u{172c}: 'ᜬ'
    pub const LETTER_YA: char = 'ᜬ';
    /// \u{172d}: 'ᜭ'
    pub const LETTER_RA: char = 'ᜭ';
    /// \u{172e}: 'ᜮ'
    pub const LETTER_LA: char = 'ᜮ';
    /// \u{172f}: 'ᜯ'
    pub const LETTER_WA: char = 'ᜯ';
    /// \u{1730}: 'ᜰ'
    pub const LETTER_SA: char = 'ᜰ';
    /// \u{1731}: 'ᜱ'
    pub const LETTER_HA: char = 'ᜱ';
    /// \u{1732}: 'ᜲ'
    pub const VOWEL_SIGN_I: char = 'ᜲ';
    /// \u{1733}: 'ᜳ'
    pub const VOWEL_SIGN_U: char = 'ᜳ';
    /// \u{1734}: '᜴'
    pub const SIGN_PAMUDPOD: char = '᜴';
    /// \u{1735}: '᜵'
    pub const PHILIPPINE_SINGLE_PUNCTUATION: char = '᜵';
    /// \u{1736}: '᜶'
    pub const PHILIPPINE_DOUBLE_PUNCTUATION: char = '᜶';
}

/// An enum to represent all characters in the Hanunoo block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hanunoo {
    /// \u{1720}: 'ᜠ'
    LetterA,
    /// \u{1721}: 'ᜡ'
    LetterI,
    /// \u{1722}: 'ᜢ'
    LetterU,
    /// \u{1723}: 'ᜣ'
    LetterKa,
    /// \u{1724}: 'ᜤ'
    LetterGa,
    /// \u{1725}: 'ᜥ'
    LetterNga,
    /// \u{1726}: 'ᜦ'
    LetterTa,
    /// \u{1727}: 'ᜧ'
    LetterDa,
    /// \u{1728}: 'ᜨ'
    LetterNa,
    /// \u{1729}: 'ᜩ'
    LetterPa,
    /// \u{172a}: 'ᜪ'
    LetterBa,
    /// \u{172b}: 'ᜫ'
    LetterMa,
    /// \u{172c}: 'ᜬ'
    LetterYa,
    /// \u{172d}: 'ᜭ'
    LetterRa,
    /// \u{172e}: 'ᜮ'
    LetterLa,
    /// \u{172f}: 'ᜯ'
    LetterWa,
    /// \u{1730}: 'ᜰ'
    LetterSa,
    /// \u{1731}: 'ᜱ'
    LetterHa,
    /// \u{1732}: 'ᜲ'
    VowelSignI,
    /// \u{1733}: 'ᜳ'
    VowelSignU,
    /// \u{1734}: '᜴'
    SignPamudpod,
    /// \u{1735}: '᜵'
    PhilippineSinglePunctuation,
    /// \u{1736}: '᜶'
    PhilippineDoublePunctuation,
}

impl Into<char> for Hanunoo {
    fn into(self) -> char {
        use constants::*;
        match self {
            Hanunoo::LetterA => LETTER_A,
            Hanunoo::LetterI => LETTER_I,
            Hanunoo::LetterU => LETTER_U,
            Hanunoo::LetterKa => LETTER_KA,
            Hanunoo::LetterGa => LETTER_GA,
            Hanunoo::LetterNga => LETTER_NGA,
            Hanunoo::LetterTa => LETTER_TA,
            Hanunoo::LetterDa => LETTER_DA,
            Hanunoo::LetterNa => LETTER_NA,
            Hanunoo::LetterPa => LETTER_PA,
            Hanunoo::LetterBa => LETTER_BA,
            Hanunoo::LetterMa => LETTER_MA,
            Hanunoo::LetterYa => LETTER_YA,
            Hanunoo::LetterRa => LETTER_RA,
            Hanunoo::LetterLa => LETTER_LA,
            Hanunoo::LetterWa => LETTER_WA,
            Hanunoo::LetterSa => LETTER_SA,
            Hanunoo::LetterHa => LETTER_HA,
            Hanunoo::VowelSignI => VOWEL_SIGN_I,
            Hanunoo::VowelSignU => VOWEL_SIGN_U,
            Hanunoo::SignPamudpod => SIGN_PAMUDPOD,
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
            LETTER_A => Ok(Hanunoo::LetterA),
            LETTER_I => Ok(Hanunoo::LetterI),
            LETTER_U => Ok(Hanunoo::LetterU),
            LETTER_KA => Ok(Hanunoo::LetterKa),
            LETTER_GA => Ok(Hanunoo::LetterGa),
            LETTER_NGA => Ok(Hanunoo::LetterNga),
            LETTER_TA => Ok(Hanunoo::LetterTa),
            LETTER_DA => Ok(Hanunoo::LetterDa),
            LETTER_NA => Ok(Hanunoo::LetterNa),
            LETTER_PA => Ok(Hanunoo::LetterPa),
            LETTER_BA => Ok(Hanunoo::LetterBa),
            LETTER_MA => Ok(Hanunoo::LetterMa),
            LETTER_YA => Ok(Hanunoo::LetterYa),
            LETTER_RA => Ok(Hanunoo::LetterRa),
            LETTER_LA => Ok(Hanunoo::LetterLa),
            LETTER_WA => Ok(Hanunoo::LetterWa),
            LETTER_SA => Ok(Hanunoo::LetterSa),
            LETTER_HA => Ok(Hanunoo::LetterHa),
            VOWEL_SIGN_I => Ok(Hanunoo::VowelSignI),
            VOWEL_SIGN_U => Ok(Hanunoo::VowelSignU),
            SIGN_PAMUDPOD => Ok(Hanunoo::SignPamudpod),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Hanunoo::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Hanunoo::LetterA => "hanunoo letter a",
            Hanunoo::LetterI => "hanunoo letter i",
            Hanunoo::LetterU => "hanunoo letter u",
            Hanunoo::LetterKa => "hanunoo letter ka",
            Hanunoo::LetterGa => "hanunoo letter ga",
            Hanunoo::LetterNga => "hanunoo letter nga",
            Hanunoo::LetterTa => "hanunoo letter ta",
            Hanunoo::LetterDa => "hanunoo letter da",
            Hanunoo::LetterNa => "hanunoo letter na",
            Hanunoo::LetterPa => "hanunoo letter pa",
            Hanunoo::LetterBa => "hanunoo letter ba",
            Hanunoo::LetterMa => "hanunoo letter ma",
            Hanunoo::LetterYa => "hanunoo letter ya",
            Hanunoo::LetterRa => "hanunoo letter ra",
            Hanunoo::LetterLa => "hanunoo letter la",
            Hanunoo::LetterWa => "hanunoo letter wa",
            Hanunoo::LetterSa => "hanunoo letter sa",
            Hanunoo::LetterHa => "hanunoo letter ha",
            Hanunoo::VowelSignI => "hanunoo vowel sign i",
            Hanunoo::VowelSignU => "hanunoo vowel sign u",
            Hanunoo::SignPamudpod => "hanunoo sign pamudpod",
            Hanunoo::PhilippineSinglePunctuation => "philippine single punctuation",
            Hanunoo::PhilippineDoublePunctuation => "philippine double punctuation",
        }
    }
}
