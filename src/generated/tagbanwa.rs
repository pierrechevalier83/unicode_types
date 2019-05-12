/// \u{1760} → \u{177f}\
///\
/// ᝠ ᝡ ᝢ ᝣ ᝤ ᝥ ᝦ ᝧ ᝨ ᝩ ᝪ ᝫ ᝬ ᝮ ᝯ ᝰ
/// ᝲ ᝳ
pub mod constants {
    /// \u{1760}: 'ᝠ'
    pub const TAGBANWA_LETTER_A: char = 'ᝠ';
    /// \u{1761}: 'ᝡ'
    pub const TAGBANWA_LETTER_I: char = 'ᝡ';
    /// \u{1762}: 'ᝢ'
    pub const TAGBANWA_LETTER_U: char = 'ᝢ';
    /// \u{1763}: 'ᝣ'
    pub const TAGBANWA_LETTER_KA: char = 'ᝣ';
    /// \u{1764}: 'ᝤ'
    pub const TAGBANWA_LETTER_GA: char = 'ᝤ';
    /// \u{1765}: 'ᝥ'
    pub const TAGBANWA_LETTER_NGA: char = 'ᝥ';
    /// \u{1766}: 'ᝦ'
    pub const TAGBANWA_LETTER_TA: char = 'ᝦ';
    /// \u{1767}: 'ᝧ'
    pub const TAGBANWA_LETTER_DA: char = 'ᝧ';
    /// \u{1768}: 'ᝨ'
    pub const TAGBANWA_LETTER_NA: char = 'ᝨ';
    /// \u{1769}: 'ᝩ'
    pub const TAGBANWA_LETTER_PA: char = 'ᝩ';
    /// \u{176a}: 'ᝪ'
    pub const TAGBANWA_LETTER_BA: char = 'ᝪ';
    /// \u{176b}: 'ᝫ'
    pub const TAGBANWA_LETTER_MA: char = 'ᝫ';
    /// \u{176c}: 'ᝬ'
    pub const TAGBANWA_LETTER_YA: char = 'ᝬ';
    /// \u{176e}: 'ᝮ'
    pub const TAGBANWA_LETTER_LA: char = 'ᝮ';
    /// \u{176f}: 'ᝯ'
    pub const TAGBANWA_LETTER_WA: char = 'ᝯ';
    /// \u{1770}: 'ᝰ'
    pub const TAGBANWA_LETTER_SA: char = 'ᝰ';
    /// \u{1772}: 'ᝲ'
    pub const TAGBANWA_VOWEL_SIGN_I: char = 'ᝲ';
    /// \u{1773}: 'ᝳ'
    pub const TAGBANWA_VOWEL_SIGN_U: char = 'ᝳ';
}

/// \u{1760} → \u{177f}\
///\
/// ᝠ ᝡ ᝢ ᝣ ᝤ ᝥ ᝦ ᝧ ᝨ ᝩ ᝪ ᝫ ᝬ ᝮ ᝯ ᝰ
/// ᝲ ᝳ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tagbanwa {
    /// \u{1760}: 'ᝠ'
    TagbanwaLetterA,
    /// \u{1761}: 'ᝡ'
    TagbanwaLetterI,
    /// \u{1762}: 'ᝢ'
    TagbanwaLetterU,
    /// \u{1763}: 'ᝣ'
    TagbanwaLetterKa,
    /// \u{1764}: 'ᝤ'
    TagbanwaLetterGa,
    /// \u{1765}: 'ᝥ'
    TagbanwaLetterNga,
    /// \u{1766}: 'ᝦ'
    TagbanwaLetterTa,
    /// \u{1767}: 'ᝧ'
    TagbanwaLetterDa,
    /// \u{1768}: 'ᝨ'
    TagbanwaLetterNa,
    /// \u{1769}: 'ᝩ'
    TagbanwaLetterPa,
    /// \u{176a}: 'ᝪ'
    TagbanwaLetterBa,
    /// \u{176b}: 'ᝫ'
    TagbanwaLetterMa,
    /// \u{176c}: 'ᝬ'
    TagbanwaLetterYa,
    /// \u{176e}: 'ᝮ'
    TagbanwaLetterLa,
    /// \u{176f}: 'ᝯ'
    TagbanwaLetterWa,
    /// \u{1770}: 'ᝰ'
    TagbanwaLetterSa,
    /// \u{1772}: 'ᝲ'
    TagbanwaVowelSignI,
    /// \u{1773}: 'ᝳ'
    TagbanwaVowelSignU,
}

