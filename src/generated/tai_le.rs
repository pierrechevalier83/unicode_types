/// \u{1950} → \u{197f}\
///\
/// ᥐ ᥑ ᥒ ᥓ ᥔ ᥕ ᥖ ᥗ ᥘ ᥙ ᥚ ᥛ ᥜ ᥝ ᥞ ᥟ
/// ᥠ ᥡ ᥢ ᥣ ᥤ ᥥ ᥦ ᥧ ᥨ ᥩ ᥪ ᥫ ᥬ ᥭ ᥰ ᥱ
/// ᥲ ᥳ ᥴ
pub mod constants {
    /// \u{1950}: 'ᥐ'
    pub const TAI_LE_LETTER_KA: char = 'ᥐ';
    /// \u{1951}: 'ᥑ'
    pub const TAI_LE_LETTER_XA: char = 'ᥑ';
    /// \u{1952}: 'ᥒ'
    pub const TAI_LE_LETTER_NGA: char = 'ᥒ';
    /// \u{1953}: 'ᥓ'
    pub const TAI_LE_LETTER_TSA: char = 'ᥓ';
    /// \u{1954}: 'ᥔ'
    pub const TAI_LE_LETTER_SA: char = 'ᥔ';
    /// \u{1955}: 'ᥕ'
    pub const TAI_LE_LETTER_YA: char = 'ᥕ';
    /// \u{1956}: 'ᥖ'
    pub const TAI_LE_LETTER_TA: char = 'ᥖ';
    /// \u{1957}: 'ᥗ'
    pub const TAI_LE_LETTER_THA: char = 'ᥗ';
    /// \u{1958}: 'ᥘ'
    pub const TAI_LE_LETTER_LA: char = 'ᥘ';
    /// \u{1959}: 'ᥙ'
    pub const TAI_LE_LETTER_PA: char = 'ᥙ';
    /// \u{195a}: 'ᥚ'
    pub const TAI_LE_LETTER_PHA: char = 'ᥚ';
    /// \u{195b}: 'ᥛ'
    pub const TAI_LE_LETTER_MA: char = 'ᥛ';
    /// \u{195c}: 'ᥜ'
    pub const TAI_LE_LETTER_FA: char = 'ᥜ';
    /// \u{195d}: 'ᥝ'
    pub const TAI_LE_LETTER_VA: char = 'ᥝ';
    /// \u{195e}: 'ᥞ'
    pub const TAI_LE_LETTER_HA: char = 'ᥞ';
    /// \u{195f}: 'ᥟ'
    pub const TAI_LE_LETTER_QA: char = 'ᥟ';
    /// \u{1960}: 'ᥠ'
    pub const TAI_LE_LETTER_KHA: char = 'ᥠ';
    /// \u{1961}: 'ᥡ'
    pub const TAI_LE_LETTER_TSHA: char = 'ᥡ';
    /// \u{1962}: 'ᥢ'
    pub const TAI_LE_LETTER_NA: char = 'ᥢ';
    /// \u{1963}: 'ᥣ'
    pub const TAI_LE_LETTER_A: char = 'ᥣ';
    /// \u{1964}: 'ᥤ'
    pub const TAI_LE_LETTER_I: char = 'ᥤ';
    /// \u{1965}: 'ᥥ'
    pub const TAI_LE_LETTER_EE: char = 'ᥥ';
    /// \u{1966}: 'ᥦ'
    pub const TAI_LE_LETTER_EH: char = 'ᥦ';
    /// \u{1967}: 'ᥧ'
    pub const TAI_LE_LETTER_U: char = 'ᥧ';
    /// \u{1968}: 'ᥨ'
    pub const TAI_LE_LETTER_OO: char = 'ᥨ';
    /// \u{1969}: 'ᥩ'
    pub const TAI_LE_LETTER_O: char = 'ᥩ';
    /// \u{196a}: 'ᥪ'
    pub const TAI_LE_LETTER_UE: char = 'ᥪ';
    /// \u{196b}: 'ᥫ'
    pub const TAI_LE_LETTER_E: char = 'ᥫ';
    /// \u{196c}: 'ᥬ'
    pub const TAI_LE_LETTER_AUE: char = 'ᥬ';
    /// \u{196d}: 'ᥭ'
    pub const TAI_LE_LETTER_AI: char = 'ᥭ';
    /// \u{1970}: 'ᥰ'
    pub const TAI_LE_LETTER_TONE_DASH_2: char = 'ᥰ';
    /// \u{1971}: 'ᥱ'
    pub const TAI_LE_LETTER_TONE_DASH_3: char = 'ᥱ';
    /// \u{1972}: 'ᥲ'
    pub const TAI_LE_LETTER_TONE_DASH_4: char = 'ᥲ';
    /// \u{1973}: 'ᥳ'
    pub const TAI_LE_LETTER_TONE_DASH_5: char = 'ᥳ';
    /// \u{1974}: 'ᥴ'
    pub const TAI_LE_LETTER_TONE_DASH_6: char = 'ᥴ';
}

