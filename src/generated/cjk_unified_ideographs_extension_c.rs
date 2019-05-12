/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{2a700}: '𪜀'
    pub const CJK_IDEOGRAPH_EXTENSION_C_FIRST: char = '𪜀';
    /// \u{2b734}: '𫜴'
    pub const CJK_IDEOGRAPH_EXTENSION_C_LAST: char = '𫜴';
}

/// An enum to represent all characters in the CJKUnifiedIdeographsExtensionC block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKUnifiedIdeographsExtensionC {
    /// \u{2a700}: '𪜀'
    CjkIdeographExtensionCFirst,
    /// \u{2b734}: '𫜴'
    CjkIdeographExtensionCLast,
}

impl Into<char> for CJKUnifiedIdeographsExtensionC {
    fn into(self) -> char {
        use constants::*;
        match self {
            CJKUnifiedIdeographsExtensionC::CjkIdeographExtensionCFirst => CJK_IDEOGRAPH_EXTENSION_C_FIRST,
            CJKUnifiedIdeographsExtensionC::CjkIdeographExtensionCLast => CJK_IDEOGRAPH_EXTENSION_C_LAST,
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionC {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CJK_IDEOGRAPH_EXTENSION_C_FIRST => Ok(CJKUnifiedIdeographsExtensionC::CjkIdeographExtensionCFirst),
            CJK_IDEOGRAPH_EXTENSION_C_LAST => Ok(CJKUnifiedIdeographsExtensionC::CjkIdeographExtensionCLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKUnifiedIdeographsExtensionC {
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

impl std::convert::TryFrom<u32> for CJKUnifiedIdeographsExtensionC {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKUnifiedIdeographsExtensionC {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKUnifiedIdeographsExtensionC {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKUnifiedIdeographsExtensionC::CjkIdeographExtensionCFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKUnifiedIdeographsExtensionC::CjkIdeographExtensionCFirst => "cjk ideograph extension c first",
            CJKUnifiedIdeographsExtensionC::CjkIdeographExtensionCLast => "cjk ideograph extension c last",
        }
    }
}
