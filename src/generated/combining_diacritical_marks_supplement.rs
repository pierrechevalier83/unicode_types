
/// An enum to represent all characters in the CombiningDiacriticalMarksSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CombiningDiacriticalMarksSupplement {
    /// \u{1dc0}: '᷀'
    CombiningDottedGraveAccent,
    /// \u{1dc1}: '᷁'
    CombiningDottedAcuteAccent,
    /// \u{1dc2}: '᷂'
    CombiningSnakeBelow,
    /// \u{1dc3}: '᷃'
    CombiningSuspensionMark,
    /// \u{1dc4}: '᷄'
    CombiningMacronDashAcute,
    /// \u{1dc5}: '᷅'
    CombiningGraveDashMacron,
    /// \u{1dc6}: '᷆'
    CombiningMacronDashGrave,
    /// \u{1dc7}: '᷇'
    CombiningAcuteDashMacron,
    /// \u{1dc8}: '᷈'
    CombiningGraveDashAcuteDashGrave,
    /// \u{1dc9}: '᷉'
    CombiningAcuteDashGraveDashAcute,
    /// \u{1dca}: '᷊'
    CombiningLatinSmallLetterRBelow,
    /// \u{1dcb}: '᷋'
    CombiningBreveDashMacron,
    /// \u{1dcc}: '᷌'
    CombiningMacronDashBreve,
    /// \u{1dcd}: '᷍'
    CombiningDoubleCircumflexAbove,
    /// \u{1dce}: '᷎'
    CombiningOgonekAbove,
    /// \u{1dcf}: '᷏'
    CombiningZigzagBelow,
    /// \u{1dd0}: '᷐'
    CombiningIsBelow,
    /// \u{1dd1}: '᷑'
    CombiningUrAbove,
    /// \u{1dd2}: '᷒'
    CombiningUsAbove,
    /// \u{1dd3}: 'ᷓ'
    CombiningLatinSmallLetterFlattenedOpenAAbove,
    /// \u{1dd4}: 'ᷔ'
    CombiningLatinSmallLetterAe,
    /// \u{1dd5}: 'ᷕ'
    CombiningLatinSmallLetterAo,
    /// \u{1dd6}: 'ᷖ'
    CombiningLatinSmallLetterAv,
    /// \u{1dd7}: 'ᷗ'
    CombiningLatinSmallLetterCCedilla,
    /// \u{1dd8}: 'ᷘ'
    CombiningLatinSmallLetterInsularD,
    /// \u{1dd9}: 'ᷙ'
    CombiningLatinSmallLetterEth,
    /// \u{1dda}: 'ᷚ'
    CombiningLatinSmallLetterG,
    /// \u{1ddb}: 'ᷛ'
    CombiningLatinLetterSmallCapitalG,
    /// \u{1ddc}: 'ᷜ'
    CombiningLatinSmallLetterK,
    /// \u{1ddd}: 'ᷝ'
    CombiningLatinSmallLetterL,
    /// \u{1dde}: 'ᷞ'
    CombiningLatinLetterSmallCapitalL,
    /// \u{1ddf}: 'ᷟ'
    CombiningLatinLetterSmallCapitalM,
    /// \u{1de0}: 'ᷠ'
    CombiningLatinSmallLetterN,
    /// \u{1de1}: 'ᷡ'
    CombiningLatinLetterSmallCapitalN,
    /// \u{1de2}: 'ᷢ'
    CombiningLatinLetterSmallCapitalR,
    /// \u{1de3}: 'ᷣ'
    CombiningLatinSmallLetterRRotunda,
    /// \u{1de4}: 'ᷤ'
    CombiningLatinSmallLetterS,
    /// \u{1de5}: 'ᷥ'
    CombiningLatinSmallLetterLongS,
    /// \u{1de6}: 'ᷦ'
    CombiningLatinSmallLetterZ,
    /// \u{1de7}: 'ᷧ'
    CombiningLatinSmallLetterAlpha,
    /// \u{1de8}: 'ᷨ'
    CombiningLatinSmallLetterB,
    /// \u{1de9}: 'ᷩ'
    CombiningLatinSmallLetterBeta,
    /// \u{1dea}: 'ᷪ'
    CombiningLatinSmallLetterSchwa,
    /// \u{1deb}: 'ᷫ'
    CombiningLatinSmallLetterF,
    /// \u{1dec}: 'ᷬ'
    CombiningLatinSmallLetterLWithDoubleMiddleTilde,
    /// \u{1ded}: 'ᷭ'
    CombiningLatinSmallLetterOWithLightCentralizationStroke,
    /// \u{1dee}: 'ᷮ'
    CombiningLatinSmallLetterP,
    /// \u{1def}: 'ᷯ'
    CombiningLatinSmallLetterEsh,
    /// \u{1df0}: 'ᷰ'
    CombiningLatinSmallLetterUWithLightCentralizationStroke,
    /// \u{1df1}: 'ᷱ'
    CombiningLatinSmallLetterW,
    /// \u{1df2}: 'ᷲ'
    CombiningLatinSmallLetterAWithDiaeresis,
    /// \u{1df3}: 'ᷳ'
    CombiningLatinSmallLetterOWithDiaeresis,
    /// \u{1df4}: 'ᷴ'
    CombiningLatinSmallLetterUWithDiaeresis,
    /// \u{1df5}: '᷵'
    CombiningUpTackAbove,
    /// \u{1df6}: '᷶'
    CombiningKavykaAboveRight,
    /// \u{1df7}: '᷷'
    CombiningKavykaAboveLeft,
    /// \u{1df8}: '᷸'
    CombiningDotAboveLeft,
    /// \u{1df9}: '᷹'
    CombiningWideInvertedBridgeBelow,
    /// \u{1dfb}: '᷻'
    CombiningDeletionMark,
    /// \u{1dfc}: '᷼'
    CombiningDoubleInvertedBreveBelow,
    /// \u{1dfd}: '᷽'
    CombiningAlmostEqualToBelow,
    /// \u{1dfe}: '᷾'
    CombiningLeftArrowheadAbove,
}