/// \u{1950} → \u{197f}\
///\
/// ᥐ ᥑ ᥒ ᥓ ᥔ ᥕ ᥖ ᥗ ᥘ ᥙ ᥚ ᥛ ᥜ ᥝ ᥞ ᥟ
/// ᥠ ᥡ ᥢ ᥣ ᥤ ᥥ ᥦ ᥧ ᥨ ᥩ ᥪ ᥫ ᥬ ᥭ ᥰ ᥱ
/// ᥲ ᥳ ᥴ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaiLe {
    /// \u{1950}: 'ᥐ'
    TaiLeLetterKa,
    /// \u{1951}: 'ᥑ'
    TaiLeLetterXa,
    /// \u{1952}: 'ᥒ'
    TaiLeLetterNga,
    /// \u{1953}: 'ᥓ'
    TaiLeLetterTsa,
    /// \u{1954}: 'ᥔ'
    TaiLeLetterSa,
    /// \u{1955}: 'ᥕ'
    TaiLeLetterYa,
    /// \u{1956}: 'ᥖ'
    TaiLeLetterTa,
    /// \u{1957}: 'ᥗ'
    TaiLeLetterTha,
    /// \u{1958}: 'ᥘ'
    TaiLeLetterLa,
    /// \u{1959}: 'ᥙ'
    TaiLeLetterPa,
    /// \u{195a}: 'ᥚ'
    TaiLeLetterPha,
    /// \u{195b}: 'ᥛ'
    TaiLeLetterMa,
    /// \u{195c}: 'ᥜ'
    TaiLeLetterFa,
    /// \u{195d}: 'ᥝ'
    TaiLeLetterVa,
    /// \u{195e}: 'ᥞ'
    TaiLeLetterHa,
    /// \u{195f}: 'ᥟ'
    TaiLeLetterQa,
    /// \u{1960}: 'ᥠ'
    TaiLeLetterKha,
    /// \u{1961}: 'ᥡ'
    TaiLeLetterTsha,
    /// \u{1962}: 'ᥢ'
    TaiLeLetterNa,
    /// \u{1963}: 'ᥣ'
    TaiLeLetterA,
    /// \u{1964}: 'ᥤ'
    TaiLeLetterI,
    /// \u{1965}: 'ᥥ'
    TaiLeLetterEe,
    /// \u{1966}: 'ᥦ'
    TaiLeLetterEh,
    /// \u{1967}: 'ᥧ'
    TaiLeLetterU,
    /// \u{1968}: 'ᥨ'
    TaiLeLetterOo,
    /// \u{1969}: 'ᥩ'
    TaiLeLetterO,
    /// \u{196a}: 'ᥪ'
    TaiLeLetterUe,
    /// \u{196b}: 'ᥫ'
    TaiLeLetterE,
    /// \u{196c}: 'ᥬ'
    TaiLeLetterAue,
    /// \u{196d}: 'ᥭ'
    TaiLeLetterAi,
    /// \u{1970}: 'ᥰ'
    TaiLeLetterToneDash2,
    /// \u{1971}: 'ᥱ'
    TaiLeLetterToneDash3,
    /// \u{1972}: 'ᥲ'
    TaiLeLetterToneDash4,
    /// \u{1973}: 'ᥳ'
    TaiLeLetterToneDash5,
    /// \u{1974}: 'ᥴ'
    TaiLeLetterToneDash6,
}

