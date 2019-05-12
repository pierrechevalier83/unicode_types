/// \u{fe30} → \u{fe4f}\
///\
/// ︰ ︱ ︲ ︳ ︴ ︵ ︶ ︷ ︸ ︹ ︺ ︻ ︼ ︽ ︾ ︿
/// ﹀ ﹁ ﹂ ﹃ ﹄ ﹅ ﹆ ﹇ ﹈ ﹉ ﹊ ﹋ ﹌ ﹍ ﹎
pub mod constants {
    /// \u{fe30}: '︰'
    pub const PRESENTATION_FORM_FOR_VERTICAL_TWO_DOT_LEADER: char = '︰';
    /// \u{fe31}: '︱'
    pub const PRESENTATION_FORM_FOR_VERTICAL_EM_DASH: char = '︱';
    /// \u{fe32}: '︲'
    pub const PRESENTATION_FORM_FOR_VERTICAL_EN_DASH: char = '︲';
    /// \u{fe33}: '︳'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LOW_LINE: char = '︳';
    /// \u{fe34}: '︴'
    pub const PRESENTATION_FORM_FOR_VERTICAL_WAVY_LOW_LINE: char = '︴';
    /// \u{fe35}: '︵'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_PARENTHESIS: char = '︵';
    /// \u{fe36}: '︶'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_PARENTHESIS: char = '︶';
    /// \u{fe37}: '︷'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_CURLY_BRACKET: char = '︷';
    /// \u{fe38}: '︸'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_CURLY_BRACKET: char = '︸';
    /// \u{fe39}: '︹'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_TORTOISE_SHELL_BRACKET: char = '︹';
    /// \u{fe3a}: '︺'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_TORTOISE_SHELL_BRACKET: char = '︺';
    /// \u{fe3b}: '︻'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_BLACK_LENTICULAR_BRACKET: char = '︻';
    /// \u{fe3c}: '︼'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_BLACK_LENTICULAR_BRACKET: char = '︼';
    /// \u{fe3d}: '︽'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_DOUBLE_ANGLE_BRACKET: char = '︽';
    /// \u{fe3e}: '︾'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_DOUBLE_ANGLE_BRACKET: char = '︾';
    /// \u{fe3f}: '︿'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_ANGLE_BRACKET: char = '︿';
    /// \u{fe40}: '﹀'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_ANGLE_BRACKET: char = '﹀';
    /// \u{fe41}: '﹁'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_CORNER_BRACKET: char = '﹁';
    /// \u{fe42}: '﹂'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_CORNER_BRACKET: char = '﹂';
    /// \u{fe43}: '﹃'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_WHITE_CORNER_BRACKET: char = '﹃';
    /// \u{fe44}: '﹄'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_WHITE_CORNER_BRACKET: char = '﹄';
    /// \u{fe45}: '﹅'
    pub const SESAME_DOT: char = '﹅';
    /// \u{fe46}: '﹆'
    pub const WHITE_SESAME_DOT: char = '﹆';
    /// \u{fe47}: '﹇'
    pub const PRESENTATION_FORM_FOR_VERTICAL_LEFT_SQUARE_BRACKET: char = '﹇';
    /// \u{fe48}: '﹈'
    pub const PRESENTATION_FORM_FOR_VERTICAL_RIGHT_SQUARE_BRACKET: char = '﹈';
    /// \u{fe49}: '﹉'
    pub const DASHED_OVERLINE: char = '﹉';
    /// \u{fe4a}: '﹊'
    pub const CENTRELINE_OVERLINE: char = '﹊';
    /// \u{fe4b}: '﹋'
    pub const WAVY_OVERLINE: char = '﹋';
    /// \u{fe4c}: '﹌'
    pub const DOUBLE_WAVY_OVERLINE: char = '﹌';
    /// \u{fe4d}: '﹍'
    pub const DASHED_LOW_LINE: char = '﹍';
    /// \u{fe4e}: '﹎'
    pub const CENTRELINE_LOW_LINE: char = '﹎';
}

