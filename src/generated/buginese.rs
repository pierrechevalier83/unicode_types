/// \u{1a00} → \u{1a1f}
///
/// ᨀ ᨁ ᨂ ᨃ ᨄ ᨅ ᨆ ᨇ ᨈ ᨉ ᨊ ᨋ ᨌ ᨍ ᨎ ᨏ\
/// ᨐ ᨑ ᨒ ᨓ ᨔ ᨕ ᨖ ᨗ ᨘ ᨙ ᨚ ᨛ ᨞\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1a00}: 'ᨀ'
    pub const LETTER_KA: char = 'ᨀ';
    /// \u{1a01}: 'ᨁ'
    pub const LETTER_GA: char = 'ᨁ';
    /// \u{1a02}: 'ᨂ'
    pub const LETTER_NGA: char = 'ᨂ';
    /// \u{1a03}: 'ᨃ'
    pub const LETTER_NGKA: char = 'ᨃ';
    /// \u{1a04}: 'ᨄ'
    pub const LETTER_PA: char = 'ᨄ';
    /// \u{1a05}: 'ᨅ'
    pub const LETTER_BA: char = 'ᨅ';
    /// \u{1a06}: 'ᨆ'
    pub const LETTER_MA: char = 'ᨆ';
    /// \u{1a07}: 'ᨇ'
    pub const LETTER_MPA: char = 'ᨇ';
    /// \u{1a08}: 'ᨈ'
    pub const LETTER_TA: char = 'ᨈ';
    /// \u{1a09}: 'ᨉ'
    pub const LETTER_DA: char = 'ᨉ';
    /// \u{1a0a}: 'ᨊ'
    pub const LETTER_NA: char = 'ᨊ';
    /// \u{1a0b}: 'ᨋ'
    pub const LETTER_NRA: char = 'ᨋ';
    /// \u{1a0c}: 'ᨌ'
    pub const LETTER_CA: char = 'ᨌ';
    /// \u{1a0d}: 'ᨍ'
    pub const LETTER_JA: char = 'ᨍ';
    /// \u{1a0e}: 'ᨎ'
    pub const LETTER_NYA: char = 'ᨎ';
    /// \u{1a0f}: 'ᨏ'
    pub const LETTER_NYCA: char = 'ᨏ';
    /// \u{1a10}: 'ᨐ'
    pub const LETTER_YA: char = 'ᨐ';
    /// \u{1a11}: 'ᨑ'
    pub const LETTER_RA: char = 'ᨑ';
    /// \u{1a12}: 'ᨒ'
    pub const LETTER_LA: char = 'ᨒ';
    /// \u{1a13}: 'ᨓ'
    pub const LETTER_VA: char = 'ᨓ';
    /// \u{1a14}: 'ᨔ'
    pub const LETTER_SA: char = 'ᨔ';
    /// \u{1a15}: 'ᨕ'
    pub const LETTER_A: char = 'ᨕ';
    /// \u{1a16}: 'ᨖ'
    pub const LETTER_HA: char = 'ᨖ';
    /// \u{1a17}: 'ᨗ'
    pub const VOWEL_SIGN_I: char = 'ᨗ';
    /// \u{1a18}: 'ᨘ'
    pub const VOWEL_SIGN_U: char = 'ᨘ';
    /// \u{1a19}: 'ᨙ'
    pub const VOWEL_SIGN_E: char = 'ᨙ';
    /// \u{1a1a}: 'ᨚ'
    pub const VOWEL_SIGN_O: char = 'ᨚ';
    /// \u{1a1b}: 'ᨛ'
    pub const VOWEL_SIGN_AE: char = 'ᨛ';
    /// \u{1a1e}: '᨞'
    pub const PALLAWA: char = '᨞';
}

/// An enum to represent all characters in the Buginese block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Buginese {
    /// \u{1a00}: 'ᨀ'
    LetterKa,
    /// \u{1a01}: 'ᨁ'
    LetterGa,
    /// \u{1a02}: 'ᨂ'
    LetterNga,
    /// \u{1a03}: 'ᨃ'
    LetterNgka,
    /// \u{1a04}: 'ᨄ'
    LetterPa,
    /// \u{1a05}: 'ᨅ'
    LetterBa,
    /// \u{1a06}: 'ᨆ'
    LetterMa,
    /// \u{1a07}: 'ᨇ'
    LetterMpa,
    /// \u{1a08}: 'ᨈ'
    LetterTa,
    /// \u{1a09}: 'ᨉ'
    LetterDa,
    /// \u{1a0a}: 'ᨊ'
    LetterNa,
    /// \u{1a0b}: 'ᨋ'
    LetterNra,
    /// \u{1a0c}: 'ᨌ'
    LetterCa,
    /// \u{1a0d}: 'ᨍ'
    LetterJa,
    /// \u{1a0e}: 'ᨎ'
    LetterNya,
    /// \u{1a0f}: 'ᨏ'
    LetterNyca,
    /// \u{1a10}: 'ᨐ'
    LetterYa,
    /// \u{1a11}: 'ᨑ'
    LetterRa,
    /// \u{1a12}: 'ᨒ'
    LetterLa,
    /// \u{1a13}: 'ᨓ'
    LetterVa,
    /// \u{1a14}: 'ᨔ'
    LetterSa,
    /// \u{1a15}: 'ᨕ'
    LetterA,
    /// \u{1a16}: 'ᨖ'
    LetterHa,
    /// \u{1a17}: 'ᨗ'
    VowelSignI,
    /// \u{1a18}: 'ᨘ'
    VowelSignU,
    /// \u{1a19}: 'ᨙ'
    VowelSignE,
    /// \u{1a1a}: 'ᨚ'
    VowelSignO,
    /// \u{1a1b}: 'ᨛ'
    VowelSignAe,
    /// \u{1a1e}: '᨞'
    Pallawa,
}

