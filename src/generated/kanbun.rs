/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{3190}: '㆐'
    pub const IDEOGRAPHIC_ANNOTATION_LINKING_MARK: char = '㆐';
    /// \u{3191}: '㆑'
    pub const IDEOGRAPHIC_ANNOTATION_REVERSE_MARK: char = '㆑';
    /// \u{3192}: '㆒'
    pub const IDEOGRAPHIC_ANNOTATION_ONE_MARK: char = '㆒';
    /// \u{3193}: '㆓'
    pub const IDEOGRAPHIC_ANNOTATION_TWO_MARK: char = '㆓';
    /// \u{3194}: '㆔'
    pub const IDEOGRAPHIC_ANNOTATION_THREE_MARK: char = '㆔';
    /// \u{3195}: '㆕'
    pub const IDEOGRAPHIC_ANNOTATION_FOUR_MARK: char = '㆕';
    /// \u{3196}: '㆖'
    pub const IDEOGRAPHIC_ANNOTATION_TOP_MARK: char = '㆖';
    /// \u{3197}: '㆗'
    pub const IDEOGRAPHIC_ANNOTATION_MIDDLE_MARK: char = '㆗';
    /// \u{3198}: '㆘'
    pub const IDEOGRAPHIC_ANNOTATION_BOTTOM_MARK: char = '㆘';
    /// \u{3199}: '㆙'
    pub const IDEOGRAPHIC_ANNOTATION_FIRST_MARK: char = '㆙';
    /// \u{319a}: '㆚'
    pub const IDEOGRAPHIC_ANNOTATION_SECOND_MARK: char = '㆚';
    /// \u{319b}: '㆛'
    pub const IDEOGRAPHIC_ANNOTATION_THIRD_MARK: char = '㆛';
    /// \u{319c}: '㆜'
    pub const IDEOGRAPHIC_ANNOTATION_FOURTH_MARK: char = '㆜';
    /// \u{319d}: '㆝'
    pub const IDEOGRAPHIC_ANNOTATION_HEAVEN_MARK: char = '㆝';
    /// \u{319e}: '㆞'
    pub const IDEOGRAPHIC_ANNOTATION_EARTH_MARK: char = '㆞';
}

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
        use constants::*;
        match self {
            Kanbun::IdeographicAnnotationLinkingMark => IDEOGRAPHIC_ANNOTATION_LINKING_MARK,
            Kanbun::IdeographicAnnotationReverseMark => IDEOGRAPHIC_ANNOTATION_REVERSE_MARK,
            Kanbun::IdeographicAnnotationOneMark => IDEOGRAPHIC_ANNOTATION_ONE_MARK,
            Kanbun::IdeographicAnnotationTwoMark => IDEOGRAPHIC_ANNOTATION_TWO_MARK,
            Kanbun::IdeographicAnnotationThreeMark => IDEOGRAPHIC_ANNOTATION_THREE_MARK,
            Kanbun::IdeographicAnnotationFourMark => IDEOGRAPHIC_ANNOTATION_FOUR_MARK,
            Kanbun::IdeographicAnnotationTopMark => IDEOGRAPHIC_ANNOTATION_TOP_MARK,
            Kanbun::IdeographicAnnotationMiddleMark => IDEOGRAPHIC_ANNOTATION_MIDDLE_MARK,
            Kanbun::IdeographicAnnotationBottomMark => IDEOGRAPHIC_ANNOTATION_BOTTOM_MARK,
            Kanbun::IdeographicAnnotationFirstMark => IDEOGRAPHIC_ANNOTATION_FIRST_MARK,
            Kanbun::IdeographicAnnotationSecondMark => IDEOGRAPHIC_ANNOTATION_SECOND_MARK,
            Kanbun::IdeographicAnnotationThirdMark => IDEOGRAPHIC_ANNOTATION_THIRD_MARK,
            Kanbun::IdeographicAnnotationFourthMark => IDEOGRAPHIC_ANNOTATION_FOURTH_MARK,
            Kanbun::IdeographicAnnotationHeavenMark => IDEOGRAPHIC_ANNOTATION_HEAVEN_MARK,
            Kanbun::IdeographicAnnotationEarthMark => IDEOGRAPHIC_ANNOTATION_EARTH_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for Kanbun {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            IDEOGRAPHIC_ANNOTATION_LINKING_MARK => Ok(Kanbun::IdeographicAnnotationLinkingMark),
            IDEOGRAPHIC_ANNOTATION_REVERSE_MARK => Ok(Kanbun::IdeographicAnnotationReverseMark),
            IDEOGRAPHIC_ANNOTATION_ONE_MARK => Ok(Kanbun::IdeographicAnnotationOneMark),
            IDEOGRAPHIC_ANNOTATION_TWO_MARK => Ok(Kanbun::IdeographicAnnotationTwoMark),
            IDEOGRAPHIC_ANNOTATION_THREE_MARK => Ok(Kanbun::IdeographicAnnotationThreeMark),
            IDEOGRAPHIC_ANNOTATION_FOUR_MARK => Ok(Kanbun::IdeographicAnnotationFourMark),
            IDEOGRAPHIC_ANNOTATION_TOP_MARK => Ok(Kanbun::IdeographicAnnotationTopMark),
            IDEOGRAPHIC_ANNOTATION_MIDDLE_MARK => Ok(Kanbun::IdeographicAnnotationMiddleMark),
            IDEOGRAPHIC_ANNOTATION_BOTTOM_MARK => Ok(Kanbun::IdeographicAnnotationBottomMark),
            IDEOGRAPHIC_ANNOTATION_FIRST_MARK => Ok(Kanbun::IdeographicAnnotationFirstMark),
            IDEOGRAPHIC_ANNOTATION_SECOND_MARK => Ok(Kanbun::IdeographicAnnotationSecondMark),
            IDEOGRAPHIC_ANNOTATION_THIRD_MARK => Ok(Kanbun::IdeographicAnnotationThirdMark),
            IDEOGRAPHIC_ANNOTATION_FOURTH_MARK => Ok(Kanbun::IdeographicAnnotationFourthMark),
            IDEOGRAPHIC_ANNOTATION_HEAVEN_MARK => Ok(Kanbun::IdeographicAnnotationHeavenMark),
            IDEOGRAPHIC_ANNOTATION_EARTH_MARK => Ok(Kanbun::IdeographicAnnotationEarthMark),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Kanbun::IdeographicAnnotationLinkingMark => "ideographic annotation linking mark",
            Kanbun::IdeographicAnnotationReverseMark => "ideographic annotation reverse mark",
            Kanbun::IdeographicAnnotationOneMark => "ideographic annotation one mark",
            Kanbun::IdeographicAnnotationTwoMark => "ideographic annotation two mark",
            Kanbun::IdeographicAnnotationThreeMark => "ideographic annotation three mark",
            Kanbun::IdeographicAnnotationFourMark => "ideographic annotation four mark",
            Kanbun::IdeographicAnnotationTopMark => "ideographic annotation top mark",
            Kanbun::IdeographicAnnotationMiddleMark => "ideographic annotation middle mark",
            Kanbun::IdeographicAnnotationBottomMark => "ideographic annotation bottom mark",
            Kanbun::IdeographicAnnotationFirstMark => "ideographic annotation first mark",
            Kanbun::IdeographicAnnotationSecondMark => "ideographic annotation second mark",
            Kanbun::IdeographicAnnotationThirdMark => "ideographic annotation third mark",
            Kanbun::IdeographicAnnotationFourthMark => "ideographic annotation fourth mark",
            Kanbun::IdeographicAnnotationHeavenMark => "ideographic annotation heaven mark",
            Kanbun::IdeographicAnnotationEarthMark => "ideographic annotation earth mark",
        }
    }
}
