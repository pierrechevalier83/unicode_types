/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1760}: 'ᝠ'
    pub const LETTER_A: char = 'ᝠ';
    /// \u{1761}: 'ᝡ'
    pub const LETTER_I: char = 'ᝡ';
    /// \u{1762}: 'ᝢ'
    pub const LETTER_U: char = 'ᝢ';
    /// \u{1763}: 'ᝣ'
    pub const LETTER_KA: char = 'ᝣ';
    /// \u{1764}: 'ᝤ'
    pub const LETTER_GA: char = 'ᝤ';
    /// \u{1765}: 'ᝥ'
    pub const LETTER_NGA: char = 'ᝥ';
    /// \u{1766}: 'ᝦ'
    pub const LETTER_TA: char = 'ᝦ';
    /// \u{1767}: 'ᝧ'
    pub const LETTER_DA: char = 'ᝧ';
    /// \u{1768}: 'ᝨ'
    pub const LETTER_NA: char = 'ᝨ';
    /// \u{1769}: 'ᝩ'
    pub const LETTER_PA: char = 'ᝩ';
    /// \u{176a}: 'ᝪ'
    pub const LETTER_BA: char = 'ᝪ';
    /// \u{176b}: 'ᝫ'
    pub const LETTER_MA: char = 'ᝫ';
    /// \u{176c}: 'ᝬ'
    pub const LETTER_YA: char = 'ᝬ';
    /// \u{176e}: 'ᝮ'
    pub const LETTER_LA: char = 'ᝮ';
    /// \u{176f}: 'ᝯ'
    pub const LETTER_WA: char = 'ᝯ';
    /// \u{1770}: 'ᝰ'
    pub const LETTER_SA: char = 'ᝰ';
    /// \u{1772}: 'ᝲ'
    pub const VOWEL_SIGN_I: char = 'ᝲ';
    /// \u{1773}: 'ᝳ'
    pub const VOWEL_SIGN_U: char = 'ᝳ';
}

/// An enum to represent all characters in the Tagbanwa block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tagbanwa {
    /// \u{1760}: 'ᝠ'
    LetterA,
    /// \u{1761}: 'ᝡ'
    LetterI,
    /// \u{1762}: 'ᝢ'
    LetterU,
    /// \u{1763}: 'ᝣ'
    LetterKa,
    /// \u{1764}: 'ᝤ'
    LetterGa,
    /// \u{1765}: 'ᝥ'
    LetterNga,
    /// \u{1766}: 'ᝦ'
    LetterTa,
    /// \u{1767}: 'ᝧ'
    LetterDa,
    /// \u{1768}: 'ᝨ'
    LetterNa,
    /// \u{1769}: 'ᝩ'
    LetterPa,
    /// \u{176a}: 'ᝪ'
    LetterBa,
    /// \u{176b}: 'ᝫ'
    LetterMa,
    /// \u{176c}: 'ᝬ'
    LetterYa,
    /// \u{176e}: 'ᝮ'
    LetterLa,
    /// \u{176f}: 'ᝯ'
    LetterWa,
    /// \u{1770}: 'ᝰ'
    LetterSa,
    /// \u{1772}: 'ᝲ'
    VowelSignI,
    /// \u{1773}: 'ᝳ'
    VowelSignU,
}

impl Into<char> for Tagbanwa {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tagbanwa::LetterA => LETTER_A,
            Tagbanwa::LetterI => LETTER_I,
            Tagbanwa::LetterU => LETTER_U,
            Tagbanwa::LetterKa => LETTER_KA,
            Tagbanwa::LetterGa => LETTER_GA,
            Tagbanwa::LetterNga => LETTER_NGA,
            Tagbanwa::LetterTa => LETTER_TA,
            Tagbanwa::LetterDa => LETTER_DA,
            Tagbanwa::LetterNa => LETTER_NA,
            Tagbanwa::LetterPa => LETTER_PA,
            Tagbanwa::LetterBa => LETTER_BA,
            Tagbanwa::LetterMa => LETTER_MA,
            Tagbanwa::LetterYa => LETTER_YA,
            Tagbanwa::LetterLa => LETTER_LA,
            Tagbanwa::LetterWa => LETTER_WA,
            Tagbanwa::LetterSa => LETTER_SA,
            Tagbanwa::VowelSignI => VOWEL_SIGN_I,
            Tagbanwa::VowelSignU => VOWEL_SIGN_U,
        }
    }
}

impl std::convert::TryFrom<char> for Tagbanwa {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Tagbanwa::LetterA),
            LETTER_I => Ok(Tagbanwa::LetterI),
            LETTER_U => Ok(Tagbanwa::LetterU),
            LETTER_KA => Ok(Tagbanwa::LetterKa),
            LETTER_GA => Ok(Tagbanwa::LetterGa),
            LETTER_NGA => Ok(Tagbanwa::LetterNga),
            LETTER_TA => Ok(Tagbanwa::LetterTa),
            LETTER_DA => Ok(Tagbanwa::LetterDa),
            LETTER_NA => Ok(Tagbanwa::LetterNa),
            LETTER_PA => Ok(Tagbanwa::LetterPa),
            LETTER_BA => Ok(Tagbanwa::LetterBa),
            LETTER_MA => Ok(Tagbanwa::LetterMa),
            LETTER_YA => Ok(Tagbanwa::LetterYa),
            LETTER_LA => Ok(Tagbanwa::LetterLa),
            LETTER_WA => Ok(Tagbanwa::LetterWa),
            LETTER_SA => Ok(Tagbanwa::LetterSa),
            VOWEL_SIGN_I => Ok(Tagbanwa::VowelSignI),
            VOWEL_SIGN_U => Ok(Tagbanwa::VowelSignU),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tagbanwa::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tagbanwa::LetterA => "tagbanwa letter a",
            Tagbanwa::LetterI => "tagbanwa letter i",
            Tagbanwa::LetterU => "tagbanwa letter u",
            Tagbanwa::LetterKa => "tagbanwa letter ka",
            Tagbanwa::LetterGa => "tagbanwa letter ga",
            Tagbanwa::LetterNga => "tagbanwa letter nga",
            Tagbanwa::LetterTa => "tagbanwa letter ta",
            Tagbanwa::LetterDa => "tagbanwa letter da",
            Tagbanwa::LetterNa => "tagbanwa letter na",
            Tagbanwa::LetterPa => "tagbanwa letter pa",
            Tagbanwa::LetterBa => "tagbanwa letter ba",
            Tagbanwa::LetterMa => "tagbanwa letter ma",
            Tagbanwa::LetterYa => "tagbanwa letter ya",
            Tagbanwa::LetterLa => "tagbanwa letter la",
            Tagbanwa::LetterWa => "tagbanwa letter wa",
            Tagbanwa::LetterSa => "tagbanwa letter sa",
            Tagbanwa::VowelSignI => "tagbanwa vowel sign i",
            Tagbanwa::VowelSignU => "tagbanwa vowel sign u",
        }
    }
}
