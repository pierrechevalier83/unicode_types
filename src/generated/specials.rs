/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{fff9}: '￹'
    pub const INTERLINEAR_ANNOTATION_ANCHOR: char = '￹';
    /// \u{fffa}: '￺'
    pub const INTERLINEAR_ANNOTATION_SEPARATOR: char = '￺';
    /// \u{fffb}: '￻'
    pub const INTERLINEAR_ANNOTATION_TERMINATOR: char = '￻';
    /// \u{fffc}: '￼'
    pub const OBJECT_REPLACEMENT_CHARACTER: char = '￼';
    /// \u{fffd}: '�'
    pub const REPLACEMENT_CHARACTER: char = '�';
}

/// An enum to represent all characters in the Specials block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Specials {
    /// \u{fff9}: '￹'
    InterlinearAnnotationAnchor,
    /// \u{fffa}: '￺'
    InterlinearAnnotationSeparator,
    /// \u{fffb}: '￻'
    InterlinearAnnotationTerminator,
    /// \u{fffc}: '￼'
    ObjectReplacementCharacter,
    /// \u{fffd}: '�'
    ReplacementCharacter,
}

impl Into<char> for Specials {
    fn into(self) -> char {
        use constants::*;
        match self {
            Specials::InterlinearAnnotationAnchor => INTERLINEAR_ANNOTATION_ANCHOR,
            Specials::InterlinearAnnotationSeparator => INTERLINEAR_ANNOTATION_SEPARATOR,
            Specials::InterlinearAnnotationTerminator => INTERLINEAR_ANNOTATION_TERMINATOR,
            Specials::ObjectReplacementCharacter => OBJECT_REPLACEMENT_CHARACTER,
            Specials::ReplacementCharacter => REPLACEMENT_CHARACTER,
        }
    }
}

impl std::convert::TryFrom<char> for Specials {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            INTERLINEAR_ANNOTATION_ANCHOR => Ok(Specials::InterlinearAnnotationAnchor),
            INTERLINEAR_ANNOTATION_SEPARATOR => Ok(Specials::InterlinearAnnotationSeparator),
            INTERLINEAR_ANNOTATION_TERMINATOR => Ok(Specials::InterlinearAnnotationTerminator),
            OBJECT_REPLACEMENT_CHARACTER => Ok(Specials::ObjectReplacementCharacter),
            REPLACEMENT_CHARACTER => Ok(Specials::ReplacementCharacter),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Specials {
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

impl std::convert::TryFrom<u32> for Specials {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Specials {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Specials {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Specials::InterlinearAnnotationAnchor
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Specials::InterlinearAnnotationAnchor => "interlinear annotation anchor",
            Specials::InterlinearAnnotationSeparator => "interlinear annotation separator",
            Specials::InterlinearAnnotationTerminator => "interlinear annotation terminator",
            Specials::ObjectReplacementCharacter => "object replacement character",
            Specials::ReplacementCharacter => "replacement character",
        }
    }
}
