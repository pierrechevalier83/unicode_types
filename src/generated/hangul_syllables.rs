
/// An enum to represent all characters in the HangulSyllables block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HangulSyllables {
    /// \u{ac00}: '가'
    HangulSyllableFirst,
    /// \u{d7a3}: '힣'
    HangulSyllableLast,
}

impl Into<char> for HangulSyllables {
    fn into(self) -> char {
        match self {
            HangulSyllables::HangulSyllableFirst => '가',
            HangulSyllables::HangulSyllableLast => '힣',
        }
    }
}

impl std::convert::TryFrom<char> for HangulSyllables {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '가' => Ok(HangulSyllables::HangulSyllableFirst),
            '힣' => Ok(HangulSyllables::HangulSyllableLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HangulSyllables {
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

impl std::convert::TryFrom<u32> for HangulSyllables {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HangulSyllables {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HangulSyllables {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HangulSyllables::HangulSyllableFirst
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("HangulSyllables{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
