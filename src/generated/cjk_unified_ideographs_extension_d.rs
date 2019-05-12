
/// An enum to represent all characters in the CJKUnifiedIdeographsExtensionD block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKUnifiedIdeographsExtensionD {
    /// \u{2b740}: '𫝀'
    CjkIdeographExtensionDFirst,
    /// \u{2b81d}: '𫠝'
    CjkIdeographExtensionDLast,
}

impl Into<char> for CJKUnifiedIdeographsExtensionD {
    fn into(self) -> char {
        match self {
            CJKUnifiedIdeographsExtensionD::CjkIdeographExtensionDFirst => '𫝀',
            CJKUnifiedIdeographsExtensionD::CjkIdeographExtensionDLast => '𫠝',
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionD {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𫝀' => Ok(CJKUnifiedIdeographsExtensionD::CjkIdeographExtensionDFirst),
            '𫠝' => Ok(CJKUnifiedIdeographsExtensionD::CjkIdeographExtensionDLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKUnifiedIdeographsExtensionD {
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

impl std::convert::TryFrom<u32> for CJKUnifiedIdeographsExtensionD {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKUnifiedIdeographsExtensionD {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKUnifiedIdeographsExtensionD {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKUnifiedIdeographsExtensionD::CjkIdeographExtensionDFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKUnifiedIdeographsExtensionD::CjkIdeographExtensionDFirst => "cjk ideograph extension d first",
            CJKUnifiedIdeographsExtensionD::CjkIdeographExtensionDLast => "cjk ideograph extension d last",
        }
    }
}
