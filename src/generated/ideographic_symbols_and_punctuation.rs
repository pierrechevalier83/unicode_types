/// \u{16fe0} → \u{16fff}\
///\
/// 𖿠 𖿡 𖿢 𖿣\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{16fe0}: '𖿠'
    pub const TANGUT_ITERATION_MARK: char = '𖿠';
    /// \u{16fe1}: '𖿡'
    pub const NUSHU_ITERATION_MARK: char = '𖿡';
    /// \u{16fe2}: '𖿢'
    pub const OLD_CHINESE_HOOK_MARK: char = '𖿢';
    /// \u{16fe3}: '𖿣'
    pub const OLD_CHINESE_ITERATION_MARK: char = '𖿣';
}

/// An enum to represent all characters in the IdeographicSymbolsandPunctuation block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IdeographicSymbolsandPunctuation {
    /// \u{16fe0}: '𖿠'
    TangutIterationMark,
    /// \u{16fe1}: '𖿡'
    NushuIterationMark,
    /// \u{16fe2}: '𖿢'
    OldChineseHookMark,
    /// \u{16fe3}: '𖿣'
    OldChineseIterationMark,
}

impl Into<char> for IdeographicSymbolsandPunctuation {
    fn into(self) -> char {
        use constants::*;
        match self {
            IdeographicSymbolsandPunctuation::TangutIterationMark => TANGUT_ITERATION_MARK,
            IdeographicSymbolsandPunctuation::NushuIterationMark => NUSHU_ITERATION_MARK,
            IdeographicSymbolsandPunctuation::OldChineseHookMark => OLD_CHINESE_HOOK_MARK,
            IdeographicSymbolsandPunctuation::OldChineseIterationMark => OLD_CHINESE_ITERATION_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for IdeographicSymbolsandPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TANGUT_ITERATION_MARK => Ok(IdeographicSymbolsandPunctuation::TangutIterationMark),
            NUSHU_ITERATION_MARK => Ok(IdeographicSymbolsandPunctuation::NushuIterationMark),
            OLD_CHINESE_HOOK_MARK => Ok(IdeographicSymbolsandPunctuation::OldChineseHookMark),
            OLD_CHINESE_ITERATION_MARK => Ok(IdeographicSymbolsandPunctuation::OldChineseIterationMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for IdeographicSymbolsandPunctuation {
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

impl std::convert::TryFrom<u32> for IdeographicSymbolsandPunctuation {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for IdeographicSymbolsandPunctuation {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl IdeographicSymbolsandPunctuation {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        IdeographicSymbolsandPunctuation::TangutIterationMark
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            IdeographicSymbolsandPunctuation::TangutIterationMark => "tangut iteration mark",
            IdeographicSymbolsandPunctuation::NushuIterationMark => "nushu iteration mark",
            IdeographicSymbolsandPunctuation::OldChineseHookMark => "old chinese hook mark",
            IdeographicSymbolsandPunctuation::OldChineseIterationMark => "old chinese iteration mark",
        }
    }
}
