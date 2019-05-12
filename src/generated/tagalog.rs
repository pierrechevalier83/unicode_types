/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1700}: 'ᜀ'
    pub const LETTER_A: char = 'ᜀ';
    /// \u{1701}: 'ᜁ'
    pub const LETTER_I: char = 'ᜁ';
    /// \u{1702}: 'ᜂ'
    pub const LETTER_U: char = 'ᜂ';
    /// \u{1703}: 'ᜃ'
    pub const LETTER_KA: char = 'ᜃ';
    /// \u{1704}: 'ᜄ'
    pub const LETTER_GA: char = 'ᜄ';
    /// \u{1705}: 'ᜅ'
    pub const LETTER_NGA: char = 'ᜅ';
    /// \u{1706}: 'ᜆ'
    pub const LETTER_TA: char = 'ᜆ';
    /// \u{1707}: 'ᜇ'
    pub const LETTER_DA: char = 'ᜇ';
    /// \u{1708}: 'ᜈ'
    pub const LETTER_NA: char = 'ᜈ';
    /// \u{1709}: 'ᜉ'
    pub const LETTER_PA: char = 'ᜉ';
    /// \u{170a}: 'ᜊ'
    pub const LETTER_BA: char = 'ᜊ';
    /// \u{170b}: 'ᜋ'
    pub const LETTER_MA: char = 'ᜋ';
    /// \u{170c}: 'ᜌ'
    pub const LETTER_YA: char = 'ᜌ';
    /// \u{170e}: 'ᜎ'
    pub const LETTER_LA: char = 'ᜎ';
    /// \u{170f}: 'ᜏ'
    pub const LETTER_WA: char = 'ᜏ';
    /// \u{1710}: 'ᜐ'
    pub const LETTER_SA: char = 'ᜐ';
    /// \u{1711}: 'ᜑ'
    pub const LETTER_HA: char = 'ᜑ';
    /// \u{1712}: 'ᜒ'
    pub const VOWEL_SIGN_I: char = 'ᜒ';
    /// \u{1713}: 'ᜓ'
    pub const VOWEL_SIGN_U: char = 'ᜓ';
    /// \u{1714}: '᜔'
    pub const SIGN_VIRAMA: char = '᜔';
}

/// An enum to represent all characters in the Tagalog block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tagalog {
    /// \u{1700}: 'ᜀ'
    LetterA,
    /// \u{1701}: 'ᜁ'
    LetterI,
    /// \u{1702}: 'ᜂ'
    LetterU,
    /// \u{1703}: 'ᜃ'
    LetterKa,
    /// \u{1704}: 'ᜄ'
    LetterGa,
    /// \u{1705}: 'ᜅ'
    LetterNga,
    /// \u{1706}: 'ᜆ'
    LetterTa,
    /// \u{1707}: 'ᜇ'
    LetterDa,
    /// \u{1708}: 'ᜈ'
    LetterNa,
    /// \u{1709}: 'ᜉ'
    LetterPa,
    /// \u{170a}: 'ᜊ'
    LetterBa,
    /// \u{170b}: 'ᜋ'
    LetterMa,
    /// \u{170c}: 'ᜌ'
    LetterYa,
    /// \u{170e}: 'ᜎ'
    LetterLa,
    /// \u{170f}: 'ᜏ'
    LetterWa,
    /// \u{1710}: 'ᜐ'
    LetterSa,
    /// \u{1711}: 'ᜑ'
    LetterHa,
    /// \u{1712}: 'ᜒ'
    VowelSignI,
    /// \u{1713}: 'ᜓ'
    VowelSignU,
    /// \u{1714}: '᜔'
    SignVirama,
}

impl Into<char> for Tagalog {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tagalog::LetterA => LETTER_A,
            Tagalog::LetterI => LETTER_I,
            Tagalog::LetterU => LETTER_U,
            Tagalog::LetterKa => LETTER_KA,
            Tagalog::LetterGa => LETTER_GA,
            Tagalog::LetterNga => LETTER_NGA,
            Tagalog::LetterTa => LETTER_TA,
            Tagalog::LetterDa => LETTER_DA,
            Tagalog::LetterNa => LETTER_NA,
            Tagalog::LetterPa => LETTER_PA,
            Tagalog::LetterBa => LETTER_BA,
            Tagalog::LetterMa => LETTER_MA,
            Tagalog::LetterYa => LETTER_YA,
            Tagalog::LetterLa => LETTER_LA,
            Tagalog::LetterWa => LETTER_WA,
            Tagalog::LetterSa => LETTER_SA,
            Tagalog::LetterHa => LETTER_HA,
            Tagalog::VowelSignI => VOWEL_SIGN_I,
            Tagalog::VowelSignU => VOWEL_SIGN_U,
            Tagalog::SignVirama => SIGN_VIRAMA,
        }
    }
}

impl std::convert::TryFrom<char> for Tagalog {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Tagalog::LetterA),
            LETTER_I => Ok(Tagalog::LetterI),
            LETTER_U => Ok(Tagalog::LetterU),
            LETTER_KA => Ok(Tagalog::LetterKa),
            LETTER_GA => Ok(Tagalog::LetterGa),
            LETTER_NGA => Ok(Tagalog::LetterNga),
            LETTER_TA => Ok(Tagalog::LetterTa),
            LETTER_DA => Ok(Tagalog::LetterDa),
            LETTER_NA => Ok(Tagalog::LetterNa),
            LETTER_PA => Ok(Tagalog::LetterPa),
            LETTER_BA => Ok(Tagalog::LetterBa),
            LETTER_MA => Ok(Tagalog::LetterMa),
            LETTER_YA => Ok(Tagalog::LetterYa),
            LETTER_LA => Ok(Tagalog::LetterLa),
            LETTER_WA => Ok(Tagalog::LetterWa),
            LETTER_SA => Ok(Tagalog::LetterSa),
            LETTER_HA => Ok(Tagalog::LetterHa),
            VOWEL_SIGN_I => Ok(Tagalog::VowelSignI),
            VOWEL_SIGN_U => Ok(Tagalog::VowelSignU),
            SIGN_VIRAMA => Ok(Tagalog::SignVirama),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tagalog {
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

impl std::convert::TryFrom<u32> for Tagalog {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tagalog {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tagalog {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tagalog::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tagalog::LetterA => "tagalog letter a",
            Tagalog::LetterI => "tagalog letter i",
            Tagalog::LetterU => "tagalog letter u",
            Tagalog::LetterKa => "tagalog letter ka",
            Tagalog::LetterGa => "tagalog letter ga",
            Tagalog::LetterNga => "tagalog letter nga",
            Tagalog::LetterTa => "tagalog letter ta",
            Tagalog::LetterDa => "tagalog letter da",
            Tagalog::LetterNa => "tagalog letter na",
            Tagalog::LetterPa => "tagalog letter pa",
            Tagalog::LetterBa => "tagalog letter ba",
            Tagalog::LetterMa => "tagalog letter ma",
            Tagalog::LetterYa => "tagalog letter ya",
            Tagalog::LetterLa => "tagalog letter la",
            Tagalog::LetterWa => "tagalog letter wa",
            Tagalog::LetterSa => "tagalog letter sa",
            Tagalog::LetterHa => "tagalog letter ha",
            Tagalog::VowelSignI => "tagalog vowel sign i",
            Tagalog::VowelSignU => "tagalog vowel sign u",
            Tagalog::SignVirama => "tagalog sign virama",
        }
    }
}
