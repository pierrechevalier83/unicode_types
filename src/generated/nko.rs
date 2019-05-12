
/// An enum to represent all characters in the NKo block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NKo {
    /// \u{7c0}: '߀'
    NkoDigitZero,
    /// \u{7c1}: '߁'
    NkoDigitOne,
    /// \u{7c2}: '߂'
    NkoDigitTwo,
    /// \u{7c3}: '߃'
    NkoDigitThree,
    /// \u{7c4}: '߄'
    NkoDigitFour,
    /// \u{7c5}: '߅'
    NkoDigitFive,
    /// \u{7c6}: '߆'
    NkoDigitSix,
    /// \u{7c7}: '߇'
    NkoDigitSeven,
    /// \u{7c8}: '߈'
    NkoDigitEight,
    /// \u{7c9}: '߉'
    NkoDigitNine,
    /// \u{7ca}: 'ߊ'
    NkoLetterA,
    /// \u{7cb}: 'ߋ'
    NkoLetterEe,
    /// \u{7cc}: 'ߌ'
    NkoLetterI,
    /// \u{7cd}: 'ߍ'
    NkoLetterE,
    /// \u{7ce}: 'ߎ'
    NkoLetterU,
    /// \u{7cf}: 'ߏ'
    NkoLetterOo,
    /// \u{7d0}: 'ߐ'
    NkoLetterO,
    /// \u{7d1}: 'ߑ'
    NkoLetterDagbasinna,
    /// \u{7d2}: 'ߒ'
    NkoLetterN,
    /// \u{7d3}: 'ߓ'
    NkoLetterBa,
    /// \u{7d4}: 'ߔ'
    NkoLetterPa,
    /// \u{7d5}: 'ߕ'
    NkoLetterTa,
    /// \u{7d6}: 'ߖ'
    NkoLetterJa,
    /// \u{7d7}: 'ߗ'
    NkoLetterCha,
    /// \u{7d8}: 'ߘ'
    NkoLetterDa,
    /// \u{7d9}: 'ߙ'
    NkoLetterRa,
    /// \u{7da}: 'ߚ'
    NkoLetterRra,
    /// \u{7db}: 'ߛ'
    NkoLetterSa,
    /// \u{7dc}: 'ߜ'
    NkoLetterGba,
    /// \u{7dd}: 'ߝ'
    NkoLetterFa,
    /// \u{7de}: 'ߞ'
    NkoLetterKa,
    /// \u{7df}: 'ߟ'
    NkoLetterLa,
    /// \u{7e0}: 'ߠ'
    NkoLetterNaWoloso,
    /// \u{7e1}: 'ߡ'
    NkoLetterMa,
    /// \u{7e2}: 'ߢ'
    NkoLetterNya,
    /// \u{7e3}: 'ߣ'
    NkoLetterNa,
    /// \u{7e4}: 'ߤ'
    NkoLetterHa,
    /// \u{7e5}: 'ߥ'
    NkoLetterWa,
    /// \u{7e6}: 'ߦ'
    NkoLetterYa,
    /// \u{7e7}: 'ߧ'
    NkoLetterNyaWoloso,
    /// \u{7e8}: 'ߨ'
    NkoLetterJonaJa,
    /// \u{7e9}: 'ߩ'
    NkoLetterJonaCha,
    /// \u{7ea}: 'ߪ'
    NkoLetterJonaRa,
    /// \u{7eb}: '߫'
    NkoCombiningShortHighTone,
    /// \u{7ec}: '߬'
    NkoCombiningShortLowTone,
    /// \u{7ed}: '߭'
    NkoCombiningShortRisingTone,
    /// \u{7ee}: '߮'
    NkoCombiningLongDescendingTone,
    /// \u{7ef}: '߯'
    NkoCombiningLongHighTone,
    /// \u{7f0}: '߰'
    NkoCombiningLongLowTone,
    /// \u{7f1}: '߱'
    NkoCombiningLongRisingTone,
    /// \u{7f2}: '߲'
    NkoCombiningNasalizationMark,
    /// \u{7f3}: '߳'
    NkoCombiningDoubleDotAbove,
    /// \u{7f4}: 'ߴ'
    NkoHighToneApostrophe,
    /// \u{7f5}: 'ߵ'
    NkoLowToneApostrophe,
    /// \u{7f6}: '߶'
    NkoSymbolOoDennen,
    /// \u{7f7}: '߷'
    NkoSymbolGbakurunen,
    /// \u{7f8}: '߸'
    NkoComma,
    /// \u{7f9}: '߹'
    NkoExclamationMark,
    /// \u{7fa}: 'ߺ'
    NkoLajanyalan,
    /// \u{7fd}: '߽'
    NkoDantayalan,
    /// \u{7fe}: '߾'
    NkoDoromeSign,
}