impl Into<char> for CombiningDiacriticalMarksSupplement {
    fn into(self) -> char {
        match self {
            CombiningDiacriticalMarksSupplement::CombiningDottedGraveAccent => '᷀',
            CombiningDiacriticalMarksSupplement::CombiningDottedAcuteAccent => '᷁',
            CombiningDiacriticalMarksSupplement::CombiningSnakeBelow => '᷂',
            CombiningDiacriticalMarksSupplement::CombiningSuspensionMark => '᷃',
            CombiningDiacriticalMarksSupplement::CombiningMacronDashAcute => '᷄',
            CombiningDiacriticalMarksSupplement::CombiningGraveDashMacron => '᷅',
            CombiningDiacriticalMarksSupplement::CombiningMacronDashGrave => '᷆',
            CombiningDiacriticalMarksSupplement::CombiningAcuteDashMacron => '᷇',
            CombiningDiacriticalMarksSupplement::CombiningGraveDashAcuteDashGrave => '᷈',
            CombiningDiacriticalMarksSupplement::CombiningAcuteDashGraveDashAcute => '᷉',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterRBelow => '᷊',
            CombiningDiacriticalMarksSupplement::CombiningBreveDashMacron => '᷋',
            CombiningDiacriticalMarksSupplement::CombiningMacronDashBreve => '᷌',
            CombiningDiacriticalMarksSupplement::CombiningDoubleCircumflexAbove => '᷍',
            CombiningDiacriticalMarksSupplement::CombiningOgonekAbove => '᷎',
            CombiningDiacriticalMarksSupplement::CombiningZigzagBelow => '᷏',
            CombiningDiacriticalMarksSupplement::CombiningIsBelow => '᷐',
            CombiningDiacriticalMarksSupplement::CombiningUrAbove => '᷑',
            CombiningDiacriticalMarksSupplement::CombiningUsAbove => '᷒',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterFlattenedOpenAAbove => 'ᷓ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAe => 'ᷔ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAo => 'ᷕ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAv => 'ᷖ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterCCedilla => 'ᷗ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterInsularD => 'ᷘ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterEth => 'ᷙ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterG => 'ᷚ',
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalG => 'ᷛ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterK => 'ᷜ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterL => 'ᷝ',
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalL => 'ᷞ',
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalM => 'ᷟ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterN => 'ᷠ',
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalN => 'ᷡ',
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalR => 'ᷢ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterRRotunda => 'ᷣ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterS => 'ᷤ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterLongS => 'ᷥ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterZ => 'ᷦ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAlpha => 'ᷧ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterB => 'ᷨ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterBeta => 'ᷩ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterSchwa => 'ᷪ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterF => 'ᷫ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterLWithDoubleMiddleTilde => 'ᷬ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterOWithLightCentralizationStroke => 'ᷭ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterP => 'ᷮ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterEsh => 'ᷯ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterUWithLightCentralizationStroke => 'ᷰ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterW => 'ᷱ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAWithDiaeresis => 'ᷲ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterOWithDiaeresis => 'ᷳ',
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterUWithDiaeresis => 'ᷴ',
            CombiningDiacriticalMarksSupplement::CombiningUpTackAbove => '᷵',
            CombiningDiacriticalMarksSupplement::CombiningKavykaAboveRight => '᷶',
            CombiningDiacriticalMarksSupplement::CombiningKavykaAboveLeft => '᷷',
            CombiningDiacriticalMarksSupplement::CombiningDotAboveLeft => '᷸',
            CombiningDiacriticalMarksSupplement::CombiningWideInvertedBridgeBelow => '᷹',
            CombiningDiacriticalMarksSupplement::CombiningDeletionMark => '᷻',
            CombiningDiacriticalMarksSupplement::CombiningDoubleInvertedBreveBelow => '᷼',
            CombiningDiacriticalMarksSupplement::CombiningAlmostEqualToBelow => '᷽',
            CombiningDiacriticalMarksSupplement::CombiningLeftArrowheadAbove => '᷾',
        }
    }
}

