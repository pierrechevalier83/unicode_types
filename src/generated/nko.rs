/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{7c0}: '߀'
    pub const DIGIT_ZERO: char = '߀';
    /// \u{7c1}: '߁'
    pub const DIGIT_ONE: char = '߁';
    /// \u{7c2}: '߂'
    pub const DIGIT_TWO: char = '߂';
    /// \u{7c3}: '߃'
    pub const DIGIT_THREE: char = '߃';
    /// \u{7c4}: '߄'
    pub const DIGIT_FOUR: char = '߄';
    /// \u{7c5}: '߅'
    pub const DIGIT_FIVE: char = '߅';
    /// \u{7c6}: '߆'
    pub const DIGIT_SIX: char = '߆';
    /// \u{7c7}: '߇'
    pub const DIGIT_SEVEN: char = '߇';
    /// \u{7c8}: '߈'
    pub const DIGIT_EIGHT: char = '߈';
    /// \u{7c9}: '߉'
    pub const DIGIT_NINE: char = '߉';
    /// \u{7ca}: 'ߊ'
    pub const LETTER_A: char = 'ߊ';
    /// \u{7cb}: 'ߋ'
    pub const LETTER_EE: char = 'ߋ';
    /// \u{7cc}: 'ߌ'
    pub const LETTER_I: char = 'ߌ';
    /// \u{7cd}: 'ߍ'
    pub const LETTER_E: char = 'ߍ';
    /// \u{7ce}: 'ߎ'
    pub const LETTER_U: char = 'ߎ';
    /// \u{7cf}: 'ߏ'
    pub const LETTER_OO: char = 'ߏ';
    /// \u{7d0}: 'ߐ'
    pub const LETTER_O: char = 'ߐ';
    /// \u{7d1}: 'ߑ'
    pub const LETTER_DAGBASINNA: char = 'ߑ';
    /// \u{7d2}: 'ߒ'
    pub const LETTER_N: char = 'ߒ';
    /// \u{7d3}: 'ߓ'
    pub const LETTER_BA: char = 'ߓ';
    /// \u{7d4}: 'ߔ'
    pub const LETTER_PA: char = 'ߔ';
    /// \u{7d5}: 'ߕ'
    pub const LETTER_TA: char = 'ߕ';
    /// \u{7d6}: 'ߖ'
    pub const LETTER_JA: char = 'ߖ';
    /// \u{7d7}: 'ߗ'
    pub const LETTER_CHA: char = 'ߗ';
    /// \u{7d8}: 'ߘ'
    pub const LETTER_DA: char = 'ߘ';
    /// \u{7d9}: 'ߙ'
    pub const LETTER_RA: char = 'ߙ';
    /// \u{7da}: 'ߚ'
    pub const LETTER_RRA: char = 'ߚ';
    /// \u{7db}: 'ߛ'
    pub const LETTER_SA: char = 'ߛ';
    /// \u{7dc}: 'ߜ'
    pub const LETTER_GBA: char = 'ߜ';
    /// \u{7dd}: 'ߝ'
    pub const LETTER_FA: char = 'ߝ';
    /// \u{7de}: 'ߞ'
    pub const LETTER_KA: char = 'ߞ';
    /// \u{7df}: 'ߟ'
    pub const LETTER_LA: char = 'ߟ';
    /// \u{7e0}: 'ߠ'
    pub const LETTER_NA_WOLOSO: char = 'ߠ';
    /// \u{7e1}: 'ߡ'
    pub const LETTER_MA: char = 'ߡ';
    /// \u{7e2}: 'ߢ'
    pub const LETTER_NYA: char = 'ߢ';
    /// \u{7e3}: 'ߣ'
    pub const LETTER_NA: char = 'ߣ';
    /// \u{7e4}: 'ߤ'
    pub const LETTER_HA: char = 'ߤ';
    /// \u{7e5}: 'ߥ'
    pub const LETTER_WA: char = 'ߥ';
    /// \u{7e6}: 'ߦ'
    pub const LETTER_YA: char = 'ߦ';
    /// \u{7e7}: 'ߧ'
    pub const LETTER_NYA_WOLOSO: char = 'ߧ';
    /// \u{7e8}: 'ߨ'
    pub const LETTER_JONA_JA: char = 'ߨ';
    /// \u{7e9}: 'ߩ'
    pub const LETTER_JONA_CHA: char = 'ߩ';
    /// \u{7ea}: 'ߪ'
    pub const LETTER_JONA_RA: char = 'ߪ';
    /// \u{7eb}: '߫'
    pub const COMBINING_SHORT_HIGH_TONE: char = '߫';
    /// \u{7ec}: '߬'
    pub const COMBINING_SHORT_LOW_TONE: char = '߬';
    /// \u{7ed}: '߭'
    pub const COMBINING_SHORT_RISING_TONE: char = '߭';
    /// \u{7ee}: '߮'
    pub const COMBINING_LONG_DESCENDING_TONE: char = '߮';
    /// \u{7ef}: '߯'
    pub const COMBINING_LONG_HIGH_TONE: char = '߯';
    /// \u{7f0}: '߰'
    pub const COMBINING_LONG_LOW_TONE: char = '߰';
    /// \u{7f1}: '߱'
    pub const COMBINING_LONG_RISING_TONE: char = '߱';
    /// \u{7f2}: '߲'
    pub const COMBINING_NASALIZATION_MARK: char = '߲';
    /// \u{7f3}: '߳'
    pub const COMBINING_DOUBLE_DOT_ABOVE: char = '߳';
    /// \u{7f4}: 'ߴ'
    pub const HIGH_TONE_APOSTROPHE: char = 'ߴ';
    /// \u{7f5}: 'ߵ'
    pub const LOW_TONE_APOSTROPHE: char = 'ߵ';
    /// \u{7f6}: '߶'
    pub const SYMBOL_OO_DENNEN: char = '߶';
    /// \u{7f7}: '߷'
    pub const SYMBOL_GBAKURUNEN: char = '߷';
    /// \u{7f8}: '߸'
    pub const COMMA: char = '߸';
    /// \u{7f9}: '߹'
    pub const EXCLAMATION_MARK: char = '߹';
    /// \u{7fa}: 'ߺ'
    pub const LAJANYALAN: char = 'ߺ';
    /// \u{7fd}: '߽'
    pub const DANTAYALAN: char = '߽';
    /// \u{7fe}: '߾'
    pub const DOROME_SIGN: char = '߾';
}

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
        use constants::*;
        match self {
            NKo::NkoDigitZero => DIGIT_ZERO,
            NKo::NkoDigitOne => DIGIT_ONE,
            NKo::NkoDigitTwo => DIGIT_TWO,
            NKo::NkoDigitThree => DIGIT_THREE,
            NKo::NkoDigitFour => DIGIT_FOUR,
            NKo::NkoDigitFive => DIGIT_FIVE,
            NKo::NkoDigitSix => DIGIT_SIX,
            NKo::NkoDigitSeven => DIGIT_SEVEN,
            NKo::NkoDigitEight => DIGIT_EIGHT,
            NKo::NkoDigitNine => DIGIT_NINE,
            NKo::NkoLetterA => LETTER_A,
            NKo::NkoLetterEe => LETTER_EE,
            NKo::NkoLetterI => LETTER_I,
            NKo::NkoLetterE => LETTER_E,
            NKo::NkoLetterU => LETTER_U,
            NKo::NkoLetterOo => LETTER_OO,
            NKo::NkoLetterO => LETTER_O,
            NKo::NkoLetterDagbasinna => LETTER_DAGBASINNA,
            NKo::NkoLetterN => LETTER_N,
            NKo::NkoLetterBa => LETTER_BA,
            NKo::NkoLetterPa => LETTER_PA,
            NKo::NkoLetterTa => LETTER_TA,
            NKo::NkoLetterJa => LETTER_JA,
            NKo::NkoLetterCha => LETTER_CHA,
            NKo::NkoLetterDa => LETTER_DA,
            NKo::NkoLetterRa => LETTER_RA,
            NKo::NkoLetterRra => LETTER_RRA,
            NKo::NkoLetterSa => LETTER_SA,
            NKo::NkoLetterGba => LETTER_GBA,
            NKo::NkoLetterFa => LETTER_FA,
            NKo::NkoLetterKa => LETTER_KA,
            NKo::NkoLetterLa => LETTER_LA,
            NKo::NkoLetterNaWoloso => LETTER_NA_WOLOSO,
            NKo::NkoLetterMa => LETTER_MA,
            NKo::NkoLetterNya => LETTER_NYA,
            NKo::NkoLetterNa => LETTER_NA,
            NKo::NkoLetterHa => LETTER_HA,
            NKo::NkoLetterWa => LETTER_WA,
            NKo::NkoLetterYa => LETTER_YA,
            NKo::NkoLetterNyaWoloso => LETTER_NYA_WOLOSO,
            NKo::NkoLetterJonaJa => LETTER_JONA_JA,
            NKo::NkoLetterJonaCha => LETTER_JONA_CHA,
            NKo::NkoLetterJonaRa => LETTER_JONA_RA,
            NKo::NkoCombiningShortHighTone => COMBINING_SHORT_HIGH_TONE,
            NKo::NkoCombiningShortLowTone => COMBINING_SHORT_LOW_TONE,
            NKo::NkoCombiningShortRisingTone => COMBINING_SHORT_RISING_TONE,
            NKo::NkoCombiningLongDescendingTone => COMBINING_LONG_DESCENDING_TONE,
            NKo::NkoCombiningLongHighTone => COMBINING_LONG_HIGH_TONE,
            NKo::NkoCombiningLongLowTone => COMBINING_LONG_LOW_TONE,
            NKo::NkoCombiningLongRisingTone => COMBINING_LONG_RISING_TONE,
            NKo::NkoCombiningNasalizationMark => COMBINING_NASALIZATION_MARK,
            NKo::NkoCombiningDoubleDotAbove => COMBINING_DOUBLE_DOT_ABOVE,
            NKo::NkoHighToneApostrophe => HIGH_TONE_APOSTROPHE,
            NKo::NkoLowToneApostrophe => LOW_TONE_APOSTROPHE,
            NKo::NkoSymbolOoDennen => SYMBOL_OO_DENNEN,
            NKo::NkoSymbolGbakurunen => SYMBOL_GBAKURUNEN,
            NKo::NkoComma => COMMA,
            NKo::NkoExclamationMark => EXCLAMATION_MARK,
            NKo::NkoLajanyalan => LAJANYALAN,
            NKo::NkoDantayalan => DANTAYALAN,
            NKo::NkoDoromeSign => DOROME_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for NKo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            DIGIT_ZERO => Ok(NKo::NkoDigitZero),
            DIGIT_ONE => Ok(NKo::NkoDigitOne),
            DIGIT_TWO => Ok(NKo::NkoDigitTwo),
            DIGIT_THREE => Ok(NKo::NkoDigitThree),
            DIGIT_FOUR => Ok(NKo::NkoDigitFour),
            DIGIT_FIVE => Ok(NKo::NkoDigitFive),
            DIGIT_SIX => Ok(NKo::NkoDigitSix),
            DIGIT_SEVEN => Ok(NKo::NkoDigitSeven),
            DIGIT_EIGHT => Ok(NKo::NkoDigitEight),
            DIGIT_NINE => Ok(NKo::NkoDigitNine),
            LETTER_A => Ok(NKo::NkoLetterA),
            LETTER_EE => Ok(NKo::NkoLetterEe),
            LETTER_I => Ok(NKo::NkoLetterI),
            LETTER_E => Ok(NKo::NkoLetterE),
            LETTER_U => Ok(NKo::NkoLetterU),
            LETTER_OO => Ok(NKo::NkoLetterOo),
            LETTER_O => Ok(NKo::NkoLetterO),
            LETTER_DAGBASINNA => Ok(NKo::NkoLetterDagbasinna),
            LETTER_N => Ok(NKo::NkoLetterN),
            LETTER_BA => Ok(NKo::NkoLetterBa),
            LETTER_PA => Ok(NKo::NkoLetterPa),
            LETTER_TA => Ok(NKo::NkoLetterTa),
            LETTER_JA => Ok(NKo::NkoLetterJa),
            LETTER_CHA => Ok(NKo::NkoLetterCha),
            LETTER_DA => Ok(NKo::NkoLetterDa),
            LETTER_RA => Ok(NKo::NkoLetterRa),
            LETTER_RRA => Ok(NKo::NkoLetterRra),
            LETTER_SA => Ok(NKo::NkoLetterSa),
            LETTER_GBA => Ok(NKo::NkoLetterGba),
            LETTER_FA => Ok(NKo::NkoLetterFa),
            LETTER_KA => Ok(NKo::NkoLetterKa),
            LETTER_LA => Ok(NKo::NkoLetterLa),
            LETTER_NA_WOLOSO => Ok(NKo::NkoLetterNaWoloso),
            LETTER_MA => Ok(NKo::NkoLetterMa),
            LETTER_NYA => Ok(NKo::NkoLetterNya),
            LETTER_NA => Ok(NKo::NkoLetterNa),
            LETTER_HA => Ok(NKo::NkoLetterHa),
            LETTER_WA => Ok(NKo::NkoLetterWa),
            LETTER_YA => Ok(NKo::NkoLetterYa),
            LETTER_NYA_WOLOSO => Ok(NKo::NkoLetterNyaWoloso),
            LETTER_JONA_JA => Ok(NKo::NkoLetterJonaJa),
            LETTER_JONA_CHA => Ok(NKo::NkoLetterJonaCha),
            LETTER_JONA_RA => Ok(NKo::NkoLetterJonaRa),
            COMBINING_SHORT_HIGH_TONE => Ok(NKo::NkoCombiningShortHighTone),
            COMBINING_SHORT_LOW_TONE => Ok(NKo::NkoCombiningShortLowTone),
            COMBINING_SHORT_RISING_TONE => Ok(NKo::NkoCombiningShortRisingTone),
            COMBINING_LONG_DESCENDING_TONE => Ok(NKo::NkoCombiningLongDescendingTone),
            COMBINING_LONG_HIGH_TONE => Ok(NKo::NkoCombiningLongHighTone),
            COMBINING_LONG_LOW_TONE => Ok(NKo::NkoCombiningLongLowTone),
            COMBINING_LONG_RISING_TONE => Ok(NKo::NkoCombiningLongRisingTone),
            COMBINING_NASALIZATION_MARK => Ok(NKo::NkoCombiningNasalizationMark),
            COMBINING_DOUBLE_DOT_ABOVE => Ok(NKo::NkoCombiningDoubleDotAbove),
            HIGH_TONE_APOSTROPHE => Ok(NKo::NkoHighToneApostrophe),
            LOW_TONE_APOSTROPHE => Ok(NKo::NkoLowToneApostrophe),
            SYMBOL_OO_DENNEN => Ok(NKo::NkoSymbolOoDennen),
            SYMBOL_GBAKURUNEN => Ok(NKo::NkoSymbolGbakurunen),
            COMMA => Ok(NKo::NkoComma),
            EXCLAMATION_MARK => Ok(NKo::NkoExclamationMark),
            LAJANYALAN => Ok(NKo::NkoLajanyalan),
            DANTAYALAN => Ok(NKo::NkoDantayalan),
            DOROME_SIGN => Ok(NKo::NkoDoromeSign),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            NKo::NkoDigitZero => "nko digit zero",
            NKo::NkoDigitOne => "nko digit one",
            NKo::NkoDigitTwo => "nko digit two",
            NKo::NkoDigitThree => "nko digit three",
            NKo::NkoDigitFour => "nko digit four",
            NKo::NkoDigitFive => "nko digit five",
            NKo::NkoDigitSix => "nko digit six",
            NKo::NkoDigitSeven => "nko digit seven",
            NKo::NkoDigitEight => "nko digit eight",
            NKo::NkoDigitNine => "nko digit nine",
            NKo::NkoLetterA => "nko letter a",
            NKo::NkoLetterEe => "nko letter ee",
            NKo::NkoLetterI => "nko letter i",
            NKo::NkoLetterE => "nko letter e",
            NKo::NkoLetterU => "nko letter u",
            NKo::NkoLetterOo => "nko letter oo",
            NKo::NkoLetterO => "nko letter o",
            NKo::NkoLetterDagbasinna => "nko letter dagbasinna",
            NKo::NkoLetterN => "nko letter n",
            NKo::NkoLetterBa => "nko letter ba",
            NKo::NkoLetterPa => "nko letter pa",
            NKo::NkoLetterTa => "nko letter ta",
            NKo::NkoLetterJa => "nko letter ja",
            NKo::NkoLetterCha => "nko letter cha",
            NKo::NkoLetterDa => "nko letter da",
            NKo::NkoLetterRa => "nko letter ra",
            NKo::NkoLetterRra => "nko letter rra",
            NKo::NkoLetterSa => "nko letter sa",
            NKo::NkoLetterGba => "nko letter gba",
            NKo::NkoLetterFa => "nko letter fa",
            NKo::NkoLetterKa => "nko letter ka",
            NKo::NkoLetterLa => "nko letter la",
            NKo::NkoLetterNaWoloso => "nko letter na woloso",
            NKo::NkoLetterMa => "nko letter ma",
            NKo::NkoLetterNya => "nko letter nya",
            NKo::NkoLetterNa => "nko letter na",
            NKo::NkoLetterHa => "nko letter ha",
            NKo::NkoLetterWa => "nko letter wa",
            NKo::NkoLetterYa => "nko letter ya",
            NKo::NkoLetterNyaWoloso => "nko letter nya woloso",
            NKo::NkoLetterJonaJa => "nko letter jona ja",
            NKo::NkoLetterJonaCha => "nko letter jona cha",
            NKo::NkoLetterJonaRa => "nko letter jona ra",
            NKo::NkoCombiningShortHighTone => "nko combining short high tone",
            NKo::NkoCombiningShortLowTone => "nko combining short low tone",
            NKo::NkoCombiningShortRisingTone => "nko combining short rising tone",
            NKo::NkoCombiningLongDescendingTone => "nko combining long descending tone",
            NKo::NkoCombiningLongHighTone => "nko combining long high tone",
            NKo::NkoCombiningLongLowTone => "nko combining long low tone",
            NKo::NkoCombiningLongRisingTone => "nko combining long rising tone",
            NKo::NkoCombiningNasalizationMark => "nko combining nasalization mark",
            NKo::NkoCombiningDoubleDotAbove => "nko combining double dot above",
            NKo::NkoHighToneApostrophe => "nko high tone apostrophe",
            NKo::NkoLowToneApostrophe => "nko low tone apostrophe",
            NKo::NkoSymbolOoDennen => "nko symbol oo dennen",
            NKo::NkoSymbolGbakurunen => "nko symbol gbakurunen",
            NKo::NkoComma => "nko comma",
            NKo::NkoExclamationMark => "nko exclamation mark",
            NKo::NkoLajanyalan => "nko lajanyalan",
            NKo::NkoDantayalan => "nko dantayalan",
            NKo::NkoDoromeSign => "nko dorome sign",
        }
    }
}
