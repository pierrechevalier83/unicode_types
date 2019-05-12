
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
        match self {
            ModifierToneLetters::ModifierLetterChineseToneYinPing => '꜀',
            ModifierToneLetters::ModifierLetterChineseToneYangPing => '꜁',
            ModifierToneLetters::ModifierLetterChineseToneYinShang => '꜂',
            ModifierToneLetters::ModifierLetterChineseToneYangShang => '꜃',
            ModifierToneLetters::ModifierLetterChineseToneYinQu => '꜄',
            ModifierToneLetters::ModifierLetterChineseToneYangQu => '꜅',
            ModifierToneLetters::ModifierLetterChineseToneYinRu => '꜆',
            ModifierToneLetters::ModifierLetterChineseToneYangRu => '꜇',
            ModifierToneLetters::ModifierLetterExtraDashHighDottedToneBar => '꜈',
            ModifierToneLetters::ModifierLetterHighDottedToneBar => '꜉',
            ModifierToneLetters::ModifierLetterMidDottedToneBar => '꜊',
            ModifierToneLetters::ModifierLetterLowDottedToneBar => '꜋',
            ModifierToneLetters::ModifierLetterExtraDashLowDottedToneBar => '꜌',
            ModifierToneLetters::ModifierLetterExtraDashHighDottedLeftDashStemToneBar => '꜍',
            ModifierToneLetters::ModifierLetterHighDottedLeftDashStemToneBar => '꜎',
            ModifierToneLetters::ModifierLetterMidDottedLeftDashStemToneBar => '꜏',
            ModifierToneLetters::ModifierLetterLowDottedLeftDashStemToneBar => '꜐',
            ModifierToneLetters::ModifierLetterExtraDashLowDottedLeftDashStemToneBar => '꜑',
            ModifierToneLetters::ModifierLetterExtraDashHighLeftDashStemToneBar => '꜒',
            ModifierToneLetters::ModifierLetterHighLeftDashStemToneBar => '꜓',
            ModifierToneLetters::ModifierLetterMidLeftDashStemToneBar => '꜔',
            ModifierToneLetters::ModifierLetterLowLeftDashStemToneBar => '꜕',
            ModifierToneLetters::ModifierLetterExtraDashLowLeftDashStemToneBar => '꜖',
            ModifierToneLetters::ModifierLetterDotVerticalBar => 'ꜗ',
            ModifierToneLetters::ModifierLetterDotSlash => 'ꜘ',
            ModifierToneLetters::ModifierLetterDotHorizontalBar => 'ꜙ',
            ModifierToneLetters::ModifierLetterLowerRightCornerAngle => 'ꜚ',
            ModifierToneLetters::ModifierLetterRaisedUpArrow => 'ꜛ',
            ModifierToneLetters::ModifierLetterRaisedDownArrow => 'ꜜ',
            ModifierToneLetters::ModifierLetterRaisedExclamationMark => 'ꜝ',
            ModifierToneLetters::ModifierLetterRaisedInvertedExclamationMark => 'ꜞ',
        }
    }
}

impl std::convert::TryFrom<char> for ModifierToneLetters {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '꜀' => Ok(ModifierToneLetters::ModifierLetterChineseToneYinPing),
            '꜁' => Ok(ModifierToneLetters::ModifierLetterChineseToneYangPing),
            '꜂' => Ok(ModifierToneLetters::ModifierLetterChineseToneYinShang),
            '꜃' => Ok(ModifierToneLetters::ModifierLetterChineseToneYangShang),
            '꜄' => Ok(ModifierToneLetters::ModifierLetterChineseToneYinQu),
            '꜅' => Ok(ModifierToneLetters::ModifierLetterChineseToneYangQu),
            '꜆' => Ok(ModifierToneLetters::ModifierLetterChineseToneYinRu),
            '꜇' => Ok(ModifierToneLetters::ModifierLetterChineseToneYangRu),
            '꜈' => Ok(ModifierToneLetters::ModifierLetterExtraDashHighDottedToneBar),
            '꜉' => Ok(ModifierToneLetters::ModifierLetterHighDottedToneBar),
            '꜊' => Ok(ModifierToneLetters::ModifierLetterMidDottedToneBar),
            '꜋' => Ok(ModifierToneLetters::ModifierLetterLowDottedToneBar),
            '꜌' => Ok(ModifierToneLetters::ModifierLetterExtraDashLowDottedToneBar),
            '꜍' => Ok(ModifierToneLetters::ModifierLetterExtraDashHighDottedLeftDashStemToneBar),
            '꜎' => Ok(ModifierToneLetters::ModifierLetterHighDottedLeftDashStemToneBar),
            '꜏' => Ok(ModifierToneLetters::ModifierLetterMidDottedLeftDashStemToneBar),
            '꜐' => Ok(ModifierToneLetters::ModifierLetterLowDottedLeftDashStemToneBar),
            '꜑' => Ok(ModifierToneLetters::ModifierLetterExtraDashLowDottedLeftDashStemToneBar),
            '꜒' => Ok(ModifierToneLetters::ModifierLetterExtraDashHighLeftDashStemToneBar),
            '꜓' => Ok(ModifierToneLetters::ModifierLetterHighLeftDashStemToneBar),
            '꜔' => Ok(ModifierToneLetters::ModifierLetterMidLeftDashStemToneBar),
            '꜕' => Ok(ModifierToneLetters::ModifierLetterLowLeftDashStemToneBar),
            '꜖' => Ok(ModifierToneLetters::ModifierLetterExtraDashLowLeftDashStemToneBar),
            'ꜗ' => Ok(ModifierToneLetters::ModifierLetterDotVerticalBar),
            'ꜘ' => Ok(ModifierToneLetters::ModifierLetterDotSlash),
            'ꜙ' => Ok(ModifierToneLetters::ModifierLetterDotHorizontalBar),
            'ꜚ' => Ok(ModifierToneLetters::ModifierLetterLowerRightCornerAngle),
            'ꜛ' => Ok(ModifierToneLetters::ModifierLetterRaisedUpArrow),
            'ꜜ' => Ok(ModifierToneLetters::ModifierLetterRaisedDownArrow),
            'ꜝ' => Ok(ModifierToneLetters::ModifierLetterRaisedExclamationMark),
            'ꜞ' => Ok(ModifierToneLetters::ModifierLetterRaisedInvertedExclamationMark),
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
