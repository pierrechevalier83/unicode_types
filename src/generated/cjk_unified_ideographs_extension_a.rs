/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{3400}: '㐀'
    pub const CJK_IDEOGRAPH_EXTENSION_A_FIRST: char = '㐀';
    /// \u{4db5}: '䶵'
    pub const CJK_IDEOGRAPH_EXTENSION_A_LAST: char = '䶵';
}

/// An enum to represent all characters in the CJKUnifiedIdeographsExtensionA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKUnifiedIdeographsExtensionA {
    /// \u{3400}: '㐀'
    CjkIdeographExtensionAFirst,
    /// \u{4db5}: '䶵'
    CjkIdeographExtensionALast,
}

impl Into<char> for CJKUnifiedIdeographsExtensionA {
    fn into(self) -> char {
        use constants::*;
        match self {
            CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionAFirst => CJK_IDEOGRAPH_EXTENSION_A_FIRST,
            CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionALast => CJK_IDEOGRAPH_EXTENSION_A_LAST,
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CJK_IDEOGRAPH_EXTENSION_A_FIRST => Ok(CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionAFirst),
            CJK_IDEOGRAPH_EXTENSION_A_LAST => Ok(CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionALast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKUnifiedIdeographsExtensionA {
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

impl std::convert::TryFrom<u32> for CJKUnifiedIdeographsExtensionA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKUnifiedIdeographsExtensionA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKUnifiedIdeographsExtensionA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionAFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionAFirst => "cjk ideograph extension a first",
            CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionALast => "cjk ideograph extension a last",
        }
    }
}
