/// \u{1cc0} → \u{1ccf}\
///\
/// ᳀ ᳁ ᳂ ᳃ ᳄ ᳅ ᳆ ᳇
pub mod constants {
    /// \u{1cc0}: '᳀'
    pub const SUNDANESE_PUNCTUATION_BINDU_SURYA: char = '᳀';
    /// \u{1cc1}: '᳁'
    pub const SUNDANESE_PUNCTUATION_BINDU_PANGLONG: char = '᳁';
    /// \u{1cc2}: '᳂'
    pub const SUNDANESE_PUNCTUATION_BINDU_PURNAMA: char = '᳂';
    /// \u{1cc3}: '᳃'
    pub const SUNDANESE_PUNCTUATION_BINDU_CAKRA: char = '᳃';
    /// \u{1cc4}: '᳄'
    pub const SUNDANESE_PUNCTUATION_BINDU_LEU_SATANGA: char = '᳄';
    /// \u{1cc5}: '᳅'
    pub const SUNDANESE_PUNCTUATION_BINDU_KA_SATANGA: char = '᳅';
    /// \u{1cc6}: '᳆'
    pub const SUNDANESE_PUNCTUATION_BINDU_DA_SATANGA: char = '᳆';
    /// \u{1cc7}: '᳇'
    pub const SUNDANESE_PUNCTUATION_BINDU_BA_SATANGA: char = '᳇';
}

/// \u{1cc0} → \u{1ccf}\
///\
/// ᳀ ᳁ ᳂ ᳃ ᳄ ᳅ ᳆ ᳇
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SundaneseSupplement {
    /// \u{1cc0}: '᳀'
    SundanesePunctuationBinduSurya,
    /// \u{1cc1}: '᳁'
    SundanesePunctuationBinduPanglong,
    /// \u{1cc2}: '᳂'
    SundanesePunctuationBinduPurnama,
    /// \u{1cc3}: '᳃'
    SundanesePunctuationBinduCakra,
    /// \u{1cc4}: '᳄'
    SundanesePunctuationBinduLeuSatanga,
    /// \u{1cc5}: '᳅'
    SundanesePunctuationBinduKaSatanga,
    /// \u{1cc6}: '᳆'
    SundanesePunctuationBinduDaSatanga,
    /// \u{1cc7}: '᳇'
    SundanesePunctuationBinduBaSatanga,
}

impl Into<char> for SundaneseSupplement {
    fn into(self) -> char {
        use constants::*;
        match self {
            SundaneseSupplement::SundanesePunctuationBinduSurya => SUNDANESE_PUNCTUATION_BINDU_SURYA,
            SundaneseSupplement::SundanesePunctuationBinduPanglong => SUNDANESE_PUNCTUATION_BINDU_PANGLONG,
            SundaneseSupplement::SundanesePunctuationBinduPurnama => SUNDANESE_PUNCTUATION_BINDU_PURNAMA,
            SundaneseSupplement::SundanesePunctuationBinduCakra => SUNDANESE_PUNCTUATION_BINDU_CAKRA,
            SundaneseSupplement::SundanesePunctuationBinduLeuSatanga => SUNDANESE_PUNCTUATION_BINDU_LEU_SATANGA,
            SundaneseSupplement::SundanesePunctuationBinduKaSatanga => SUNDANESE_PUNCTUATION_BINDU_KA_SATANGA,
            SundaneseSupplement::SundanesePunctuationBinduDaSatanga => SUNDANESE_PUNCTUATION_BINDU_DA_SATANGA,
            SundaneseSupplement::SundanesePunctuationBinduBaSatanga => SUNDANESE_PUNCTUATION_BINDU_BA_SATANGA,
        }
    }
}

impl std::convert::TryFrom<char> for SundaneseSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SUNDANESE_PUNCTUATION_BINDU_SURYA => Ok(SundaneseSupplement::SundanesePunctuationBinduSurya),
            SUNDANESE_PUNCTUATION_BINDU_PANGLONG => Ok(SundaneseSupplement::SundanesePunctuationBinduPanglong),
            SUNDANESE_PUNCTUATION_BINDU_PURNAMA => Ok(SundaneseSupplement::SundanesePunctuationBinduPurnama),
            SUNDANESE_PUNCTUATION_BINDU_CAKRA => Ok(SundaneseSupplement::SundanesePunctuationBinduCakra),
            SUNDANESE_PUNCTUATION_BINDU_LEU_SATANGA => Ok(SundaneseSupplement::SundanesePunctuationBinduLeuSatanga),
            SUNDANESE_PUNCTUATION_BINDU_KA_SATANGA => Ok(SundaneseSupplement::SundanesePunctuationBinduKaSatanga),
            SUNDANESE_PUNCTUATION_BINDU_DA_SATANGA => Ok(SundaneseSupplement::SundanesePunctuationBinduDaSatanga),
            SUNDANESE_PUNCTUATION_BINDU_BA_SATANGA => Ok(SundaneseSupplement::SundanesePunctuationBinduBaSatanga),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SundaneseSupplement {
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

impl std::convert::TryFrom<u32> for SundaneseSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SundaneseSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SundaneseSupplement {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        SundaneseSupplement::SundanesePunctuationBinduSurya
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SundaneseSupplement::SundanesePunctuationBinduSurya => "sundanese punctuation bindu surya",
            SundaneseSupplement::SundanesePunctuationBinduPanglong => "sundanese punctuation bindu panglong",
            SundaneseSupplement::SundanesePunctuationBinduPurnama => "sundanese punctuation bindu purnama",
            SundaneseSupplement::SundanesePunctuationBinduCakra => "sundanese punctuation bindu cakra",
            SundaneseSupplement::SundanesePunctuationBinduLeuSatanga => "sundanese punctuation bindu leu satanga",
            SundaneseSupplement::SundanesePunctuationBinduKaSatanga => "sundanese punctuation bindu ka satanga",
            SundaneseSupplement::SundanesePunctuationBinduDaSatanga => "sundanese punctuation bindu da satanga",
            SundaneseSupplement::SundanesePunctuationBinduBaSatanga => "sundanese punctuation bindu ba satanga",
        }
    }
}