impl Into<char> for NKo {
    fn into(self) -> char {
        match self {
            NKo::NkoDigitZero => '߀',
            NKo::NkoDigitOne => '߁',
            NKo::NkoDigitTwo => '߂',
            NKo::NkoDigitThree => '߃',
            NKo::NkoDigitFour => '߄',
            NKo::NkoDigitFive => '߅',
            NKo::NkoDigitSix => '߆',
            NKo::NkoDigitSeven => '߇',
            NKo::NkoDigitEight => '߈',
            NKo::NkoDigitNine => '߉',
            NKo::NkoLetterA => 'ߊ',
            NKo::NkoLetterEe => 'ߋ',
            NKo::NkoLetterI => 'ߌ',
            NKo::NkoLetterE => 'ߍ',
            NKo::NkoLetterU => 'ߎ',
            NKo::NkoLetterOo => 'ߏ',
            NKo::NkoLetterO => 'ߐ',
            NKo::NkoLetterDagbasinna => 'ߑ',
            NKo::NkoLetterN => 'ߒ',
            NKo::NkoLetterBa => 'ߓ',
            NKo::NkoLetterPa => 'ߔ',
            NKo::NkoLetterTa => 'ߕ',
            NKo::NkoLetterJa => 'ߖ',
            NKo::NkoLetterCha => 'ߗ',
            NKo::NkoLetterDa => 'ߘ',
            NKo::NkoLetterRa => 'ߙ',
            NKo::NkoLetterRra => 'ߚ',
            NKo::NkoLetterSa => 'ߛ',
            NKo::NkoLetterGba => 'ߜ',
            NKo::NkoLetterFa => 'ߝ',
            NKo::NkoLetterKa => 'ߞ',
            NKo::NkoLetterLa => 'ߟ',
            NKo::NkoLetterNaWoloso => 'ߠ',
            NKo::NkoLetterMa => 'ߡ',
            NKo::NkoLetterNya => 'ߢ',
            NKo::NkoLetterNa => 'ߣ',
            NKo::NkoLetterHa => 'ߤ',
            NKo::NkoLetterWa => 'ߥ',
            NKo::NkoLetterYa => 'ߦ',
            NKo::NkoLetterNyaWoloso => 'ߧ',
            NKo::NkoLetterJonaJa => 'ߨ',
            NKo::NkoLetterJonaCha => 'ߩ',
            NKo::NkoLetterJonaRa => 'ߪ',
            NKo::NkoCombiningShortHighTone => '߫',
            NKo::NkoCombiningShortLowTone => '߬',
            NKo::NkoCombiningShortRisingTone => '߭',
            NKo::NkoCombiningLongDescendingTone => '߮',
            NKo::NkoCombiningLongHighTone => '߯',
            NKo::NkoCombiningLongLowTone => '߰',
            NKo::NkoCombiningLongRisingTone => '߱',
            NKo::NkoCombiningNasalizationMark => '߲',
            NKo::NkoCombiningDoubleDotAbove => '߳',
            NKo::NkoHighToneApostrophe => 'ߴ',
            NKo::NkoLowToneApostrophe => 'ߵ',
            NKo::NkoSymbolOoDennen => '߶',
            NKo::NkoSymbolGbakurunen => '߷',
            NKo::NkoComma => '߸',
            NKo::NkoExclamationMark => '߹',
            NKo::NkoLajanyalan => 'ߺ',
            NKo::NkoDantayalan => '߽',
            NKo::NkoDoromeSign => '߾',
        }
    }
}

