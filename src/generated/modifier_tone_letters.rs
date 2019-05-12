
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ModifierToneLetters{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
