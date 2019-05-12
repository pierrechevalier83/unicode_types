
/// An enum to represent all characters in the IdeographicSymbolsandPunctuation block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IdeographicSymbolsandPunctuation {
    /// \u{16fe0}: '𖿠'
    TangutIterationMark,
    /// \u{16fe1}: '𖿡'
    NushuIterationMark,
    /// \u{16fe2}: '𖿢'
    OldChineseHookMark,
    /// \u{16fe3}: '𖿣'
    OldChineseIterationMark,
}

impl Into<char> for IdeographicSymbolsandPunctuation {
    fn into(self) -> char {
        match self {
            IdeographicSymbolsandPunctuation::TangutIterationMark => '𖿠',
            IdeographicSymbolsandPunctuation::NushuIterationMark => '𖿡',
            IdeographicSymbolsandPunctuation::OldChineseHookMark => '𖿢',
            IdeographicSymbolsandPunctuation::OldChineseIterationMark => '𖿣',
        }
    }
}

impl std::convert::TryFrom<char> for IdeographicSymbolsandPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𖿠' => Ok(IdeographicSymbolsandPunctuation::TangutIterationMark),
            '𖿡' => Ok(IdeographicSymbolsandPunctuation::NushuIterationMark),
            '𖿢' => Ok(IdeographicSymbolsandPunctuation::OldChineseHookMark),
            '𖿣' => Ok(IdeographicSymbolsandPunctuation::OldChineseIterationMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for IdeographicSymbolsandPunctuation {
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

impl std::convert::TryFrom<u32> for IdeographicSymbolsandPunctuation {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for IdeographicSymbolsandPunctuation {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl IdeographicSymbolsandPunctuation {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        IdeographicSymbolsandPunctuation::TangutIterationMark
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("IdeographicSymbolsandPunctuation{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
