
/// An enum to represent all characters in the CJKUnifiedIdeographsExtensionE block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKUnifiedIdeographsExtensionE {
    /// \u{2b820}: '𫠠'
    CjkIdeographExtensionEFirst,
    /// \u{2cea1}: '𬺡'
    CjkIdeographExtensionELast,
}

impl Into<char> for CJKUnifiedIdeographsExtensionE {
    fn into(self) -> char {
        match self {
            CJKUnifiedIdeographsExtensionE::CjkIdeographExtensionEFirst => '𫠠',
            CJKUnifiedIdeographsExtensionE::CjkIdeographExtensionELast => '𬺡',
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionE {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𫠠' => Ok(CJKUnifiedIdeographsExtensionE::CjkIdeographExtensionEFirst),
            '𬺡' => Ok(CJKUnifiedIdeographsExtensionE::CjkIdeographExtensionELast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKUnifiedIdeographsExtensionE {
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

impl std::convert::TryFrom<u32> for CJKUnifiedIdeographsExtensionE {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKUnifiedIdeographsExtensionE {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKUnifiedIdeographsExtensionE {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKUnifiedIdeographsExtensionE::CjkIdeographExtensionEFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKUnifiedIdeographsExtensionE::CjkIdeographExtensionEFirst => "cjk ideograph extension e first",
            CJKUnifiedIdeographsExtensionE::CjkIdeographExtensionELast => "cjk ideograph extension e last",
        }
    }
}
