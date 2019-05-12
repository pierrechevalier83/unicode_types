/// \u{1a00} → \u{1a1f}\
///\
/// ᨀ ᨁ ᨂ ᨃ ᨄ ᨅ ᨆ ᨇ ᨈ ᨉ ᨊ ᨋ ᨌ ᨍ ᨎ ᨏ
/// ᨐ ᨑ ᨒ ᨓ ᨔ ᨕ ᨖ ᨗ ᨘ ᨙ ᨚ ᨛ ᨞
pub mod constants {
    /// \u{1a00}: 'ᨀ'
    pub const BUGINESE_LETTER_KA: char = 'ᨀ';
    /// \u{1a01}: 'ᨁ'
    pub const BUGINESE_LETTER_GA: char = 'ᨁ';
    /// \u{1a02}: 'ᨂ'
    pub const BUGINESE_LETTER_NGA: char = 'ᨂ';
    /// \u{1a03}: 'ᨃ'
    pub const BUGINESE_LETTER_NGKA: char = 'ᨃ';
    /// \u{1a04}: 'ᨄ'
    pub const BUGINESE_LETTER_PA: char = 'ᨄ';
    /// \u{1a05}: 'ᨅ'
    pub const BUGINESE_LETTER_BA: char = 'ᨅ';
    /// \u{1a06}: 'ᨆ'
    pub const BUGINESE_LETTER_MA: char = 'ᨆ';
    /// \u{1a07}: 'ᨇ'
    pub const BUGINESE_LETTER_MPA: char = 'ᨇ';
    /// \u{1a08}: 'ᨈ'
    pub const BUGINESE_LETTER_TA: char = 'ᨈ';
    /// \u{1a09}: 'ᨉ'
    pub const BUGINESE_LETTER_DA: char = 'ᨉ';
    /// \u{1a0a}: 'ᨊ'
    pub const BUGINESE_LETTER_NA: char = 'ᨊ';
    /// \u{1a0b}: 'ᨋ'
    pub const BUGINESE_LETTER_NRA: char = 'ᨋ';
    /// \u{1a0c}: 'ᨌ'
    pub const BUGINESE_LETTER_CA: char = 'ᨌ';
    /// \u{1a0d}: 'ᨍ'
    pub const BUGINESE_LETTER_JA: char = 'ᨍ';
    /// \u{1a0e}: 'ᨎ'
    pub const BUGINESE_LETTER_NYA: char = 'ᨎ';
    /// \u{1a0f}: 'ᨏ'
    pub const BUGINESE_LETTER_NYCA: char = 'ᨏ';
    /// \u{1a10}: 'ᨐ'
    pub const BUGINESE_LETTER_YA: char = 'ᨐ';
    /// \u{1a11}: 'ᨑ'
    pub const BUGINESE_LETTER_RA: char = 'ᨑ';
    /// \u{1a12}: 'ᨒ'
    pub const BUGINESE_LETTER_LA: char = 'ᨒ';
    /// \u{1a13}: 'ᨓ'
    pub const BUGINESE_LETTER_VA: char = 'ᨓ';
    /// \u{1a14}: 'ᨔ'
    pub const BUGINESE_LETTER_SA: char = 'ᨔ';
    /// \u{1a15}: 'ᨕ'
    pub const BUGINESE_LETTER_A: char = 'ᨕ';
    /// \u{1a16}: 'ᨖ'
    pub const BUGINESE_LETTER_HA: char = 'ᨖ';
    /// \u{1a17}: 'ᨗ'
    pub const BUGINESE_VOWEL_SIGN_I: char = 'ᨗ';
    /// \u{1a18}: 'ᨘ'
    pub const BUGINESE_VOWEL_SIGN_U: char = 'ᨘ';
    /// \u{1a19}: 'ᨙ'
    pub const BUGINESE_VOWEL_SIGN_E: char = 'ᨙ';
    /// \u{1a1a}: 'ᨚ'
    pub const BUGINESE_VOWEL_SIGN_O: char = 'ᨚ';
    /// \u{1a1b}: 'ᨛ'
    pub const BUGINESE_VOWEL_SIGN_AE: char = 'ᨛ';
    /// \u{1a1e}: '᨞'
    pub const BUGINESE_PALLAWA: char = '᨞';
}

