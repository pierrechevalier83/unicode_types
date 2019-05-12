/// \u{1b130} → \u{1b16f}\
///\
/// 𛅐 𛅑 𛅒 𛅤 𛅥 𛅦 𛅧\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1b150}: '𛅐'
    pub const HIRAGANA_LETTER_SMALL_WI: char = '𛅐';
    /// \u{1b151}: '𛅑'
    pub const HIRAGANA_LETTER_SMALL_WE: char = '𛅑';
    /// \u{1b152}: '𛅒'
    pub const HIRAGANA_LETTER_SMALL_WO: char = '𛅒';
    /// \u{1b164}: '𛅤'
    pub const KATAKANA_LETTER_SMALL_WI: char = '𛅤';
    /// \u{1b165}: '𛅥'
    pub const KATAKANA_LETTER_SMALL_WE: char = '𛅥';
    /// \u{1b166}: '𛅦'
    pub const KATAKANA_LETTER_SMALL_WO: char = '𛅦';
    /// \u{1b167}: '𛅧'
    pub const KATAKANA_LETTER_SMALL_N: char = '𛅧';
}

/// An enum to represent all characters in the SmallKanaExtension block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SmallKanaExtension {
    /// \u{1b150}: '𛅐'
    HiraganaLetterSmallWi,
    /// \u{1b151}: '𛅑'
    HiraganaLetterSmallWe,
    /// \u{1b152}: '𛅒'
    HiraganaLetterSmallWo,
    /// \u{1b164}: '𛅤'
    KatakanaLetterSmallWi,
    /// \u{1b165}: '𛅥'
    KatakanaLetterSmallWe,
    /// \u{1b166}: '𛅦'
    KatakanaLetterSmallWo,
    /// \u{1b167}: '𛅧'
    KatakanaLetterSmallN,
}

impl Into<char> for SmallKanaExtension {
    fn into(self) -> char {
        use constants::*;
        match self {
            SmallKanaExtension::HiraganaLetterSmallWi => HIRAGANA_LETTER_SMALL_WI,
            SmallKanaExtension::HiraganaLetterSmallWe => HIRAGANA_LETTER_SMALL_WE,
            SmallKanaExtension::HiraganaLetterSmallWo => HIRAGANA_LETTER_SMALL_WO,
            SmallKanaExtension::KatakanaLetterSmallWi => KATAKANA_LETTER_SMALL_WI,
            SmallKanaExtension::KatakanaLetterSmallWe => KATAKANA_LETTER_SMALL_WE,
            SmallKanaExtension::KatakanaLetterSmallWo => KATAKANA_LETTER_SMALL_WO,
            SmallKanaExtension::KatakanaLetterSmallN => KATAKANA_LETTER_SMALL_N,
        }
    }
}

impl std::convert::TryFrom<char> for SmallKanaExtension {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HIRAGANA_LETTER_SMALL_WI => Ok(SmallKanaExtension::HiraganaLetterSmallWi),
            HIRAGANA_LETTER_SMALL_WE => Ok(SmallKanaExtension::HiraganaLetterSmallWe),
            HIRAGANA_LETTER_SMALL_WO => Ok(SmallKanaExtension::HiraganaLetterSmallWo),
            KATAKANA_LETTER_SMALL_WI => Ok(SmallKanaExtension::KatakanaLetterSmallWi),
            KATAKANA_LETTER_SMALL_WE => Ok(SmallKanaExtension::KatakanaLetterSmallWe),
            KATAKANA_LETTER_SMALL_WO => Ok(SmallKanaExtension::KatakanaLetterSmallWo),
            KATAKANA_LETTER_SMALL_N => Ok(SmallKanaExtension::KatakanaLetterSmallN),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SmallKanaExtension {
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

impl std::convert::TryFrom<u32> for SmallKanaExtension {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SmallKanaExtension {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SmallKanaExtension {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SmallKanaExtension::HiraganaLetterSmallWi
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SmallKanaExtension::HiraganaLetterSmallWi => "hiragana letter small wi",
            SmallKanaExtension::HiraganaLetterSmallWe => "hiragana letter small we",
            SmallKanaExtension::HiraganaLetterSmallWo => "hiragana letter small wo",
            SmallKanaExtension::KatakanaLetterSmallWi => "katakana letter small wi",
            SmallKanaExtension::KatakanaLetterSmallWe => "katakana letter small we",
            SmallKanaExtension::KatakanaLetterSmallWo => "katakana letter small wo",
            SmallKanaExtension::KatakanaLetterSmallN => "katakana letter small n",
        }
    }
}
