
/// An enum to represent all characters in the IdeographicDescriptionCharacters block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IdeographicDescriptionCharacters {
    /// \u{2ff0}: '⿰'
    IdeographicDescriptionCharacterLeftToRight,
    /// \u{2ff1}: '⿱'
    IdeographicDescriptionCharacterAboveToBelow,
    /// \u{2ff2}: '⿲'
    IdeographicDescriptionCharacterLeftToMiddleAndRight,
    /// \u{2ff3}: '⿳'
    IdeographicDescriptionCharacterAboveToMiddleAndBelow,
    /// \u{2ff4}: '⿴'
    IdeographicDescriptionCharacterFullSurround,
    /// \u{2ff5}: '⿵'
    IdeographicDescriptionCharacterSurroundFromAbove,
    /// \u{2ff6}: '⿶'
    IdeographicDescriptionCharacterSurroundFromBelow,
    /// \u{2ff7}: '⿷'
    IdeographicDescriptionCharacterSurroundFromLeft,
    /// \u{2ff8}: '⿸'
    IdeographicDescriptionCharacterSurroundFromUpperLeft,
    /// \u{2ff9}: '⿹'
    IdeographicDescriptionCharacterSurroundFromUpperRight,
    /// \u{2ffa}: '⿺'
    IdeographicDescriptionCharacterSurroundFromLowerLeft,
    /// \u{2ffb}: '⿻'
    IdeographicDescriptionCharacterOverlaid,
}

impl Into<char> for IdeographicDescriptionCharacters {
    fn into(self) -> char {
        match self {
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToRight => '⿰',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToBelow => '⿱',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToMiddleAndRight => '⿲',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToMiddleAndBelow => '⿳',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterFullSurround => '⿴',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromAbove => '⿵',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromBelow => '⿶',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLeft => '⿷',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperLeft => '⿸',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperRight => '⿹',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLowerLeft => '⿺',
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterOverlaid => '⿻',
        }
    }
}

impl std::convert::TryFrom<char> for IdeographicDescriptionCharacters {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⿰' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToRight),
            '⿱' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToBelow),
            '⿲' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToMiddleAndRight),
            '⿳' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToMiddleAndBelow),
            '⿴' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterFullSurround),
            '⿵' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromAbove),
            '⿶' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromBelow),
            '⿷' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLeft),
            '⿸' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperLeft),
            '⿹' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperRight),
            '⿺' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLowerLeft),
            '⿻' => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterOverlaid),
            _ => Err(()),
        }
    }
}

impl Into<u32> for IdeographicDescriptionCharacters {
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

impl std::convert::TryFrom<u32> for IdeographicDescriptionCharacters {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for IdeographicDescriptionCharacters {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl IdeographicDescriptionCharacters {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToRight
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("IdeographicDescriptionCharacters{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