impl std::convert::TryFrom<char> for CombiningDiacriticalMarksSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᷀' => Ok(CombiningDiacriticalMarksSupplement::CombiningDottedGraveAccent),
            '᷁' => Ok(CombiningDiacriticalMarksSupplement::CombiningDottedAcuteAccent),
            '᷂' => Ok(CombiningDiacriticalMarksSupplement::CombiningSnakeBelow),
            '᷃' => Ok(CombiningDiacriticalMarksSupplement::CombiningSuspensionMark),
            '᷄' => Ok(CombiningDiacriticalMarksSupplement::CombiningMacronDashAcute),
            '᷅' => Ok(CombiningDiacriticalMarksSupplement::CombiningGraveDashMacron),
            '᷆' => Ok(CombiningDiacriticalMarksSupplement::CombiningMacronDashGrave),
            '᷇' => Ok(CombiningDiacriticalMarksSupplement::CombiningAcuteDashMacron),
            '᷈' => Ok(CombiningDiacriticalMarksSupplement::CombiningGraveDashAcuteDashGrave),
            '᷉' => Ok(CombiningDiacriticalMarksSupplement::CombiningAcuteDashGraveDashAcute),
            '᷊' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterRBelow),
            '᷋' => Ok(CombiningDiacriticalMarksSupplement::CombiningBreveDashMacron),
            '᷌' => Ok(CombiningDiacriticalMarksSupplement::CombiningMacronDashBreve),
            '᷍' => Ok(CombiningDiacriticalMarksSupplement::CombiningDoubleCircumflexAbove),
            '᷎' => Ok(CombiningDiacriticalMarksSupplement::CombiningOgonekAbove),
            '᷏' => Ok(CombiningDiacriticalMarksSupplement::CombiningZigzagBelow),
            '᷐' => Ok(CombiningDiacriticalMarksSupplement::CombiningIsBelow),
            '᷑' => Ok(CombiningDiacriticalMarksSupplement::CombiningUrAbove),
            '᷒' => Ok(CombiningDiacriticalMarksSupplement::CombiningUsAbove),
            'ᷓ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterFlattenedOpenAAbove),
            'ᷔ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAe),
            'ᷕ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAo),
            'ᷖ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAv),
            'ᷗ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterCCedilla),
            'ᷘ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterInsularD),
            'ᷙ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterEth),
            'ᷚ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterG),
            'ᷛ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalG),
            'ᷜ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterK),
            'ᷝ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterL),
            'ᷞ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalL),
            'ᷟ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalM),
            'ᷠ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterN),
            'ᷡ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalN),
            'ᷢ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalR),
            'ᷣ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterRRotunda),
            'ᷤ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterS),
            'ᷥ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterLongS),
            'ᷦ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterZ),
            'ᷧ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAlpha),
            'ᷨ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterB),
            'ᷩ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterBeta),
            'ᷪ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterSchwa),
            'ᷫ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterF),
            'ᷬ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterLWithDoubleMiddleTilde),
            'ᷭ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterOWithLightCentralizationStroke),
            'ᷮ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterP),
            'ᷯ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterEsh),
            'ᷰ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterUWithLightCentralizationStroke),
            'ᷱ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterW),
            'ᷲ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAWithDiaeresis),
            'ᷳ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterOWithDiaeresis),
            'ᷴ' => Ok(CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterUWithDiaeresis),
            '᷵' => Ok(CombiningDiacriticalMarksSupplement::CombiningUpTackAbove),
            '᷶' => Ok(CombiningDiacriticalMarksSupplement::CombiningKavykaAboveRight),
            '᷷' => Ok(CombiningDiacriticalMarksSupplement::CombiningKavykaAboveLeft),
            '᷸' => Ok(CombiningDiacriticalMarksSupplement::CombiningDotAboveLeft),
            '᷹' => Ok(CombiningDiacriticalMarksSupplement::CombiningWideInvertedBridgeBelow),
            '᷻' => Ok(CombiningDiacriticalMarksSupplement::CombiningDeletionMark),
            '᷼' => Ok(CombiningDiacriticalMarksSupplement::CombiningDoubleInvertedBreveBelow),
            '᷽' => Ok(CombiningDiacriticalMarksSupplement::CombiningAlmostEqualToBelow),
            '᷾' => Ok(CombiningDiacriticalMarksSupplement::CombiningLeftArrowheadAbove),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CombiningDiacriticalMarksSupplement {
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

impl std::convert::TryFrom<u32> for CombiningDiacriticalMarksSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CombiningDiacriticalMarksSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CombiningDiacriticalMarksSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CombiningDiacriticalMarksSupplement::CombiningDottedGraveAccent
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CombiningDiacriticalMarksSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
