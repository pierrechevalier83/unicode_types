
/// An enum to represent all characters in the CJKCompatibilityForms block.
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
        match self {
            CJKCompatibilityForms::PresentationFormForVerticalTwoDotLeader => '︰',
            CJKCompatibilityForms::PresentationFormForVerticalEmDash => '︱',
            CJKCompatibilityForms::PresentationFormForVerticalEnDash => '︲',
            CJKCompatibilityForms::PresentationFormForVerticalLowLine => '︳',
            CJKCompatibilityForms::PresentationFormForVerticalWavyLowLine => '︴',
            CJKCompatibilityForms::PresentationFormForVerticalLeftParenthesis => '︵',
            CJKCompatibilityForms::PresentationFormForVerticalRightParenthesis => '︶',
            CJKCompatibilityForms::PresentationFormForVerticalLeftCurlyBracket => '︷',
            CJKCompatibilityForms::PresentationFormForVerticalRightCurlyBracket => '︸',
            CJKCompatibilityForms::PresentationFormForVerticalLeftTortoiseShellBracket => '︹',
            CJKCompatibilityForms::PresentationFormForVerticalRightTortoiseShellBracket => '︺',
            CJKCompatibilityForms::PresentationFormForVerticalLeftBlackLenticularBracket => '︻',
            CJKCompatibilityForms::PresentationFormForVerticalRightBlackLenticularBracket => '︼',
            CJKCompatibilityForms::PresentationFormForVerticalLeftDoubleAngleBracket => '︽',
            CJKCompatibilityForms::PresentationFormForVerticalRightDoubleAngleBracket => '︾',
            CJKCompatibilityForms::PresentationFormForVerticalLeftAngleBracket => '︿',
            CJKCompatibilityForms::PresentationFormForVerticalRightAngleBracket => '﹀',
            CJKCompatibilityForms::PresentationFormForVerticalLeftCornerBracket => '﹁',
            CJKCompatibilityForms::PresentationFormForVerticalRightCornerBracket => '﹂',
            CJKCompatibilityForms::PresentationFormForVerticalLeftWhiteCornerBracket => '﹃',
            CJKCompatibilityForms::PresentationFormForVerticalRightWhiteCornerBracket => '﹄',
            CJKCompatibilityForms::SesameDot => '﹅',
            CJKCompatibilityForms::WhiteSesameDot => '﹆',
            CJKCompatibilityForms::PresentationFormForVerticalLeftSquareBracket => '﹇',
            CJKCompatibilityForms::PresentationFormForVerticalRightSquareBracket => '﹈',
            CJKCompatibilityForms::DashedOverline => '﹉',
            CJKCompatibilityForms::CentrelineOverline => '﹊',
            CJKCompatibilityForms::WavyOverline => '﹋',
            CJKCompatibilityForms::DoubleWavyOverline => '﹌',
            CJKCompatibilityForms::DashedLowLine => '﹍',
            CJKCompatibilityForms::CentrelineLowLine => '﹎',
        }
    }
}

impl std::convert::TryFrom<char> for CJKCompatibilityForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '︰' => Ok(CJKCompatibilityForms::PresentationFormForVerticalTwoDotLeader),
            '︱' => Ok(CJKCompatibilityForms::PresentationFormForVerticalEmDash),
            '︲' => Ok(CJKCompatibilityForms::PresentationFormForVerticalEnDash),
            '︳' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLowLine),
            '︴' => Ok(CJKCompatibilityForms::PresentationFormForVerticalWavyLowLine),
            '︵' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftParenthesis),
            '︶' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightParenthesis),
            '︷' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftCurlyBracket),
            '︸' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightCurlyBracket),
            '︹' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftTortoiseShellBracket),
            '︺' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightTortoiseShellBracket),
            '︻' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftBlackLenticularBracket),
            '︼' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightBlackLenticularBracket),
            '︽' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftDoubleAngleBracket),
            '︾' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightDoubleAngleBracket),
            '︿' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftAngleBracket),
            '﹀' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightAngleBracket),
            '﹁' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftCornerBracket),
            '﹂' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightCornerBracket),
            '﹃' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftWhiteCornerBracket),
            '﹄' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightWhiteCornerBracket),
            '﹅' => Ok(CJKCompatibilityForms::SesameDot),
            '﹆' => Ok(CJKCompatibilityForms::WhiteSesameDot),
            '﹇' => Ok(CJKCompatibilityForms::PresentationFormForVerticalLeftSquareBracket),
            '﹈' => Ok(CJKCompatibilityForms::PresentationFormForVerticalRightSquareBracket),
            '﹉' => Ok(CJKCompatibilityForms::DashedOverline),
            '﹊' => Ok(CJKCompatibilityForms::CentrelineOverline),
            '﹋' => Ok(CJKCompatibilityForms::WavyOverline),
            '﹌' => Ok(CJKCompatibilityForms::DoubleWavyOverline),
            '﹍' => Ok(CJKCompatibilityForms::DashedLowLine),
            '﹎' => Ok(CJKCompatibilityForms::CentrelineLowLine),
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
    /// The character with the lowest index in this unicode block
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