/// \u{fe30} → \u{fe4f}\
///\
/// ︰ ︱ ︲ ︳ ︴ ︵ ︶ ︷ ︸ ︹ ︺ ︻ ︼ ︽ ︾ ︿
/// ﹀ ﹁ ﹂ ﹃ ﹄ ﹅ ﹆ ﹇ ﹈ ﹉ ﹊ ﹋ ﹌ ﹍ ﹎
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKCompatibilityForms {
    /// \u{fe30}: '︰'
    PresentationFormForVerticalTwoDotLeader,
    /// \u{fe31}: '︱'
    PresentationFormForVerticalEmDash,
    /// \u{fe32}: '︲'
    PresentationFormForVerticalEnDash,
    /// \u{fe33}: '︳'
    PresentationFormForVerticalLowLine,
    /// \u{fe34}: '︴'
    PresentationFormForVerticalWavyLowLine,
    /// \u{fe35}: '︵'
    PresentationFormForVerticalLeftParenthesis,
    /// \u{fe36}: '︶'
    PresentationFormForVerticalRightParenthesis,
    /// \u{fe37}: '︷'
    PresentationFormForVerticalLeftCurlyBracket,
    /// \u{fe38}: '︸'
    PresentationFormForVerticalRightCurlyBracket,
    /// \u{fe39}: '︹'
    PresentationFormForVerticalLeftTortoiseShellBracket,
    /// \u{fe3a}: '︺'
    PresentationFormForVerticalRightTortoiseShellBracket,
    /// \u{fe3b}: '︻'
    PresentationFormForVerticalLeftBlackLenticularBracket,
    /// \u{fe3c}: '︼'
    PresentationFormForVerticalRightBlackLenticularBracket,
    /// \u{fe3d}: '︽'
    PresentationFormForVerticalLeftDoubleAngleBracket,
    /// \u{fe3e}: '︾'
    PresentationFormForVerticalRightDoubleAngleBracket,
    /// \u{fe3f}: '︿'
    PresentationFormForVerticalLeftAngleBracket,
    /// \u{fe40}: '﹀'
    PresentationFormForVerticalRightAngleBracket,
    /// \u{fe41}: '﹁'
    PresentationFormForVerticalLeftCornerBracket,
    /// \u{fe42}: '﹂'
    PresentationFormForVerticalRightCornerBracket,
    /// \u{fe43}: '﹃'
    PresentationFormForVerticalLeftWhiteCornerBracket,
    /// \u{fe44}: '﹄'
    PresentationFormForVerticalRightWhiteCornerBracket,
    /// \u{fe45}: '﹅'
    SesameDot,
    /// \u{fe46}: '﹆'
    WhiteSesameDot,
    /// \u{fe47}: '﹇'
    PresentationFormForVerticalLeftSquareBracket,
    /// \u{fe48}: '﹈'
    PresentationFormForVerticalRightSquareBracket,
    /// \u{fe49}: '﹉'
    DashedOverline,
    /// \u{fe4a}: '﹊'
    CentrelineOverline,
    /// \u{fe4b}: '﹋'
    WavyOverline,
    /// \u{fe4c}: '﹌'
    DoubleWavyOverline,
    /// \u{fe4d}: '﹍'
    DashedLowLine,
    /// \u{fe4e}: '﹎'
    CentrelineLowLine,
}

