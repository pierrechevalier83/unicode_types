
/// An enum to represent all characters in the SpacingModifierLetters block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SpacingModifierLetters {
    /// \u{2b0}: 'ʰ'
    ModifierLetterSmallH,
    /// \u{2b1}: 'ʱ'
    ModifierLetterSmallHWithHook,
    /// \u{2b2}: 'ʲ'
    ModifierLetterSmallJ,
    /// \u{2b3}: 'ʳ'
    ModifierLetterSmallR,
    /// \u{2b4}: 'ʴ'
    ModifierLetterSmallTurnedR,
    /// \u{2b5}: 'ʵ'
    ModifierLetterSmallTurnedRWithHook,
    /// \u{2b6}: 'ʶ'
    ModifierLetterSmallCapitalInvertedR,
    /// \u{2b7}: 'ʷ'
    ModifierLetterSmallW,
    /// \u{2b8}: 'ʸ'
    ModifierLetterSmallY,
    /// \u{2b9}: 'ʹ'
    ModifierLetterPrime,
    /// \u{2ba}: 'ʺ'
    ModifierLetterDoublePrime,
    /// \u{2bb}: 'ʻ'
    ModifierLetterTurnedComma,
    /// \u{2bc}: 'ʼ'
    ModifierLetterApostrophe,
    /// \u{2bd}: 'ʽ'
    ModifierLetterReversedComma,
    /// \u{2be}: 'ʾ'
    ModifierLetterRightHalfRing,
    /// \u{2bf}: 'ʿ'
    ModifierLetterLeftHalfRing,
    /// \u{2c0}: 'ˀ'
    ModifierLetterGlottalStop,
    /// \u{2c1}: 'ˁ'
    ModifierLetterReversedGlottalStop,
    /// \u{2c2}: '˂'
    ModifierLetterLeftArrowhead,
    /// \u{2c3}: '˃'
    ModifierLetterRightArrowhead,
    /// \u{2c4}: '˄'
    ModifierLetterUpArrowhead,
    /// \u{2c5}: '˅'
    ModifierLetterDownArrowhead,
    /// \u{2c6}: 'ˆ'
    ModifierLetterCircumflexAccent,
    /// \u{2c7}: 'ˇ'
    Caron,
    /// \u{2c8}: 'ˈ'
    ModifierLetterVerticalLine,
    /// \u{2c9}: 'ˉ'
    ModifierLetterMacron,
    /// \u{2ca}: 'ˊ'
    ModifierLetterAcuteAccent,
    /// \u{2cb}: 'ˋ'
    ModifierLetterGraveAccent,
    /// \u{2cc}: 'ˌ'
    ModifierLetterLowVerticalLine,
    /// \u{2cd}: 'ˍ'
    ModifierLetterLowMacron,
    /// \u{2ce}: 'ˎ'
    ModifierLetterLowGraveAccent,
    /// \u{2cf}: 'ˏ'
    ModifierLetterLowAcuteAccent,
    /// \u{2d0}: 'ː'
    ModifierLetterTriangularColon,
    /// \u{2d1}: 'ˑ'
    ModifierLetterHalfTriangularColon,
    /// \u{2d2}: '˒'
    ModifierLetterCentredRightHalfRing,
    /// \u{2d3}: '˓'
    ModifierLetterCentredLeftHalfRing,
    /// \u{2d4}: '˔'
    ModifierLetterUpTack,
    /// \u{2d5}: '˕'
    ModifierLetterDownTack,
    /// \u{2d6}: '˖'
    ModifierLetterPlusSign,
    /// \u{2d7}: '˗'
    ModifierLetterMinusSign,
    /// \u{2d8}: '˘'
    Breve,
    /// \u{2d9}: '˙'
    DotAbove,
    /// \u{2da}: '˚'
    RingAbove,
    /// \u{2db}: '˛'
    Ogonek,
    /// \u{2dc}: '˜'
    SmallTilde,
    /// \u{2dd}: '˝'
    DoubleAcuteAccent,
    /// \u{2de}: '˞'
    ModifierLetterRhoticHook,
    /// \u{2df}: '˟'
    ModifierLetterCrossAccent,
    /// \u{2e0}: 'ˠ'
    ModifierLetterSmallGamma,
    /// \u{2e1}: 'ˡ'
    ModifierLetterSmallL,
    /// \u{2e2}: 'ˢ'
    ModifierLetterSmallS,
    /// \u{2e3}: 'ˣ'
    ModifierLetterSmallX,
    /// \u{2e4}: 'ˤ'
    ModifierLetterSmallReversedGlottalStop,
    /// \u{2e5}: '˥'
    ModifierLetterExtraDashHighToneBar,
    /// \u{2e6}: '˦'
    ModifierLetterHighToneBar,
    /// \u{2e7}: '˧'
    ModifierLetterMidToneBar,
    /// \u{2e8}: '˨'
    ModifierLetterLowToneBar,
    /// \u{2e9}: '˩'
    ModifierLetterExtraDashLowToneBar,
    /// \u{2ea}: '˪'
    ModifierLetterYinDepartingToneMark,
    /// \u{2eb}: '˫'
    ModifierLetterYangDepartingToneMark,
    /// \u{2ec}: 'ˬ'
    ModifierLetterVoicing,
    /// \u{2ed}: '˭'
    ModifierLetterUnaspirated,
    /// \u{2ee}: 'ˮ'
    ModifierLetterDoubleApostrophe,
    /// \u{2ef}: '˯'
    ModifierLetterLowDownArrowhead,
    /// \u{2f0}: '˰'
    ModifierLetterLowUpArrowhead,
    /// \u{2f1}: '˱'
    ModifierLetterLowLeftArrowhead,
    /// \u{2f2}: '˲'
    ModifierLetterLowRightArrowhead,
    /// \u{2f3}: '˳'
    ModifierLetterLowRing,
    /// \u{2f4}: '˴'
    ModifierLetterMiddleGraveAccent,
    /// \u{2f5}: '˵'
    ModifierLetterMiddleDoubleGraveAccent,
    /// \u{2f6}: '˶'
    ModifierLetterMiddleDoubleAcuteAccent,
    /// \u{2f7}: '˷'
    ModifierLetterLowTilde,
    /// \u{2f8}: '˸'
    ModifierLetterRaisedColon,
    /// \u{2f9}: '˹'
    ModifierLetterBeginHighTone,
    /// \u{2fa}: '˺'
    ModifierLetterEndHighTone,
    /// \u{2fb}: '˻'
    ModifierLetterBeginLowTone,
    /// \u{2fc}: '˼'
    ModifierLetterEndLowTone,
    /// \u{2fd}: '˽'
    ModifierLetterShelf,
    /// \u{2fe}: '˾'
    ModifierLetterOpenShelf,
}

