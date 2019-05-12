
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CombiningDiacriticalMarksSupplement::CombiningDottedGraveAccent => "combining dotted grave accent",
            CombiningDiacriticalMarksSupplement::CombiningDottedAcuteAccent => "combining dotted acute accent",
            CombiningDiacriticalMarksSupplement::CombiningSnakeBelow => "combining snake below",
            CombiningDiacriticalMarksSupplement::CombiningSuspensionMark => "combining suspension mark",
            CombiningDiacriticalMarksSupplement::CombiningMacronDashAcute => "combining macron-acute",
            CombiningDiacriticalMarksSupplement::CombiningGraveDashMacron => "combining grave-macron",
            CombiningDiacriticalMarksSupplement::CombiningMacronDashGrave => "combining macron-grave",
            CombiningDiacriticalMarksSupplement::CombiningAcuteDashMacron => "combining acute-macron",
            CombiningDiacriticalMarksSupplement::CombiningGraveDashAcuteDashGrave => "combining grave-acute-grave",
            CombiningDiacriticalMarksSupplement::CombiningAcuteDashGraveDashAcute => "combining acute-grave-acute",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterRBelow => "combining latin small letter r below",
            CombiningDiacriticalMarksSupplement::CombiningBreveDashMacron => "combining breve-macron",
            CombiningDiacriticalMarksSupplement::CombiningMacronDashBreve => "combining macron-breve",
            CombiningDiacriticalMarksSupplement::CombiningDoubleCircumflexAbove => "combining double circumflex above",
            CombiningDiacriticalMarksSupplement::CombiningOgonekAbove => "combining ogonek above",
            CombiningDiacriticalMarksSupplement::CombiningZigzagBelow => "combining zigzag below",
            CombiningDiacriticalMarksSupplement::CombiningIsBelow => "combining is below",
            CombiningDiacriticalMarksSupplement::CombiningUrAbove => "combining ur above",
            CombiningDiacriticalMarksSupplement::CombiningUsAbove => "combining us above",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterFlattenedOpenAAbove => "combining latin small letter flattened open a above",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAe => "combining latin small letter ae",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAo => "combining latin small letter ao",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAv => "combining latin small letter av",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterCCedilla => "combining latin small letter c cedilla",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterInsularD => "combining latin small letter insular d",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterEth => "combining latin small letter eth",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterG => "combining latin small letter g",
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalG => "combining latin letter small capital g",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterK => "combining latin small letter k",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterL => "combining latin small letter l",
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalL => "combining latin letter small capital l",
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalM => "combining latin letter small capital m",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterN => "combining latin small letter n",
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalN => "combining latin letter small capital n",
            CombiningDiacriticalMarksSupplement::CombiningLatinLetterSmallCapitalR => "combining latin letter small capital r",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterRRotunda => "combining latin small letter r rotunda",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterS => "combining latin small letter s",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterLongS => "combining latin small letter long s",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterZ => "combining latin small letter z",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAlpha => "combining latin small letter alpha",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterB => "combining latin small letter b",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterBeta => "combining latin small letter beta",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterSchwa => "combining latin small letter schwa",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterF => "combining latin small letter f",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterLWithDoubleMiddleTilde => "combining latin small letter l with double middle tilde",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterOWithLightCentralizationStroke => "combining latin small letter o with light centralization stroke",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterP => "combining latin small letter p",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterEsh => "combining latin small letter esh",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterUWithLightCentralizationStroke => "combining latin small letter u with light centralization stroke",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterW => "combining latin small letter w",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterAWithDiaeresis => "combining latin small letter a with diaeresis",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterOWithDiaeresis => "combining latin small letter o with diaeresis",
            CombiningDiacriticalMarksSupplement::CombiningLatinSmallLetterUWithDiaeresis => "combining latin small letter u with diaeresis",
            CombiningDiacriticalMarksSupplement::CombiningUpTackAbove => "combining up tack above",
            CombiningDiacriticalMarksSupplement::CombiningKavykaAboveRight => "combining kavyka above right",
            CombiningDiacriticalMarksSupplement::CombiningKavykaAboveLeft => "combining kavyka above left",
            CombiningDiacriticalMarksSupplement::CombiningDotAboveLeft => "combining dot above left",
            CombiningDiacriticalMarksSupplement::CombiningWideInvertedBridgeBelow => "combining wide inverted bridge below",
            CombiningDiacriticalMarksSupplement::CombiningDeletionMark => "combining deletion mark",
            CombiningDiacriticalMarksSupplement::CombiningDoubleInvertedBreveBelow => "combining double inverted breve below",
            CombiningDiacriticalMarksSupplement::CombiningAlmostEqualToBelow => "combining almost equal to below",
            CombiningDiacriticalMarksSupplement::CombiningLeftArrowheadAbove => "combining left arrowhead above",
        }
    }
}
