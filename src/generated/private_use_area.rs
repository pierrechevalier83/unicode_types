
/// An enum to represent all characters in the PrivateUseArea block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PrivateUseArea {
    /// \u{e000}: ''
    PrivateUseFirst,
}

impl Into<char> for PrivateUseArea {
    fn into(self) -> char {
        match self {
            PrivateUseArea::PrivateUseFirst => '',
        }
    }
}

impl std::convert::TryFrom<char> for PrivateUseArea {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '' => Ok(PrivateUseArea::PrivateUseFirst),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PrivateUseArea {
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

impl std::convert::TryFrom<u32> for PrivateUseArea {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PrivateUseArea {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PrivateUseArea {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PrivateUseArea::PrivateUseFirst
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("PrivateUseArea{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
