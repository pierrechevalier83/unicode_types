
/// An enum to represent all characters in the CJKUnifiedIdeographsExtensionF block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKUnifiedIdeographsExtensionF {
    /// \u{2ceb0}: '𬺰'
    CjkIdeographExtensionFFirst,
    /// \u{2ebe0}: '𮯠'
    CjkIdeographExtensionFLast,
}

impl Into<char> for CJKUnifiedIdeographsExtensionF {
    fn into(self) -> char {
        match self {
            CJKUnifiedIdeographsExtensionF::CjkIdeographExtensionFFirst => '𬺰',
            CJKUnifiedIdeographsExtensionF::CjkIdeographExtensionFLast => '𮯠',
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionF {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𬺰' => Ok(CJKUnifiedIdeographsExtensionF::CjkIdeographExtensionFFirst),
            '𮯠' => Ok(CJKUnifiedIdeographsExtensionF::CjkIdeographExtensionFLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKUnifiedIdeographsExtensionF {
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

impl std::convert::TryFrom<u32> for CJKUnifiedIdeographsExtensionF {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKUnifiedIdeographsExtensionF {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKUnifiedIdeographsExtensionF {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKUnifiedIdeographsExtensionF::CjkIdeographExtensionFFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKUnifiedIdeographsExtensionF::CjkIdeographExtensionFFirst => "cjk ideograph extension f first",
            CJKUnifiedIdeographsExtensionF::CjkIdeographExtensionFLast => "cjk ideograph extension f last",
        }
    }
}
