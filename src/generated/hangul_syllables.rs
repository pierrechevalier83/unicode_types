/// \u{ac00} → \u{d7af}\
///\
/// 가 힣
pub mod constants {
    /// \u{ac00}: '가'
    pub const HANGUL_SYLLABLE_FIRST: char = '가';
    /// \u{d7a3}: '힣'
    pub const HANGUL_SYLLABLE_LAST: char = '힣';
}

/// \u{ac00} → \u{d7af}\
///\
/// 가 힣
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HangulSyllables {
    /// \u{ac00}: '가'
    HangulSyllableFirst,
    /// \u{d7a3}: '힣'
    HangulSyllableLast,
}

impl Into<char> for HangulSyllables {
    fn into(self) -> char {
        use constants::*;
        match self {
            HangulSyllables::HangulSyllableFirst => HANGUL_SYLLABLE_FIRST,
            HangulSyllables::HangulSyllableLast => HANGUL_SYLLABLE_LAST,
        }
    }
}

impl std::convert::TryFrom<char> for HangulSyllables {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HANGUL_SYLLABLE_FIRST => Ok(HangulSyllables::HangulSyllableFirst),
            HANGUL_SYLLABLE_LAST => Ok(HangulSyllables::HangulSyllableLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HangulSyllables {
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

impl std::convert::TryFrom<u32> for HangulSyllables {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HangulSyllables {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HangulSyllables {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        HangulSyllables::HangulSyllableFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            HangulSyllables::HangulSyllableFirst => "hangul syllable first",
            HangulSyllables::HangulSyllableLast => "hangul syllable last",
        }
    }
}
