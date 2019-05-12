/// \u{1700} → \u{171f}\
///\
/// ᜀ ᜁ ᜂ ᜃ ᜄ ᜅ ᜆ ᜇ ᜈ ᜉ ᜊ ᜋ ᜌ ᜎ ᜏ ᜐ
/// ᜑ ᜒ ᜓ ᜔
pub mod constants {
    /// \u{1700}: 'ᜀ'
    pub const TAGALOG_LETTER_A: char = 'ᜀ';
    /// \u{1701}: 'ᜁ'
    pub const TAGALOG_LETTER_I: char = 'ᜁ';
    /// \u{1702}: 'ᜂ'
    pub const TAGALOG_LETTER_U: char = 'ᜂ';
    /// \u{1703}: 'ᜃ'
    pub const TAGALOG_LETTER_KA: char = 'ᜃ';
    /// \u{1704}: 'ᜄ'
    pub const TAGALOG_LETTER_GA: char = 'ᜄ';
    /// \u{1705}: 'ᜅ'
    pub const TAGALOG_LETTER_NGA: char = 'ᜅ';
    /// \u{1706}: 'ᜆ'
    pub const TAGALOG_LETTER_TA: char = 'ᜆ';
    /// \u{1707}: 'ᜇ'
    pub const TAGALOG_LETTER_DA: char = 'ᜇ';
    /// \u{1708}: 'ᜈ'
    pub const TAGALOG_LETTER_NA: char = 'ᜈ';
    /// \u{1709}: 'ᜉ'
    pub const TAGALOG_LETTER_PA: char = 'ᜉ';
    /// \u{170a}: 'ᜊ'
    pub const TAGALOG_LETTER_BA: char = 'ᜊ';
    /// \u{170b}: 'ᜋ'
    pub const TAGALOG_LETTER_MA: char = 'ᜋ';
    /// \u{170c}: 'ᜌ'
    pub const TAGALOG_LETTER_YA: char = 'ᜌ';
    /// \u{170e}: 'ᜎ'
    pub const TAGALOG_LETTER_LA: char = 'ᜎ';
    /// \u{170f}: 'ᜏ'
    pub const TAGALOG_LETTER_WA: char = 'ᜏ';
    /// \u{1710}: 'ᜐ'
    pub const TAGALOG_LETTER_SA: char = 'ᜐ';
    /// \u{1711}: 'ᜑ'
    pub const TAGALOG_LETTER_HA: char = 'ᜑ';
    /// \u{1712}: 'ᜒ'
    pub const TAGALOG_VOWEL_SIGN_I: char = 'ᜒ';
    /// \u{1713}: 'ᜓ'
    pub const TAGALOG_VOWEL_SIGN_U: char = 'ᜓ';
    /// \u{1714}: '᜔'
    pub const TAGALOG_SIGN_VIRAMA: char = '᜔';
}

/// \u{1700} → \u{171f}\
///\
/// ᜀ ᜁ ᜂ ᜃ ᜄ ᜅ ᜆ ᜇ ᜈ ᜉ ᜊ ᜋ ᜌ ᜎ ᜏ ᜐ
/// ᜑ ᜒ ᜓ ᜔
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tagalog {
    /// \u{1700}: 'ᜀ'
    TagalogLetterA,
    /// \u{1701}: 'ᜁ'
    TagalogLetterI,
    /// \u{1702}: 'ᜂ'
    TagalogLetterU,
    /// \u{1703}: 'ᜃ'
    TagalogLetterKa,
    /// \u{1704}: 'ᜄ'
    TagalogLetterGa,
    /// \u{1705}: 'ᜅ'
    TagalogLetterNga,
    /// \u{1706}: 'ᜆ'
    TagalogLetterTa,
    /// \u{1707}: 'ᜇ'
    TagalogLetterDa,
    /// \u{1708}: 'ᜈ'
    TagalogLetterNa,
    /// \u{1709}: 'ᜉ'
    TagalogLetterPa,
    /// \u{170a}: 'ᜊ'
    TagalogLetterBa,
    /// \u{170b}: 'ᜋ'
    TagalogLetterMa,
    /// \u{170c}: 'ᜌ'
    TagalogLetterYa,
    /// \u{170e}: 'ᜎ'
    TagalogLetterLa,
    /// \u{170f}: 'ᜏ'
    TagalogLetterWa,
    /// \u{1710}: 'ᜐ'
    TagalogLetterSa,
    /// \u{1711}: 'ᜑ'
    TagalogLetterHa,
    /// \u{1712}: 'ᜒ'
    TagalogVowelSignI,
    /// \u{1713}: 'ᜓ'
    TagalogVowelSignU,
    /// \u{1714}: '᜔'
    TagalogSignVirama,
}

