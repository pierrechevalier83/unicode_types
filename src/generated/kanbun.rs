
/// An enum to represent all characters in the Kanbun block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kanbun {
    /// \u{3190}: '㆐'
    IdeographicAnnotationLinkingMark,
    /// \u{3191}: '㆑'
    IdeographicAnnotationReverseMark,
    /// \u{3192}: '㆒'
    IdeographicAnnotationOneMark,
    /// \u{3193}: '㆓'
    IdeographicAnnotationTwoMark,
    /// \u{3194}: '㆔'
    IdeographicAnnotationThreeMark,
    /// \u{3195}: '㆕'
    IdeographicAnnotationFourMark,
    /// \u{3196}: '㆖'
    IdeographicAnnotationTopMark,
    /// \u{3197}: '㆗'
    IdeographicAnnotationMiddleMark,
    /// \u{3198}: '㆘'
    IdeographicAnnotationBottomMark,
    /// \u{3199}: '㆙'
    IdeographicAnnotationFirstMark,
    /// \u{319a}: '㆚'
    IdeographicAnnotationSecondMark,
    /// \u{319b}: '㆛'
    IdeographicAnnotationThirdMark,
    /// \u{319c}: '㆜'
    IdeographicAnnotationFourthMark,
    /// \u{319d}: '㆝'
    IdeographicAnnotationHeavenMark,
    /// \u{319e}: '㆞'
    IdeographicAnnotationEarthMark,
}

impl Into<char> for Kanbun {
    fn into(self) -> char {
        match self {
            Kanbun::IdeographicAnnotationLinkingMark => '㆐',
            Kanbun::IdeographicAnnotationReverseMark => '㆑',
            Kanbun::IdeographicAnnotationOneMark => '㆒',
            Kanbun::IdeographicAnnotationTwoMark => '㆓',
            Kanbun::IdeographicAnnotationThreeMark => '㆔',
            Kanbun::IdeographicAnnotationFourMark => '㆕',
            Kanbun::IdeographicAnnotationTopMark => '㆖',
            Kanbun::IdeographicAnnotationMiddleMark => '㆗',
            Kanbun::IdeographicAnnotationBottomMark => '㆘',
            Kanbun::IdeographicAnnotationFirstMark => '㆙',
            Kanbun::IdeographicAnnotationSecondMark => '㆚',
            Kanbun::IdeographicAnnotationThirdMark => '㆛',
            Kanbun::IdeographicAnnotationFourthMark => '㆜',
            Kanbun::IdeographicAnnotationHeavenMark => '㆝',
            Kanbun::IdeographicAnnotationEarthMark => '㆞',
        }
    }
}

impl std::convert::TryFrom<char> for Kanbun {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '㆐' => Ok(Kanbun::IdeographicAnnotationLinkingMark),
            '㆑' => Ok(Kanbun::IdeographicAnnotationReverseMark),
            '㆒' => Ok(Kanbun::IdeographicAnnotationOneMark),
            '㆓' => Ok(Kanbun::IdeographicAnnotationTwoMark),
            '㆔' => Ok(Kanbun::IdeographicAnnotationThreeMark),
            '㆕' => Ok(Kanbun::IdeographicAnnotationFourMark),
            '㆖' => Ok(Kanbun::IdeographicAnnotationTopMark),
            '㆗' => Ok(Kanbun::IdeographicAnnotationMiddleMark),
            '㆘' => Ok(Kanbun::IdeographicAnnotationBottomMark),
            '㆙' => Ok(Kanbun::IdeographicAnnotationFirstMark),
            '㆚' => Ok(Kanbun::IdeographicAnnotationSecondMark),
            '㆛' => Ok(Kanbun::IdeographicAnnotationThirdMark),
            '㆜' => Ok(Kanbun::IdeographicAnnotationFourthMark),
            '㆝' => Ok(Kanbun::IdeographicAnnotationHeavenMark),
            '㆞' => Ok(Kanbun::IdeographicAnnotationEarthMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Kanbun {
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

impl std::convert::TryFrom<u32> for Kanbun {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Kanbun {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Kanbun {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Kanbun::IdeographicAnnotationLinkingMark
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Kanbun{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