impl Into<char> for SpacingModifierLetters {
    fn into(self) -> char {
        match self {
            SpacingModifierLetters::ModifierLetterSmallH => 'ʰ',
            SpacingModifierLetters::ModifierLetterSmallHWithHook => 'ʱ',
            SpacingModifierLetters::ModifierLetterSmallJ => 'ʲ',
            SpacingModifierLetters::ModifierLetterSmallR => 'ʳ',
            SpacingModifierLetters::ModifierLetterSmallTurnedR => 'ʴ',
            SpacingModifierLetters::ModifierLetterSmallTurnedRWithHook => 'ʵ',
            SpacingModifierLetters::ModifierLetterSmallCapitalInvertedR => 'ʶ',
            SpacingModifierLetters::ModifierLetterSmallW => 'ʷ',
            SpacingModifierLetters::ModifierLetterSmallY => 'ʸ',
            SpacingModifierLetters::ModifierLetterPrime => 'ʹ',
            SpacingModifierLetters::ModifierLetterDoublePrime => 'ʺ',
            SpacingModifierLetters::ModifierLetterTurnedComma => 'ʻ',
            SpacingModifierLetters::ModifierLetterApostrophe => 'ʼ',
            SpacingModifierLetters::ModifierLetterReversedComma => 'ʽ',
            SpacingModifierLetters::ModifierLetterRightHalfRing => 'ʾ',
            SpacingModifierLetters::ModifierLetterLeftHalfRing => 'ʿ',
            SpacingModifierLetters::ModifierLetterGlottalStop => 'ˀ',
            SpacingModifierLetters::ModifierLetterReversedGlottalStop => 'ˁ',
            SpacingModifierLetters::ModifierLetterLeftArrowhead => '˂',
            SpacingModifierLetters::ModifierLetterRightArrowhead => '˃',
            SpacingModifierLetters::ModifierLetterUpArrowhead => '˄',
            SpacingModifierLetters::ModifierLetterDownArrowhead => '˅',
            SpacingModifierLetters::ModifierLetterCircumflexAccent => 'ˆ',
            SpacingModifierLetters::Caron => 'ˇ',
            SpacingModifierLetters::ModifierLetterVerticalLine => 'ˈ',
            SpacingModifierLetters::ModifierLetterMacron => 'ˉ',
            SpacingModifierLetters::ModifierLetterAcuteAccent => 'ˊ',
            SpacingModifierLetters::ModifierLetterGraveAccent => 'ˋ',
            SpacingModifierLetters::ModifierLetterLowVerticalLine => 'ˌ',
            SpacingModifierLetters::ModifierLetterLowMacron => 'ˍ',
            SpacingModifierLetters::ModifierLetterLowGraveAccent => 'ˎ',
            SpacingModifierLetters::ModifierLetterLowAcuteAccent => 'ˏ',
            SpacingModifierLetters::ModifierLetterTriangularColon => 'ː',
            SpacingModifierLetters::ModifierLetterHalfTriangularColon => 'ˑ',
            SpacingModifierLetters::ModifierLetterCentredRightHalfRing => '˒',
            SpacingModifierLetters::ModifierLetterCentredLeftHalfRing => '˓',
            SpacingModifierLetters::ModifierLetterUpTack => '˔',
            SpacingModifierLetters::ModifierLetterDownTack => '˕',
            SpacingModifierLetters::ModifierLetterPlusSign => '˖',
            SpacingModifierLetters::ModifierLetterMinusSign => '˗',
            SpacingModifierLetters::Breve => '˘',
            SpacingModifierLetters::DotAbove => '˙',
            SpacingModifierLetters::RingAbove => '˚',
            SpacingModifierLetters::Ogonek => '˛',
            SpacingModifierLetters::SmallTilde => '˜',
            SpacingModifierLetters::DoubleAcuteAccent => '˝',
            SpacingModifierLetters::ModifierLetterRhoticHook => '˞',
            SpacingModifierLetters::ModifierLetterCrossAccent => '˟',
            SpacingModifierLetters::ModifierLetterSmallGamma => 'ˠ',
            SpacingModifierLetters::ModifierLetterSmallL => 'ˡ',
            SpacingModifierLetters::ModifierLetterSmallS => 'ˢ',
            SpacingModifierLetters::ModifierLetterSmallX => 'ˣ',
            SpacingModifierLetters::ModifierLetterSmallReversedGlottalStop => 'ˤ',
            SpacingModifierLetters::ModifierLetterExtraDashHighToneBar => '˥',
            SpacingModifierLetters::ModifierLetterHighToneBar => '˦',
            SpacingModifierLetters::ModifierLetterMidToneBar => '˧',
            SpacingModifierLetters::ModifierLetterLowToneBar => '˨',
            SpacingModifierLetters::ModifierLetterExtraDashLowToneBar => '˩',
            SpacingModifierLetters::ModifierLetterYinDepartingToneMark => '˪',
            SpacingModifierLetters::ModifierLetterYangDepartingToneMark => '˫',
            SpacingModifierLetters::ModifierLetterVoicing => 'ˬ',
            SpacingModifierLetters::ModifierLetterUnaspirated => '˭',
            SpacingModifierLetters::ModifierLetterDoubleApostrophe => 'ˮ',
            SpacingModifierLetters::ModifierLetterLowDownArrowhead => '˯',
            SpacingModifierLetters::ModifierLetterLowUpArrowhead => '˰',
            SpacingModifierLetters::ModifierLetterLowLeftArrowhead => '˱',
            SpacingModifierLetters::ModifierLetterLowRightArrowhead => '˲',
            SpacingModifierLetters::ModifierLetterLowRing => '˳',
            SpacingModifierLetters::ModifierLetterMiddleGraveAccent => '˴',
            SpacingModifierLetters::ModifierLetterMiddleDoubleGraveAccent => '˵',
            SpacingModifierLetters::ModifierLetterMiddleDoubleAcuteAccent => '˶',
            SpacingModifierLetters::ModifierLetterLowTilde => '˷',
            SpacingModifierLetters::ModifierLetterRaisedColon => '˸',
            SpacingModifierLetters::ModifierLetterBeginHighTone => '˹',
            SpacingModifierLetters::ModifierLetterEndHighTone => '˺',
            SpacingModifierLetters::ModifierLetterBeginLowTone => '˻',
            SpacingModifierLetters::ModifierLetterEndLowTone => '˼',
            SpacingModifierLetters::ModifierLetterShelf => '˽',
            SpacingModifierLetters::ModifierLetterOpenShelf => '˾',
        }
    }
}

