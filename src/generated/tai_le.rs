/// \u{1950} → \u{197f}
///
/// ᥐ ᥑ ᥒ ᥓ ᥔ ᥕ ᥖ ᥗ ᥘ ᥙ ᥚ ᥛ ᥜ ᥝ ᥞ ᥟ\
/// ᥠ ᥡ ᥢ ᥣ ᥤ ᥥ ᥦ ᥧ ᥨ ᥩ ᥪ ᥫ ᥬ ᥭ ᥰ ᥱ\
/// ᥲ ᥳ ᥴ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1950}: 'ᥐ'
    pub const LETTER_KA: char = 'ᥐ';
    /// \u{1951}: 'ᥑ'
    pub const LETTER_XA: char = 'ᥑ';
    /// \u{1952}: 'ᥒ'
    pub const LETTER_NGA: char = 'ᥒ';
    /// \u{1953}: 'ᥓ'
    pub const LETTER_TSA: char = 'ᥓ';
    /// \u{1954}: 'ᥔ'
    pub const LETTER_SA: char = 'ᥔ';
    /// \u{1955}: 'ᥕ'
    pub const LETTER_YA: char = 'ᥕ';
    /// \u{1956}: 'ᥖ'
    pub const LETTER_TA: char = 'ᥖ';
    /// \u{1957}: 'ᥗ'
    pub const LETTER_THA: char = 'ᥗ';
    /// \u{1958}: 'ᥘ'
    pub const LETTER_LA: char = 'ᥘ';
    /// \u{1959}: 'ᥙ'
    pub const LETTER_PA: char = 'ᥙ';
    /// \u{195a}: 'ᥚ'
    pub const LETTER_PHA: char = 'ᥚ';
    /// \u{195b}: 'ᥛ'
    pub const LETTER_MA: char = 'ᥛ';
    /// \u{195c}: 'ᥜ'
    pub const LETTER_FA: char = 'ᥜ';
    /// \u{195d}: 'ᥝ'
    pub const LETTER_VA: char = 'ᥝ';
    /// \u{195e}: 'ᥞ'
    pub const LETTER_HA: char = 'ᥞ';
    /// \u{195f}: 'ᥟ'
    pub const LETTER_QA: char = 'ᥟ';
    /// \u{1960}: 'ᥠ'
    pub const LETTER_KHA: char = 'ᥠ';
    /// \u{1961}: 'ᥡ'
    pub const LETTER_TSHA: char = 'ᥡ';
    /// \u{1962}: 'ᥢ'
    pub const LETTER_NA: char = 'ᥢ';
    /// \u{1963}: 'ᥣ'
    pub const LETTER_A: char = 'ᥣ';
    /// \u{1964}: 'ᥤ'
    pub const LETTER_I: char = 'ᥤ';
    /// \u{1965}: 'ᥥ'
    pub const LETTER_EE: char = 'ᥥ';
    /// \u{1966}: 'ᥦ'
    pub const LETTER_EH: char = 'ᥦ';
    /// \u{1967}: 'ᥧ'
    pub const LETTER_U: char = 'ᥧ';
    /// \u{1968}: 'ᥨ'
    pub const LETTER_OO: char = 'ᥨ';
    /// \u{1969}: 'ᥩ'
    pub const LETTER_O: char = 'ᥩ';
    /// \u{196a}: 'ᥪ'
    pub const LETTER_UE: char = 'ᥪ';
    /// \u{196b}: 'ᥫ'
    pub const LETTER_E: char = 'ᥫ';
    /// \u{196c}: 'ᥬ'
    pub const LETTER_AUE: char = 'ᥬ';
    /// \u{196d}: 'ᥭ'
    pub const LETTER_AI: char = 'ᥭ';
    /// \u{1970}: 'ᥰ'
    pub const LETTER_TONE_DASH_2: char = 'ᥰ';
    /// \u{1971}: 'ᥱ'
    pub const LETTER_TONE_DASH_3: char = 'ᥱ';
    /// \u{1972}: 'ᥲ'
    pub const LETTER_TONE_DASH_4: char = 'ᥲ';
    /// \u{1973}: 'ᥳ'
    pub const LETTER_TONE_DASH_5: char = 'ᥳ';
    /// \u{1974}: 'ᥴ'
    pub const LETTER_TONE_DASH_6: char = 'ᥴ';
}

