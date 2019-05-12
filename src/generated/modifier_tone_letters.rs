/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{a700}: '꜀'
    pub const MODIFIER_LETTER_CHINESE_TONE_YIN_PING: char = '꜀';
    /// \u{a701}: '꜁'
    pub const MODIFIER_LETTER_CHINESE_TONE_YANG_PING: char = '꜁';
    /// \u{a702}: '꜂'
    pub const MODIFIER_LETTER_CHINESE_TONE_YIN_SHANG: char = '꜂';
    /// \u{a703}: '꜃'
    pub const MODIFIER_LETTER_CHINESE_TONE_YANG_SHANG: char = '꜃';
    /// \u{a704}: '꜄'
    pub const MODIFIER_LETTER_CHINESE_TONE_YIN_QU: char = '꜄';
    /// \u{a705}: '꜅'
    pub const MODIFIER_LETTER_CHINESE_TONE_YANG_QU: char = '꜅';
    /// \u{a706}: '꜆'
    pub const MODIFIER_LETTER_CHINESE_TONE_YIN_RU: char = '꜆';
    /// \u{a707}: '꜇'
    pub const MODIFIER_LETTER_CHINESE_TONE_YANG_RU: char = '꜇';
    /// \u{a708}: '꜈'
    pub const MODIFIER_LETTER_EXTRA_DASH_HIGH_DOTTED_TONE_BAR: char = '꜈';
    /// \u{a709}: '꜉'
    pub const MODIFIER_LETTER_HIGH_DOTTED_TONE_BAR: char = '꜉';
    /// \u{a70a}: '꜊'
    pub const MODIFIER_LETTER_MID_DOTTED_TONE_BAR: char = '꜊';
    /// \u{a70b}: '꜋'
    pub const MODIFIER_LETTER_LOW_DOTTED_TONE_BAR: char = '꜋';
    /// \u{a70c}: '꜌'
    pub const MODIFIER_LETTER_EXTRA_DASH_LOW_DOTTED_TONE_BAR: char = '꜌';
    /// \u{a70d}: '꜍'
    pub const MODIFIER_LETTER_EXTRA_DASH_HIGH_DOTTED_LEFT_DASH_STEM_TONE_BAR: char = '꜍';
    /// \u{a70e}: '꜎'
    pub const MODIFIER_LETTER_HIGH_DOTTED_LEFT_DASH_STEM_TONE_BAR: char = '꜎';
    /// \u{a70f}: '꜏'
    pub const MODIFIER_LETTER_MID_DOTTED_LEFT_DASH_STEM_TONE_BAR: char = '꜏';
    /// \u{a710}: '꜐'
    pub const MODIFIER_LETTER_LOW_DOTTED_LEFT_DASH_STEM_TONE_BAR: char = '꜐';
    /// \u{a711}: '꜑'
    pub const MODIFIER_LETTER_EXTRA_DASH_LOW_DOTTED_LEFT_DASH_STEM_TONE_BAR: char = '꜑';
    /// \u{a712}: '꜒'
    pub const MODIFIER_LETTER_EXTRA_DASH_HIGH_LEFT_DASH_STEM_TONE_BAR: char = '꜒';
    /// \u{a713}: '꜓'
    pub const MODIFIER_LETTER_HIGH_LEFT_DASH_STEM_TONE_BAR: char = '꜓';
    /// \u{a714}: '꜔'
    pub const MODIFIER_LETTER_MID_LEFT_DASH_STEM_TONE_BAR: char = '꜔';
    /// \u{a715}: '꜕'
    pub const MODIFIER_LETTER_LOW_LEFT_DASH_STEM_TONE_BAR: char = '꜕';
    /// \u{a716}: '꜖'
    pub const MODIFIER_LETTER_EXTRA_DASH_LOW_LEFT_DASH_STEM_TONE_BAR: char = '꜖';
    /// \u{a717}: 'ꜗ'
    pub const MODIFIER_LETTER_DOT_VERTICAL_BAR: char = 'ꜗ';
    /// \u{a718}: 'ꜘ'
    pub const MODIFIER_LETTER_DOT_SLASH: char = 'ꜘ';
    /// \u{a719}: 'ꜙ'
    pub const MODIFIER_LETTER_DOT_HORIZONTAL_BAR: char = 'ꜙ';
    /// \u{a71a}: 'ꜚ'
    pub const MODIFIER_LETTER_LOWER_RIGHT_CORNER_ANGLE: char = 'ꜚ';
    /// \u{a71b}: 'ꜛ'
    pub const MODIFIER_LETTER_RAISED_UP_ARROW: char = 'ꜛ';
    /// \u{a71c}: 'ꜜ'
    pub const MODIFIER_LETTER_RAISED_DOWN_ARROW: char = 'ꜜ';
    /// \u{a71d}: 'ꜝ'
    pub const MODIFIER_LETTER_RAISED_EXCLAMATION_MARK: char = 'ꜝ';
    /// \u{a71e}: 'ꜞ'
    pub const MODIFIER_LETTER_RAISED_INVERTED_EXCLAMATION_MARK: char = 'ꜞ';
}