impl Into<char> for CJKCompatibilityForms {
    fn into(self) -> char {
        use constants::*;
        match self {
            CJKCompatibilityForms::PresentationFormForVerticalTwoDotLeader => PRESENTATION_FORM_FOR_VERTICAL_TWO_DOT_LEADER,
            CJKCompatibilityForms::PresentationFormForVerticalEmDash => PRESENTATION_FORM_FOR_VERTICAL_EM_DASH,
            CJKCompatibilityForms::PresentationFormForVerticalEnDash => PRESENTATION_FORM_FOR_VERTICAL_EN_DASH,
            CJKCompatibilityForms::PresentationFormForVerticalLowLine => PRESENTATION_FORM_FOR_VERTICAL_LOW_LINE,
            CJKCompatibilityForms::PresentationFormForVerticalWavyLowLine => PRESENTATION_FORM_FOR_VERTICAL_WAVY_LOW_LINE,
            CJKCompatibilityForms::PresentationFormForVerticalLeftParenthesis => PRESENTATION_FORM_FOR_VERTICAL_LEFT_PARENTHESIS,
            CJKCompatibilityForms::PresentationFormForVerticalRightParenthesis => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_PARENTHESIS,
            CJKCompatibilityForms::PresentationFormForVerticalLeftCurlyBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_CURLY_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightCurlyBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_CURLY_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalLeftTortoiseShellBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_TORTOISE_SHELL_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightTortoiseShellBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_TORTOISE_SHELL_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalLeftBlackLenticularBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_BLACK_LENTICULAR_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightBlackLenticularBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_BLACK_LENTICULAR_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalLeftDoubleAngleBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_DOUBLE_ANGLE_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightDoubleAngleBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_DOUBLE_ANGLE_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalLeftAngleBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_ANGLE_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightAngleBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_ANGLE_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalLeftCornerBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_CORNER_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightCornerBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_CORNER_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalLeftWhiteCornerBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_WHITE_CORNER_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightWhiteCornerBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_WHITE_CORNER_BRACKET,
            CJKCompatibilityForms::SesameDot => SESAME_DOT,
            CJKCompatibilityForms::WhiteSesameDot => WHITE_SESAME_DOT,
            CJKCompatibilityForms::PresentationFormForVerticalLeftSquareBracket => PRESENTATION_FORM_FOR_VERTICAL_LEFT_SQUARE_BRACKET,
            CJKCompatibilityForms::PresentationFormForVerticalRightSquareBracket => PRESENTATION_FORM_FOR_VERTICAL_RIGHT_SQUARE_BRACKET,
            CJKCompatibilityForms::DashedOverline => DASHED_OVERLINE,
            CJKCompatibilityForms::CentrelineOverline => CENTRELINE_OVERLINE,
            CJKCompatibilityForms::WavyOverline => WAVY_OVERLINE,
            CJKCompatibilityForms::DoubleWavyOverline => DOUBLE_WAVY_OVERLINE,
            CJKCompatibilityForms::DashedLowLine => DASHED_LOW_LINE,
            CJKCompatibilityForms::CentrelineLowLine => CENTRELINE_LOW_LINE,
        }
    }
}