/// An enum to represent all characters in the TaiLe block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaiLe {
    /// \u{1950}: 'ᥐ'
    LetterKa,
    /// \u{1951}: 'ᥑ'
    LetterXa,
    /// \u{1952}: 'ᥒ'
    LetterNga,
    /// \u{1953}: 'ᥓ'
    LetterTsa,
    /// \u{1954}: 'ᥔ'
    LetterSa,
    /// \u{1955}: 'ᥕ'
    LetterYa,
    /// \u{1956}: 'ᥖ'
    LetterTa,
    /// \u{1957}: 'ᥗ'
    LetterTha,
    /// \u{1958}: 'ᥘ'
    LetterLa,
    /// \u{1959}: 'ᥙ'
    LetterPa,
    /// \u{195a}: 'ᥚ'
    LetterPha,
    /// \u{195b}: 'ᥛ'
    LetterMa,
    /// \u{195c}: 'ᥜ'
    LetterFa,
    /// \u{195d}: 'ᥝ'
    LetterVa,
    /// \u{195e}: 'ᥞ'
    LetterHa,
    /// \u{195f}: 'ᥟ'
    LetterQa,
    /// \u{1960}: 'ᥠ'
    LetterKha,
    /// \u{1961}: 'ᥡ'
    LetterTsha,
    /// \u{1962}: 'ᥢ'
    LetterNa,
    /// \u{1963}: 'ᥣ'
    LetterA,
    /// \u{1964}: 'ᥤ'
    LetterI,
    /// \u{1965}: 'ᥥ'
    LetterEe,
    /// \u{1966}: 'ᥦ'
    LetterEh,
    /// \u{1967}: 'ᥧ'
    LetterU,
    /// \u{1968}: 'ᥨ'
    LetterOo,
    /// \u{1969}: 'ᥩ'
    LetterO,
    /// \u{196a}: 'ᥪ'
    LetterUe,
    /// \u{196b}: 'ᥫ'
    LetterE,
    /// \u{196c}: 'ᥬ'
    LetterAue,
    /// \u{196d}: 'ᥭ'
    LetterAi,
    /// \u{1970}: 'ᥰ'
    LetterToneDash2,
    /// \u{1971}: 'ᥱ'
    LetterToneDash3,
    /// \u{1972}: 'ᥲ'
    LetterToneDash4,
    /// \u{1973}: 'ᥳ'
    LetterToneDash5,
    /// \u{1974}: 'ᥴ'
    LetterToneDash6,
}

impl Into<char> for TaiLe {
    fn into(self) -> char {
        use constants::*;
        match self {
            TaiLe::LetterKa => LETTER_KA,
            TaiLe::LetterXa => LETTER_XA,
            TaiLe::LetterNga => LETTER_NGA,
            TaiLe::LetterTsa => LETTER_TSA,
            TaiLe::LetterSa => LETTER_SA,
            TaiLe::LetterYa => LETTER_YA,
            TaiLe::LetterTa => LETTER_TA,
            TaiLe::LetterTha => LETTER_THA,
            TaiLe::LetterLa => LETTER_LA,
            TaiLe::LetterPa => LETTER_PA,
            TaiLe::LetterPha => LETTER_PHA,
            TaiLe::LetterMa => LETTER_MA,
            TaiLe::LetterFa => LETTER_FA,
            TaiLe::LetterVa => LETTER_VA,
            TaiLe::LetterHa => LETTER_HA,
            TaiLe::LetterQa => LETTER_QA,
            TaiLe::LetterKha => LETTER_KHA,
            TaiLe::LetterTsha => LETTER_TSHA,
            TaiLe::LetterNa => LETTER_NA,
            TaiLe::LetterA => LETTER_A,
            TaiLe::LetterI => LETTER_I,
            TaiLe::LetterEe => LETTER_EE,
            TaiLe::LetterEh => LETTER_EH,
            TaiLe::LetterU => LETTER_U,
            TaiLe::LetterOo => LETTER_OO,
            TaiLe::LetterO => LETTER_O,
            TaiLe::LetterUe => LETTER_UE,
            TaiLe::LetterE => LETTER_E,
            TaiLe::LetterAue => LETTER_AUE,
            TaiLe::LetterAi => LETTER_AI,
            TaiLe::LetterToneDash2 => LETTER_TONE_DASH_2,
            TaiLe::LetterToneDash3 => LETTER_TONE_DASH_3,
            TaiLe::LetterToneDash4 => LETTER_TONE_DASH_4,
            TaiLe::LetterToneDash5 => LETTER_TONE_DASH_5,
            TaiLe::LetterToneDash6 => LETTER_TONE_DASH_6,
        }
    }
}