impl std::convert::TryFrom<char> for NKo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '߀' => Ok(NKo::NkoDigitZero),
            '߁' => Ok(NKo::NkoDigitOne),
            '߂' => Ok(NKo::NkoDigitTwo),
            '߃' => Ok(NKo::NkoDigitThree),
            '߄' => Ok(NKo::NkoDigitFour),
            '߅' => Ok(NKo::NkoDigitFive),
            '߆' => Ok(NKo::NkoDigitSix),
            '߇' => Ok(NKo::NkoDigitSeven),
            '߈' => Ok(NKo::NkoDigitEight),
            '߉' => Ok(NKo::NkoDigitNine),
            'ߊ' => Ok(NKo::NkoLetterA),
            'ߋ' => Ok(NKo::NkoLetterEe),
            'ߌ' => Ok(NKo::NkoLetterI),
            'ߍ' => Ok(NKo::NkoLetterE),
            'ߎ' => Ok(NKo::NkoLetterU),
            'ߏ' => Ok(NKo::NkoLetterOo),
            'ߐ' => Ok(NKo::NkoLetterO),
            'ߑ' => Ok(NKo::NkoLetterDagbasinna),
            'ߒ' => Ok(NKo::NkoLetterN),
            'ߓ' => Ok(NKo::NkoLetterBa),
            'ߔ' => Ok(NKo::NkoLetterPa),
            'ߕ' => Ok(NKo::NkoLetterTa),
            'ߖ' => Ok(NKo::NkoLetterJa),
            'ߗ' => Ok(NKo::NkoLetterCha),
            'ߘ' => Ok(NKo::NkoLetterDa),
            'ߙ' => Ok(NKo::NkoLetterRa),
            'ߚ' => Ok(NKo::NkoLetterRra),
            'ߛ' => Ok(NKo::NkoLetterSa),
            'ߜ' => Ok(NKo::NkoLetterGba),
            'ߝ' => Ok(NKo::NkoLetterFa),
            'ߞ' => Ok(NKo::NkoLetterKa),
            'ߟ' => Ok(NKo::NkoLetterLa),
            'ߠ' => Ok(NKo::NkoLetterNaWoloso),
            'ߡ' => Ok(NKo::NkoLetterMa),
            'ߢ' => Ok(NKo::NkoLetterNya),
            'ߣ' => Ok(NKo::NkoLetterNa),
            'ߤ' => Ok(NKo::NkoLetterHa),
            'ߥ' => Ok(NKo::NkoLetterWa),
            'ߦ' => Ok(NKo::NkoLetterYa),
            'ߧ' => Ok(NKo::NkoLetterNyaWoloso),
            'ߨ' => Ok(NKo::NkoLetterJonaJa),
            'ߩ' => Ok(NKo::NkoLetterJonaCha),
            'ߪ' => Ok(NKo::NkoLetterJonaRa),
            '߫' => Ok(NKo::NkoCombiningShortHighTone),
            '߬' => Ok(NKo::NkoCombiningShortLowTone),
            '߭' => Ok(NKo::NkoCombiningShortRisingTone),
            '߮' => Ok(NKo::NkoCombiningLongDescendingTone),
            '߯' => Ok(NKo::NkoCombiningLongHighTone),
            '߰' => Ok(NKo::NkoCombiningLongLowTone),
            '߱' => Ok(NKo::NkoCombiningLongRisingTone),
            '߲' => Ok(NKo::NkoCombiningNasalizationMark),
            '߳' => Ok(NKo::NkoCombiningDoubleDotAbove),
            'ߴ' => Ok(NKo::NkoHighToneApostrophe),
            'ߵ' => Ok(NKo::NkoLowToneApostrophe),
            '߶' => Ok(NKo::NkoSymbolOoDennen),
            '߷' => Ok(NKo::NkoSymbolGbakurunen),
            '߸' => Ok(NKo::NkoComma),
            '߹' => Ok(NKo::NkoExclamationMark),
            'ߺ' => Ok(NKo::NkoLajanyalan),
            '߽' => Ok(NKo::NkoDantayalan),
            '߾' => Ok(NKo::NkoDoromeSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for NKo {
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

impl std::convert::TryFrom<u32> for NKo {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for NKo {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl NKo {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        NKo::NkoDigitZero
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("NKo{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