impl Into<char> for Buginese {
    fn into(self) -> char {
        use constants::*;
        match self {
            Buginese::LetterKa => LETTER_KA,
            Buginese::LetterGa => LETTER_GA,
            Buginese::LetterNga => LETTER_NGA,
            Buginese::LetterNgka => LETTER_NGKA,
            Buginese::LetterPa => LETTER_PA,
            Buginese::LetterBa => LETTER_BA,
            Buginese::LetterMa => LETTER_MA,
            Buginese::LetterMpa => LETTER_MPA,
            Buginese::LetterTa => LETTER_TA,
            Buginese::LetterDa => LETTER_DA,
            Buginese::LetterNa => LETTER_NA,
            Buginese::LetterNra => LETTER_NRA,
            Buginese::LetterCa => LETTER_CA,
            Buginese::LetterJa => LETTER_JA,
            Buginese::LetterNya => LETTER_NYA,
            Buginese::LetterNyca => LETTER_NYCA,
            Buginese::LetterYa => LETTER_YA,
            Buginese::LetterRa => LETTER_RA,
            Buginese::LetterLa => LETTER_LA,
            Buginese::LetterVa => LETTER_VA,
            Buginese::LetterSa => LETTER_SA,
            Buginese::LetterA => LETTER_A,
            Buginese::LetterHa => LETTER_HA,
            Buginese::VowelSignI => VOWEL_SIGN_I,
            Buginese::VowelSignU => VOWEL_SIGN_U,
            Buginese::VowelSignE => VOWEL_SIGN_E,
            Buginese::VowelSignO => VOWEL_SIGN_O,
            Buginese::VowelSignAe => VOWEL_SIGN_AE,
            Buginese::Pallawa => PALLAWA,
        }
    }
}

impl std::convert::TryFrom<char> for Buginese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KA => Ok(Buginese::LetterKa),
            LETTER_GA => Ok(Buginese::LetterGa),
            LETTER_NGA => Ok(Buginese::LetterNga),
            LETTER_NGKA => Ok(Buginese::LetterNgka),
            LETTER_PA => Ok(Buginese::LetterPa),
            LETTER_BA => Ok(Buginese::LetterBa),
            LETTER_MA => Ok(Buginese::LetterMa),
            LETTER_MPA => Ok(Buginese::LetterMpa),
            LETTER_TA => Ok(Buginese::LetterTa),
            LETTER_DA => Ok(Buginese::LetterDa),
            LETTER_NA => Ok(Buginese::LetterNa),
            LETTER_NRA => Ok(Buginese::LetterNra),
            LETTER_CA => Ok(Buginese::LetterCa),
            LETTER_JA => Ok(Buginese::LetterJa),
            LETTER_NYA => Ok(Buginese::LetterNya),
            LETTER_NYCA => Ok(Buginese::LetterNyca),
            LETTER_YA => Ok(Buginese::LetterYa),
            LETTER_RA => Ok(Buginese::LetterRa),
            LETTER_LA => Ok(Buginese::LetterLa),
            LETTER_VA => Ok(Buginese::LetterVa),
            LETTER_SA => Ok(Buginese::LetterSa),
            LETTER_A => Ok(Buginese::LetterA),
            LETTER_HA => Ok(Buginese::LetterHa),
            VOWEL_SIGN_I => Ok(Buginese::VowelSignI),
            VOWEL_SIGN_U => Ok(Buginese::VowelSignU),
            VOWEL_SIGN_E => Ok(Buginese::VowelSignE),
            VOWEL_SIGN_O => Ok(Buginese::VowelSignO),
            VOWEL_SIGN_AE => Ok(Buginese::VowelSignAe),
            PALLAWA => Ok(Buginese::Pallawa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Buginese {
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

impl std::convert::TryFrom<u32> for Buginese {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Buginese {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Buginese {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Buginese::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Buginese::LetterKa => "buginese letter ka",
            Buginese::LetterGa => "buginese letter ga",
            Buginese::LetterNga => "buginese letter nga",
            Buginese::LetterNgka => "buginese letter ngka",
            Buginese::LetterPa => "buginese letter pa",
            Buginese::LetterBa => "buginese letter ba",
            Buginese::LetterMa => "buginese letter ma",
            Buginese::LetterMpa => "buginese letter mpa",
            Buginese::LetterTa => "buginese letter ta",
            Buginese::LetterDa => "buginese letter da",
            Buginese::LetterNa => "buginese letter na",
            Buginese::LetterNra => "buginese letter nra",
            Buginese::LetterCa => "buginese letter ca",
            Buginese::LetterJa => "buginese letter ja",
            Buginese::LetterNya => "buginese letter nya",
            Buginese::LetterNyca => "buginese letter nyca",
            Buginese::LetterYa => "buginese letter ya",
            Buginese::LetterRa => "buginese letter ra",
            Buginese::LetterLa => "buginese letter la",
            Buginese::LetterVa => "buginese letter va",
            Buginese::LetterSa => "buginese letter sa",
            Buginese::LetterA => "buginese letter a",
            Buginese::LetterHa => "buginese letter ha",
            Buginese::VowelSignI => "buginese vowel sign i",
            Buginese::VowelSignU => "buginese vowel sign u",
            Buginese::VowelSignE => "buginese vowel sign e",
            Buginese::VowelSignO => "buginese vowel sign o",
            Buginese::VowelSignAe => "buginese vowel sign ae",
            Buginese::Pallawa => "buginese pallawa",
        }
    }
}