impl Into<char> for Tagbanwa {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tagbanwa::TagbanwaLetterA => TAGBANWA_LETTER_A,
            Tagbanwa::TagbanwaLetterI => TAGBANWA_LETTER_I,
            Tagbanwa::TagbanwaLetterU => TAGBANWA_LETTER_U,
            Tagbanwa::TagbanwaLetterKa => TAGBANWA_LETTER_KA,
            Tagbanwa::TagbanwaLetterGa => TAGBANWA_LETTER_GA,
            Tagbanwa::TagbanwaLetterNga => TAGBANWA_LETTER_NGA,
            Tagbanwa::TagbanwaLetterTa => TAGBANWA_LETTER_TA,
            Tagbanwa::TagbanwaLetterDa => TAGBANWA_LETTER_DA,
            Tagbanwa::TagbanwaLetterNa => TAGBANWA_LETTER_NA,
            Tagbanwa::TagbanwaLetterPa => TAGBANWA_LETTER_PA,
            Tagbanwa::TagbanwaLetterBa => TAGBANWA_LETTER_BA,
            Tagbanwa::TagbanwaLetterMa => TAGBANWA_LETTER_MA,
            Tagbanwa::TagbanwaLetterYa => TAGBANWA_LETTER_YA,
            Tagbanwa::TagbanwaLetterLa => TAGBANWA_LETTER_LA,
            Tagbanwa::TagbanwaLetterWa => TAGBANWA_LETTER_WA,
            Tagbanwa::TagbanwaLetterSa => TAGBANWA_LETTER_SA,
            Tagbanwa::TagbanwaVowelSignI => TAGBANWA_VOWEL_SIGN_I,
            Tagbanwa::TagbanwaVowelSignU => TAGBANWA_VOWEL_SIGN_U,
        }
    }
}

impl std::convert::TryFrom<char> for Tagbanwa {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TAGBANWA_LETTER_A => Ok(Tagbanwa::TagbanwaLetterA),
            TAGBANWA_LETTER_I => Ok(Tagbanwa::TagbanwaLetterI),
            TAGBANWA_LETTER_U => Ok(Tagbanwa::TagbanwaLetterU),
            TAGBANWA_LETTER_KA => Ok(Tagbanwa::TagbanwaLetterKa),
            TAGBANWA_LETTER_GA => Ok(Tagbanwa::TagbanwaLetterGa),
            TAGBANWA_LETTER_NGA => Ok(Tagbanwa::TagbanwaLetterNga),
            TAGBANWA_LETTER_TA => Ok(Tagbanwa::TagbanwaLetterTa),
            TAGBANWA_LETTER_DA => Ok(Tagbanwa::TagbanwaLetterDa),
            TAGBANWA_LETTER_NA => Ok(Tagbanwa::TagbanwaLetterNa),
            TAGBANWA_LETTER_PA => Ok(Tagbanwa::TagbanwaLetterPa),
            TAGBANWA_LETTER_BA => Ok(Tagbanwa::TagbanwaLetterBa),
            TAGBANWA_LETTER_MA => Ok(Tagbanwa::TagbanwaLetterMa),
            TAGBANWA_LETTER_YA => Ok(Tagbanwa::TagbanwaLetterYa),
            TAGBANWA_LETTER_LA => Ok(Tagbanwa::TagbanwaLetterLa),
            TAGBANWA_LETTER_WA => Ok(Tagbanwa::TagbanwaLetterWa),
            TAGBANWA_LETTER_SA => Ok(Tagbanwa::TagbanwaLetterSa),
            TAGBANWA_VOWEL_SIGN_I => Ok(Tagbanwa::TagbanwaVowelSignI),
            TAGBANWA_VOWEL_SIGN_U => Ok(Tagbanwa::TagbanwaVowelSignU),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tagbanwa {
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

impl std::convert::TryFrom<u32> for Tagbanwa {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tagbanwa {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tagbanwa {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Tagbanwa::TagbanwaLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tagbanwa::TagbanwaLetterA => "tagbanwa letter a",
            Tagbanwa::TagbanwaLetterI => "tagbanwa letter i",
            Tagbanwa::TagbanwaLetterU => "tagbanwa letter u",
            Tagbanwa::TagbanwaLetterKa => "tagbanwa letter ka",
            Tagbanwa::TagbanwaLetterGa => "tagbanwa letter ga",
            Tagbanwa::TagbanwaLetterNga => "tagbanwa letter nga",
            Tagbanwa::TagbanwaLetterTa => "tagbanwa letter ta",
            Tagbanwa::TagbanwaLetterDa => "tagbanwa letter da",
            Tagbanwa::TagbanwaLetterNa => "tagbanwa letter na",
            Tagbanwa::TagbanwaLetterPa => "tagbanwa letter pa",
            Tagbanwa::TagbanwaLetterBa => "tagbanwa letter ba",
            Tagbanwa::TagbanwaLetterMa => "tagbanwa letter ma",
            Tagbanwa::TagbanwaLetterYa => "tagbanwa letter ya",
            Tagbanwa::TagbanwaLetterLa => "tagbanwa letter la",
            Tagbanwa::TagbanwaLetterWa => "tagbanwa letter wa",
            Tagbanwa::TagbanwaLetterSa => "tagbanwa letter sa",
            Tagbanwa::TagbanwaVowelSignI => "tagbanwa vowel sign i",
            Tagbanwa::TagbanwaVowelSignU => "tagbanwa vowel sign u",
        }
    }
}
