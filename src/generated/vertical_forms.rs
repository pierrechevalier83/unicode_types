/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{fe10}: '︐'
    pub const PRESENTATION_FORM_FOR_VERTICAL_COMMA: char = '︐';
    /// \u{fe11}: '︑'
    pub const PRESENTATION_FORM_FOR_VERTICAL_IDEOGRAPHIC_COMMA: char = '︑';
    /// \u{fe12}: '︒'
    pub const PRESENTATION_FORM_FOR_VERTICAL_IDEOGRAPHIC_FULL_STOP: char = '︒';
    /// \u{fe13}: '︓'
    pub const PRESENTATION_FORM_FOR_VERTICAL_COLON: char = '︓';
    /// \u{fe14}: '︔'
    pub const PRESENTATION_FORM_FOR_VERTICAL_SEMICOLON: char = '︔';
    /// \u{fe15}: '︕'
    pub const PRESENTATION_FORM_FOR_VERTICAL_EXCLAMATION_MARK: char = '︕';
    /// \u{fe16}: '︖'
    pub const PRESENTATION_FORM_FOR_VERTICAL_QUESTION_MARK: char = '︖';
    /// \u{fe17}: '︗'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_WHITE_LENTICULAR_BRACKET: char = '︗';
    /// \u{fe18}: '︘'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_WHITE_LENTICULAR_BRAKCET: char = '︘';
    /// \u{fe19}: '︙'
    pub const PRESENTATION_FORM_FOR_VERTICAL_HORIZONTAL_ELLIPSIS: char = '︙';
}

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
        use constants::*;
        match self {
            VerticalForms::PresentationFormForVerticalComma => PRESENTATION_FORM_FOR_VERTICAL_COMMA,
            VerticalForms::PresentationFormForVerticalIdeographicComma => PRESENTATION_FORM_FOR_VERTICAL_IDEOGRAPHIC_COMMA,
            VerticalForms::PresentationFormForVerticalIdeographicFullStop => PRESENTATION_FORM_FOR_VERTICAL_IDEOGRAPHIC_FULL_STOP,
            VerticalForms::PresentationFormForVerticalColon => PRESENTATION_FORM_FOR_VERTICAL_COLON,
            VerticalForms::PresentationFormForVerticalSemicolon => PRESENTATION_FORM_FOR_VERTICAL_SEMICOLON,
            VerticalForms::PresentationFormForVerticalExclamationMark => PRESENTATION_FORM_FOR_VERTICAL_EXCLAMATION_MARK,
            VerticalForms::PresentationFormForVerticalQuestionMark => PRESENTATION_FORM_FOR_VERTICAL_QUESTION_MARK,
            VerticalForms::PresentationFormForVerticalLeftWhiteLenticularBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_WHITE_LENTICULAR_BRACKET,
            VerticalForms::PresentationFormForVerticalRightWhiteLenticularBrakcet => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_WHITE_LENTICULAR_BRAKCET,
            VerticalForms::PresentationFormForVerticalHorizontalEllipsis => PRESENTATION_FORM_FOR_VERTICAL_HORIZONTAL_ELLIPSIS,
        }
    }
}

impl std::convert::TryFrom<char> for VerticalForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            PRESENTATION_FORM_FOR_VERTICAL_COMMA => Ok(VerticalForms::PresentationFormForVerticalComma),
            PRESENTATION_FORM_FOR_VERTICAL_IDEOGRAPHIC_COMMA => Ok(VerticalForms::PresentationFormForVerticalIdeographicComma),
            PRESENTATION_FORM_FOR_VERTICAL_IDEOGRAPHIC_FULL_STOP => Ok(VerticalForms::PresentationFormForVerticalIdeographicFullStop),
            PRESENTATION_FORM_FOR_VERTICAL_COLON => Ok(VerticalForms::PresentationFormForVerticalColon),
            PRESENTATION_FORM_FOR_VERTICAL_SEMICOLON => Ok(VerticalForms::PresentationFormForVerticalSemicolon),
            PRESENTATION_FORM_FOR_VERTICAL_EXCLAMATION_MARK => Ok(VerticalForms::PresentationFormForVerticalExclamationMark),
            PRESENTATION_FORM_FOR_VERTICAL_QUESTION_MARK => Ok(VerticalForms::PresentationFormForVerticalQuestionMark),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_WHITE_LENTICULAR_BRACKET => Ok(VerticalForms::PresentationFormForVerticalLeftWhiteLenticularBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_WHITE_LENTICULAR_BRAKCET => Ok(VerticalForms::PresentationFormForVerticalRightWhiteLenticularBrakcet),
            PRESENTATION_FORM_FOR_VERTICAL_HORIZONTAL_ELLIPSIS => Ok(VerticalForms::PresentationFormForVerticalHorizontalEllipsis),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            VerticalForms::PresentationFormForVerticalComma => "presentation form for vertical comma",
            VerticalForms::PresentationFormForVerticalIdeographicComma => "presentation form for vertical ideographic comma",
            VerticalForms::PresentationFormForVerticalIdeographicFullStop => "presentation form for vertical ideographic full stop",
            VerticalForms::PresentationFormForVerticalColon => "presentation form for vertical colon",
            VerticalForms::PresentationFormForVerticalSemicolon => "presentation form for vertical semicolon",
            VerticalForms::PresentationFormForVerticalExclamationMark => "presentation form for vertical exclamation mark",
            VerticalForms::PresentationFormForVerticalQuestionMark => "presentation form for vertical question mark",
            VerticalForms::PresentationFormForVerticalLeftWhiteLenticularBracket => "presentation form for vertical left white lenticular bracket",
            VerticalForms::PresentationFormForVerticalRightWhiteLenticularBrakcet => "presentation form for vertical right white lenticular brakcet",
            VerticalForms::PresentationFormForVerticalHorizontalEllipsis => "presentation form for vertical horizontal ellipsis",
        }
    }
}
