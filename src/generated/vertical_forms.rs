
/// An enum to represent all characters in the VerticalForms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum VerticalForms {
    /// \u{fe10}: '︐'
    PresentationFormForVerticalComma,
    /// \u{fe11}: '︑'
    PresentationFormForVerticalIdeographicComma,
    /// \u{fe12}: '︒'
    PresentationFormForVerticalIdeographicFullStop,
    /// \u{fe13}: '︓'
    PresentationFormForVerticalColon,
    /// \u{fe14}: '︔'
    PresentationFormForVerticalSemicolon,
    /// \u{fe15}: '︕'
    PresentationFormForVerticalExclamationMark,
    /// \u{fe16}: '︖'
    PresentationFormForVerticalQuestionMark,
    /// \u{fe17}: '︗'
    PresentationFormForVerticalLeftWhiteLenticularBracket,
    /// \u{fe18}: '︘'
    PresentationFormForVerticalRightWhiteLenticularBrakcet,
    /// \u{fe19}: '︙'
    PresentationFormForVerticalHorizontalEllipsis,
}

impl Into<char> for VerticalForms {
    fn into(self) -> char {
        match self {
            VerticalForms::PresentationFormForVerticalComma => '︐',
            VerticalForms::PresentationFormForVerticalIdeographicComma => '︑',
            VerticalForms::PresentationFormForVerticalIdeographicFullStop => '︒',
            VerticalForms::PresentationFormForVerticalColon => '︓',
            VerticalForms::PresentationFormForVerticalSemicolon => '︔',
            VerticalForms::PresentationFormForVerticalExclamationMark => '︕',
            VerticalForms::PresentationFormForVerticalQuestionMark => '︖',
            VerticalForms::PresentationFormForVerticalLeftWhiteLenticularBracket => '︗',
            VerticalForms::PresentationFormForVerticalRightWhiteLenticularBrakcet => '︘',
            VerticalForms::PresentationFormForVerticalHorizontalEllipsis => '︙',
        }
    }
}

impl std::convert::TryFrom<char> for VerticalForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '︐' => Ok(VerticalForms::PresentationFormForVerticalComma),
            '︑' => Ok(VerticalForms::PresentationFormForVerticalIdeographicComma),
            '︒' => Ok(VerticalForms::PresentationFormForVerticalIdeographicFullStop),
            '︓' => Ok(VerticalForms::PresentationFormForVerticalColon),
            '︔' => Ok(VerticalForms::PresentationFormForVerticalSemicolon),
            '︕' => Ok(VerticalForms::PresentationFormForVerticalExclamationMark),
            '︖' => Ok(VerticalForms::PresentationFormForVerticalQuestionMark),
            '︗' => Ok(VerticalForms::PresentationFormForVerticalLeftWhiteLenticularBracket),
            '︘' => Ok(VerticalForms::PresentationFormForVerticalRightWhiteLenticularBrakcet),
            '︙' => Ok(VerticalForms::PresentationFormForVerticalHorizontalEllipsis),
            _ => Err(()),
        }
    }
}

impl Into<u32> for VerticalForms {
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

impl std::convert::TryFrom<u32> for VerticalForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for VerticalForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl VerticalForms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        VerticalForms::PresentationFormForVerticalComma
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("VerticalForms{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