impl std::convert::TryFrom<char> for SpacingModifierLetters {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ʰ' => Ok(SpacingModifierLetters::ModifierLetterSmallH),
            'ʱ' => Ok(SpacingModifierLetters::ModifierLetterSmallHWithHook),
            'ʲ' => Ok(SpacingModifierLetters::ModifierLetterSmallJ),
            'ʳ' => Ok(SpacingModifierLetters::ModifierLetterSmallR),
            'ʴ' => Ok(SpacingModifierLetters::ModifierLetterSmallTurnedR),
            'ʵ' => Ok(SpacingModifierLetters::ModifierLetterSmallTurnedRWithHook),
            'ʶ' => Ok(SpacingModifierLetters::ModifierLetterSmallCapitalInvertedR),
            'ʷ' => Ok(SpacingModifierLetters::ModifierLetterSmallW),
            'ʸ' => Ok(SpacingModifierLetters::ModifierLetterSmallY),
            'ʹ' => Ok(SpacingModifierLetters::ModifierLetterPrime),
            'ʺ' => Ok(SpacingModifierLetters::ModifierLetterDoublePrime),
            'ʻ' => Ok(SpacingModifierLetters::ModifierLetterTurnedComma),
            'ʼ' => Ok(SpacingModifierLetters::ModifierLetterApostrophe),
            'ʽ' => Ok(SpacingModifierLetters::ModifierLetterReversedComma),
            'ʾ' => Ok(SpacingModifierLetters::ModifierLetterRightHalfRing),
            'ʿ' => Ok(SpacingModifierLetters::ModifierLetterLeftHalfRing),
            'ˀ' => Ok(SpacingModifierLetters::ModifierLetterGlottalStop),
            'ˁ' => Ok(SpacingModifierLetters::ModifierLetterReversedGlottalStop),
            '˂' => Ok(SpacingModifierLetters::ModifierLetterLeftArrowhead),
            '˃' => Ok(SpacingModifierLetters::ModifierLetterRightArrowhead),
            '˄' => Ok(SpacingModifierLetters::ModifierLetterUpArrowhead),
            '˅' => Ok(SpacingModifierLetters::ModifierLetterDownArrowhead),
            'ˆ' => Ok(SpacingModifierLetters::ModifierLetterCircumflexAccent),
            'ˇ' => Ok(SpacingModifierLetters::Caron),
            'ˈ' => Ok(SpacingModifierLetters::ModifierLetterVerticalLine),
            'ˉ' => Ok(SpacingModifierLetters::ModifierLetterMacron),
            'ˊ' => Ok(SpacingModifierLetters::ModifierLetterAcuteAccent),
            'ˋ' => Ok(SpacingModifierLetters::ModifierLetterGraveAccent),
            'ˌ' => Ok(SpacingModifierLetters::ModifierLetterLowVerticalLine),
            'ˍ' => Ok(SpacingModifierLetters::ModifierLetterLowMacron),
            'ˎ' => Ok(SpacingModifierLetters::ModifierLetterLowGraveAccent),
            'ˏ' => Ok(SpacingModifierLetters::ModifierLetterLowAcuteAccent),
            'ː' => Ok(SpacingModifierLetters::ModifierLetterTriangularColon),
            'ˑ' => Ok(SpacingModifierLetters::ModifierLetterHalfTriangularColon),
            '˒' => Ok(SpacingModifierLetters::ModifierLetterCentredRightHalfRing),
            '˓' => Ok(SpacingModifierLetters::ModifierLetterCentredLeftHalfRing),
            '˔' => Ok(SpacingModifierLetters::ModifierLetterUpTack),
            '˕' => Ok(SpacingModifierLetters::ModifierLetterDownTack),
            '˖' => Ok(SpacingModifierLetters::ModifierLetterPlusSign),
            '˗' => Ok(SpacingModifierLetters::ModifierLetterMinusSign),
            '˘' => Ok(SpacingModifierLetters::Breve),
            '˙' => Ok(SpacingModifierLetters::DotAbove),
            '˚' => Ok(SpacingModifierLetters::RingAbove),
            '˛' => Ok(SpacingModifierLetters::Ogonek),
            '˜' => Ok(SpacingModifierLetters::SmallTilde),
            '˝' => Ok(SpacingModifierLetters::DoubleAcuteAccent),
            '˞' => Ok(SpacingModifierLetters::ModifierLetterRhoticHook),
            '˟' => Ok(SpacingModifierLetters::ModifierLetterCrossAccent),
            'ˠ' => Ok(SpacingModifierLetters::ModifierLetterSmallGamma),
            'ˡ' => Ok(SpacingModifierLetters::ModifierLetterSmallL),
            'ˢ' => Ok(SpacingModifierLetters::ModifierLetterSmallS),
            'ˣ' => Ok(SpacingModifierLetters::ModifierLetterSmallX),
            'ˤ' => Ok(SpacingModifierLetters::ModifierLetterSmallReversedGlottalStop),
            '˥' => Ok(SpacingModifierLetters::ModifierLetterExtraDashHighToneBar),
            '˦' => Ok(SpacingModifierLetters::ModifierLetterHighToneBar),
            '˧' => Ok(SpacingModifierLetters::ModifierLetterMidToneBar),
            '˨' => Ok(SpacingModifierLetters::ModifierLetterLowToneBar),
            '˩' => Ok(SpacingModifierLetters::ModifierLetterExtraDashLowToneBar),
            '˪' => Ok(SpacingModifierLetters::ModifierLetterYinDepartingToneMark),
            '˫' => Ok(SpacingModifierLetters::ModifierLetterYangDepartingToneMark),
            'ˬ' => Ok(SpacingModifierLetters::ModifierLetterVoicing),
            '˭' => Ok(SpacingModifierLetters::ModifierLetterUnaspirated),
            'ˮ' => Ok(SpacingModifierLetters::ModifierLetterDoubleApostrophe),
            '˯' => Ok(SpacingModifierLetters::ModifierLetterLowDownArrowhead),
            '˰' => Ok(SpacingModifierLetters::ModifierLetterLowUpArrowhead),
            '˱' => Ok(SpacingModifierLetters::ModifierLetterLowLeftArrowhead),
            '˲' => Ok(SpacingModifierLetters::ModifierLetterLowRightArrowhead),
            '˳' => Ok(SpacingModifierLetters::ModifierLetterLowRing),
            '˴' => Ok(SpacingModifierLetters::ModifierLetterMiddleGraveAccent),
            '˵' => Ok(SpacingModifierLetters::ModifierLetterMiddleDoubleGraveAccent),
            '˶' => Ok(SpacingModifierLetters::ModifierLetterMiddleDoubleAcuteAccent),
            '˷' => Ok(SpacingModifierLetters::ModifierLetterLowTilde),
            '˸' => Ok(SpacingModifierLetters::ModifierLetterRaisedColon),
            '˹' => Ok(SpacingModifierLetters::ModifierLetterBeginHighTone),
            '˺' => Ok(SpacingModifierLetters::ModifierLetterEndHighTone),
            '˻' => Ok(SpacingModifierLetters::ModifierLetterBeginLowTone),
            '˼' => Ok(SpacingModifierLetters::ModifierLetterEndLowTone),
            '˽' => Ok(SpacingModifierLetters::ModifierLetterShelf),
            '˾' => Ok(SpacingModifierLetters::ModifierLetterOpenShelf),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SpacingModifierLetters {
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

impl std::convert::TryFrom<u32> for SpacingModifierLetters {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SpacingModifierLetters {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SpacingModifierLetters {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SpacingModifierLetters::ModifierLetterSmallH
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SpacingModifierLetters{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