/// \u{1a00} → \u{1a1f}\
///\
/// ᨀ ᨁ ᨂ ᨃ ᨄ ᨅ ᨆ ᨇ ᨈ ᨉ ᨊ ᨋ ᨌ ᨍ ᨎ ᨏ
/// ᨐ ᨑ ᨒ ᨓ ᨔ ᨕ ᨖ ᨗ ᨘ ᨙ ᨚ ᨛ ᨞
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Buginese {
    /// \u{1a00}: 'ᨀ'
    BugineseLetterKa,
    /// \u{1a01}: 'ᨁ'
    BugineseLetterGa,
    /// \u{1a02}: 'ᨂ'
    BugineseLetterNga,
    /// \u{1a03}: 'ᨃ'
    BugineseLetterNgka,
    /// \u{1a04}: 'ᨄ'
    BugineseLetterPa,
    /// \u{1a05}: 'ᨅ'
    BugineseLetterBa,
    /// \u{1a06}: 'ᨆ'
    BugineseLetterMa,
    /// \u{1a07}: 'ᨇ'
    BugineseLetterMpa,
    /// \u{1a08}: 'ᨈ'
    BugineseLetterTa,
    /// \u{1a09}: 'ᨉ'
    BugineseLetterDa,
    /// \u{1a0a}: 'ᨊ'
    BugineseLetterNa,
    /// \u{1a0b}: 'ᨋ'
    BugineseLetterNra,
    /// \u{1a0c}: 'ᨌ'
    BugineseLetterCa,
    /// \u{1a0d}: 'ᨍ'
    BugineseLetterJa,
    /// \u{1a0e}: 'ᨎ'
    BugineseLetterNya,
    /// \u{1a0f}: 'ᨏ'
    BugineseLetterNyca,
    /// \u{1a10}: 'ᨐ'
    BugineseLetterYa,
    /// \u{1a11}: 'ᨑ'
    BugineseLetterRa,
    /// \u{1a12}: 'ᨒ'
    BugineseLetterLa,
    /// \u{1a13}: 'ᨓ'
    BugineseLetterVa,
    /// \u{1a14}: 'ᨔ'
    BugineseLetterSa,
    /// \u{1a15}: 'ᨕ'
    BugineseLetterA,
    /// \u{1a16}: 'ᨖ'
    BugineseLetterHa,
    /// \u{1a17}: 'ᨗ'
    BugineseVowelSignI,
    /// \u{1a18}: 'ᨘ'
    BugineseVowelSignU,
    /// \u{1a19}: 'ᨙ'
    BugineseVowelSignE,
    /// \u{1a1a}: 'ᨚ'
    BugineseVowelSignO,
    /// \u{1a1b}: 'ᨛ'
    BugineseVowelSignAe,
    /// \u{1a1e}: '᨞'
    BuginesePallawa,
}

impl Into<char> for Buginese {
    fn into(self) -> char {
        use constants::*;
        match self {
            Buginese::BugineseLetterKa => BUGINESE_LETTER_KA,
            Buginese::BugineseLetterGa => BUGINESE_LETTER_GA,
            Buginese::BugineseLetterNga => BUGINESE_LETTER_NGA,
            Buginese::BugineseLetterNgka => BUGINESE_LETTER_NGKA,
            Buginese::BugineseLetterPa => BUGINESE_LETTER_PA,
            Buginese::BugineseLetterBa => BUGINESE_LETTER_BA,
            Buginese::BugineseLetterMa => BUGINESE_LETTER_MA,
            Buginese::BugineseLetterMpa => BUGINESE_LETTER_MPA,
            Buginese::BugineseLetterTa => BUGINESE_LETTER_TA,
            Buginese::BugineseLetterDa => BUGINESE_LETTER_DA,
            Buginese::BugineseLetterNa => BUGINESE_LETTER_NA,
            Buginese::BugineseLetterNra => BUGINESE_LETTER_NRA,
            Buginese::BugineseLetterCa => BUGINESE_LETTER_CA,
            Buginese::BugineseLetterJa => BUGINESE_LETTER_JA,
            Buginese::BugineseLetterNya => BUGINESE_LETTER_NYA,
            Buginese::BugineseLetterNyca => BUGINESE_LETTER_NYCA,
            Buginese::BugineseLetterYa => BUGINESE_LETTER_YA,
            Buginese::BugineseLetterRa => BUGINESE_LETTER_RA,
            Buginese::BugineseLetterLa => BUGINESE_LETTER_LA,
            Buginese::BugineseLetterVa => BUGINESE_LETTER_VA,
            Buginese::BugineseLetterSa => BUGINESE_LETTER_SA,
            Buginese::BugineseLetterA => BUGINESE_LETTER_A,
            Buginese::BugineseLetterHa => BUGINESE_LETTER_HA,
            Buginese::BugineseVowelSignI => BUGINESE_VOWEL_SIGN_I,
            Buginese::BugineseVowelSignU => BUGINESE_VOWEL_SIGN_U,
            Buginese::BugineseVowelSignE => BUGINESE_VOWEL_SIGN_E,
            Buginese::BugineseVowelSignO => BUGINESE_VOWEL_SIGN_O,
            Buginese::BugineseVowelSignAe => BUGINESE_VOWEL_SIGN_AE,
            Buginese::BuginesePallawa => BUGINESE_PALLAWA,
        }
    }
}