/// An enum to represent all characters in the ModifierToneLetters block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ModifierToneLetters {
    /// \u{a700}: '꜀'
    ModifierLetterChineseToneYinPing,
    /// \u{a701}: '꜁'
    ModifierLetterChineseToneYangPing,
    /// \u{a702}: '꜂'
    ModifierLetterChineseToneYinShang,
    /// \u{a703}: '꜃'
    ModifierLetterChineseToneYangShang,
    /// \u{a704}: '꜄'
    ModifierLetterChineseToneYinQu,
    /// \u{a705}: '꜅'
    ModifierLetterChineseToneYangQu,
    /// \u{a706}: '꜆'
    ModifierLetterChineseToneYinRu,
    /// \u{a707}: '꜇'
    ModifierLetterChineseToneYangRu,
    /// \u{a708}: '꜈'
    ModifierLetterExtraDashHighDottedToneBar,
    /// \u{a709}: '꜉'
    ModifierLetterHighDottedToneBar,
    /// \u{a70a}: '꜊'
    ModifierLetterMidDottedToneBar,
    /// \u{a70b}: '꜋'
    ModifierLetterLowDottedToneBar,
    /// \u{a70c}: '꜌'
    ModifierLetterExtraDashLowDottedToneBar,
    /// \u{a70d}: '꜍'
    ModifierLetterExtraDashHighDottedLeftDashStemToneBar,
    /// \u{a70e}: '꜎'
    ModifierLetterHighDottedLeftDashStemToneBar,
    /// \u{a70f}: '꜏'
    ModifierLetterMidDottedLeftDashStemToneBar,
    /// \u{a710}: '꜐'
    ModifierLetterLowDottedLeftDashStemToneBar,
    /// \u{a711}: '꜑'
    ModifierLetterExtraDashLowDottedLeftDashStemToneBar,
    /// \u{a712}: '꜒'
    ModifierLetterExtraDashHighLeftDashStemToneBar,
    /// \u{a713}: '꜓'
    ModifierLetterHighLeftDashStemToneBar,
    /// \u{a714}: '꜔'
    ModifierLetterMidLeftDashStemToneBar,
    /// \u{a715}: '꜕'
    ModifierLetterLowLeftDashStemToneBar,
    /// \u{a716}: '꜖'
    ModifierLetterExtraDashLowLeftDashStemToneBar,
    /// \u{a717}: 'ꜗ'
    ModifierLetterDotVerticalBar,
    /// \u{a718}: 'ꜘ'
    ModifierLetterDotSlash,
    /// \u{a719}: 'ꜙ'
    ModifierLetterDotHorizontalBar,
    /// \u{a71a}: 'ꜚ'
    ModifierLetterLowerRightCornerAngle,
    /// \u{a71b}: 'ꜛ'
    ModifierLetterRaisedUpArrow,
    /// \u{a71c}: 'ꜜ'
    ModifierLetterRaisedDownArrow,
    /// \u{a71d}: 'ꜝ'
    ModifierLetterRaisedExclamationMark,
    /// \u{a71e}: 'ꜞ'
    ModifierLetterRaisedInvertedExclamationMark,
}