impl Into<char> for TaiLe {
    fn into(self) -> char {
        use constants::*;
        match self {
            TaiLe::TaiLeLetterKa => TAI_LE_LETTER_KA,
            TaiLe::TaiLeLetterXa => TAI_LE_LETTER_XA,
            TaiLe::TaiLeLetterNga => TAI_LE_LETTER_NGA,
            TaiLe::TaiLeLetterTsa => TAI_LE_LETTER_TSA,
            TaiLe::TaiLeLetterSa => TAI_LE_LETTER_SA,
            TaiLe::TaiLeLetterYa => TAI_LE_LETTER_YA,
            TaiLe::TaiLeLetterTa => TAI_LE_LETTER_TA,
            TaiLe::TaiLeLetterTha => TAI_LE_LETTER_THA,
            TaiLe::TaiLeLetterLa => TAI_LE_LETTER_LA,
            TaiLe::TaiLeLetterPa => TAI_LE_LETTER_PA,
            TaiLe::TaiLeLetterPha => TAI_LE_LETTER_PHA,
            TaiLe::TaiLeLetterMa => TAI_LE_LETTER_MA,
            TaiLe::TaiLeLetterFa => TAI_LE_LETTER_FA,
            TaiLe::TaiLeLetterVa => TAI_LE_LETTER_VA,
            TaiLe::TaiLeLetterHa => TAI_LE_LETTER_HA,
            TaiLe::TaiLeLetterQa => TAI_LE_LETTER_QA,
            TaiLe::TaiLeLetterKha => TAI_LE_LETTER_KHA,
            TaiLe::TaiLeLetterTsha => TAI_LE_LETTER_TSHA,
            TaiLe::TaiLeLetterNa => TAI_LE_LETTER_NA,
            TaiLe::TaiLeLetterA => TAI_LE_LETTER_A,
            TaiLe::TaiLeLetterI => TAI_LE_LETTER_I,
            TaiLe::TaiLeLetterEe => TAI_LE_LETTER_EE,
            TaiLe::TaiLeLetterEh => TAI_LE_LETTER_EH,
            TaiLe::TaiLeLetterU => TAI_LE_LETTER_U,
            TaiLe::TaiLeLetterOo => TAI_LE_LETTER_OO,
            TaiLe::TaiLeLetterO => TAI_LE_LETTER_O,
            TaiLe::TaiLeLetterUe => TAI_LE_LETTER_UE,
            TaiLe::TaiLeLetterE => TAI_LE_LETTER_E,
            TaiLe::TaiLeLetterAue => TAI_LE_LETTER_AUE,
            TaiLe::TaiLeLetterAi => TAI_LE_LETTER_AI,
            TaiLe::TaiLeLetterToneDash2 => TAI_LE_LETTER_TONE_DASH_2,
            TaiLe::TaiLeLetterToneDash3 => TAI_LE_LETTER_TONE_DASH_3,
            TaiLe::TaiLeLetterToneDash4 => TAI_LE_LETTER_TONE_DASH_4,
            TaiLe::TaiLeLetterToneDash5 => TAI_LE_LETTER_TONE_DASH_5,
            TaiLe::TaiLeLetterToneDash6 => TAI_LE_LETTER_TONE_DASH_6,
        }
    }
}