impl std::convert::TryFrom<char> for TaiLe {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KA => Ok(TaiLe::LetterKa),
            LETTER_XA => Ok(TaiLe::LetterXa),
            LETTER_NGA => Ok(TaiLe::LetterNga),
            LETTER_TSA => Ok(TaiLe::LetterTsa),
            LETTER_SA => Ok(TaiLe::LetterSa),
            LETTER_YA => Ok(TaiLe::LetterYa),
            LETTER_TA => Ok(TaiLe::LetterTa),
            LETTER_THA => Ok(TaiLe::LetterTha),
            LETTER_LA => Ok(TaiLe::LetterLa),
            LETTER_PA => Ok(TaiLe::LetterPa),
            LETTER_PHA => Ok(TaiLe::LetterPha),
            LETTER_MA => Ok(TaiLe::LetterMa),
            LETTER_FA => Ok(TaiLe::LetterFa),
            LETTER_VA => Ok(TaiLe::LetterVa),
            LETTER_HA => Ok(TaiLe::LetterHa),
            LETTER_QA => Ok(TaiLe::LetterQa),
            LETTER_KHA => Ok(TaiLe::LetterKha),
            LETTER_TSHA => Ok(TaiLe::LetterTsha),
            LETTER_NA => Ok(TaiLe::LetterNa),
            LETTER_A => Ok(TaiLe::LetterA),
            LETTER_I => Ok(TaiLe::LetterI),
            LETTER_EE => Ok(TaiLe::LetterEe),
            LETTER_EH => Ok(TaiLe::LetterEh),
            LETTER_U => Ok(TaiLe::LetterU),
            LETTER_OO => Ok(TaiLe::LetterOo),
            LETTER_O => Ok(TaiLe::LetterO),
            LETTER_UE => Ok(TaiLe::LetterUe),
            LETTER_E => Ok(TaiLe::LetterE),
            LETTER_AUE => Ok(TaiLe::LetterAue),
            LETTER_AI => Ok(TaiLe::LetterAi),
            LETTER_TONE_DASH_2 => Ok(TaiLe::LetterToneDash2),
            LETTER_TONE_DASH_3 => Ok(TaiLe::LetterToneDash3),
            LETTER_TONE_DASH_4 => Ok(TaiLe::LetterToneDash4),
            LETTER_TONE_DASH_5 => Ok(TaiLe::LetterToneDash5),
            LETTER_TONE_DASH_6 => Ok(TaiLe::LetterToneDash6),
            _ => Err(()),
        }
    }
}

impl Into<u32> for TaiLe {
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

impl std::convert::TryFrom<u32> for TaiLe {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for TaiLe {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl TaiLe {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        TaiLe::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            TaiLe::LetterKa => "tai le letter ka",
            TaiLe::LetterXa => "tai le letter xa",
            TaiLe::LetterNga => "tai le letter nga",
            TaiLe::LetterTsa => "tai le letter tsa",
            TaiLe::LetterSa => "tai le letter sa",
            TaiLe::LetterYa => "tai le letter ya",
            TaiLe::LetterTa => "tai le letter ta",
            TaiLe::LetterTha => "tai le letter tha",
            TaiLe::LetterLa => "tai le letter la",
            TaiLe::LetterPa => "tai le letter pa",
            TaiLe::LetterPha => "tai le letter pha",
            TaiLe::LetterMa => "tai le letter ma",
            TaiLe::LetterFa => "tai le letter fa",
            TaiLe::LetterVa => "tai le letter va",
            TaiLe::LetterHa => "tai le letter ha",
            TaiLe::LetterQa => "tai le letter qa",
            TaiLe::LetterKha => "tai le letter kha",
            TaiLe::LetterTsha => "tai le letter tsha",
            TaiLe::LetterNa => "tai le letter na",
            TaiLe::LetterA => "tai le letter a",
            TaiLe::LetterI => "tai le letter i",
            TaiLe::LetterEe => "tai le letter ee",
            TaiLe::LetterEh => "tai le letter eh",
            TaiLe::LetterU => "tai le letter u",
            TaiLe::LetterOo => "tai le letter oo",
            TaiLe::LetterO => "tai le letter o",
            TaiLe::LetterUe => "tai le letter ue",
            TaiLe::LetterE => "tai le letter e",
            TaiLe::LetterAue => "tai le letter aue",
            TaiLe::LetterAi => "tai le letter ai",
            TaiLe::LetterToneDash2 => "tai le letter tone-2",
            TaiLe::LetterToneDash3 => "tai le letter tone-3",
            TaiLe::LetterToneDash4 => "tai le letter tone-4",
            TaiLe::LetterToneDash5 => "tai le letter tone-5",
            TaiLe::LetterToneDash6 => "tai le letter tone-6",
        }
    }
}
