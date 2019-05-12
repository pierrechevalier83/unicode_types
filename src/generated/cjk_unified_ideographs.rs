
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
        match self {
            CJKUnifiedIdeographs::CjkIdeographFirst => '一',
            CJKUnifiedIdeographs::CjkIdeographLast => '鿯',
        }
    }
}

impl std::convert::TryFrom<char> for CJKUnifiedIdeographs {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '一' => Ok(CJKUnifiedIdeographs::CjkIdeographFirst),
            '鿯' => Ok(CJKUnifiedIdeographs::CjkIdeographLast),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CJKUnifiedIdeographs{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
