/// \u{2ff0} → \u{2fff}\
///\
/// ⿰ ⿱ ⿲ ⿳ ⿴ ⿵ ⿶ ⿷ ⿸ ⿹ ⿺ ⿻
pub mod constants {
    /// \u{2ff0}: '⿰'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_LEFT_TO_RIGHT: char = '⿰';
    /// \u{2ff1}: '⿱'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_ABOVE_TO_BELOW: char = '⿱';
    /// \u{2ff2}: '⿲'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_LEFT_TO_MIDDLE_AND_RIGHT: char = '⿲';
    /// \u{2ff3}: '⿳'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_ABOVE_TO_MIDDLE_AND_BELOW: char = '⿳';
    /// \u{2ff4}: '⿴'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_FULL_SURROUND: char = '⿴';
    /// \u{2ff5}: '⿵'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_ABOVE: char = '⿵';
    /// \u{2ff6}: '⿶'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_BELOW: char = '⿶';
    /// \u{2ff7}: '⿷'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_LEFT: char = '⿷';
    /// \u{2ff8}: '⿸'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_UPPER_LEFT: char = '⿸';
    /// \u{2ff9}: '⿹'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_UPPER_RIGHT: char = '⿹';
    /// \u{2ffa}: '⿺'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_LOWER_LEFT: char = '⿺';
    /// \u{2ffb}: '⿻'
    pub const IDEOGRAPHIC_DESCRIPTION_CHARACTER_OVERLAID: char = '⿻';
}

/// \u{2ff0} → \u{2fff}\
///\
/// ⿰ ⿱ ⿲ ⿳ ⿴ ⿵ ⿶ ⿷ ⿸ ⿹ ⿺ ⿻
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
        use constants::*;
        match self {
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToRight => IDEOGRAPHIC_DESCRIPTION_CHARACTER_LEFT_TO_RIGHT,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToBelow => IDEOGRAPHIC_DESCRIPTION_CHARACTER_ABOVE_TO_BELOW,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToMiddleAndRight => IDEOGRAPHIC_DESCRIPTION_CHARACTER_LEFT_TO_MIDDLE_AND_RIGHT,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToMiddleAndBelow => IDEOGRAPHIC_DESCRIPTION_CHARACTER_ABOVE_TO_MIDDLE_AND_BELOW,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterFullSurround => IDEOGRAPHIC_DESCRIPTION_CHARACTER_FULL_SURROUND,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromAbove => IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_ABOVE,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromBelow => IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_BELOW,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLeft => IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_LEFT,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperLeft => IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_UPPER_LEFT,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperRight => IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_UPPER_RIGHT,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLowerLeft => IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_LOWER_LEFT,
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterOverlaid => IDEOGRAPHIC_DESCRIPTION_CHARACTER_OVERLAID,
        }
    }
}

impl std::convert::TryFrom<char> for IdeographicDescriptionCharacters {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_LEFT_TO_RIGHT => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToRight),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_ABOVE_TO_BELOW => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToBelow),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_LEFT_TO_MIDDLE_AND_RIGHT => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToMiddleAndRight),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_ABOVE_TO_MIDDLE_AND_BELOW => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToMiddleAndBelow),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_FULL_SURROUND => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterFullSurround),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_ABOVE => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromAbove),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_BELOW => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromBelow),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_LEFT => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLeft),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_UPPER_LEFT => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperLeft),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_UPPER_RIGHT => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperRight),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_SURROUND_FROM_LOWER_LEFT => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLowerLeft),
            IDEOGRAPHIC_DESCRIPTION_CHARACTER_OVERLAID => Ok(IdeographicDescriptionCharacters::IdeographicDescriptionCharacterOverlaid),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToRight
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToRight => "ideographic description character left to right",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToBelow => "ideographic description character above to below",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterLeftToMiddleAndRight => "ideographic description character left to middle and right",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterAboveToMiddleAndBelow => "ideographic description character above to middle and below",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterFullSurround => "ideographic description character full surround",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromAbove => "ideographic description character surround from above",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromBelow => "ideographic description character surround from below",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLeft => "ideographic description character surround from left",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperLeft => "ideographic description character surround from upper left",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromUpperRight => "ideographic description character surround from upper right",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterSurroundFromLowerLeft => "ideographic description character surround from lower left",
            IdeographicDescriptionCharacters::IdeographicDescriptionCharacterOverlaid => "ideographic description character overlaid",
        }
    }
}
