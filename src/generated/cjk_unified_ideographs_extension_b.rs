
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
        match self {
            CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBFirst => '𠀀',
            CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBLast => '𪛖',
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𠀀' => Ok(CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBFirst),
            '𪛖' => Ok(CJKUnifiedIdeographsExtensionB::CjkIdeographExtensionBLast),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CJKUnifiedIdeographsExtensionB{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
