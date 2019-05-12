
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
        match self {
            Specials::InterlinearAnnotationAnchor => '￹',
            Specials::InterlinearAnnotationSeparator => '￺',
            Specials::InterlinearAnnotationTerminator => '￻',
            Specials::ObjectReplacementCharacter => '￼',
            Specials::ReplacementCharacter => '�',
        }
    }
}

impl std::convert::TryFrom<char> for Specials {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '￹' => Ok(Specials::InterlinearAnnotationAnchor),
            '￺' => Ok(Specials::InterlinearAnnotationSeparator),
            '￻' => Ok(Specials::InterlinearAnnotationTerminator),
            '￼' => Ok(Specials::ObjectReplacementCharacter),
            '�' => Ok(Specials::ReplacementCharacter),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Specials{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