impl Into<char> for Tagalog {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tagalog::TagalogLetterA => TAGALOG_LETTER_A,
            Tagalog::TagalogLetterI => TAGALOG_LETTER_I,
            Tagalog::TagalogLetterU => TAGALOG_LETTER_U,
            Tagalog::TagalogLetterKa => TAGALOG_LETTER_KA,
            Tagalog::TagalogLetterGa => TAGALOG_LETTER_GA,
            Tagalog::TagalogLetterNga => TAGALOG_LETTER_NGA,
            Tagalog::TagalogLetterTa => TAGALOG_LETTER_TA,
            Tagalog::TagalogLetterDa => TAGALOG_LETTER_DA,
            Tagalog::TagalogLetterNa => TAGALOG_LETTER_NA,
            Tagalog::TagalogLetterPa => TAGALOG_LETTER_PA,
            Tagalog::TagalogLetterBa => TAGALOG_LETTER_BA,
            Tagalog::TagalogLetterMa => TAGALOG_LETTER_MA,
            Tagalog::TagalogLetterYa => TAGALOG_LETTER_YA,
            Tagalog::TagalogLetterLa => TAGALOG_LETTER_LA,
            Tagalog::TagalogLetterWa => TAGALOG_LETTER_WA,
            Tagalog::TagalogLetterSa => TAGALOG_LETTER_SA,
            Tagalog::TagalogLetterHa => TAGALOG_LETTER_HA,
            Tagalog::TagalogVowelSignI => TAGALOG_VOWEL_SIGN_I,
            Tagalog::TagalogVowelSignU => TAGALOG_VOWEL_SIGN_U,
            Tagalog::TagalogSignVirama => TAGALOG_SIGN_VIRAMA,
        }
    }
}

impl std::convert::TryFrom<char> for Tagalog {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TAGALOG_LETTER_A => Ok(Tagalog::TagalogLetterA),
            TAGALOG_LETTER_I => Ok(Tagalog::TagalogLetterI),
            TAGALOG_LETTER_U => Ok(Tagalog::TagalogLetterU),
            TAGALOG_LETTER_KA => Ok(Tagalog::TagalogLetterKa),
            TAGALOG_LETTER_GA => Ok(Tagalog::TagalogLetterGa),
            TAGALOG_LETTER_NGA => Ok(Tagalog::TagalogLetterNga),
            TAGALOG_LETTER_TA => Ok(Tagalog::TagalogLetterTa),
            TAGALOG_LETTER_DA => Ok(Tagalog::TagalogLetterDa),
            TAGALOG_LETTER_NA => Ok(Tagalog::TagalogLetterNa),
            TAGALOG_LETTER_PA => Ok(Tagalog::TagalogLetterPa),
            TAGALOG_LETTER_BA => Ok(Tagalog::TagalogLetterBa),
            TAGALOG_LETTER_MA => Ok(Tagalog::TagalogLetterMa),
            TAGALOG_LETTER_YA => Ok(Tagalog::TagalogLetterYa),
            TAGALOG_LETTER_LA => Ok(Tagalog::TagalogLetterLa),
            TAGALOG_LETTER_WA => Ok(Tagalog::TagalogLetterWa),
            TAGALOG_LETTER_SA => Ok(Tagalog::TagalogLetterSa),
            TAGALOG_LETTER_HA => Ok(Tagalog::TagalogLetterHa),
            TAGALOG_VOWEL_SIGN_I => Ok(Tagalog::TagalogVowelSignI),
            TAGALOG_VOWEL_SIGN_U => Ok(Tagalog::TagalogVowelSignU),
            TAGALOG_SIGN_VIRAMA => Ok(Tagalog::TagalogSignVirama),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Tagalog::TagalogLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tagalog::TagalogLetterA => "tagalog letter a",
            Tagalog::TagalogLetterI => "tagalog letter i",
            Tagalog::TagalogLetterU => "tagalog letter u",
            Tagalog::TagalogLetterKa => "tagalog letter ka",
            Tagalog::TagalogLetterGa => "tagalog letter ga",
            Tagalog::TagalogLetterNga => "tagalog letter nga",
            Tagalog::TagalogLetterTa => "tagalog letter ta",
            Tagalog::TagalogLetterDa => "tagalog letter da",
            Tagalog::TagalogLetterNa => "tagalog letter na",
            Tagalog::TagalogLetterPa => "tagalog letter pa",
            Tagalog::TagalogLetterBa => "tagalog letter ba",
            Tagalog::TagalogLetterMa => "tagalog letter ma",
            Tagalog::TagalogLetterYa => "tagalog letter ya",
            Tagalog::TagalogLetterLa => "tagalog letter la",
            Tagalog::TagalogLetterWa => "tagalog letter wa",
            Tagalog::TagalogLetterSa => "tagalog letter sa",
            Tagalog::TagalogLetterHa => "tagalog letter ha",
            Tagalog::TagalogVowelSignI => "tagalog vowel sign i",
            Tagalog::TagalogVowelSignU => "tagalog vowel sign u",
            Tagalog::TagalogSignVirama => "tagalog sign virama",
        }
    }
}