impl std::convert::TryFrom<char> for CJKCompatibilityForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            PRESENTATION_FORM_FOR_VERTICAL_TWO_DOT_LEADER => Ok(CJKCompatibilityForms::PresentationFormForVerticalTwoDotLeader),
            PRESENTATION_FORM_FOR_VERTICAL_EM_DASH => Ok(CJKCompatibilityForms::PresentationFormForVerticalEmDash),
            PRESENTATION_FORM_FOR_VERTICAL_EN_DASH => Ok(CJKCompatibilityForms::PresentationFormForVerticalEnDash),
            PRESENTATION_FORM_FOR_VERTICAL_LOW_LINE => Ok(CJKCompatibilityForms::PresentationFormForVerticalLowLine),
            PRESENTATION_FORM_FOR_VERTICAL_WAVY_LOW_LINE => Ok(CJKCompatibilityForms::PresentationFormForVerticalWavyLowLine),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_PARENTHESIS => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftParenthesis),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_PARENTHESIS => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightParenthesis),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_CURLY_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftCurlyBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_CURLY_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightCurlyBracket),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_TORTOISE_SHELL_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftTortoiseShellBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_TORTOISE_SHELL_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightTortoiseShellBracket),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_BLACK_LENTICULAR_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftBlackLenticularBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_BLACK_LENTICULAR_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightBlackLenticularBracket),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_DOUBLE_ANGLE_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftDoubleAngleBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_DOUBLE_ANGLE_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightDoubleAngleBracket),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_ANGLE_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftAngleBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_ANGLE_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightAngleBracket),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_CORNER_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftCornerBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_CORNER_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightCornerBracket),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_WHITE_CORNER_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftWhiteCornerBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_WHITE_CORNER_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightWhiteCornerBracket),
            SESAME_DOT => Ok(CJKCompatibilityForms::SesameDot),
            WHITE_SESAME_DOT => Ok(CJKCompatibilityForms::WhiteSesameDot),
            PRESENTATION_FORM_FOR_VERTICAL_LEFT_SQUARE_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftSquareBracket),
            PRESENTATION_FORM_FOR_VERTICAL_RIGHT_SQUARE_BRACKET => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightSquareBracket),
            DASHED_OVERLINE => Ok(CJKCompatibilityForms::DashedOverline),
            CENTRELINE_OVERLINE => Ok(CJKCompatibilityForms::CentrelineOverline),
            WAVY_OVERLINE => Ok(CJKCompatibilityForms::WavyOverline),
            DOUBLE_WAVY_OVERLINE => Ok(CJKCompatibilityForms::DoubleWavyOverline),
            DASHED_LOW_LINE => Ok(CJKCompatibilityForms::DashedLowLine),
            CENTRELINE_LOW_LINE => Ok(CJKCompatibilityForms::CentrelineLowLine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKCompatibilityForms {
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

impl std::convert::TryFrom<u32> for CJKCompatibilityForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKCompatibilityForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKCompatibilityForms {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        CJKCompatibilityForms::PresentationFormForVerticalTwoDotLeader
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKCompatibilityForms::PresentationFormForVerticalTwoDotLeader => "presentation form for vertical two dot leader",
            CJKCompatibilityForms::PresentationFormForVerticalEmDash => "presentation form for vertical em dash",
            CJKCompatibilityForms::PresentationFormForVerticalEnDash => "presentation form for vertical en dash",
            CJKCompatibilityForms::PresentationFormForVerticalLowLine => "presentation form for vertical low line",
            CJKCompatibilityForms::PresentationFormForVerticalWavyLowLine => "presentation form for vertical wavy low line",
            CJKCompatibilityForms::PresentationFormForVerticalLeftParenthesis => "presentation form for vertical left parenthesis",
            CJKCompatibilityForms::PresentationFormForVerticalRightParenthesis => "presentation form for vertical right parenthesis",
            CJKCompatibilityForms::PresentationFormForVerticalLeftCurlyBracket => "presentation form for vertical left curly bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightCurlyBracket => "presentation form for vertical right curly bracket",
            CJKCompatibilityForms::PresentationFormForVerticalLeftTortoiseShellBracket => "presentation form for vertical left tortoise shell bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightTortoiseShellBracket => "presentation form for vertical right tortoise shell bracket",
            CJKCompatibilityForms::PresentationFormForVerticalLeftBlackLenticularBracket => "presentation form for vertical left black lenticular bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightBlackLenticularBracket => "presentation form for vertical right black lenticular bracket",
            CJKCompatibilityForms::PresentationFormForVerticalLeftDoubleAngleBracket => "presentation form for vertical left double angle bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightDoubleAngleBracket => "presentation form for vertical right double angle bracket",
            CJKCompatibilityForms::PresentationFormForVerticalLeftAngleBracket => "presentation form for vertical left angle bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightAngleBracket => "presentation form for vertical right angle bracket",
            CJKCompatibilityForms::PresentationFormForVerticalLeftCornerBracket => "presentation form for vertical left corner bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightCornerBracket => "presentation form for vertical right corner bracket",
            CJKCompatibilityForms::PresentationFormForVerticalLeftWhiteCornerBracket => "presentation form for vertical left white corner bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightWhiteCornerBracket => "presentation form for vertical right white corner bracket",
            CJKCompatibilityForms::SesameDot => "sesame dot",
            CJKCompatibilityForms::WhiteSesameDot => "white sesame dot",
            CJKCompatibilityForms::PresentationFormForVerticalLeftSquareBracket => "presentation form for vertical left square bracket",
            CJKCompatibilityForms::PresentationFormForVerticalRightSquareBracket => "presentation form for vertical right square bracket",
            CJKCompatibilityForms::DashedOverline => "dashed overline",
            CJKCompatibilityForms::CentrelineOverline => "centreline overline",
            CJKCompatibilityForms::WavyOverline => "wavy overline",
            CJKCompatibilityForms::DoubleWavyOverline => "double wavy overline",
            CJKCompatibilityForms::DashedLowLine => "dashed low line",
            CJKCompatibilityForms::CentrelineLowLine => "centreline low line",
        }
    }
}
