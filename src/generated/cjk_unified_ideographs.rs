/// \u{4e00} → \u{9fff}
///
/// 一 鿯\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{4e00}: '一'
    pub const CJK_IDEOGRAPH_FIRST: char = '一';
    /// \u{9fef}: '鿯'
    pub const CJK_IDEOGRAPH_LAST: char = '鿯';
}

/// An enum to represent all characters in the CJKUnifiedIdeographs block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKUnifiedIdeographs {
    /// \u{4e00}: '一'
    CjkIdeographFirst,
    /// \u{9fef}: '鿯'
    CjkIdeographLast,
}

impl Into<char> for CJKUnifiedIdeographs {
    fn into(self) -> char {
        use constants::*;
        match self {
            CJKUnifiedIdeographs::CjkIdeographFirst => CJK_IDEOGRAPH_FIRST,
            CJKUnifiedIdeographs::CjkIdeographLast => CJK_IDEOGRAPH_LAST,
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographs {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CJK_IDEOGRAPH_FIRST => Ok(CJKUnifiedIdeographs::CjkIdeographFirst),
            CJK_IDEOGRAPH_LAST => Ok(CJKUnifiedIdeographs::CjkIdeographLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKUnifiedIdeographs {
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

impl std::convert::TryFrom<u32> for CJKUnifiedIdeographs {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKUnifiedIdeographs {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKUnifiedIdeographs {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKUnifiedIdeographs::CjkIdeographFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKUnifiedIdeographs::CjkIdeographFirst => "cjk ideograph first",
            CJKUnifiedIdeographs::CjkIdeographLast => "cjk ideograph last",
        }
    }
}
