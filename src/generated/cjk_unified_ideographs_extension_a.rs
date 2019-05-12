
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
        match self {
            CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionAFirst => '㐀',
            CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionALast => '䶵',
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographsExtensionA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '㐀' => Ok(CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionAFirst),
            '䶵' => Ok(CJKUnifiedIdeographsExtensionA::CjkIdeographExtensionALast),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CJKUnifiedIdeographsExtensionA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