impl Into<char> for ModifierToneLetters {
    fn into(self) -> char {
        use constants::*;
        match self {
            ModifierToneLetters::ModifierLetterChineseToneYinPing => MODIFIER_LETTER_CHINESE_TONE_YIN_PING,
            ModifierToneLetters::ModifierLetterChineseToneYangPing => MODIFIER_LETTER_CHINESE_TONE_YANG_PING,
            ModifierToneLetters::ModifierLetterChineseToneYinShang => MODIFIER_LETTER_CHINESE_TONE_YIN_SHANG,
            ModifierToneLetters::ModifierLetterChineseToneYangShang => MODIFIER_LETTER_CHINESE_TONE_YANG_SHANG,
            ModifierToneLetters::ModifierLetterChineseToneYinQu => MODIFIER_LETTER_CHINESE_TONE_YIN_QU,
            ModifierToneLetters::ModifierLetterChineseToneYangQu => MODIFIER_LETTER_CHINESE_TONE_YANG_QU,
            ModifierToneLetters::ModifierLetterChineseToneYinRu => MODIFIER_LETTER_CHINESE_TONE_YIN_RU,
            ModifierToneLetters::ModifierLetterChineseToneYangRu => MODIFIER_LETTER_CHINESE_TONE_YANG_RU,
            ModifierToneLetters::ModifierLetterExtraDashHighDottedToneBar => MODIFIER_LETTER_EXTRA_DASH_HIGH_DOTTED_TONE_BAR,
            ModifierToneLetters::ModifierLetterHighDottedToneBar => MODIFIER_LETTER_HIGH_DOTTED_TONE_BAR,
            ModifierToneLetters::ModifierLetterMidDottedToneBar => MODIFIER_LETTER_MID_DOTTED_TONE_BAR,
            ModifierToneLetters::ModifierLetterLowDottedToneBar => MODIFIER_LETTER_LOW_DOTTED_TONE_BAR,
            ModifierToneLetters::ModifierLetterExtraDashLowDottedToneBar => MODIFIER_LETTER_EXTRA_DASH_LOW_DOTTED_TONE_BAR,
            ModifierToneLetters::ModifierLetterExtraDashHighDottedLeftDashStemToneBar => MODIFIER_LETTER_EXTRA_DASH_HIGH_DOTTED_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterHighDottedLeftDashStemToneBar => MODIFIER_LETTER_HIGH_DOTTED_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterMidDottedLeftDashStemToneBar => MODIFIER_LETTER_MID_DOTTED_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterLowDottedLeftDashStemToneBar => MODIFIER_LETTER_LOW_DOTTED_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterExtraDashLowDottedLeftDashStemToneBar => MODIFIER_LETTER_EXTRA_DASH_LOW_DOTTED_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterExtraDashHighLeftDashStemToneBar => MODIFIER_LETTER_EXTRA_DASH_HIGH_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterHighLeftDashStemToneBar => MODIFIER_LETTER_HIGH_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterMidLeftDashStemToneBar => MODIFIER_LETTER_MID_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterLowLeftDashStemToneBar => MODIFIER_LETTER_LOW_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterExtraDashLowLeftDashStemToneBar => MODIFIER_LETTER_EXTRA_DASH_LOW_LEFT_DASH_STEM_TONE_BAR,
            ModifierToneLetters::ModifierLetterDotVerticalBar => MODIFIER_LETTER_DOT_VERTICAL_BAR,
            ModifierToneLetters::ModifierLetterDotSlash => MODIFIER_LETTER_DOT_SLASH,
            ModifierToneLetters::ModifierLetterDotHorizontalBar => MODIFIER_LETTER_DOT_HORIZONTAL_BAR,
            ModifierToneLetters::ModifierLetterLowerRightCornerAngle => MODIFIER_LETTER_LOWER_RIGHT_CORNER_ANGLE,
            ModifierToneLetters::ModifierLetterRaisedUpArrow => MODIFIER_LETTER_RAISED_UP_ARROW,
            ModifierToneLetters::ModifierLetterRaisedDownArrow => MODIFIER_LETTER_RAISED_DOWN_ARROW,
            ModifierToneLetters::ModifierLetterRaisedExclamationMark => MODIFIER_LETTER_RAISED_EXCLAMATION_MARK,
            ModifierToneLetters::ModifierLetterRaisedInvertedExclamationMark => MODIFIER_LETTER_RAISED_INVERTED_EXCLAMATION_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for ModifierToneLetters {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MODIFIER_LETTER_CHINESE_TONE_YIN_PING => Ok(ModifierToneLetters::ModifierLetterChineseToneYinPing),
            MODIFIER_LETTER_CHINESE_TONE_YANG_PING => Ok(ModifierToneLetters::ModifierLetterChineseToneYangPing),
            MODIFIER_LETTER_CHINESE_TONE_YIN_SHANG => Ok(ModifierToneLetters::ModifierLetterChineseToneYinShang),
            MODIFIER_LETTER_CHINESE_TONE_YANG_SHANG => Ok(ModifierToneLetters::ModifierLetterChineseToneYangShang),
            MODIFIER_LETTER_CHINESE_TONE_YIN_QU => Ok(ModifierToneLetters::ModifierLetterChineseToneYinQu),
            MODIFIER_LETTER_CHINESE_TONE_YANG_QU => Ok(ModifierToneLetters::ModifierLetterChineseToneYangQu),
            MODIFIER_LETTER_CHINESE_TONE_YIN_RU => Ok(ModifierToneLetters::ModifierLetterChineseToneYinRu),
            MODIFIER_LETTER_CHINESE_TONE_YANG_RU => Ok(ModifierToneLetters::ModifierLetterChineseToneYangRu),
            MODIFIER_LETTER_EXTRA_DASH_HIGH_DOTTED_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterExtraDashHighDottedToneBar),
            MODIFIER_LETTER_HIGH_DOTTED_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterHighDottedToneBar),
            MODIFIER_LETTER_MID_DOTTED_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterMidDottedToneBar),
            MODIFIER_LETTER_LOW_DOTTED_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterLowDottedToneBar),
            MODIFIER_LETTER_EXTRA_DASH_LOW_DOTTED_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterExtraDashLowDottedToneBar),
            MODIFIER_LETTER_EXTRA_DASH_HIGH_DOTTED_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterExtraDashHighDottedLeftDashStemToneBar),
            MODIFIER_LETTER_HIGH_DOTTED_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterHighDottedLeftDashStemToneBar),
            MODIFIER_LETTER_MID_DOTTED_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterMidDottedLeftDashStemToneBar),
            MODIFIER_LETTER_LOW_DOTTED_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterLowDottedLeftDashStemToneBar),
            MODIFIER_LETTER_EXTRA_DASH_LOW_DOTTED_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterExtraDashLowDottedLeftDashStemToneBar),
            MODIFIER_LETTER_EXTRA_DASH_HIGH_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterExtraDashHighLeftDashStemToneBar),
            MODIFIER_LETTER_HIGH_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterHighLeftDashStemToneBar),
            MODIFIER_LETTER_MID_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterMidLeftDashStemToneBar),
            MODIFIER_LETTER_LOW_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterLowLeftDashStemToneBar),
            MODIFIER_LETTER_EXTRA_DASH_LOW_LEFT_DASH_STEM_TONE_BAR => Ok(ModifierToneLetters::ModifierLetterExtraDashLowLeftDashStemToneBar),
            MODIFIER_LETTER_DOT_VERTICAL_BAR => Ok(ModifierToneLetters::ModifierLetterDotVerticalBar),
            MODIFIER_LETTER_DOT_SLASH => Ok(ModifierToneLetters::ModifierLetterDotSlash),
            MODIFIER_LETTER_DOT_HORIZONTAL_BAR => Ok(ModifierToneLetters::ModifierLetterDotHorizontalBar),
            MODIFIER_LETTER_LOWER_RIGHT_CORNER_ANGLE => Ok(ModifierToneLetters::ModifierLetterLowerRightCornerAngle),
            MODIFIER_LETTER_RAISED_UP_ARROW => Ok(ModifierToneLetters::ModifierLetterRaisedUpArrow),
            MODIFIER_LETTER_RAISED_DOWN_ARROW => Ok(ModifierToneLetters::ModifierLetterRaisedDownArrow),
            MODIFIER_LETTER_RAISED_EXCLAMATION_MARK => Ok(ModifierToneLetters::ModifierLetterRaisedExclamationMark),
            MODIFIER_LETTER_RAISED_INVERTED_EXCLAMATION_MARK => Ok(ModifierToneLetters::ModifierLetterRaisedInvertedExclamationMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ModifierToneLetters {
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

impl std::convert::TryFrom<u32> for ModifierToneLetters {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ModifierToneLetters {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ModifierToneLetters {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ModifierToneLetters::ModifierLetterChineseToneYinPing
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            ModifierToneLetters::ModifierLetterChineseToneYinPing => "modifier letter chinese tone yin ping",
            ModifierToneLetters::ModifierLetterChineseToneYangPing => "modifier letter chinese tone yang ping",
            ModifierToneLetters::ModifierLetterChineseToneYinShang => "modifier letter chinese tone yin shang",
            ModifierToneLetters::ModifierLetterChineseToneYangShang => "modifier letter chinese tone yang shang",
            ModifierToneLetters::ModifierLetterChineseToneYinQu => "modifier letter chinese tone yin qu",
            ModifierToneLetters::ModifierLetterChineseToneYangQu => "modifier letter chinese tone yang qu",
            ModifierToneLetters::ModifierLetterChineseToneYinRu => "modifier letter chinese tone yin ru",
            ModifierToneLetters::ModifierLetterChineseToneYangRu => "modifier letter chinese tone yang ru",
            ModifierToneLetters::ModifierLetterExtraDashHighDottedToneBar => "modifier letter extra-high dotted tone bar",
            ModifierToneLetters::ModifierLetterHighDottedToneBar => "modifier letter high dotted tone bar",
            ModifierToneLetters::ModifierLetterMidDottedToneBar => "modifier letter mid dotted tone bar",
            ModifierToneLetters::ModifierLetterLowDottedToneBar => "modifier letter low dotted tone bar",
            ModifierToneLetters::ModifierLetterExtraDashLowDottedToneBar => "modifier letter extra-low dotted tone bar",
            ModifierToneLetters::ModifierLetterExtraDashHighDottedLeftDashStemToneBar => "modifier letter extra-high dotted left-stem tone bar",
            ModifierToneLetters::ModifierLetterHighDottedLeftDashStemToneBar => "modifier letter high dotted left-stem tone bar",
            ModifierToneLetters::ModifierLetterMidDottedLeftDashStemToneBar => "modifier letter mid dotted left-stem tone bar",
            ModifierToneLetters::ModifierLetterLowDottedLeftDashStemToneBar => "modifier letter low dotted left-stem tone bar",
            ModifierToneLetters::ModifierLetterExtraDashLowDottedLeftDashStemToneBar => "modifier letter extra-low dotted left-stem tone bar",
            ModifierToneLetters::ModifierLetterExtraDashHighLeftDashStemToneBar => "modifier letter extra-high left-stem tone bar",
            ModifierToneLetters::ModifierLetterHighLeftDashStemToneBar => "modifier letter high left-stem tone bar",
            ModifierToneLetters::ModifierLetterMidLeftDashStemToneBar => "modifier letter mid left-stem tone bar",
            ModifierToneLetters::ModifierLetterLowLeftDashStemToneBar => "modifier letter low left-stem tone bar",
            ModifierToneLetters::ModifierLetterExtraDashLowLeftDashStemToneBar => "modifier letter extra-low left-stem tone bar",
            ModifierToneLetters::ModifierLetterDotVerticalBar => "modifier letter dot vertical bar",
            ModifierToneLetters::ModifierLetterDotSlash => "modifier letter dot slash",
            ModifierToneLetters::ModifierLetterDotHorizontalBar => "modifier letter dot horizontal bar",
            ModifierToneLetters::ModifierLetterLowerRightCornerAngle => "modifier letter lower right corner angle",
            ModifierToneLetters::ModifierLetterRaisedUpArrow => "modifier letter raised up arrow",
            ModifierToneLetters::ModifierLetterRaisedDownArrow => "modifier letter raised down arrow",
            ModifierToneLetters::ModifierLetterRaisedExclamationMark => "modifier letter raised exclamation mark",
            ModifierToneLetters::ModifierLetterRaisedInvertedExclamationMark => "modifier letter raised inverted exclamation mark",
        }
    }
}
