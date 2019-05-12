/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{20000}: '𠀀'
    pub const CJK_IDEOGRAPH_EXTENSION_B_FIRST: char = '𠀀';
    /// \u{2a6d6}: '𪛖'
    pub const CJK_IDEOGRAPH_EXTENSION_B_LAST: char = '𪛖';
}

/// An enum to represent all characters in the CJKUnifiedIdeographsExtensionB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKUnifiedIdeographsExtensionB {
    /// \u{20000}: '𠀀'
    CjkIdeographExtensionBFirst,
    /// \u{2a6d6}: '𪛖'
    CjkIdeographExtensionBLast,
}

impl Into<char> for CJKUnifiedIdeographsExtensionB {
    fn into(self) -> char {
        use constants::*;
        match self {
            CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBFirst => CJK_IDEOGRAPH_EXTENSION_B_FIRST,
            CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBLast => CJK_IDEOGRAPH_EXTENSION_B_LAST,
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CJK_IDEOGRAPH_EXTENSION_B_FIRST => Ok(CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBFirst),
            CJK_IDEOGRAPH_EXTENSION_B_LAST => Ok(CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKUnifiedIdeographsExtensionB {
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

impl std::convert::TryFrom<u32> for CJKUnifiedIdeographsExtensionB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKUnifiedIdeographsExtensionB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKUnifiedIdeographsExtensionB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBFirst => "cjk ideograph extension b first",
            CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBLast => "cjk ideograph extension b last",
        }
    }
}
