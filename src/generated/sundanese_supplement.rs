
/// An enum to represent all characters in the SundaneseSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SundaneseSupplement {
    /// \u{1cc0}: '᳀'
    SundanesePunctuationBinduSurya,
    /// \u{1cc1}: '᳁'
    SundanesePunctuationBinduPanglong,
    /// \u{1cc2}: '᳂'
    SundanesePunctuationBinduPurnama,
    /// \u{1cc3}: '᳃'
    SundanesePunctuationBinduCakra,
    /// \u{1cc4}: '᳄'
    SundanesePunctuationBinduLeuSatanga,
    /// \u{1cc5}: '᳅'
    SundanesePunctuationBinduKaSatanga,
    /// \u{1cc6}: '᳆'
    SundanesePunctuationBinduDaSatanga,
    /// \u{1cc7}: '᳇'
    SundanesePunctuationBinduBaSatanga,
}

impl Into<char> for SundaneseSupplement {
    fn into(self) -> char {
        match self {
            SundaneseSupplement::SundanesePunctuationBinduSurya => '᳀',
            SundaneseSupplement::SundanesePunctuationBinduPanglong => '᳁',
            SundaneseSupplement::SundanesePunctuationBinduPurnama => '᳂',
            SundaneseSupplement::SundanesePunctuationBinduCakra => '᳃',
            SundaneseSupplement::SundanesePunctuationBinduLeuSatanga => '᳄',
            SundaneseSupplement::SundanesePunctuationBinduKaSatanga => '᳅',
            SundaneseSupplement::SundanesePunctuationBinduDaSatanga => '᳆',
            SundaneseSupplement::SundanesePunctuationBinduBaSatanga => '᳇',
        }
    }
}

impl std::convert::TryFrom<char> for SundaneseSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᳀' => Ok(SundaneseSupplement::SundanesePunctuationBinduSurya),
            '᳁' => Ok(SundaneseSupplement::SundanesePunctuationBinduPanglong),
            '᳂' => Ok(SundaneseSupplement::SundanesePunctuationBinduPurnama),
            '᳃' => Ok(SundaneseSupplement::SundanesePunctuationBinduCakra),
            '᳄' => Ok(SundaneseSupplement::SundanesePunctuationBinduLeuSatanga),
            '᳅' => Ok(SundaneseSupplement::SundanesePunctuationBinduKaSatanga),
            '᳆' => Ok(SundaneseSupplement::SundanesePunctuationBinduDaSatanga),
            '᳇' => Ok(SundaneseSupplement::SundanesePunctuationBinduBaSatanga),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SundaneseSupplement {
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

impl std::convert::TryFrom<u32> for SundaneseSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SundaneseSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SundaneseSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SundaneseSupplement::SundanesePunctuationBinduSurya
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SundaneseSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