impl std::convert::TryFrom<char> for TaiLe {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TAI_LE_LETTER_KA => Ok(TaiLe::TaiLeLetterKa),
            TAI_LE_LETTER_XA => Ok(TaiLe::TaiLeLetterXa),
            TAI_LE_LETTER_NGA => Ok(TaiLe::TaiLeLetterNga),
            TAI_LE_LETTER_TSA => Ok(TaiLe::TaiLeLetterTsa),
            TAI_LE_LETTER_SA => Ok(TaiLe::TaiLeLetterSa),
            TAI_LE_LETTER_YA => Ok(TaiLe::TaiLeLetterYa),
            TAI_LE_LETTER_TA => Ok(TaiLe::TaiLeLetterTa),
            TAI_LE_LETTER_THA => Ok(TaiLe::TaiLeLetterTha),
            TAI_LE_LETTER_LA => Ok(TaiLe::TaiLeLetterLa),
            TAI_LE_LETTER_PA => Ok(TaiLe::TaiLeLetterPa),
            TAI_LE_LETTER_PHA => Ok(TaiLe::TaiLeLetterPha),
            TAI_LE_LETTER_MA => Ok(TaiLe::TaiLeLetterMa),
            TAI_LE_LETTER_FA => Ok(TaiLe::TaiLeLetterFa),
            TAI_LE_LETTER_VA => Ok(TaiLe::TaiLeLetterVa),
            TAI_LE_LETTER_HA => Ok(TaiLe::TaiLeLetterHa),
            TAI_LE_LETTER_QA => Ok(TaiLe::TaiLeLetterQa),
            TAI_LE_LETTER_KHA => Ok(TaiLe::TaiLeLetterKha),
            TAI_LE_LETTER_TSHA => Ok(TaiLe::TaiLeLetterTsha),
            TAI_LE_LETTER_NA => Ok(TaiLe::TaiLeLetterNa),
            TAI_LE_LETTER_A => Ok(TaiLe::TaiLeLetterA),
            TAI_LE_LETTER_I => Ok(TaiLe::TaiLeLetterI),
            TAI_LE_LETTER_EE => Ok(TaiLe::TaiLeLetterEe),
            TAI_LE_LETTER_EH => Ok(TaiLe::TaiLeLetterEh),
            TAI_LE_LETTER_U => Ok(TaiLe::TaiLeLetterU),
            TAI_LE_LETTER_OO => Ok(TaiLe::TaiLeLetterOo),
            TAI_LE_LETTER_O => Ok(TaiLe::TaiLeLetterO),
            TAI_LE_LETTER_UE => Ok(TaiLe::TaiLeLetterUe),
            TAI_LE_LETTER_E => Ok(TaiLe::TaiLeLetterE),
            TAI_LE_LETTER_AUE => Ok(TaiLe::TaiLeLetterAue),
            TAI_LE_LETTER_AI => Ok(TaiLe::TaiLeLetterAi),
            TAI_LE_LETTER_TONE_DASH_2 => Ok(TaiLe::TaiLeLetterToneDash2),
            TAI_LE_LETTER_TONE_DASH_3 => Ok(TaiLe::TaiLeLetterToneDash3),
            TAI_LE_LETTER_TONE_DASH_4 => Ok(TaiLe::TaiLeLetterToneDash4),
            TAI_LE_LETTER_TONE_DASH_5 => Ok(TaiLe::TaiLeLetterToneDash5),
            TAI_LE_LETTER_TONE_DASH_6 => Ok(TaiLe::TaiLeLetterToneDash6),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        TaiLe::TaiLeLetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            TaiLe::TaiLeLetterKa => "tai le letter ka",
            TaiLe::TaiLeLetterXa => "tai le letter xa",
            TaiLe::TaiLeLetterNga => "tai le letter nga",
            TaiLe::TaiLeLetterTsa => "tai le letter tsa",
            TaiLe::TaiLeLetterSa => "tai le letter sa",
            TaiLe::TaiLeLetterYa => "tai le letter ya",
            TaiLe::TaiLeLetterTa => "tai le letter ta",
            TaiLe::TaiLeLetterTha => "tai le letter tha",
            TaiLe::TaiLeLetterLa => "tai le letter la",
            TaiLe::TaiLeLetterPa => "tai le letter pa",
            TaiLe::TaiLeLetterPha => "tai le letter pha",
            TaiLe::TaiLeLetterMa => "tai le letter ma",
            TaiLe::TaiLeLetterFa => "tai le letter fa",
            TaiLe::TaiLeLetterVa => "tai le letter va",
            TaiLe::TaiLeLetterHa => "tai le letter ha",
            TaiLe::TaiLeLetterQa => "tai le letter qa",
            TaiLe::TaiLeLetterKha => "tai le letter kha",
            TaiLe::TaiLeLetterTsha => "tai le letter tsha",
            TaiLe::TaiLeLetterNa => "tai le letter na",
            TaiLe::TaiLeLetterA => "tai le letter a",
            TaiLe::TaiLeLetterI => "tai le letter i",
            TaiLe::TaiLeLetterEe => "tai le letter ee",
            TaiLe::TaiLeLetterEh => "tai le letter eh",
            TaiLe::TaiLeLetterU => "tai le letter u",
            TaiLe::TaiLeLetterOo => "tai le letter oo",
            TaiLe::TaiLeLetterO => "tai le letter o",
            TaiLe::TaiLeLetterUe => "tai le letter ue",
            TaiLe::TaiLeLetterE => "tai le letter e",
            TaiLe::TaiLeLetterAue => "tai le letter aue",
            TaiLe::TaiLeLetterAi => "tai le letter ai",
            TaiLe::TaiLeLetterToneDash2 => "tai le letter tone-2",
            TaiLe::TaiLeLetterToneDash3 => "tai le letter tone-3",
            TaiLe::TaiLeLetterToneDash4 => "tai le letter tone-4",
            TaiLe::TaiLeLetterToneDash5 => "tai le letter tone-5",
            TaiLe::TaiLeLetterToneDash6 => "tai le letter tone-6",
        }
    }
}