impl std::convert::TryFrom<char> for Buginese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BUGINESE_LETTER_KA => Ok(Buginese::BugineseLetterKa),
            BUGINESE_LETTER_GA => Ok(Buginese::BugineseLetterGa),
            BUGINESE_LETTER_NGA => Ok(Buginese::BugineseLetterNga),
            BUGINESE_LETTER_NGKA => Ok(Buginese::BugineseLetterNgka),
            BUGINESE_LETTER_PA => Ok(Buginese::BugineseLetterPa),
            BUGINESE_LETTER_BA => Ok(Buginese::BugineseLetterBa),
            BUGINESE_LETTER_MA => Ok(Buginese::BugineseLetterMa),
            BUGINESE_LETTER_MPA => Ok(Buginese::BugineseLetterMpa),
            BUGINESE_LETTER_TA => Ok(Buginese::BugineseLetterTa),
            BUGINESE_LETTER_DA => Ok(Buginese::BugineseLetterDa),
            BUGINESE_LETTER_NA => Ok(Buginese::BugineseLetterNa),
            BUGINESE_LETTER_NRA => Ok(Buginese::BugineseLetterNra),
            BUGINESE_LETTER_CA => Ok(Buginese::BugineseLetterCa),
            BUGINESE_LETTER_JA => Ok(Buginese::BugineseLetterJa),
            BUGINESE_LETTER_NYA => Ok(Buginese::BugineseLetterNya),
            BUGINESE_LETTER_NYCA => Ok(Buginese::BugineseLetterNyca),
            BUGINESE_LETTER_YA => Ok(Buginese::BugineseLetterYa),
            BUGINESE_LETTER_RA => Ok(Buginese::BugineseLetterRa),
            BUGINESE_LETTER_LA => Ok(Buginese::BugineseLetterLa),
            BUGINESE_LETTER_VA => Ok(Buginese::BugineseLetterVa),
            BUGINESE_LETTER_SA => Ok(Buginese::BugineseLetterSa),
            BUGINESE_LETTER_A => Ok(Buginese::BugineseLetterA),
            BUGINESE_LETTER_HA => Ok(Buginese::BugineseLetterHa),
            BUGINESE_VOWEL_SIGN_I => Ok(Buginese::BugineseVowelSignI),
            BUGINESE_VOWEL_SIGN_U => Ok(Buginese::BugineseVowelSignU),
            BUGINESE_VOWEL_SIGN_E => Ok(Buginese::BugineseVowelSignE),
            BUGINESE_VOWEL_SIGN_O => Ok(Buginese::BugineseVowelSignO),
            BUGINESE_VOWEL_SIGN_AE => Ok(Buginese::BugineseVowelSignAe),
            BUGINESE_PALLAWA => Ok(Buginese::BuginesePallawa),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Buginese::BugineseLetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Buginese::BugineseLetterKa => "buginese letter ka",
            Buginese::BugineseLetterGa => "buginese letter ga",
            Buginese::BugineseLetterNga => "buginese letter nga",
            Buginese::BugineseLetterNgka => "buginese letter ngka",
            Buginese::BugineseLetterPa => "buginese letter pa",
            Buginese::BugineseLetterBa => "buginese letter ba",
            Buginese::BugineseLetterMa => "buginese letter ma",
            Buginese::BugineseLetterMpa => "buginese letter mpa",
            Buginese::BugineseLetterTa => "buginese letter ta",
            Buginese::BugineseLetterDa => "buginese letter da",
            Buginese::BugineseLetterNa => "buginese letter na",
            Buginese::BugineseLetterNra => "buginese letter nra",
            Buginese::BugineseLetterCa => "buginese letter ca",
            Buginese::BugineseLetterJa => "buginese letter ja",
            Buginese::BugineseLetterNya => "buginese letter nya",
            Buginese::BugineseLetterNyca => "buginese letter nyca",
            Buginese::BugineseLetterYa => "buginese letter ya",
            Buginese::BugineseLetterRa => "buginese letter ra",
            Buginese::BugineseLetterLa => "buginese letter la",
            Buginese::BugineseLetterVa => "buginese letter va",
            Buginese::BugineseLetterSa => "buginese letter sa",
            Buginese::BugineseLetterA => "buginese letter a",
            Buginese::BugineseLetterHa => "buginese letter ha",
            Buginese::BugineseVowelSignI => "buginese vowel sign i",
            Buginese::BugineseVowelSignU => "buginese vowel sign u",
            Buginese::BugineseVowelSignE => "buginese vowel sign e",
            Buginese::BugineseVowelSignO => "buginese vowel sign o",
            Buginese::BugineseVowelSignAe => "buginese vowel sign ae",
            Buginese::BuginesePallawa => "buginese pallawa",
        }
    }
}
